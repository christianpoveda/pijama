use crate::{error::LowerResult, lowering::Lower};

use pijama_tycheck::Unifier;

/// The main structure to lower the HIR.
pub(crate) struct LowerContext {
    /// The unifier used to instantiate types.
    pub(crate) unifier: Unifier,
}

impl LowerContext {
    /// Create a new lowering context.
    pub(crate) fn new(unifier: Unifier) -> Self {
        Self { unifier }
    }

    /// Lower a term that implements the [Lower] trait.
    pub(crate) fn lower<T: Lower>(&mut self, term: T) -> LowerResult<T::Output> {
        term.lower_with(self)
    }
}
