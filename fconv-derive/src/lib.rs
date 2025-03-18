use easy_ext::ext;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    parse2 as parse, Error, Field, Fields, GenericParam, Generics, Ident, Item, ItemEnum,
    ItemStruct, Token, Type, Variant, WhereClause,
};

use crate::FieldSpec::*;

#[proc_macro_derive(Extract)]
pub fn extract_derive(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    extract_derive_pm2(item.into()).into()
}

#[derive(Debug, Clone)]
enum FieldSpec {
    Indexed,
    Named(Ident),
}

fn extract_derive_pm2(item: TokenStream) -> TokenStream {
    extract_derive_res(item).unwrap_or_else(Error::into_compile_error)
}

fn extract_derive_res(item: TokenStream) -> syn::Result<TokenStream> {
    let item: Item = parse(item)?;
    item.generate_impls()
}

fn generate_impls<I>(tyid: Ident, generics: Generics, fieldses: I) -> syn::Result<TokenStream>
where
    I: IntoIterator<Item = (Option<Ident>, Fields)>,
{
    let (gparams, optwhere) = generics.into_gparams_and_where_clause()?;

    let mut variant_pat_or_ctrs = vec![];
    let mut field_pat_or_ctrs = vec![];
    let mut ftys = vec![];
    let mut is_enum = false;

    for (optvarid, fields) in fieldses {
        variant_pat_or_ctrs.push(
            optvarid
                .map(|varid| {
                    is_enum = true;
                    quote! { #tyid :: #varid }
                })
                .unwrap_or_else(|| quote! { #tyid }),
        );

        let (fspec, fty) = fields.try_into_field_translation_info()?;
        field_pat_or_ctrs.push(match fspec {
            Indexed => quote! {
                ( x )
            },
            Named(ident) => quote! {
                { #ident : x }
            },
        });

        ftys.push(fty);
    }

    let catch_all_pat = if is_enum {
        quote! { other => Err(other) }
    } else {
        quote! {}
    };

    Ok(quote! {
        #(
            #[automatically_derived]
            impl #gparams ::sappho_fconv::Extract< #ftys > for ( #tyid #gparams ) #optwhere {
                fn extract(self) -> Result< #ftys, Self > {
                    match self {
                        #variant_pat_or_ctrs #field_pat_or_ctrs => Ok(x),
                        #catch_all_pat
                    }
                }
            }

            #[automatically_derived]
            impl #gparams ::sappho_fconv::Embed< #ftys > for ( #tyid #gparams ) #optwhere {
                fn embed(x: #ftys ) -> Self {
                    #variant_pat_or_ctrs #field_pat_or_ctrs
                }
            }
        )*
    })
}

#[ext]
impl Item {
    fn generate_impls(self) -> syn::Result<TokenStream> {
        use Item::{Enum, Struct};

        match self {
            Struct(x) => x.generate_impls(),
            Enum(x) => x.generate_impls(),
            _ => Err(Error::new(self.span(), "expected struct or enum")),
        }
    }
}

#[ext]
impl ItemStruct {
    fn generate_impls(self) -> syn::Result<TokenStream> {
        generate_impls(
            self.ident,
            self.generics,
            std::iter::once((None, self.fields)),
        )
    }
}

#[ext]
impl ItemEnum {
    fn generate_impls(self) -> syn::Result<TokenStream> {
        generate_impls(
            self.ident,
            self.generics,
            self.variants
                .into_iter()
                .map(|Variant { ident, fields, .. }| (Some(ident), fields)),
        )
    }
}

struct GenericsParams {
    lt: Token![<],
    gps: Punctuated<GenericParam, Token![,]>,
    gt: Token![>],
}

impl ToTokens for GenericsParams {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.lt.to_tokens(tokens);
        self.gps.to_tokens(tokens);
        self.gt.to_tokens(tokens);
    }
}

#[ext]
impl Generics {
    fn into_gparams_and_where_clause(
        self,
    ) -> syn::Result<(Option<GenericsParams>, Option<WhereClause>)> {
        let err = Err(self.error("unexpected or inconsistent generics"));

        let Generics {
            lt_token,
            params,
            gt_token,
            where_clause,
        } = self;

        let optgtup = match (lt_token, params.is_empty(), gt_token) {
            (None, true, None) => Ok(None),
            (Some(lt), false, Some(gt)) => Ok(Some(GenericsParams {
                lt,
                gps: params,
                gt,
            })),
            _ => err,
        }?;

        Ok((optgtup, where_clause))
    }
}

#[ext]
impl Fields {
    fn try_into_field_translation_info(self) -> syn::Result<(FieldSpec, Type)> {
        let field = self.take_only_single_field()?;
        let fty = field.ty;
        let fspec = field.ident.map(Named).unwrap_or(Indexed);
        Ok((fspec, fty))
    }

    fn take_only_single_field(self) -> syn::Result<Field> {
        let error = self.error("only single field struct or variants are supported");
        let punc = self.into_punctuated()?;
        punc.take_only_singleton().ok_or(error)
    }

    fn into_punctuated(self) -> syn::Result<Punctuated<Field, Token![,]>> {
        use Fields::*;

        match self {
            Named(x) => Ok(x.named),
            Unnamed(x) => Ok(x.unnamed),
            Unit => Err(self.error("expected a single field, not unit")),
        }
    }
}

#[ext]
impl<T> T
where
    Self: IntoIterator,
{
    fn take_only_singleton(self) -> Option<Self::Item> {
        let mut it = self.into_iter();
        it.next().filter(|_| it.next().is_none())
    }
}

#[ext]
impl<T> T
where
    Self: Spanned,
{
    fn error<M>(&self, msg: M) -> Error
    where
        M: std::fmt::Display,
    {
        Error::new(self.span(), msg)
    }
}

#[cfg(test)]
mod tests;
