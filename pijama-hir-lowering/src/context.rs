use crate::{error::LowerResult, lowering::Lower};

use pijama_tycheck::{Table, Unifier};

/// The main structure to lower the HIR.
pub(crate) struct LowerContext {
    /// The unifier used to instantiate types.
    pub(crate) unifier: Unifier,
    pub(crate) table: Table,
}

impl LowerContext {
    /// Create a new lowering context.
    pub(crate) fn new(unifier: Unifier, table: Table) -> Self {
        Self { unifier, table }
    }

    /// Lower a term that implements the [Lower] trait.
    pub(crate) fn lower<T: Lower>(&mut self, term: T) -> LowerResult<T::Output> {
        term.lower_with(self)
    }
}
