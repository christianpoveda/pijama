use crate::compile::Compile;

use pijama_core::{Expr, FuncId, Local, Program};
use pijama_ty::{base::BaseTy, ty::Ty};
use pijama_utils::index::IndexMap;

use inkwell::{
    basic_block::BasicBlock,
    builder::Builder,
    context::Context,
    module::Module,
    types::{BasicType, BasicTypeEnum},
    values::{BasicValueEnum, FunctionValue},
    AddressSpace, OptimizationLevel,
};

pub(crate) struct FuncCompiler<'ctx, 'func> {
    compiler: &'func Compiler<'ctx>,
    func: FunctionValue<'ctx>,
    locals: IndexMap<Local, BasicValueEnum<'ctx>>,
}

impl<'ctx, 'func> FuncCompiler<'ctx, 'func> {
    fn new(func_id: FuncId, compiler: &'func Compiler<'ctx>) -> Self {
        let func = *compiler.funcs.get(func_id).unwrap();
        let locals = IndexMap::from_raw(func.get_params());

        let entry_bb = compiler.ctx.append_basic_block(func, "");
        compiler.builder.position_at_end(entry_bb);

        Self {
            compiler,
            func,
            locals,
        }
    }

    fn compile_func(mut self, body: Expr) {
        let return_value = self.compile(body);
        self.compiler.builder.build_return(Some(&return_value));
    }

    pub(crate) fn compile<T: Compile<'ctx>>(&mut self, term: T) -> T::Output {
        term.compile_with(self)
    }

    pub(crate) fn insert_local(&mut self, local: Local, value: BasicValueEnum<'ctx>) {
        let new_local = self.locals.insert(value);
        assert_eq!(local, new_local);
    }

    pub(crate) fn get_local(&self, local: Local) -> Option<BasicValueEnum<'ctx>> {
        self.locals.get(local).copied()
    }

    pub(crate) fn get_func(&self, func_id: FuncId) -> Option<BasicValueEnum<'ctx>> {
        self.compiler
            .funcs
            .get(func_id)
            .map(|value| value.as_global_value().as_pointer_value().into())
    }

    pub(crate) fn add_bb(&self) -> BasicBlock<'ctx> {
        self.ctx().append_basic_block(self.func, "")
    }

    pub(crate) fn ctx(&self) -> &'ctx Context {
        self.compiler.ctx
    }

    pub(crate) fn builder(&self) -> &Builder<'ctx> {
        &self.compiler.builder
    }
}

pub(crate) struct Compiler<'ctx> {
    ctx: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    funcs: IndexMap<FuncId, FunctionValue<'ctx>>,
}

impl<'ctx> Compiler<'ctx> {
    pub(crate) fn new(ctx: &'ctx Context) -> Self {
        Self {
            ctx,
            module: ctx.create_module(""),
            builder: ctx.create_builder(),
            funcs: IndexMap::new(),
        }
    }

    fn lower_ty(&self, ty: &Ty) -> BasicTypeEnum<'ctx> {
        match ty {
            Ty::Base(base_ty) => {
                let basic_type = match base_ty {
                    BaseTy::Unit => self.ctx.i8_type(),
                    BaseTy::Bool => self.ctx.i8_type(),
                    BaseTy::Integer => self.ctx.i64_type(),
                };
                basic_type.into()
            }
            Ty::Func {
                params_ty,
                return_ty,
            } => {
                let params_ty: Vec<_> = params_ty.iter().map(|ty| self.lower_ty(ty)).collect();

                let return_ty = self.lower_ty(return_ty.as_ref());

                return_ty
                    .fn_type(&params_ty, false)
                    .ptr_type(AddressSpace::Global)
                    .into()
            }
        }
    }

    pub(crate) fn compile_and_run(mut self, program: Program) {
        for (func_id, func) in &program.functions {
            let params_ty: Vec<_> = func
                .locals
                .iter()
                .take(func.arity)
                .map(|(_, ty)| self.lower_ty(ty))
                .collect();

            let return_ty = self.lower_ty(&func.return_ty);

            let func_ty = return_ty.fn_type(&params_ty, false);

            let func_value = self.module.add_function("", func_ty, None);
            assert_eq!(func_id, self.funcs.insert(func_value));
        }

        for (func_id, func) in program.functions {
            FuncCompiler::new(func_id, &self).compile_func(func.body);
        }

        self.module.print_to_stderr();

        let execution_engine = self
            .module
            .create_jit_execution_engine(OptimizationLevel::Aggressive)
            .unwrap();

        let result =
            unsafe { execution_engine.run_function(*self.funcs.get(FuncId::main()).unwrap(), &[]) };
        println!("{}", result.as_int(true));
    }
}
