use crate::{compile::Compile, compiler::FuncCompiler};

use pijama_mir::Name;

use inkwell::values::BasicValueEnum;

impl<'ctx> Compile<'ctx> for Name {
    type Output = BasicValueEnum<'ctx>;

    fn compile_with(self, compiler: &mut FuncCompiler<'ctx, '_>) -> Self::Output {
        // Get the name from the compiler.
        match self {
            Name::Local(local) => compiler
                .get_local(local)
                .expect("Could not find local inside function compiler."),
            Name::FuncPtr(func_id) => compiler
                .get_func(func_id)
                .expect("Could not find function pointer inside function compiler."),
        }
    }
}
