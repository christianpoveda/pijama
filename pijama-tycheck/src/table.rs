use pijama_ty::{inference::Ty, ExprId};
use pijama_utils::index::IndexGen;

use std::{collections::HashMap, rc::Rc};

// FIXME: Split into a builder and the actual table. The builder is backed by
// `IndexMap<ExprId, Option<inference::Ty>>`, the actual table is backed by
// `IndexMap<ExprId, ty::Ty>`.
pub struct Table {
    generator: Rc<IndexGen<ExprId>>,
    types: HashMap<ExprId, Ty>,
}

impl Table {
    pub(crate) fn new(generator: Rc<IndexGen<ExprId>>) -> Self {
        Self {
            generator,
            types: HashMap::new(),
        }
    }

    pub fn store_ty(&mut self, expr_id: ExprId, ty: Ty) {
        assert!(self.types.insert(expr_id, ty).is_none());
    }

    pub fn new_expr_id(&self) -> ExprId {
        self.generator.generate()
    }

    pub fn get_ty(&self, expr_id: ExprId) -> Option<&Ty> {
        self.types.get(&expr_id)
    }
}
