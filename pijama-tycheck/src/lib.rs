mod checker;
mod constraint;
mod error;
mod inference;
mod substitution;

use checker::Checker;
use error::TyResult;
use pijama_hir::Program;
use pijama_ty::inference::TyContext;

pub fn check_program(tcx: &TyContext, program: &Program) -> TyResult {
    Checker::new(tcx).check_program(program)
}
