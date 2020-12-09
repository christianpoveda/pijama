use pijama_ty::base::BaseTy;

/// A value that represents itself.
///
/// The bit representation of a literal follows Rust's data layout.
#[derive(Debug, Clone, Copy)]
pub struct Literal {
    bits: isize,
    ty: BaseTy,
}

impl Literal {
    /// The type of the literal.
    pub fn base_ty(&self) -> BaseTy {
        self.ty
    }

    /// The bit representation of the literal.
    pub fn bits(&self) -> isize {
        self.bits
    }
}

impl From<isize> for Literal {
    fn from(int: isize) -> Self {
        Self {
            bits: int,
            ty: BaseTy::Integer,
        }
    }
}

impl From<bool> for Literal {
    fn from(b: bool) -> Self {
        Self {
            bits: b as isize,
            ty: BaseTy::Bool,
        }
    }
}

impl From<()> for Literal {
    fn from((): ()) -> Self {
        Self {
            bits: 0,
            ty: BaseTy::Unit,
        }
    }
}
