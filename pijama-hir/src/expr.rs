use crate::{
    atom::Atom,
    name::{Local, Name},
    prim_op::{BinOp, UnOp},
};

use pijama_ty::ExprId;

/// An expression.
///
/// This type tries to avoid nesting as much as possible by using atoms for all the control-flow
/// related expressions.
#[derive(Debug)]
pub struct Expr {
    pub id: ExprId,
    pub kind: ExprKind,
}

#[derive(Debug)]
pub enum ExprKind {
    /// An atomic expression.
    Atom(Atom),
    /// A local binding.
    ///
    /// Shadowing is not allowed in this IR. Which means that a local cannot be rebound inside the
    /// body of a `let` binding.
    Let {
        /// The local to be bound.
        ///
        /// The type of this local can be found in the function's `locals` field.
        lhs: Local,
        /// The expression whose value will be bound to the local.
        ///
        /// This expression is guaranteed to have the same type as the LHS.
        rhs: Box<Expr>,
        /// The expression where this binding is valid.
        body: Box<Expr>,
    },
    /// A function call.
    Call {
        /// The name of the called function.
        ///
        /// This can be a local because of expressions like: `(if cond do f else g end)(arg)`.
        func: Name,
        /// The arguments of the call.
        args: Vec<Expr>,
    },
    /// A primitive unary operation.
    UnaryOp {
        /// The primitive unary operator.
        un_op: UnOp,
        /// The operand of the operation.
        op: Box<Expr>,
    },
    /// A primitive binary operation.
    BinaryOp {
        /// The primitive unary operator.
        bin_op: BinOp,
        /// The left-hand side operand of the operation.
        left_op: Box<Expr>,
        /// The right-hand side operand of the operation.
        right_op: Box<Expr>,
    },
    /// A conditional expression.
    Cond {
        /// The condition being tested.
        cond: Box<Expr>,
        /// The expression to be evaluated if the condition is true.
        do_branch: Box<Expr>,
        /// The expression to be evaluated if the condition is false.
        else_branch: Box<Expr>,
    },
    Tuple {
        fields: Vec<Expr>,
    },
}
