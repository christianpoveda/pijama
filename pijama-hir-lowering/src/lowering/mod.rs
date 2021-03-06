mod atom;
mod expr;
mod func;
mod literal;
mod local;
mod name;
mod program;
mod ty;

use crate::{context::LowerContext, error::LowerResult};

/// A trait that every HIR term that can be lowered into a core term must implement.
pub(crate) trait Lower {
    /// The type of the lowered core term.
    type Output;
    /// Consume the current term and return a lowered one.
    fn lower_with(self, lcx: &mut LowerContext) -> LowerResult<Self::Output>;
}

impl<L: Lower> Lower for Box<L> {
    type Output = Box<L::Output>;

    fn lower_with(self, lcx: &mut LowerContext) -> LowerResult<Self::Output> {
        Ok(Box::new((*self).lower_with(lcx)?))
    }
}
