use derive_more::Constructor;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Ident;

use crate::embeddings::{EmbeddingField, EmbeddingVariant, Embeddings, Subembeddings};
use crate::intgen::IntermediateGenerics;
use crate::spannedext::SpannedExt;

#[derive(Debug, Constructor)]
pub(crate) struct Intermediate {
    pub(crate) container_type: Ident,
    pub(crate) generics: IntermediateGenerics,
    pub(crate) embeddings: Embeddings,
}

impl ToTokens for Intermediate {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        use crate::embeddings::EmbeddingFieldSpec::{Indexed, Named};

        let container_type = &self.container_type;
        let genparams = &self.generics.optparams;
        let optwhere = &self.generics.optwhere;

        let (varctrs, fspecs, ftypes): (Vec<_>, Vec<_>, Vec<_>) =
            unzip_twice(self.embeddings.iter().map(|(optvarid, emfield)| {
                (
                    optvarid
                        .map(|varid| quote! { #container_type :: #varid })
                        .unwrap_or_else(|| quote! { #container_type }),
                    match &emfield.fspec {
                        Indexed => quote! { (x) },
                        Named(ident) => quote! { { #ident : x } },
                    },
                    &emfield.ftype,
                )
            }));

        let catch_all_pat = if self.embeddings.is_struct() {
            quote! {}
        } else {
            quote! { other => Err(other) }
        };

        tokens.extend(std::iter::once(quote! {
            #(
                #[automatically_derived]
                impl #genparams ::sappho_fconv::Extract< #ftypes > for ( #container_type #genparams ) #optwhere {
                    fn extract(self) -> Result< #ftypes, Self > {
                        match self {
                            #varctrs #fspecs => Ok(x),
                            #catch_all_pat
                        }
                    }
                }

                #[automatically_derived]
                impl #genparams ::sappho_fconv::Embed< #ftypes > for ( #container_type #genparams ) #optwhere {
                    fn embed(x: #ftypes ) -> Self {
                        #varctrs #fspecs
                    }
                }
            )*
        }));
    }
}

impl TryFrom<TokenStream> for Intermediate {
    type Error = syn::Error;

    fn try_from(tokens: TokenStream) -> syn::Result<Self> {
        let item: syn::Item = syn::parse2(tokens)?;
        Self::try_from(item)
    }
}

impl TryFrom<syn::Item> for Intermediate {
    type Error = syn::Error;

    fn try_from(item: syn::Item) -> syn::Result<Self> {
        use syn::Item::{Enum, Struct};

        match item {
            Struct(x) => Self::try_from(x),
            Enum(x) => Self::try_from(x),
            _ => item.err("expected struct or enum"),
        }
    }
}

impl TryFrom<syn::ItemStruct> for Intermediate {
    type Error = syn::Error;

    fn try_from(item: syn::ItemStruct) -> syn::Result<Self> {
        let subs = Subembeddings::try_from(item.attrs)?;
        let efield = EmbeddingField::try_from((subs, item.fields))?;

        Ok(Self::new(
            item.ident,
            item.generics.try_into()?,
            efield.into(),
        ))
    }
}

impl TryFrom<syn::ItemEnum> for Intermediate {
    type Error = syn::Error;

    fn try_from(item: syn::ItemEnum) -> syn::Result<Self> {
        Ok(Self::new(
            item.ident,
            item.generics.try_into()?,
            item.variants
                .into_iter()
                .map(EmbeddingVariant::try_from)
                .collect::<syn::Result<Vec<_>>>()?
                .into(),
        ))
    }
}

fn unzip_twice<I, A, B, C>(it: I) -> (Vec<A>, Vec<B>, Vec<C>)
where
    I: Iterator<Item = (A, B, C)>,
{
    let (veca, vecbc): (Vec<A>, Vec<(B, C)>) = it.map(|(a, b, c)| (a, (b, c))).unzip();
    let (vecb, vecc): (Vec<B>, Vec<C>) = vecbc.into_iter().unzip();
    (veca, vecb, vecc)
}
