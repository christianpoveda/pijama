mod checker;
mod constraint;
pub mod error;
mod inference;
mod substitution;
mod unifier;
mod table;

use checker::Checker;
use error::TyResult;
use pijama_hir::Program;
use pijama_ty::inference::TyContext;
pub use unifier::Unifier;
pub use table::Table;

pub fn check_program(tcx: &TyContext, program: &Program) -> TyResult<(Unifier, Table)> {
    Checker::new(tcx).check_program(program)
}
