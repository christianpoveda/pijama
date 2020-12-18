use crate::{compile::Compile, compiler::FuncCompiler};

use pijama_core::Literal;
use pijama_ty::base::BaseTy;

use inkwell::values::BasicValueEnum;

impl<'ctx> Compile<'ctx> for Literal {
    type Output = BasicValueEnum<'ctx>;

    fn compile_with(self, compiler: &mut FuncCompiler<'ctx, '_>) -> Self::Output {
        let basic_type = match self.base_ty() {
            BaseTy::Unit => compiler.ctx().i8_type(),
            BaseTy::Bool => compiler.ctx().i8_type(),
            BaseTy::Integer => compiler.ctx().i64_type(),
        };

        basic_type.const_int(self.bits() as u64, false).into()
    }
}
