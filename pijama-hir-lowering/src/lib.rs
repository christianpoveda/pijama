mod context;
mod error;
mod lowering;

use context::LowerContext;
use error::LowerResult;

use pijama_core as core;
use pijama_hir as hir;
use pijama_tycheck::{Table, Unifier};

/// Lower the HIR of a program into the core representation.
///
/// This method consumes the HIR and requires an [Unifier] to instantiate inference variables.
pub fn lower_hir(
    unifier: Unifier,
    table: Table,
    program: hir::Program,
) -> LowerResult<(core::Program, Table)> {
    let mut lcx = LowerContext::new(unifier, table);
    let program = lcx.lower(program)?;

    Ok((program, lcx.table))
}
