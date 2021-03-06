use crate::{context::LowerContext, error::LowerResult, lowering::Lower};

use pijama_hir as hir;
use pijama_mir as mir;

impl Lower for hir::Atom {
    type Output = mir::Atom;

    fn lower_with(self, lcx: &mut LowerContext) -> LowerResult<Self::Output> {
        // Lowering an atom calls the lowering methods of the literal or identifier inside the
        // atom.
        let atom = match self {
            hir::Atom::Literal(literal) => mir::Atom::Literal(lcx.lower(literal)?),
            hir::Atom::Name(name) => mir::Atom::Name(lcx.lower(name)?),
        };

        Ok(atom)
    }
}
