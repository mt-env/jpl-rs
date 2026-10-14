use cranelift::frontend::{self, FunctionBuilderContext};

use crate::typechecker::ast::TypedProgram;

mod codegen_ctx;
mod gen_cmd;
mod gen_expr;
mod gen_stmt;

pub fn gen_obj_file(typed_program: &TypedProgram) -> Vec<u8> {
    for item in typed_program {}
    let ctx = FunctionBuilderContext::new();
    todo!()
}
