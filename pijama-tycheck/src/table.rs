use crate::Unifier;

use pijama_ty::{inference, ty, ExprId};
use pijama_utils::index::IndexMap;

pub struct Table {
    types: IndexMap<ExprId, ty::Ty>,
}

impl Table {
    pub fn store_ty(&mut self, ty: ty::Ty) -> ExprId {
        self.types.insert(ty)
    }

    pub fn get_ty(&self, expr_id: ExprId) -> Option<&ty::Ty> {
        self.types.get(expr_id)
    }

    pub fn builder(len_table: usize) -> TableBuilder {
        TableBuilder {
            types: IndexMap::from_raw((0..len_table).map(|_| None).collect()),
        }
    }
}

pub struct TableBuilder {
    types: IndexMap<ExprId, Option<inference::Ty>>,
}

impl TableBuilder {
    pub fn store_ty(&mut self, expr_id: ExprId, ty: inference::Ty) {
        assert!(self.types.get_mut(expr_id).unwrap().replace(ty).is_none())
    }

    pub fn get_ty(&self, expr_id: ExprId) -> Option<&inference::Ty> {
        self.types.get(expr_id).and_then(|ty| ty.as_ref())
    }

    pub fn build(self, unifier: &Unifier) -> Result<Table, ExprId> {
        let types = IndexMap::from_raw(
            self.types
                .into_iter()
                .map(|(expr_id, ty)| {
                    if let Some(ty) = ty {
                        Ok(unifier.instantiate(ty))
                    } else {
                        Err(expr_id)
                    }
                })
                .collect::<Result<Vec<ty::Ty>, ExprId>>()?,
        );

        Ok(Table { types })
    }
}
