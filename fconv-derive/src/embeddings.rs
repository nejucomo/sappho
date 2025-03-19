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
}

#[derive(Debug, Clone)]
pub(crate) enum EmbeddingFieldSpec {
    Indexed,
    Named(syn::Ident),
}

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
        Ok(Self::new(synvar.ident, synvar.fields.try_into()?))
    }
}

impl TryFrom<syn::Fields> for EmbeddingField {
    type Error = syn::Error;

    fn try_from(fields: syn::Fields) -> syn::Result<Self> {
        let span = fields.span();
        let mut it = fields.into_iter();
        let field = it
            .next()
            .ok_or_else(|| span.clone().error("expected at least one field"))?;
        if it.next().is_some() {
            span.err("expected at most one field")
        } else {
            Ok(Self::from(field))
        }
    }
}

impl From<syn::Field> for EmbeddingField {
    fn from(f: syn::Field) -> Self {
        let syn::Field {
            ident: fspec,
            ty: ftype,
            ..
        } = f;

        let fspec = EmbeddingFieldSpec::from(fspec);

        Self { fspec, ftype }
    }
}

impl From<Option<syn::Ident>> for EmbeddingFieldSpec {
    fn from(optid: Option<syn::Ident>) -> Self {
        use EmbeddingFieldSpec::{Indexed, Named};

        optid.map(Named).unwrap_or(Indexed)
    }
}
