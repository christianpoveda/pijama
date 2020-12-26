mod context;
pub mod error;
mod lowering;
mod scope;

use context::LowerContext;
use error::LowerResult;

use pijama_ast as ast;
use pijama_hir as hir;
use pijama_ty::inference::TyContext;

/// Lower the AST representation of a program into the HIR.
///
/// This method consumes the AST and requires a reference to the [TyContext] to introduce inference
/// variables for the unknown types.
pub fn lower_ast<'source>(
    tcx: &TyContext,
    program: ast::Program<'source>,
) -> LowerResult<'source, hir::Program> {
    LowerContext::new(tcx).lower_program(program)
}
