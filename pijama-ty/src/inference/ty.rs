use crate::{
    base::BaseTy,
    inference::row::{Row, RowVar},
};

use pijama_utils::new_index;

new_index! {
    #[doc = "An unique ID used to represent inference variables."]
    #[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
    TyVar
}

/// A type with holes.
///
/// This is the type representation used for type-checking and type inference. The only difference
/// between this representation and the concrete representation found in [crate::ty::Ty] is the
/// [Ty::Hole] variant.
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Ty {
    /// A base type.
    Base(BaseTy),
    /// A type to be infered.
    Var(TyVar),
    /// A function type.
    Func {
        /// The type of each parameter.
        params_ty: Vec<Self>,
        /// The return type.
        return_ty: Box<Self>,
    },
    /// A record type.
    Record(Row),
}

impl Ty {
    /// Check if the type has a "hole" type with the given ID.
    pub fn contains_ty(&self, ty_var: TyVar) -> bool {
        match self {
            Ty::Base(_) => false,
            Ty::Var(var) => *var == ty_var,
            Ty::Func {
                params_ty,
                return_ty,
            } => params_ty.iter().any(|ty| ty.contains_ty(ty_var)) || return_ty.contains_ty(ty_var),
            Ty::Record(row) => row.contains_ty(ty_var),
        }
    }

    pub fn contains_row(&self, row_var: RowVar) -> bool {
        match self {
            Ty::Base(_) | Ty::Var(_) => false,
            Ty::Func {
                params_ty,
                return_ty,
            } => {
                params_ty.iter().any(|ty| ty.contains_row(row_var))
                    || return_ty.contains_row(row_var)
            }
            Ty::Record(row) => row.contains_row(row_var),
        }
    }
}
