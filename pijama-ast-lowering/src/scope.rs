use pijama_ast::Ident;
use pijama_hir::Name;

/// A stack-based lexical scope.
///
/// This type is used to track the bindings in the AST in order to map each [Ident] into an
/// unambiguous [Name].
pub(crate) struct Scope<'source> {
    stack: Vec<(Ident<'source>, Name)>,
}

impl<'source> Scope<'source> {
    /// Create a new and empty scope.
    pub(crate) fn new() -> Self {
        Self { stack: Vec::new() }
    }

    /// Push a new identifier onto the current scope and map it to a name.
    pub(crate) fn push_ident(&mut self, ident: Ident<'source>, name: Name) {
        self.stack.push((ident, name));
    }

    /// Remove a the last pushed identifier from the scope.
    ///
    /// This function panics if the scope is empty.
    pub(crate) fn pop_ident(&mut self) {
        self.stack.pop().expect("Scope is empty");
    }

    /// Find an identifier in the current scope and return the name associated with it. Return
    /// `None` if the identifier is not in scope.
    ///
    /// This search is done giving priority to the last identifier pushed onto the current scope.
    pub(crate) fn find_ident(&self, target: &Ident<'source>) -> Option<Name> {
        for (ident, name) in self.stack.iter().rev() {
            if ident.symbol == target.symbol {
                return Some(name.clone());
            }
        }
        None
    }
}
