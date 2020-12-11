use crate::func::FuncDef;

/// The AST representation of a program.
#[derive(Debug)]
pub struct Program<'source> {
    /// The functions of the program.
    pub functions: Vec<FuncDef<'source>>,
}
