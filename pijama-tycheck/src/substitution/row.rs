use pijama_ty::inference::{Row, RowVar, Ty};

#[derive(Debug)]
pub(crate) struct RowSubstitution {
    /// The row variable to be replaced.
    pub(crate) input: RowVar,
    /// The replacement.
    pub(crate) output: Row,
}

impl RowSubstitution {
    /// Create a new substitution.
    pub(crate) fn new(input: RowVar, output: Row) -> Self {
        Self { input, output }
    }

    pub(crate) fn apply_to_row(&self, row: &mut Row) {
        for (_, ty) in row.fields_mut() {
            self.apply_to_ty(ty);
        }

        if let Some(tail) = row.tail() {
            if tail == self.input {
                for (label, ty) in self.output.fields().iter().cloned() {
                    row.extend(label, ty);
                }

                row.set_tail(self.output.tail());
            }
        }
    }

    pub(crate) fn apply_to_ty(&self, ty: &mut Ty) {
        match ty {
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
            Ty::Var(_) | Ty::Base(_) => (),
        }
    }
}
