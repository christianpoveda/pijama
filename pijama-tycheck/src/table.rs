use pijama_ty::{inference::Ty, ExprId};
use pijama_utils::index::IndexGen;

use std::{collections::HashMap, rc::Rc};

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

    pub(crate) fn store_ty(&mut self, expr_id: ExprId, ty: Ty) {
        assert!(self.types.insert(expr_id, ty).is_none());
    }
}
