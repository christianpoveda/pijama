use crate::{
    inference::ty::{Ty, TyVar},
    label::Label,
};

use pijama_utils::new_index;

new_index! {
    #[doc = "An unique ID used to represent row variables."]
    #[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
    RowVar
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Row {
    fields: Vec<(Label, Ty)>,
    tail: Option<RowVar>,
}

impl Row {
    pub fn strict(fields: Vec<(Label, Ty)>) -> Self {
        Self { fields, tail: None }
    }

    pub fn relaxed(fields: Vec<(Label, Ty)>, tail: RowVar) -> Self {
        Self {
            fields,
            tail: Some(tail),
        }
    }

    /// Check if the type has a "hole" type with the given ID.
    pub fn contains_ty(&self, ty_var: TyVar) -> bool {
        self.fields.iter().any(|(_, ty)| ty.contains_ty(ty_var))
    }

    pub fn contains_row(&self, row_var: RowVar) -> bool {
        Some(row_var) == self.tail || self.fields.iter().any(|(_, ty)| ty.contains_row(row_var))
    }

    pub fn tail(&self) -> Option<RowVar> {
        self.tail
    }

    pub fn fields(&self) -> &[(Label, Ty)] {
        &self.fields
    }

    pub fn fields_mut(&mut self) -> &mut [(Label, Ty)] {
        &mut self.fields
    }

    pub fn extend(&mut self, label: Label, ty: Ty) {
        match self.fields.binary_search_by_key(&label, |x| x.0) {
            Ok(_) => panic!(),
            Err(index) => self.fields.insert(index, (label, ty)),
        }
    }

    pub fn get_ty(&self, label: Label) -> Option<&Ty> {
        self.fields
            .binary_search_by_key(&label, |x| x.0)
            .ok()
            .map(|index| &self.fields[index].1)
    }

    pub fn remove_field(&mut self, label: Label) -> Option<Ty> {
        self.fields
            .binary_search_by_key(&label, |x| x.0)
            .ok()
            .map(|index| self.fields.remove(index).1)
    }

    pub fn pop_field(&mut self) -> Option<(Label, Ty)> {
        self.fields.pop()
    }

    pub fn set_tail(&mut self, tail: Option<RowVar>) {
        self.tail = tail;
    }

    pub fn into_iter(self) -> impl Iterator<Item = (Label, Ty)> {
        self.fields.into_iter()
    }
}
