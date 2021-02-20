mod checker;
mod constraint;
pub mod error;
mod inference;
mod substitution;
mod table;
mod unifier;

use checker::Checker;
use error::TyResult;
use pijama_hir::Program;
use pijama_ty::inference::TyContext;
pub use table::Table;
pub use unifier::Unifier;

pub fn check_program(tcx: &TyContext, program: &Program) -> TyResult<(Unifier, Table)> {
    Checker::new(tcx).check_program(program)
}
