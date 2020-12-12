use crate::{context::LowerContext, error::LowerResult, lowering::Lower};

use pijama_ast as ast;
use pijama_hir as hir;

impl<'source, 'tcx> Lower<'source, 'tcx> for ast::Atom<'source> {
    type Output = hir::Atom;

    fn lower_with(
        self,
        lcx: &mut LowerContext<'source, 'tcx>,
    ) -> LowerResult<'source, Self::Output> {
        // Lowering an atom calls the lowering methods of the literal or identifier inside the
        // atom.
        let atom = match self {
            ast::Atom::Literal(literal) => hir::Atom::Literal(lcx.lower(literal)?),
            ast::Atom::Ident(ident) => hir::Atom::Name(lcx.lower(ident)?),
        };

        Ok(atom)
    }
}
