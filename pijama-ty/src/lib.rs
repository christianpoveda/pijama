pub mod base;
pub mod inference;
pub mod label;
pub mod ty;

use pijama_utils::new_index;

new_index! {
    #[doc = "An unique identifier for expressions, used to track information between IRs"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    ExprId
}
