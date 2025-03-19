mod embeddings;
mod intermediate;
mod intgen;
mod spannedext;

use proc_macro2::TokenStream;
use quote::ToTokens as _;
use syn::Error;

use crate::intermediate::Intermediate;

#[proc_macro_derive(Extract)]
pub fn extract_derive(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    extract_derive_pm2(item.into()).into()
}

fn extract_derive_pm2(item: TokenStream) -> TokenStream {
    extract_derive_res(item).unwrap_or_else(Error::into_compile_error)
}

fn extract_derive_res(tokens: TokenStream) -> syn::Result<TokenStream> {
    let intermediate = Intermediate::try_from(tokens)?;
    Ok(intermediate.into_token_stream())
}

#[cfg(test)]
mod tests;
