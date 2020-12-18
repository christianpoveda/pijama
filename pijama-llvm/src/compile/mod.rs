mod atom;
mod expr;
mod literal;
mod name;

use crate::compiler::FuncCompiler;

/// A trait that every core term that can be compiled into the LLVM-IR must implement.
pub(crate) trait Compile<'ctx> {
    /// The type returned after compilation.
    type Output;

    /// Compile the term using a [CompilerFunc]. This consumes the term in the process.
    fn compile_with(self, compiler: &mut FuncCompiler<'ctx, '_>) -> Self::Output;
}
