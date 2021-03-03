//! Types for inference.
mod context;
mod row;
mod ty;

pub use context::TyContext;
pub use row::{Row, RowVar};
pub use ty::{Ty, TyVar};
