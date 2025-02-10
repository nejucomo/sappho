#[derive(Copy, Clone, Debug)]
pub struct AstFuzz {
    limit: usize,
}

impl Default for AstFuzz {
    fn default() -> Self {
        AstFuzz { limit: 3 }
    }
}

impl AstFuzz {
    pub(crate) fn next_lower_level(self) -> Self {
        AstFuzz {
            limit: std::cmp::max(1, self.limit) - 1,
        }
    }

    pub(crate) fn recursive_weight_factor(self) -> u32 {
        if self.at_floor() {
            0
        } else {
            1
        }
    }

    fn at_floor(self) -> bool {
        self.limit == 0
    }
}
