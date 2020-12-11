/// A primitive operator.
#[derive(Debug)]
pub enum PrimOp {
    /// The logical negation operator.
    Not,
    /// The arithmetic negation operator.
    Neg,
    /// The arithmetic addition operator.
    Add,
    /// The arithmetic substraction operator.
    Sub,
    /// The arithmetic multiplication operator.
    Mul,
    /// The arithmetic division operator.
    Div,
    /// The arithmetic remainder operator.
    Rem,
    /// The logical conjunction operator.
    And,
    /// The logical disjunction operator.
    Or,
    /// The equality operator.
    Eq,
    /// The "not equal to" operator.
    Neq,
    /// The "less than" operator.
    Lt,
    /// The "greater than" operator.
    Gt,
    /// The "less than or equal to" operator.
    Lte,
    /// The "greater than or equal to" operator.
    Gte,
}
