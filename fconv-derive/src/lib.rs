use easy_ext::ext;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    parse2 as parse, Error, Field, Fields, Item, ItemEnum, ItemStruct, Token, Type, Variant,
};

#[proc_macro_derive(Extract)]
pub fn extract_derive(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    extract_derive_inner(item.into())
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

fn extract_derive_inner(item: TokenStream) -> syn::Result<TokenStream> {
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
        // BUG: The `Embed` impl uses TupleStruct syntax only
        let ItemStruct { ident, fields, .. } = self;
        let (fid, fty) = fields.try_into_field_translation_info()?;

        Ok(quote! {
            impl ::sappho_fconv::Extract< #fty > for #ident {
                fn extract(self) -> Result< #fty, Self > {
                    Ok( self . #fid )
                }
            }

            impl ::sappho_fconv::Embed< #fty > for #ident {
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
            ..
        } = self;

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
                impl ::sappho_fconv::Extract< #ftys > for #enumid {
                    fn extract(self) -> Result< #ftys, Self > {
                        match self {
                            #enumid :: #varids ( x ) => Ok(x),
                            other => Err(other),
                        }
                    }
                }

                impl ::sappho_fconv::Embed< #ftys > for #enumid {
                    fn embed(thing: #ftys ) -> Self {
                        #enumid :: #varids ( thing )
                    }
                }
            )*
        })
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
            .unwrap_or_else(|| quote! { #fix });

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
