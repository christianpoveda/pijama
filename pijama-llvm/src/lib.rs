mod compile;
mod compiler;

use compiler::Compiler;

use pijama_core::Program;

use inkwell::{context::Context, support::LLVMString};

use std::path::Path;

pub fn compile(program: Program, path: &Path) -> Result<(), LLVMString> {
    let context = Context::create();
    Compiler::new(&context).compile(program, path)
}
