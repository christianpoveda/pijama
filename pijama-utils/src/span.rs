//! Utilities to track the location of items in the source code.
use std::fmt;

#[derive(Clone, Copy)]
/// The location of a segment of source code.
///
/// Locations are measured in bytes from the start of the file being compiled. Nothing enforces
/// that `start` > `end` but it is expected to be that way.
pub struct Span {
    /// The location of the beginning of the segment.
    pub start: usize,
    /// The location of the end of the segment.
    pub end: usize,
}

impl Span {
    /// Create a new span.
    pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// Create a placeholder span that cannot be an actual location in the source code.
    pub const fn dummy() -> Self {
        Self::new(usize::MAX, usize::MAX)
    }

    /// Check if the current span is a dummy
    pub const fn is_dummy(&self) -> bool {
        let dummy = Self::dummy();
        self.start == dummy.start && self.end == dummy.end
    }

    /// Consume the current span and return a new one with a different `start`.
    pub const fn with_start(mut self, start: usize) -> Self {
        self.start = start;
        self
    }

    /// Consume the current span and return a new one with a different `end`.
    pub fn with_end(mut self, end: usize) -> Self {
        self.end = end;
        self
    }

    /// Create a new span that whose `start` and `end` are equal to the `start` of the current
    /// span.
    pub fn at_start(&self) -> Self {
        Self::new(self.start, self.start)
    }

    /// Create a new span that whose `start` and `end` are equal to the `end` of the current
    /// span.
    pub fn at_end(&self) -> Self {
        Self::new(self.end, self.end)
    }

    /// Join two spans by taking the smallest `start` between the two as the `start` and the
    /// largest `end` as the `end` of the new span.
    pub fn join(self, rhs: Self) -> Self {
        Self::new(self.start.min(rhs.start), self.end.max(rhs.end))
    }
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

/// Create a new type that has two fields `kind` and `span`.
#[macro_export]
macro_rules! spanned_type {
    ($vis:vis $type:ident$(<$($lifetime:lifetime ),* $($param:ident $(= $default:path )? ),*>)?, $kind:ident) => {
        #[derive(Debug)]
        $vis struct $type$(<$($lifetime,)* $($param $(= $default)?,)*>)* {
            pub kind: $kind$(<$($lifetime,)* $($param,)*>)?,
            pub span: $crate::span::Span,
        }
    };
}
