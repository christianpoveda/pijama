use crate::{
    error::{LowerErrorKind, LowerResult},
    lowering::Lower,
    scope::Scope,
};

use pijama_ast as ast;
use pijama_hir as hir;
use pijama_ty::{
    inference::{Ty, TyContext},
    ExprId,
};
use pijama_utils::{index::IndexMap, span::Span};

/// The main structure to lower the AST.
pub(crate) struct LowerContext<'source, 'tcx> {
    /// The typing context.
    pub(crate) tcx: &'tcx TyContext,
    /// Functions that have already been lowered.
    functions: IndexMap<hir::FuncId, Option<hir::Func>>,
    /// The scope for the current function.
    pub(crate) scope: Scope<'source>,
    /// The global scope.
    pub(crate) global_scope: Scope<'source>,
    /// The type annotations for the locals of the current function.
    pub(crate) locals: IndexMap<hir::Local, Ty>,
}

impl<'source, 'tcx> LowerContext<'source, 'tcx> {
    /// Create a new and empty lowering context.
    pub(crate) fn new(tcx: &'tcx TyContext) -> Self {
        Self {
            tcx,
            functions: IndexMap::new(),
            locals: IndexMap::new(),
            scope: Scope::new(),
            global_scope: Scope::new(),
        }
    }

    /// Lower the AST representation of a program and consume the context in the process.
    pub(crate) fn lower_program(
        mut self,
        mut program: ast::Program<'source>,
    ) -> LowerResult<'source, hir::Program> {
        // Find the position of the main function. Error if there is no main function.
        let (main_pos, main_ident) = program
            .functions
            .iter()
            .enumerate()
            .find_map(|(pos, func)| {
                if "main" == func.ident.symbol {
                    Some((pos, func.ident.clone()))
                } else {
                    None
                }
            })
            .ok_or_else(|| LowerErrorKind::MainNotFound.into_err(Span::dummy()))?;

        // Assign the first `FuncId` to the main function.
        let main_id = self.functions.insert(None);
        // Push the main's name onto the global scope.
        self.global_scope
            .push_ident(main_ident, hir::Name::FuncPtr(main_id));
        // Remove the main function from the program so we don't assign another `FuncId` to it.
        let main_func = program.functions.remove(main_pos);

        let mut func_ids = Vec::with_capacity(program.functions.len());
        // Assign `FuncId`s to the remaining functions.
        for function in &program.functions {
            let func_id = self.functions.insert(None);
            // Push the function's name onto the global scope.
            self.global_scope
                .push_ident(function.ident.clone(), hir::Name::FuncPtr(func_id));
            // Store the `FuncId` of the function for easy access during lowering.
            func_ids.push(func_id);
        }

        // Lower the main function.
        *self
            .functions
            .get_mut(main_id)
            .expect("Functions should be in scope before lowering them.") =
            Some(self.lower(main_func)?);

        // Lower the remaining functions.
        for (func_id, function) in func_ids.into_iter().zip(program.functions.into_iter()) {
            *self
                .functions
                .get_mut(func_id)
                .expect("Functions should be in scope before lowering them.") =
                Some(self.lower(function)?);
        }

        // Be sure that all functions have been lowererqleqaxad.
        let functions = self
            .functions
            .into_raw()
            .into_iter()
            .map(|function| function.expect("All functions should have been lowered already."))
            .collect::<Vec<hir::Func>>();

        // Return a HIR program.
        Ok(hir::Program {
            functions: IndexMap::from_raw(functions),
        })
    }

    pub(crate) fn new_id(&self) -> ExprId {
        self.tcx.new_expr_id()
    }

    /// Lower a term that implements the [Lower] trait.
    pub(crate) fn lower<T: Lower<'source, 'tcx>>(
        &mut self,
        term: T,
    ) -> LowerResult<'source, T::Output> {
        term.lower_with(self)
    }
}
