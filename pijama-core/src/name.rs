use crate::func::FuncId;

use pijama_utils::new_index;

new_index! {
    #[doc = "A value local to a function's body.\n\nLocals represent either parameters of a
    function or values bound inside the body of the function using `let` expressions."]
    #[derive(Debug)]
    Local
}

/// A value that refers to other value.
#[derive(Debug)]
pub enum Name {
    /// A value that is local to a function.
    Local(Local),
    /// A pointer to a function.
    FuncPtr(FuncId),
}
