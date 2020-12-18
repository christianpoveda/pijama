use crate::{compile::Compile, compiler::FuncCompiler};

use pijama_core::Atom;

use inkwell::values::BasicValueEnum;

impl<'ctx> Compile<'ctx> for Atom {
    type Output = BasicValueEnum<'ctx>;

    fn compile_with(self, compiler: &mut FuncCompiler<'ctx, '_>) -> Self::Output {
        match self {
            Atom::Literal(literal) => compiler.compile(literal),
            Atom::Name(name) => compiler.compile(name),
        }
    }
}
