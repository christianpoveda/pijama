use crate::func::{Func, FuncId};

use pijama_utils::index::IndexMap;

/// A program.
#[derive(Debug)]
pub struct Program {
    /// The functions of the program.
    pub functions: IndexMap<FuncId, Func>,
}
