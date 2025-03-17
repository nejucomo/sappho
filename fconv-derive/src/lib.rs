use easy_ext::ext;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    parse2 as parse, Error, Field, Fields, GenericParam, Generics, Index, Item, ItemEnum,
    ItemStruct, Token, Type, Variant, WhereClause,
};

#[proc_macro_derive(Extract)]
pub fn extract_derive(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    extract_derive_pm2(item.into()).into()
}

fn extract_derive_pm2(item: TokenStream) -> TokenStream {
    extract_derive_res(item).unwrap_or_else(Error::into_compile_error)
}

fn extract_derive_res(item: TokenStream) -> syn::Result<TokenStream> {
    let item: Item = parse(item)?;
    item.generate_impls()
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
        let ItemStruct {
            ident,
            fields,
            generics,
            ..
        } = self;
        let (fid, fty) = fields.try_into_field_translation_info()?;
        let (gparams, optwhere) = generics.into_gparams_and_where_clause()?;

        Ok(quote! {
            #[automatically_derived]
            impl #gparams ::sappho_fconv::Extract< #fty > for ( #ident #gparams ) #optwhere {
                fn extract(self) -> Result< #fty, Self > {
                    Ok( self . #fid )
                }
            }

            #[automatically_derived]
            impl #gparams ::sappho_fconv::Embed< #fty > for ( #ident #gparams ) #optwhere {
                fn embed(thing: #fty ) -> Self {
                    #ident( thing )
                }
            }
        })
    }
}

#[ext]
impl ItemEnum {
    fn generate_impls(self) -> syn::Result<TokenStream> {
        // BUG: `Embed` impl only supports tuple-like construction
        let ItemEnum {
            ident: enumid,
            variants,
            generics,
            ..
        } = self;

        let (gparams, optwhere) = generics.into_gparams_and_where_clause()?;

        let mut varids = vec![];
        let mut fids = vec![];
        let mut ftys = vec![];

        for Variant {
            ident: varid,
            fields,
            ..
        } in variants
        {
            varids.push(varid);

            let (fid, fty) = fields.try_into_field_translation_info()?;
            fids.push(fid);
            ftys.push(fty);
        }

        Ok(quote! {
            #(
                #[automatically_derived]
                impl #gparams  ::sappho_fconv::Extract< #ftys > for ( #enumid #gparams ) #optwhere {
                    fn extract(self) -> Result< #ftys, Self > {
                        match self {
                            #enumid :: #varids ( x ) => Ok(x),
                            other => Err(other),
                        }
                    }
                }

                #[automatically_derived]
                impl #gparams  ::sappho_fconv::Embed< #ftys > for ( #enumid #gparams ) #optwhere {
                    fn embed(thing: #ftys ) -> Self {
                        #enumid :: #varids ( thing )
                    }
                }
            )*
        })
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
    fn try_into_field_translation_info(self) -> syn::Result<(TokenStream, Type)> {
        let (fix, field) = self.take_only_single_field()?;
        let fty = field.ty;
        let fid = field
            .ident
            .map(|id| id.into_token_stream())
            .unwrap_or_else(|| Index::from(fix).into_token_stream());

        Ok((fid, fty))
    }

    fn take_only_single_field(self) -> syn::Result<(usize, Field)> {
        let error = self.error("only single field struct or variants are supported");
        let punc = self.into_punctuated()?;
        punc.into_iter()
            .enumerate()
            .take_only_singleton()
            .ok_or(error)
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
