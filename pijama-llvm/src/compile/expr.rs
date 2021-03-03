use crate::{compile::Compile, compiler::FuncCompiler};

use pijama_core::{BinOp, Expr, ExprKind};
use pijama_utils::index::Index;

use inkwell::{values::BasicValueEnum, IntPredicate};

impl<'ctx> Compile<'ctx> for Expr {
    type Output = BasicValueEnum<'ctx>;

    fn compile_with(self, compiler: &mut FuncCompiler<'ctx, '_>) -> Self::Output {
        match self.kind {
            // Just compile the atom inside the expression.
            ExprKind::Atom(atom) => compiler.compile(atom),
            ExprKind::Let { lhs, rhs, body } => {
                // First, compile the right-hand side. This should return a basic value.
                let rhs = compiler.compile(*rhs);
                // Bind that basic value to the left-hand side.
                compiler.insert_local(lhs, rhs);
                // Compile that value keeping the binding. It is not necessary to remove the
                // binding because locals are supposed to be unique inside each function.
                compiler.compile(*body)
            }
            ExprKind::Call { func, args } => {
                // First, compile the called function and turn the basic value into a pointer. This
                // always succeeds because functions are stored as pointers and any name referring
                // directly or indirectly to a function will be bound to a pointer.
                let func = compiler.compile(func).into_pointer_value();
                // Compile every argument into a basic value and collect them.
                let args: Vec<_> = args.into_iter().map(|arg| compiler.compile(arg)).collect();

                // Compile the actual call.
                compiler
                    .builder()
                    .build_call(func, &args, "")
                    // This never fails because don't have functions returning void.
                    .try_as_basic_value()
                    .unwrap_left()
            }

            ExprKind::BinaryOp {
                bin_op,
                left_op,
                right_op,
            } => {
                // Compile the operands into a basic value and collect them. All of these should be
                // integers.
                let left_op = compiler.compile(left_op).into_int_value();
                let right_op = compiler.compile(right_op).into_int_value();

                // FIXME: Figure out how to abstract this. Maybe use a macro?.
                // FIXME: Take an stance about overflows.
                match bin_op {
                    BinOp::Add => compiler
                        .builder()
                        .build_int_add(left_op, right_op, "")
                        .into(),
                    BinOp::Sub => compiler
                        .builder()
                        .build_int_sub(left_op, right_op, "")
                        .into(),
                    BinOp::Mul => compiler
                        .builder()
                        .build_int_mul(left_op, right_op, "")
                        .into(),
                    BinOp::Gt => compiler
                        .builder()
                        .build_int_compare(IntPredicate::SGT, left_op, right_op, "")
                        .into(),
                    // FIXME: Do the other operations.
                    _ => todo!(),
                }
            }
            // FIXME: Do the other operations.
            ExprKind::UnaryOp { .. } => todo!(),
            ExprKind::Cond {
                cond,
                do_branch,
                else_branch,
            } => {
                // Compile the condition to be discriminated. This should be a boolean (which
                // counts as an integer value for LLVM).
                let cond = compiler.compile(cond).into_int_value();

                // Add basic blocks for each branch of the conditional.
                let do_bb = compiler.add_bb();
                let else_bb = compiler.add_bb();
                // Add a basic block to join back the control flow after the branches.
                let join_bb = compiler.add_bb();

                // Build a conditional jump using the conditional value.
                compiler
                    .builder()
                    .build_conditional_branch(cond, do_bb, else_bb);

                // Compile the do branch in the do block.
                compiler.builder().position_at_end(do_bb);
                let do_value = compiler.compile(*do_branch);
                // Jump unconditionally to the join block.
                compiler.builder().build_unconditional_branch(join_bb);

                // Compile the do branch in the else block.
                compiler.builder().position_at_end(else_bb);
                let else_value = compiler.compile(*else_branch);
                // Jump unconditionally to the join block.
                compiler.builder().build_unconditional_branch(join_bb);

                // Go to the join block.
                compiler.builder().position_at_end(join_bb);
                // Build a phi value with the type of the branches (both branches should have the
                // same type).
                let join_value = compiler.builder().build_phi(do_value.get_type(), "");
                // Add the incoming values to the phi using the values and blocks of each branch.
                join_value.add_incoming(&[(&do_value, do_bb), (&else_value, else_bb)]);
                // This should be a basic value.
                join_value.as_basic_value()
            }
            ExprKind::Record { fields } => {
                let ty = compiler.get_ty(self.id).unwrap().into_struct_type();

                let mut value = ty.get_undef();

                // FIXME: THe index of a label doesn't have to be the actual index in the struct.
                for (label, field) in fields.into_iter() {
                    let field = compiler.compile(field);
                    value = compiler
                        .builder()
                        .build_insert_value(value, field, label.index() as u32, "")
                        .unwrap()
                        .into_struct_value();
                }

                value.into()
            }
            ExprKind::Projection { record, label } => {
                let value = compiler.compile(record).into_struct_value();

                // FIXME: THe index of a label doesn't have to be the actual index in the struct.
                compiler
                    .builder()
                    .build_extract_value(value, label.index() as u32, "")
                    .unwrap()
                    .into()
            }
        }
    }
}
