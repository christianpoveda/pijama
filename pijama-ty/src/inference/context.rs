use crate::inference::ty::{HoleId, Ty};

use pijama_utils::index::IndexGen;

/// A struct to keep track of the types during inference.
pub struct TyContext {
    generator: IndexGen<HoleId>,
}

impl TyContext {
    /// Create a new typing context.
    pub fn new() -> Self {
        Self {
            generator: IndexGen::new(),
        }
    }

    /// Create a new "hole" type.
    ///
    /// The type returned is guaranteed to be different from any other type created by this context.
    pub fn new_hole(&self) -> Ty {
        let id = self.generator.generate();
        Ty::Hole(id)
    }
}
