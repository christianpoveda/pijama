mod atom;
mod expr;
mod func;
mod ident;
mod literal;
mod ty;

use crate::{context::LowerContext, error::LowerResult};

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

impl<'source, 'tcx, L: Lower<'source, 'tcx>> Lower<'source, 'tcx> for Box<L> {
    type Output = Box<L::Output>;

    fn lower_with(
        self,
        lcx: &mut LowerContext<'source, 'tcx>,
    ) -> LowerResult<'source, Self::Output> {
        Ok(Box::new((*self).lower_with(lcx)?))
    }
}

impl<'source, 'tcx, L: Lower<'source, 'tcx>> Lower<'source, 'tcx> for Vec<L> {
    type Output = Vec<L::Output>;

    fn lower_with(
        self,
        lcx: &mut LowerContext<'source, 'tcx>,
    ) -> LowerResult<'source, Self::Output> {
        self.into_iter().map(|item| item.lower_with(lcx)).collect()
    }
}
