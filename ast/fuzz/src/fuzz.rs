#[derive(Debug)]
pub struct AstFuzz {
    #[allow(dead_code)]
    limit: usize,
}

impl Default for AstFuzz {
    fn default() -> Self {
        AstFuzz { limit: 3 }
    }
}
