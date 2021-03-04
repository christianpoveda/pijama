use crate::base::BaseTy;

use pijama_utils::{new_index, show::Show};

new_index! {
    #[doc = "A type inference variable."]
    #[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
    TyVar
}

impl<Ctx> Show<Ctx> for TyVar {
    fn show(&self, _ctx: &Ctx, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "?T{}", self.0)
    }
}

/// A type with holes.
///
/// This is the type representation used for type-checking and type inference. The only difference
/// between this representation and the concrete representation found in [crate::ty::Ty] is the
/// [Ty::Var] variant.
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
    /// A tuple type.
    Tuple { fields: Vec<Self> },
}

impl Ty {
    /// Check if the current type contains a type variable.
    pub fn contains_ty(&self, target: TyVar) -> bool {
        match self {
            Ty::Base(_) => false,
            Ty::Var(var) => *var == target,
            Ty::Func {
                params_ty,
                return_ty,
            } => params_ty.iter().any(|ty| ty.contains_ty(target)) || return_ty.contains_ty(target),
            Ty::Tuple { fields } => fields.iter().any(|ty| ty.contains_ty(target)),
        }
    }
}

impl<Ctx> Show<Ctx> for Ty {
    fn show(&self, ctx: &Ctx, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Base(base_ty) => base_ty.show(ctx, f),
            Self::Var(id) => id.show(ctx, f),
            Self::Func {
                params_ty,
                return_ty,
            } => {
                write!(
                    f,
                    "fn({}) -> {}",
                    Show::<Ctx>::show_sep(params_ty, ", ").wrap(ctx),
                    return_ty.wrap(ctx)
                )
            }
            Self::Tuple { fields } => {
                write!(f, "({})", Show::<Ctx>::show_sep(fields, ", ").wrap(ctx))
            }
        }
    }
}
