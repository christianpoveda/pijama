use crate::expr::Expr;
use crate::name::Local;

use pijama_ty::inference::Ty;
use pijama_utils::{index::IndexMap, new_index};

new_index! {
    #[doc = "A function's ID.\n\nBy convention, the first ID corresponds to the `main` function of the program."]
    #[derive(Debug, Clone, Copy)]
    FuncId
}

/// A function.
///
/// In this IR, functions are C-like, which means they are not closures and all of them are
/// globally defined. Each function has a globally unique [FuncId] assigned to it.
#[derive(Debug)]
pub struct Func {
    /// The number of parameters of the function.
    pub arity: usize,
    /// The local values of the function with their types.
    ///
    /// The first `arity` locals correspond to the function's parameters.
    pub locals: IndexMap<Local, Ty>,
    /// The type of the value returned by the function.
    pub return_ty: Ty,
    /// The body of the function.
    pub body: Expr,
}
