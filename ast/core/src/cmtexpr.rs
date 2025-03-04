use std::ops::Deref;

use derive_new::new;
use sappho_ast_comment::CmtPrefixed;
use sappho_ast_effect::Effect;
use sappho_attrs::Attrs;
use sappho_unparse::Unparse;

use crate::{AstProvider, FuncDef, ProcDef, QueryDef};

#[derive(Clone, Debug, PartialEq, Eq, new, derive_more::From)]
pub struct CmtExpr<XP, FX>(CmtPrefixed<Box<XP::Expr<FX>>>)
where
    XP: AstProvider,
    FX: Effect;

impl<XP, FX> CmtExpr<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    pub fn expr(&self) -> &XP::Expr<FX> {
        self.0.deref()
    }

    pub fn map_expr<F, XPD, FXD>(self, f: F) -> CmtExpr<XPD, FXD>
    where
        F: FnOnce(XP::Expr<FX>) -> XPD::Expr<FXD>,
        XPD: AstProvider,
        FXD: Effect,
    {
        CmtExpr(self.0.map_node(|b| Box::new(f(*b))))
    }

    pub fn try_map_expr<F, XPD, FXD, E>(self, f: F) -> Result<CmtExpr<XPD, FXD>, E>
    where
        F: FnOnce(XP::Expr<FX>) -> Result<XPD::Expr<FXD>, E>,
        XPD: AstProvider,
        FXD: Effect,
    {
        let xpd = self.0.map_node(|bx| f(*bx).map(Box::new)).transpose()?;

        Ok(CmtExpr(xpd))
    }

    pub fn append_comment_section<S, B>(self, section: S, body: B) -> Self
    where
        S: AsRef<str>,
        B: AsRef<str>,
    {
        CmtExpr(self.0.append_comment_section(section, body))
    }
}

impl<XP, FX> From<(Option<String>, XP::Expr<FX>)> for CmtExpr<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn from((optcmt, x): (Option<String>, XP::Expr<FX>)) -> Self {
        CmtPrefixed::new(optcmt, Box::new(x)).into()
    }
}

impl<XP, FX> From<CmtExpr<XP, FX>> for (Option<String>, XP::Expr<FX>)
where
    XP: AstProvider,
    FX: Effect,
{
    fn from(ct: CmtExpr<XP, FX>) -> Self {
        let (optcmt, boxed) = ct.0.into();
        (optcmt, *boxed)
    }
}

impl<XP, FX> From<FuncDef<XP>> for CmtExpr<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
    XP::Expr<FX>: From<FuncDef<XP>>,
{
    fn from(value: FuncDef<XP>) -> Self {
        CmtExpr::from((None, XP::Expr::from(value)))
    }
}

impl<XP, FX> From<QueryDef<XP>> for CmtExpr<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
    XP::Expr<FX>: From<QueryDef<XP>>,
{
    fn from(value: QueryDef<XP>) -> Self {
        CmtExpr::from((None, XP::Expr::from(value)))
    }
}

impl<XP, FX> From<ProcDef<XP>> for CmtExpr<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
    XP::Expr<FX>: From<ProcDef<XP>>,
{
    fn from(value: ProcDef<XP>) -> Self {
        CmtExpr::from((None, XP::Expr::from(value)))
    }
}

impl<XP, FX> From<Attrs<CmtExpr<XP, FX>>> for CmtExpr<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
    XP::Expr<FX>: From<Attrs<CmtExpr<XP, FX>>>,
{
    fn from(value: Attrs<CmtExpr<XP, FX>>) -> Self {
        CmtExpr::from((None, XP::Expr::from(value)))
    }
}

impl<XP, FX> Unparse for CmtExpr<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn unparse_into(&self, s: &mut sappho_unparse::Stream) {
        self.0.unparse_into(s)
    }
}
