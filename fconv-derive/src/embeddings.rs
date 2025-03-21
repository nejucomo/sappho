use derive_more::{Constructor, From};
use syn::spanned::Spanned as _;

use crate::spannedext::SpannedExt as _;

use self::Embeddings::*;
use self::EmbeddingsIter::*;

#[derive(Debug, From)]
pub(crate) enum Embeddings {
    Struct(EmbeddingField),
    Enum(Vec<EmbeddingVariant>),
}

#[derive(Debug, From)]
pub(crate) enum EmbeddingsIter<'a> {
    StructIter(Option<&'a EmbeddingField>),
    EnumIter(std::slice::Iter<'a, EmbeddingVariant>),
}

#[derive(Debug, From, Constructor)]
pub(crate) struct EmbeddingVariant {
    pub(crate) variant: syn::Ident,
    pub(crate) field: EmbeddingField,
}

#[derive(Debug, From, Constructor)]
pub(crate) struct EmbeddingField {
    pub(crate) fspec: EmbeddingFieldSpec,
    pub(crate) ftype: syn::Type,
    pub(crate) fsubs: Subembeddings,
}

#[derive(Debug, Clone)]
pub(crate) enum EmbeddingFieldSpec {
    Indexed,
    Named(syn::Ident),
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Subembeddings(SubembeddingsInner);

type SubembeddingsInner = syn::punctuated::Punctuated<syn::Type, syn::Token![,]>;

impl Embeddings {
    pub(crate) fn is_struct(&self) -> bool {
        matches!(self, Struct(_))
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = (Option<&syn::Ident>, &EmbeddingField)> {
        match self {
            Struct(ef) => StructIter(Some(ef)),
            Enum(v) => EnumIter(v.iter()),
        }
    }
}

impl<'a> Iterator for EmbeddingsIter<'a> {
    type Item = (Option<&'a syn::Ident>, &'a EmbeddingField);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            StructIter(optef) => optef.take().map(|ef| (None, ef)),
            EnumIter(slit) => slit.next().map(|ev| (Some(&ev.variant), &ev.field)),
        }
    }
}

impl TryFrom<syn::Variant> for EmbeddingVariant {
    type Error = syn::Error;

    fn try_from(synvar: syn::Variant) -> syn::Result<Self> {
        let subs = Subembeddings::try_from(synvar.attrs)?;
        let efield = EmbeddingField::try_from((subs, synvar.fields))?;
        Ok(Self::new(synvar.ident, efield))
    }
}

impl TryFrom<(Subembeddings, syn::Fields)> for EmbeddingField {
    type Error = syn::Error;

    fn try_from((subs, fields): (Subembeddings, syn::Fields)) -> syn::Result<Self> {
        let span = fields.span();
        unwrap_single_item(fields)
            .map(|field| Self::from((subs, field)))
            .ok_or_else(|| span.error("expected at most one field"))
    }
}

impl From<(Subembeddings, syn::Field)> for EmbeddingField {
    fn from((subs, f): (Subembeddings, syn::Field)) -> Self {
        let syn::Field {
            ident: fspec,
            ty: ftype,
            ..
        } = f;

        let fspec = EmbeddingFieldSpec::from(fspec);

        Self {
            fspec,
            ftype,
            fsubs: subs,
        }
    }
}

impl From<Option<syn::Ident>> for EmbeddingFieldSpec {
    fn from(optid: Option<syn::Ident>) -> Self {
        use EmbeddingFieldSpec::{Indexed, Named};

        optid.map(Named).unwrap_or(Indexed)
    }
}

impl Subembeddings {
    pub(crate) fn iter(&self) -> impl Iterator<Item = &syn::Type> {
        self.0.iter()
    }
}

impl TryFrom<Vec<syn::Attribute>> for Subembeddings {
    type Error = syn::Error;

    fn try_from(attrs: Vec<syn::Attribute>) -> syn::Result<Self> {
        attrs
            .into_iter()
            .map(OptSub::try_from)
            .filter_map(|res| res.map(|ose| ose.0).transpose())
            .try_fold(SubembeddingsInner::default(), |mut acc, subres| {
                let Subembeddings(inner) = subres?;
                acc.extend(inner);
                Ok(acc)
            })
            .map(Subembeddings)
    }
}

struct OptSub(Option<Subembeddings>);

impl TryFrom<syn::Attribute> for OptSub {
    type Error = syn::Error;

    fn try_from(attr: syn::Attribute) -> syn::Result<Self> {
        Self::try_from(attr.meta)
    }
}

impl TryFrom<syn::Meta> for OptSub {
    type Error = syn::Error;

    fn try_from(meta: syn::Meta) -> Result<Self, Self::Error> {
        use syn::Meta::*;

        match meta {
            Path(p) => extract_path(p)
                .map(|id| id.err("expected (ty1, ty2, ...)"))
                .transpose()
                .map(OptSub),
            List(x) => Self::try_from(x),
            NameValue(x) => Self::try_from(x),
        }
    }
}

impl TryFrom<syn::MetaList> for OptSub {
    type Error = syn::Error;

    fn try_from(ml: syn::MetaList) -> syn::Result<Self> {
        if extract_path(ml.path).is_some() {
            // Now attempt to parse the type:
            syn::parse2(ml.tokens).map(Some).map(OptSub)
        } else {
            Ok(OptSub(None))
        }
    }
}

impl TryFrom<syn::MetaNameValue> for OptSub {
    type Error = syn::Error;

    fn try_from(mnv: syn::MetaNameValue) -> Result<Self, Self::Error> {
        if let Some(id) = extract_path(mnv.path) {
            id.err("expected (ty1, ty2, ...)")
        } else {
            Ok(OptSub(None))
        }
    }
}

impl syn::parse::Parse for Subembeddings {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // TODO: Does conversion of Punctuated to Vec introduce an excessive allocation/copy?

        let punc: SubembeddingsInner = input.parse_terminated(syn::Type::parse, syn::Token![,])?;

        Ok(Self(punc))
    }
}

fn unwrap_single_item<I>(it: I) -> Option<I::Item>
where
    I: IntoIterator,
{
    let mut it = it.into_iter();
    let x = it.next()?;
    if it.next().is_some() {
        None
    } else {
        Some(x)
    }
}

fn extract_path(path: syn::Path) -> Option<syn::Ident> {
    if path.leading_colon.is_some() {
        return None;
    }

    let seg = unwrap_single_item(path.segments)?;

    if !matches!(seg.arguments, syn::PathArguments::None) {
        return None;
    }

    let id = seg.ident;

    if id == "extract" {
        Some(id)
    } else {
        None
    }
}
