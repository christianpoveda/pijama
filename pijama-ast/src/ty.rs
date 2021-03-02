use crate::ident::Ident;

use pijama_utils::spanned_type;

spanned_type!(pub Ty<'source>, TyKind);

/// The AST representation of a type.
#[derive(Debug)]
pub enum TyKind<'source> {
    /// An identifier for a base type.
    Base(Ident<'source>),
    /// A function type.
    Func {
        /// The type of each parameter.
        params_ty: Vec<Ty<'source>>,
        /// The return type.
        return_ty: Box<Ty<'source>>,
    },
    Tuple {
        fields: Vec<Ty<'source>>,
    },
}
