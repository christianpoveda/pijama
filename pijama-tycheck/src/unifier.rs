use std::collections::BTreeMap;

use crate::{
    error::{TyError, TyResult},
    substitution::Substitution,
};

use pijama_ty::{
    inference::{self, HoleId},
    ty,
};

/// A type to replace all inference variables by concrete types, if possible.
#[derive(Debug)]
pub struct Unifier {
    substitutions: BTreeMap<HoleId, ty::Ty>,
}

impl Unifier {
    /// Create a new and empty unifier builder.
    pub(crate) fn builder() -> UnifierBuilder {
        UnifierBuilder::new()
    }

    /// Apply all the substitutions to a type in order to instantiate it.
    ///
    /// This function panics if the type has inference variables that are not in the unifier.
    pub fn instantiate(&self, ty: inference::Ty) -> ty::Ty {
        match ty {
            // Change the type to the `output` if the type matches the `input`.
            inference::Ty::Hole(hole_id) => self
                .substitutions
                .get(&hole_id)
                .expect("Every type variable should have a substitution")
                .clone(),
            // if the type is a function, apply the substitutions recursively on the parameters and
            // return types.
            inference::Ty::Func {
                params_ty,
                return_ty,
            } => ty::Ty::Func {
                params_ty: params_ty
                    .into_iter()
                    .map(|ty| self.instantiate(ty))
                    .collect(),
                return_ty: Box::new(self.instantiate(*return_ty)),
            },
            // if the type is a tuple, apply the substitutions recursively on the fields.
            inference::Ty::Tuple { fields } => ty::Ty::Tuple {
                fields: fields.into_iter().map(|ty| self.instantiate(ty)).collect(),
            },
            // Otherwise, left the type as it is.
            inference::Ty::Base(base) => ty::Ty::Base(base),
        }
    }
}

/// An unifier builder.
pub(crate) struct UnifierBuilder {
    substitutions: BTreeMap<HoleId, inference::Ty>,
}

impl UnifierBuilder {
    /// Create a new and empty unifier builder.
    fn new() -> Self {
        Self {
            substitutions: BTreeMap::new(),
        }
    }

    /// Apply in-place all the substitutions to a type.
    fn apply_substitutions(&self, ty: &mut inference::Ty) {
        match ty {
            inference::Ty::Hole(hole_id) => {
                // Change the type to the `output` if the type matches the `input`.
                if let Some(output) = self.substitutions.get(hole_id) {
                    *ty = output.clone();
                }
            }
            // if the type is a function, apply the substitutions recursively on the parameters and
            // return types.
            inference::Ty::Func {
                params_ty,
                return_ty,
            } => {
                for ty in params_ty {
                    self.apply_substitutions(ty);
                }
                self.apply_substitutions(return_ty.as_mut());
            }
            // if the type is a tuple, apply the substitutions recursively on the fields.
            inference::Ty::Tuple { fields } => {
                for ty in fields {
                    self.apply_substitutions(ty);
                }
            }
            // Otherwise, left the type as it is.
            inference::Ty::Base(_) => (),
        }
    }

    /// Add a substitution to the unifier.
    pub(crate) fn add_substitution(&mut self, mut substitution: Substitution) {
        // Compose the new substitution with all the already used substitutions.
        self.apply_substitutions(&mut substitution.output);
        // Push the substitution into the substitutions list.
        self.substitutions
            .insert(substitution.input, substitution.output);
    }

    /// Consume this builder and try to create an unifier.
    ///
    /// This fails if any of the substitution's output has inference varaibles in it.
    pub(crate) fn build(self) -> TyResult<Unifier> {
        let mut substitutions = BTreeMap::new();

        for (input, output) in self.substitutions {
            let output = try_concrete(output)?;
            substitutions.insert(input, output);
        }
        Ok(Unifier { substitutions })
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
        inference::Ty::Tuple { fields } => {
            let fields = fields
                .into_iter()
                .map(try_concrete)
                .collect::<TyResult<Vec<ty::Ty>>>()?;

            Ok(ty::Ty::Tuple { fields })
        }
    }
}
