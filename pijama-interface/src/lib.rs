use pijama_ty::inference::TyContext;

use std::{
    ffi::OsStr,
    fs::read_to_string,
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};

/// The compiler's configuration.
pub struct Config {
    /// The path of the file being compiled.
    pub path: PathBuf,
    /// Generate a binary file.
    pub codegen: bool,
}

/// The compiler.
///
/// This is the structure used to compile Pijama's source code.
pub struct Compiler;

impl Compiler {
    /// Create a new compiler.
    pub fn new() -> Self {
        Self
    }

    /// Run the compiler with a specific configuration.
    pub fn run(self, config: Config) {
        // Read the source code to a string.
        let source = read_to_string(&config.path).unwrap();

        // Parse the source code.
        let ast = pijama_parser::parse(&source).unwrap();

        // Create a new typing context.
        let tcx = TyContext::new();

        // Lower the AST.
        let hir = pijama_ast_lowering::lower_ast(&tcx, ast).unwrap();

        // Run the type-checking algorithm and get an unifier.
        let (unifier, table) = pijama_tycheck::check_program(&tcx, &hir).unwrap();

        // Lower the HIR.
        let (mir, table) = pijama_hir_lowering::lower_hir(unifier, table, hir).unwrap();

        if config.codegen {
            let obj_path = config.path.with_extension("o");

            // Write the LLVM object file.
            pijama_llvm::compile(mir, table, &obj_path).unwrap();

            let exec_path = config.path.with_extension("out");

            let c_src = r#"
            #include <stdio.h>

            extern int entry();

            int main() {
                int result = entry();
                printf("%d\n", result);
                return 0;
            }"#;

            let mut clang = Command::new("clang")
                .args(&[
                    obj_path.as_os_str(),
                    OsStr::new("-o"),
                    exec_path.as_os_str(),
                    OsStr::new("-x"),
                    OsStr::new("c"),
                    OsStr::new("-"),
                ])
                .stdin(Stdio::piped())
                .spawn()
                .expect("Failed to spawn child process");

            let stdin = clang.stdin.as_mut().expect("Failed to open stdin");
            stdin
                .write_all(c_src.as_bytes())
                .expect("Failed to write to stdin");

            clang.wait_with_output().expect("Failed to run clang");
        }
    }
}
