use pijama_ty::inference::Ty;

/// A function from types to types.
///
/// This represents a substitution function between types.
#[derive(Debug)]
pub(crate) struct Substitution {
    /// The type to be replaced.
    input: Ty,
    /// The replacement.
    output: Ty,
}

impl Substitution {
    /// Create a new substitution.
    pub(crate) fn new(input: Ty, output: Ty) -> Self {
        Self { input, output }
    }

    /// Apply this substitution in-place to a type.
    pub(crate) fn apply_to(&self, ty: &mut Ty) {
        if *ty == self.input {
            // Change the type to the `output` field if the type matches the `input` field.
            *ty = self.output.clone();
        } else if let Ty::Func {
            params_ty,
            return_ty,
        } = ty
        {
            // Else, if the type is a function, apply this substitution recursively on the
            // parameters and return types.
            for param_ty in params_ty {
                self.apply_to(param_ty);
            }

            self.apply_to(return_ty.as_mut());
        }
        // Otherwise, left the type as it is.
    }

    /// Compose this substition with another in the functional sense.
    pub fn compose(&self, mut other: Self) -> Self {
        // Apply the current substitution to the output of the other substitution.
        self.apply_to(&mut other.output);
        other
    }
}
