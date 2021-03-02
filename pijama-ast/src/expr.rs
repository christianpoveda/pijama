use crate::{
    atom::Atom,
    ident::Ident,
    op::{BinOp, UnOp},
    ty::Ty,
};

use pijama_utils::spanned_type;

spanned_type!(pub Expr<'source>, ExprKind);

/// The AST representation of an expression.
#[derive(Debug)]
pub enum ExprKind<'source> {
    /// An atomic expression.
    Atom(Atom<'source>),
    /// A local binding.
    Let {
        /// The identifier to be bound.
        lhs: Ident<'source>,
        /// The type of the identfiier.
        lhs_ty: Option<Ty<'source>>,
        /// The expression whose value will be bound to the ident.
        rhs: Box<Expr<'source>>,
        /// The expression where this binding is valid.
        body: Box<Expr<'source>>,
    },
    Call {
        /// The name of the called function.
        func: Ident<'source>,
        /// The arguments of the call.
        args: Vec<Expr<'source>>,
    },
    // A primitive unary operation.
    UnaryOp(UnOp, Box<Expr<'source>>),
    // A primitive binary operation.
    BinaryOp(BinOp, Box<Expr<'source>>, Box<Expr<'source>>),
    /// A conditional expression.
    Cond {
        /// The condition being tested.
        cond: Box<Expr<'source>>,
        /// The expression to be evaluated if the condition is true.
        do_branch: Box<Expr<'source>>,
        /// The expression to be evaluated if the condition is false.
        else_branch: Box<Expr<'source>>,
    },
    Tuple {
        fields: Vec<Expr<'source>>,
    },
}
