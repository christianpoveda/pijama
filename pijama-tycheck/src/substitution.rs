use pijama_ty::inference::{Ty, TyVar};

use pijama_utils::show::Show;

/// A function from types to types.
///
/// This represents a substitution function between types.
#[derive(Debug)]
pub(crate) struct Substitution {
    /// The inference variable to be replaced.
    pub(crate) input: TyVar,
    /// The replacement.
    pub(crate) output: Ty,
}

impl Substitution {
    /// Create a new substitution.
    pub(crate) fn new(input: TyVar, output: Ty) -> Self {
        Self { input, output }
    }

    /// Apply this substitution in-place to a type.
    pub(crate) fn apply_to(&self, ty: &mut Ty) {
        match ty {
            // Change the type to the `output` field if the type matches the `input` field.
            Ty::Var(var) if *var == self.input => *ty = self.output.clone(),
            // if the type is a function, apply this substitution recursively on the parameters and
            // return types.
            Ty::Func {
                params_ty,
                return_ty,
            } => {
                for ty in params_ty {
                    self.apply_to(ty);
                }
                self.apply_to(return_ty.as_mut());
            }
            // If the type is a tuple, apply this substitution recursively on the fields.
            Ty::Tuple { fields } => {
                for ty in fields {
                    self.apply_to(ty);
                }
            }
            // Otherwise, left the type as it is.
            Ty::Var(_) | Ty::Base(_) => (),
        }
    }
}

impl<Ctx> Show<Ctx> for Substitution {
    fn show(&self, ctx: &Ctx, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.input.wrap(ctx), self.output.wrap(ctx))
    }
}
