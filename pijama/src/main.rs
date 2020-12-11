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

    println!("{:?}", ast);
}
