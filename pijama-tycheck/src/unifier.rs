use std::collections::BTreeMap;

use crate::{
    error::{TyError, TyResult},
    substitution::{RowSubstitution, TySubstitution},
};

use pijama_ty::{
    inference::{self, Row, RowVar, TyVar},
    label::Label,
    ty,
};

/// A type to replace all inference variables by concrete types, if possible.
#[derive(Debug)]
pub struct Unifier {
    ty_substitutions: BTreeMap<TyVar, ty::Ty>,
    row_substitutions: BTreeMap<RowVar, Vec<(Label, ty::Ty)>>,
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
            inference::Ty::Var(hole_id) => self
                .ty_substitutions
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
            inference::Ty::Record(row) => {
                let tail = row.tail();

                let mut fields = row
                    .into_iter()
                    .map(|(label, ty)| (label, self.instantiate(ty)))
                    .collect::<Vec<_>>();

                if let Some(tail) = tail {
                    for (label, ty) in self.row_substitutions.get(&tail).unwrap() {
                        match fields.binary_search_by_key(label, |x| x.0) {
                            Ok(_) => panic!(),
                            Err(index) => fields.insert(index, (*label, ty.clone())),
                        }
                    }
                }

                ty::Ty::Record { fields }
            }
            // Otherwise, left the type as it is.
            inference::Ty::Base(base) => ty::Ty::Base(base),
        }
    }
}

/// An unifier builder.
#[derive(Debug)]
pub(crate) struct UnifierBuilder {
    ty_substitutions: BTreeMap<TyVar, inference::Ty>,
    row_substitutions: BTreeMap<RowVar, Row>,
}

impl UnifierBuilder {
    /// Create a new and empty unifier builder.
    fn new() -> Self {
        Self {
            ty_substitutions: BTreeMap::new(),
            row_substitutions: BTreeMap::new(),
        }
    }

    /// Apply in-place all the substitutions to a type.
    fn apply_ty_substitutions_to_ty(&self, ty: &mut inference::Ty) {
        match ty {
            inference::Ty::Var(hole_id) => {
                // Change the type to the `output` if the type matches the `input`.
                if let Some(output) = self.ty_substitutions.get(hole_id) {
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
                    self.apply_ty_substitutions_to_ty(ty);
                }
                self.apply_ty_substitutions_to_ty(return_ty.as_mut());
            }
            inference::Ty::Record(row) => self.apply_ty_substitutions_to_row(row),
            // Otherwise, left the type as it is.
            inference::Ty::Base(_) => (),
        }
    }

    fn apply_ty_substitutions_to_row(&self, row: &mut Row) {
        for (_, ty) in row.fields_mut() {
            self.apply_ty_substitutions_to_ty(ty);
        }
    }

    fn apply_row_substitutions_to_ty(&self, ty: &mut inference::Ty) {
        match ty {
            inference::Ty::Func {
                params_ty,
                return_ty,
            } => {
                for ty in params_ty {
                    self.apply_row_substitutions_to_ty(ty);
                }
                self.apply_row_substitutions_to_ty(return_ty.as_mut());
            }
            inference::Ty::Record(row) => self.apply_row_substitutions_to_row(row),
            inference::Ty::Base(_) | inference::Ty::Var(_) => (),
        }
    }

    fn apply_row_substitutions_to_row(&self, row: &mut Row) {
        for (_, ty) in row.fields_mut() {
            self.apply_row_substitutions_to_ty(ty);
        }

        if let Some(tail) = row.tail() {
            if let Some(output) = self.row_substitutions.get(&tail) {
                for (label, ty) in output.fields().iter().cloned() {
                    row.extend(label, ty);
                }

                row.set_tail(output.tail());
            }
        }
    }

    pub(crate) fn add_row_substitution(&mut self, mut substitution: RowSubstitution) {
        self.apply_ty_substitutions_to_row(&mut substitution.output);
        self.apply_row_substitutions_to_row(&mut substitution.output);

        self.row_substitutions
            .insert(substitution.input, substitution.output);
    }

    /// Add a substitution to the unifier.
    pub(crate) fn add_ty_substitution(&mut self, mut substitution: TySubstitution) {
        // Compose the new substitution with all the already used substitutions.
        self.apply_ty_substitutions_to_ty(&mut substitution.output);
        self.apply_row_substitutions_to_ty(&mut substitution.output);
        // Push the substitution into the substitutions list.
        self.ty_substitutions
            .insert(substitution.input, substitution.output);
    }

    /// Consume this builder and try to create an unifier.
    ///
    /// This fails if any of the substitution's output has inference varaibles in it.
    pub(crate) fn build(self) -> TyResult<Unifier> {
        let mut ty_substitutions = BTreeMap::new();
        let mut row_substitutions = BTreeMap::new();

        for (input, output) in self.ty_substitutions {
            let output = try_concrete_ty(output)?;
            ty_substitutions.insert(input, output);
        }

        for (input, output) in self.row_substitutions {
            let output = try_concrete_row(output)?;
            row_substitutions.insert(input, output);
        }

        Ok(Unifier {
            ty_substitutions,
            row_substitutions,
        })
    }
}

/// Try to convert an inference type into a concrete type without holes. Error with
/// the first `HoleId` found otherwise.
fn try_concrete_ty(ty: inference::Ty) -> TyResult<ty::Ty> {
    match ty {
        inference::Ty::Base(base) => Ok(ty::Ty::Base(base)),
        inference::Ty::Var(hole_id) => Err(TyError::HoleFound(hole_id)),
        inference::Ty::Func {
            params_ty,
            return_ty,
        } => {
            let params_ty = params_ty
                .into_iter()
                .map(try_concrete_ty)
                .collect::<TyResult<Vec<ty::Ty>>>()?;
            let return_ty = Box::new(try_concrete_ty(*return_ty)?);

            Ok(ty::Ty::Func {
                params_ty,
                return_ty,
            })
        }
        inference::Ty::Record(row) => Ok(ty::Ty::Record {
            fields: try_concrete_row(row)?,
        }),
    }
}

fn try_concrete_row(row: Row) -> TyResult<Vec<(Label, ty::Ty)>> {
    if let Some(_) = row.tail() {
        panic!()
    }

    row.into_iter()
        .map(|(label, ty)| Ok((label, try_concrete_ty(ty)?)))
        .collect()
}
