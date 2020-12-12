use pijama_ty::inference::TyContext;

use std::{env::args, fs::read_to_string};

fn main() {
    // Get the path of the file with the source code.
    let mut args = args();
    args.next().unwrap();
    let path = args.next().unwrap();

    // Read the source code to a string.
    let source = read_to_string(path).unwrap();

    // Parse the source code.
    let ast = pijama_parser::parse(&source);

    // Create a new typing context.
    let tcx = TyContext::new();

    // Lower the AST.
    let hir = pijama_ast_lowering::lower_ast(&tcx, ast).unwrap();
    println!("{:?}", hir);

    // Run the type-checking algorithm and get an unifier.
    let unifier = pijama_tycheck::check_program(&tcx, &hir).unwrap();

    println!("{:?}", unifier);
}
