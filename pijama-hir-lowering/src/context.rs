use crate::{error::LowerResult, lowering::Lower};

use pijama_core as core;
use pijama_ty::{ty::Ty, ExprId};
use pijama_tycheck::{Table, Unifier};
use pijama_utils::index::IndexMap;

/// The main structure to lower the HIR.
pub(crate) struct LowerContext {
    /// The unifier used to instantiate types.
    pub(crate) unifier: Unifier,
    pub(crate) table: Table,
    local_types: IndexMap<core::Local, Ty>,
}

impl LowerContext {
    /// Create a new lowering context.
    pub(crate) fn new(unifier: Unifier, table: Table) -> Self {
        Self {
            unifier,
            table,
            local_types: IndexMap::new(),
        }
    }

    /// Lower a term that implements the [Lower] trait.
    pub(crate) fn lower<T: Lower>(&mut self, term: T) -> LowerResult<T::Output> {
        term.lower_with(self)
    }

    pub(crate) fn get_expr_ty(&self, id: ExprId) -> Option<Ty> {
        let ty = self.table.get_ty(id)?.clone();
        Some(self.unifier.instantiate(ty))
    }

    pub(crate) fn store_local_ty(&mut self, ty: Ty) -> core::Local {
        self.local_types.insert(ty)
    }

    pub(crate) fn get_local_types(&mut self) -> IndexMap<core::Local, Ty> {
        std::mem::replace(&mut self.local_types, IndexMap::new())
    }
}
