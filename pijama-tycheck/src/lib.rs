mod checker;
mod constraint;
mod error;
mod inference;
mod substitution;
mod unifier;

use checker::Checker;
use error::TyResult;
use pijama_hir::Program;
use pijama_ty::inference::TyContext;
pub use unifier::Unifier;

pub fn check_program(tcx: &TyContext, program: &Program) -> TyResult<Unifier> {
    Checker::new(tcx).check_program(program)
}
