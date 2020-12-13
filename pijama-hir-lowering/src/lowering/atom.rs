use crate::{context::LowerContext, error::LowerResult, lowering::Lower};

use pijama_core as core;
use pijama_hir as hir;

impl Lower for hir::Atom {
    type Output = core::Atom;

    fn lower_with(self, lcx: &mut LowerContext) -> LowerResult<Self::Output> {
        // Lowering an atom calls the lowering methods of the literal or identifier inside the
        // atom.
        let atom = match self {
            hir::Atom::Literal(literal) => core::Atom::Literal(lcx.lower(literal)?),
            hir::Atom::Name(name) => core::Atom::Name(lcx.lower(name)?),
        };

        Ok(atom)
    }
}
