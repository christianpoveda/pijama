/// Core is Pijama's last IR before compiling to LLVM.
mod atom;
mod expr;
mod func;
mod literal;
mod name;
mod prim_op;
mod program;

pub use atom::Atom;
pub use expr::{Expr, ExprKind};
pub use func::{Func, FuncId};
pub use literal::Literal;
pub use name::{Local, Name};
pub use prim_op::{BinOp, UnOp};
pub use program::Program;
