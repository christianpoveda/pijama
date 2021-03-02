mod compile;
mod compiler;

use compiler::Compiler;

use pijama_core::Program;
use pijama_tycheck::Table;

use inkwell::{context::Context, support::LLVMString};

use std::path::Path;

pub fn compile(program: Program, table: Table, path: &Path) -> Result<(), LLVMString> {
    let context = Context::create();
    Compiler::new(&context, table).compile(program, path)
}
