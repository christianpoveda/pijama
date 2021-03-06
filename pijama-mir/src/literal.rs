use pijama_ty::base::BaseTy;

/// A value that represents itself.
///
/// The bit representation of a literal follows Rust's data layout.
#[derive(Debug, Clone, Copy)]
pub struct Literal {
    bits: i64,
    ty: BaseTy,
}

impl Literal {
    /// The type of the literal.
    pub fn base_ty(&self) -> BaseTy {
        self.ty
    }

    /// The bit representation of the literal.
    pub fn bits(&self) -> i64 {
        self.bits
    }
}

impl From<i64> for Literal {
    fn from(int: i64) -> Self {
        Self {
            bits: int,
            ty: BaseTy::Int,
        }
    }
}

impl From<bool> for Literal {
    fn from(b: bool) -> Self {
        Self {
            bits: b as i64,
            ty: BaseTy::Bool,
        }
    }
}
