use crate::{compile::Compile, compiler::FuncCompiler};

use pijama_core::{Expr, PrimOp};

use inkwell::{values::BasicValueEnum, IntPredicate};

impl<'ctx> Compile<'ctx> for Expr {
    type Output = BasicValueEnum<'ctx>;

    fn compile_with(self, compiler: &mut FuncCompiler<'ctx, '_>) -> Self::Output {
        match self {
            Expr::Atom(atom) => compiler.compile(atom),
            Expr::Let { lhs, rhs, body } => {
                let rhs = compiler.compile(*rhs);

                compiler.insert_local(lhs, rhs);

                compiler.compile(*body)
            }
            Expr::Call { func, args } => {
                let func = compiler.compile(func).into_pointer_value();
                let args: Vec<_> = args.into_iter().map(|arg| compiler.compile(arg)).collect();

                compiler
                    .builder()
                    .build_call(func, &args, "")
                    .try_as_basic_value()
                    .unwrap_left()
            }

            Expr::PrimitiveOp { prim_op, ops } => {
                let ops: Vec<_> = ops.into_iter().map(|arg| compiler.compile(arg)).collect();

                match prim_op {
                    PrimOp::Sub => compiler
                        .builder()
                        .build_int_sub(ops[0].into_int_value(), ops[1].into_int_value(), "")
                        .into(),
                    PrimOp::Mul => compiler
                        .builder()
                        .build_int_mul(ops[0].into_int_value(), ops[1].into_int_value(), "")
                        .into(),
                    PrimOp::Gt => compiler
                        .builder()
                        .build_int_compare(
                            IntPredicate::SGT,
                            ops[0].into_int_value(),
                            ops[1].into_int_value(),
                            "",
                        )
                        .into(),
                    _ => todo!(),
                }
            }
            Expr::Cond {
                cond,
                do_branch,
                else_branch,
            } => {
                let cond = compiler.compile(cond).into_int_value();

                let do_bb = compiler.add_bb();
                let else_bb = compiler.add_bb();
                let join_bb = compiler.add_bb();

                compiler
                    .builder()
                    .build_conditional_branch(cond, do_bb, else_bb);

                compiler.builder().position_at_end(do_bb);
                let do_value = compiler.compile(*do_branch);
                compiler.builder().build_unconditional_branch(join_bb);

                compiler.builder().position_at_end(else_bb);
                let else_value = compiler.compile(*else_branch);
                compiler.builder().build_unconditional_branch(join_bb);

                compiler.builder().position_at_end(join_bb);
                let join_value = compiler.builder().build_phi(do_value.get_type(), "");
                join_value.add_incoming(&[(&do_value, do_bb), (&else_value, else_bb)]);
                join_value.as_basic_value()
            }
        }
    }
}
