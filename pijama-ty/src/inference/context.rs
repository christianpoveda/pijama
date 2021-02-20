use crate::{
    inference::ty::{HoleId, Ty},
    ExprId,
};

use pijama_utils::index::IndexGen;

use std::rc::Rc;

/// A struct to keep track of the types during inference.
pub struct TyContext {
    hole_gen: IndexGen<HoleId>,
    expr_id_gen: Rc<IndexGen<ExprId>>,
}

impl TyContext {
    /// Create a new typing context.
    pub fn new() -> Self {
        Self {
            hole_gen: IndexGen::new(),
            expr_id_gen: Rc::new(IndexGen::new()),
        }
    }

    /// Create a new "hole" type.
    ///
    /// The type returned is guaranteed to be different from any other type created by this context.
    pub fn new_hole(&self) -> Ty {
        let id = self.hole_gen.generate();
        Ty::Hole(id)
    }

    pub fn new_expr_id(&self) -> ExprId {
        self.expr_id_gen.generate()
    }

    pub fn expr_id_gen(&self) -> Rc<IndexGen<ExprId>> {
        Rc::clone(&self.expr_id_gen)
    }
}
