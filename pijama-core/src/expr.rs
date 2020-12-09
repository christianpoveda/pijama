use crate::atom::Atom;
use crate::name::{Local, Name};
use crate::prim_op::PrimOp;

/// An expression.
///
/// This type tries to avoid nesting as much as possible by using atoms for all the control-flow
/// related expressions.
#[derive(Debug)]
pub enum Expr {
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
        rhs: Box<Self>,
        /// The expression where this binding is valid.
        body: Box<Self>,
    },
    /// A function call.
    Call {
        /// The name of the called function.
        ///
        /// This can be a local because of expressions like:
        /// ```
        /// (if cond do f else g end)(arg)
        /// ```
        func: Name,
        /// The arguments of the call.
        ///
        /// Arguments must be atoms to avoid nesting expressions. Which means that any non-atomic
        /// arguments must be evaluated and bound to a local before.
        args: Vec<Atom>,
    },
    /// A primitive operation.
    PrimitiveOp {
        /// The primitive operator.
        prim_op: PrimOp,
        /// The operands of the operation.
        ///
        /// Operands must be atoms to avoid nesting expressions. Which means that any non-atomic
        /// operand must be evaluated and bound to a local before.
        ops: Vec<Atom>,
    },
    /// A conditional expression.
    Cond {
        /// The condition being tested.
        cond: Atom,
        /// The expression to be evaluated if the condition is true.
        do_branch: Box<Self>,
        /// The expression to be evaluated if the condition is false.
        else_branch: Box<Self>,
    },
}
