pub trait FromFx {
    type AstFx;

    fn from_fx(astfx: Self::AstFx) -> Self;
}

pub type AstFxFor<FX> = <FX as FromFx>::AstFx;
