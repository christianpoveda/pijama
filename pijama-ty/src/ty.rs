//! Concrete types.
//!
//! This is the type representation used after type inference.
use crate::base::BaseTy;

use pijama_utils::show::Show;

/// A concrete type.
#[derive(Debug, Clone)]
pub enum Ty {
    /// A base type.
    Base(BaseTy),
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

impl<Ctx> Show<Ctx> for Ty {
    fn show(&self, ctx: &Ctx, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Base(base_ty) => base_ty.show(ctx, f),
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
