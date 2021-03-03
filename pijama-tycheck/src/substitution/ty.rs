use pijama_ty::inference::{Row, Ty, TyVar};

/// A function from types to types.
///
/// This represents a substitution function between types.
#[derive(Debug)]
pub(crate) struct TySubstitution {
    /// The inference variable to be replaced.
    pub(crate) input: TyVar,
    /// The replacement.
    pub(crate) output: Ty,
}

impl TySubstitution {
    /// Create a new substitution.
    pub(crate) fn new(input: TyVar, output: Ty) -> Self {
        Self { input, output }
    }

    /// Apply this substitution to a type in-place.
    pub(crate) fn apply_to_ty(&self, ty: &mut Ty) {
        match ty {
            // Change the type to the `output` field if the type matches the `input` field.
            Ty::Var(id) if *id == self.input => *ty = self.output.clone(),
            // if the type is a function, apply this substitution recursively on the parameters and
            // return types.
            Ty::Func {
                params_ty,
                return_ty,
            } => {
                for ty in params_ty {
                    self.apply_to_ty(ty);
                }
                self.apply_to_ty(return_ty.as_mut());
            }
            Ty::Record(row) => self.apply_to_row(row),
            // Otherwise, left the type as it is.
            Ty::Var(_) | Ty::Base(_) => (),
        }
    }

    pub(crate) fn apply_to_row(&self, row: &mut Row) {
        for (_, ty) in row.fields_mut() {
            self.apply_to_ty(ty);
        }
    }
}
