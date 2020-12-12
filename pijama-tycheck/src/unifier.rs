use crate::{
    error::{TyError, TyResult},
    substitution::Substitution,
};

use pijama_ty::{inference, ty};

/// A type to replace all inference variables by concrete types, if possible.
#[derive(Debug)]
pub struct Unifier {
    substitutions: Vec<Substitution>,
}

impl Unifier {
    /// Create a new and empty unifier.
    pub(crate) fn new() -> Self {
        Self {
            substitutions: Vec::new(),
        }
    }

    /// Add a substitution to the unifier.
    pub(crate) fn add_substitution(&mut self, mut new_subst: Substitution) {
        for subst in &self.substitutions {
            // Compose the new substitution with all the already used substitutions.
            new_subst = subst.compose(new_subst);
        }
        // Push the substitution into the substitutions list.
        self.substitutions.push(new_subst)
    }

    /// Instantiate the type variables inside a type. Return an error if the type still has type
    /// variables inside it.
    ///
    /// A type might still have "holes" after instantiation if the type-checking constraints are
    /// not strict enough to enforce a type for every variable.
    pub fn instantiate(&self, mut ty: inference::Ty) -> TyResult<ty::Ty> {
        for substitution in &self.substitutions {
            substitution.apply_to(&mut ty);
        }

        try_concrete(ty)
    }
}

/// Try to convert an inference type into a concrete type without holes. Error with
/// the first `HoleId` found otherwise.
fn try_concrete(ty: inference::Ty) -> TyResult<ty::Ty> {
    match ty {
        inference::Ty::Base(base) => Ok(ty::Ty::Base(base)),
        inference::Ty::Hole(hole_id) => Err(TyError::HoleFound(hole_id)),
        inference::Ty::Func {
            params_ty,
            return_ty,
        } => {
            let params_ty = params_ty
                .into_iter()
                .map(try_concrete)
                .collect::<TyResult<Vec<ty::Ty>>>()?;
            let return_ty = Box::new(try_concrete(*return_ty)?);

            Ok(ty::Ty::Func {
                params_ty,
                return_ty,
            })
        }
    }
}
