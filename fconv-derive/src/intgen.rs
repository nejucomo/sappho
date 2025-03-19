use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{GenericParam, Generics, Token, WhereClause};

use crate::spannedext::SpannedExt;

#[derive(Debug)]
pub(crate) struct IntermediateGenerics {
    pub(crate) optparams: Option<GenericsParams>,
    pub(crate) optwhere: Option<WhereClause>,
}

impl TryFrom<Generics> for IntermediateGenerics {
    type Error = syn::Error;

    fn try_from(gs: Generics) -> syn::Result<Self> {
        let span = gs.span();

        Ok(Self {
            optparams: GenericsParams::try_from_opt(gs.lt_token, gs.params, gs.gt_token)
                .map_err(|msg| span.error(msg))?,
            optwhere: gs.where_clause,
        })
    }
}

#[derive(Debug)]
pub(crate) struct GenericsParams {
    lt: Token![<],
    gps: Punctuated<GenericParam, Token![,]>,
    gt: Token![>],
}

impl GenericsParams {
    fn try_from_opt(
        lt: Option<Token![<]>,
        params: Punctuated<GenericParam, Token![,]>,
        gt: Option<Token![>]>,
    ) -> Result<Option<Self>, &'static str> {
        match (lt, params, gt) {
            (None, params, None) if params.is_empty() => Ok(None),
            (Some(lt), params, Some(gt)) if !params.is_empty() => Ok(Some(GenericsParams {
                lt,
                gps: params,
                gt,
            })),
            _ => Err("unexpected or inconsistent generics"),
        }
    }
}

impl ToTokens for GenericsParams {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.lt.to_tokens(tokens);
        self.gps.to_tokens(tokens);
        self.gt.to_tokens(tokens);
    }
}
