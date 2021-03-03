use pijama_utils::new_index;

new_index! {
    #[doc = "An unique ID used to represent a record's label."]
    #[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
    Label
}
