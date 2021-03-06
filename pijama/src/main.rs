use pijama_interface::{Compiler, Config};

use std::env::args;

fn main() {
    // Get the path of the file with the source code.
    let mut args = args();
    args.next().unwrap();
    let path = args.next().unwrap();

    // Create configuration.
    let config = Config {
        path: path.into(),
        codegen: true,
    };

    env_logger::init();

    // Run the compiler.
    Compiler::new().run(config);
}
