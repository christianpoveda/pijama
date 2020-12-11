use crate::{expr::Expr, ident::Ident, ty::Ty};

use pijama_utils::span::Span;

#[derive(Debug)]
/// The AST representation of a function's definition.
pub struct FuncDef<'source> {
    /// The identifier of the function.
    pub ident: Ident<'source>,
    /// The identifiers for the parameters of the function and their types.
    pub params: Vec<(Ident<'source>, Option<Ty<'source>>)>,
    /// The return type of the function.
    pub return_ty: Option<Ty<'source>>,
    /// The body of the function.
    pub body: Expr<'source>,
    /// The span of the function.
    pub span: Span,
}
