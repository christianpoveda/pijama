use crate::compile::Compile;

use pijama_core::{Expr, FuncId, Local, Program};
use pijama_ty::{base::BaseTy, ty::Ty, ExprId};
use pijama_tycheck::Table;
use pijama_utils::index::IndexMap;

use inkwell::{
    basic_block::BasicBlock,
    builder::Builder,
    context::Context,
    module::{Linkage, Module},
    support::LLVMString,
    targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine},
    types::{BasicType, BasicTypeEnum},
    values::{BasicValueEnum, FunctionValue},
    AddressSpace, OptimizationLevel,
};

use std::path::Path;

/// A compiler for functions.
///
/// This is the main structure for lowering core expressions to LLVM-IR.
pub(crate) struct FuncCompiler<'ctx, 'func> {
    /// The global compiler.
    compiler: &'func Compiler<'ctx>,
    /// The value of the function being compiled.
    func: FunctionValue<'ctx>,
    /// The locals of the function as LLVM basic values.
    locals: IndexMap<Local, BasicValueEnum<'ctx>>,
}

impl<'ctx, 'func> FuncCompiler<'ctx, 'func> {
    /// Create a new compiler for a function and get it ready to lower the body of the function.
    fn new(func_id: FuncId, compiler: &'func Compiler<'ctx>) -> Self {
        // Get the value of the funciton to be compiled from the compiler.
        let func = *compiler
            .funcs
            .get(func_id)
            .expect("Every function should have a value by now.");

        // Create a new map for the locals of the function. Include all the parameters of the
        // function as values in it.
        //
        // This works because the parameters are always the first locals and they have the same
        // order in the funciton's value as in the core representation.
        let locals = IndexMap::from_raw(func.get_params());

        // Add an entry block for the function.
        let entry_bb = compiler.ctx.append_basic_block(func, "");
        compiler.builder.position_at_end(entry_bb);

        // Return the compiler.
        Self {
            compiler,
            func,
            locals,
        }
    }

    /// Compile the body of the function.
    ///
    /// This function assumes that the expression received as parameter is the body of the function
    /// being lowered.
    // FIXME: Maybe this should be called directly after initializing the compiler.
    fn compile_func(mut self, body: Expr) {
        // Compile the body expression into a basic value.
        let return_value = self.compile(body);
        // Build the return instruction with the return value.
        self.compiler.builder.build_return(Some(&return_value));
    }

    /// Compile a term that implements [Compile] using this compiler.
    ///
    /// Using this method is prefered over [Compile::compile_with].
    pub(crate) fn compile<T: Compile<'ctx>>(&mut self, term: T) -> T::Output {
        term.compile_with(self)
    }

    /// Bind a basic value to a local.
    ///
    /// This function panics if the basics values are bound in a different order as the one the
    /// locals had inside the function's core representation.
    pub(crate) fn insert_local(&mut self, local: Local, value: BasicValueEnum<'ctx>) {
        let new_local = self.locals.insert(value);
        assert_eq!(local, new_local, "Locals are in the wrong order.");
    }

    /// Get the compiled value of a local.
    pub(crate) fn get_local(&self, local: Local) -> Option<BasicValueEnum<'ctx>> {
        self.locals.get(local).copied()
    }

    /// Get the compiled pointer value of a function.
    pub(crate) fn get_func(&self, func_id: FuncId) -> Option<BasicValueEnum<'ctx>> {
        self.compiler
            .funcs
            .get(func_id)
            // FIXME: when does this panics?.
            .map(|value| value.as_global_value().as_pointer_value().into())
    }

    /// Add a new basic block at the end of the current function.
    pub(crate) fn add_bb(&self) -> BasicBlock<'ctx> {
        self.ctx().append_basic_block(self.func, "")
    }

    /// Get a reference to LLVM's [Context].
    pub(crate) fn ctx(&self) -> &'ctx Context {
        self.compiler.ctx
    }

    /// Get a reference to LLVM's [Builder].
    pub(crate) fn builder(&self) -> &Builder<'ctx> {
        &self.compiler.builder
    }

    pub(crate) fn get_ty(&self, expr_id: ExprId) -> Option<BasicTypeEnum<'ctx>> {
        let ty = self.compiler.table.get_ty(expr_id)?;

        Some(self.compiler.lower_ty(ty))
    }
}

/// A compiler for programs.
///
/// This struct holds most of the LLVM structures required to compile a program from core to
/// LLVM-IR.
pub(crate) struct Compiler<'ctx> {
    /// LLVM's context.
    ctx: &'ctx Context,
    /// The module being compiled.
    module: Module<'ctx>,
    /// LLVM's instruction builder.
    builder: Builder<'ctx>,
    /// The values of each function in the program.
    funcs: IndexMap<FuncId, FunctionValue<'ctx>>,

    table: Table,
}

impl<'ctx> Compiler<'ctx> {
    /// Create a new empty compiler.
    pub(crate) fn new(ctx: &'ctx Context, table: Table) -> Self {
        Self {
            ctx,
            // We compile everything into a single module for now.
            module: ctx.create_module(""),
            builder: ctx.create_builder(),
            funcs: IndexMap::new(),
            table,
        }
    }

    /// Lower a type into a Basic LLVM type.
    ///
    /// This can be done because function types are represented as pointer types instead.
    fn lower_ty(&self, ty: &Ty) -> BasicTypeEnum<'ctx> {
        match ty {
            Ty::Base(base_ty) => {
                let basic_type = match base_ty {
                    BaseTy::Bool => self.ctx.bool_type(),
                    BaseTy::Int => self.ctx.i64_type(),
                };
                basic_type.into()
            }
            Ty::Func {
                params_ty,
                return_ty,
            } => {
                // Lower each type parameter.
                let params_ty: Vec<_> = params_ty.iter().map(|ty| self.lower_ty(ty)).collect();

                // Lower the return type.
                let return_ty = self.lower_ty(return_ty.as_ref());

                // Build a pointer type to a function.
                return_ty
                    .fn_type(&params_ty, false)
                    // Functions are always global.
                    .ptr_type(AddressSpace::Global)
                    .into()
            }
            Ty::Tuple { fields } => {
                let fields: Vec<_> = fields.iter().map(|ty| self.lower_ty(ty)).collect();

                self.ctx.struct_type(&fields, false).into()
            }
        }
    }

    /// Compile a core program and write it as an object file.
    pub(crate) fn compile(mut self, program: Program, path: &Path) -> Result<(), LLVMString> {
        // Create an LLVM value for each function in the program.
        for (func_id, func) in &program.functions {
            // Lower the types of the parameters of the function.
            let params_ty: Vec<_> = func
                .locals
                .iter()
                .take(func.arity)
                .map(|(_, ty)| self.lower_ty(ty))
                .collect();

            // Lower the return type of the function.
            let return_ty = self.lower_ty(&func.return_ty);

            // Compute the function's type.
            let func_ty = return_ty.fn_type(&params_ty, false);

            // Add a new value with the function's type.
            let func_value = self.module.add_function("", func_ty, None);

            // Be sure that we are inserting the functions in the same order as they were defined.
            assert_eq!(
                func_id,
                self.funcs.insert(func_value),
                "Functions are unorganized."
            );
        }

        // Compile each function.
        for (func_id, func) in program.functions {
            FuncCompiler::new(func_id, &self).compile_func(func.body);
        }

        // Get the value for the main function.
        let main_fn = *self.funcs.get(FuncId::main()).unwrap();

        // Build the `entry` function that calls main and returns an integer.
        //
        // FIXME: clean this when we have printing.
        let entry_type = self.ctx.i64_type().fn_type(&[], false);
        let entry_fn = self
            .module
            .add_function("entry", entry_type, Some(Linkage::External));
        let entry_bb = self.ctx.append_basic_block(entry_fn, "");
        self.builder.position_at_end(entry_bb);
        let result = self
            .builder
            .build_call(main_fn, &[], "")
            .try_as_basic_value()
            .unwrap_left();
        self.builder.build_return(Some(&result));

        Target::initialize_all(&InitializationConfig::default());
        let target_triple = TargetMachine::get_default_triple();
        let target = Target::from_triple(&target_triple).unwrap();

        let target_machine = target
            .create_target_machine(
                &target_triple,
                "generic",
                "",
                OptimizationLevel::Aggressive,
                RelocMode::PIC,
                CodeModel::Default,
            )
            .unwrap();

        // Write the object file.
        target_machine.write_to_file(&self.module, FileType::Object, path)
    }
}
