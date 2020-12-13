pub type LowerResult<T> = Result<T, LowerError>;

/// A lowering error.
///
/// Each variant here represents the reason why it was not possible to lower the HIR.
#[derive(Debug)]
pub enum LowerError {}
