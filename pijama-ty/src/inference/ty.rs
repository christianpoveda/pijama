use crate::base::BaseTy;

use pijama_utils::new_index;

new_index! {
    #[doc = "An unique ID used to represent inference variables."]
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    HoleId
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
}

impl Ty {
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
        }
    }
}
