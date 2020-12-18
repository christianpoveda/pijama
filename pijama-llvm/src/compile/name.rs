use crate::{compile::Compile, compiler::FuncCompiler};

use pijama_core::Name;

use inkwell::values::BasicValueEnum;

impl<'ctx> Compile<'ctx> for Name {
    type Output = BasicValueEnum<'ctx>;

    fn compile_with(self, compiler: &mut FuncCompiler<'ctx, '_>) -> Self::Output {
        match self {
            Name::Local(local) => compiler.get_local(local).unwrap(),
            Name::FuncPtr(func_id) => compiler.get_func(func_id).unwrap(),
        }
    }
}
