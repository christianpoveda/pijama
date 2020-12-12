mod atom;
mod expr;
mod func;
mod ident;
mod literal;
mod ty;

use crate::{context::LowerContext, error::LowerResult};
// FIXME: add an implementaton for `Box<T>`..

/// A trait that every AST term that can be lowered into a HIR term must implement.
pub(crate) trait Lower<'source, 'tcx> {
    /// The type of the lowered HIR term.
    type Output;
    /// Consume the current term and return a lowered one.
    fn lower_with(
        self,
        lcx: &mut LowerContext<'source, 'tcx>,
    ) -> LowerResult<'source, Self::Output>;
}
