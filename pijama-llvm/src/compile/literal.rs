use crate::{compile::Compile, compiler::FuncCompiler};

use pijama_mir::Literal;
use pijama_ty::base::BaseTy;

use inkwell::values::BasicValueEnum;

impl<'ctx> Compile<'ctx> for Literal {
    type Output = BasicValueEnum<'ctx>;

    fn compile_with(self, compiler: &mut FuncCompiler<'ctx, '_>) -> Self::Output {
        // All base types can be represented as integers. Pick the right integer type.
        let basic_type = match self.base_ty() {
            // FIXME: Include this inside `Compiler` so we don't break the typing of literals by
            // accident.
            BaseTy::Bool => compiler.ctx().bool_type(),
            BaseTy::Int => compiler.ctx().i64_type(),
        };
        // Take the bits of the constant, this assumes LLVM has the same data layout as rust.
        basic_type.const_int(self.bits() as u64, false).into()
    }
}
