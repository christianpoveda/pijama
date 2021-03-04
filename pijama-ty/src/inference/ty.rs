use crate::base::BaseTy;

use pijama_utils::{new_index, show::Show};

new_index! {
    #[doc = "An unique ID used to represent inference variables."]
    #[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
    HoleId
}

impl<Ctx> Show<Ctx> for HoleId {
    fn show(&self, _ctx: &Ctx, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "?T{}", self.0)
    }
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
    Hole(HoleId),
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
    /// Check if the type has a "hole" type with the given ID.
    pub fn contains_hole(&self, hole_id: HoleId) -> bool {
        match self {
            Ty::Base(_) => false,
            Ty::Hole(id) => *id == hole_id,
            Ty::Func {
                params_ty,
                return_ty,
            } => {
                params_ty.iter().any(|ty| ty.contains_hole(hole_id))
                    || return_ty.contains_hole(hole_id)
            }
            Ty::Tuple { fields } => fields.iter().any(|ty| ty.contains_hole(hole_id)),
        }
    }
}

impl<Ctx> Show<Ctx> for Ty {
    fn show(&self, ctx: &Ctx, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Base(base_ty) => base_ty.show(ctx, f),
            Self::Hole(id) => id.show(ctx, f),
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
