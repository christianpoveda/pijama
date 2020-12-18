mod compile;
mod compiler;

use compiler::Compiler;

use pijama_core::Program;

use inkwell::context::Context;

pub fn compile_and_run(program: Program) {
    let context = Context::create();
    Compiler::new(&context).compile_and_run(program)
}
