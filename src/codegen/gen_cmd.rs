use crate::{
    codegen::codegen_ctx::CodegenCtx,
    parser::ast::Cmd,
    typechecker::ast::{
        TypedBinding, TypedCmd, TypedExpr, TypedLValue, TypedStmt, TypedStructField, TypedType,
    },
};

pub(super) fn gen_cmd(ctx: &mut CodegenCtx, cmd: TypedCmd<'_, '_>) {
    match cmd.value {
        Cmd::Read(name, lvalue) => gen_read(ctx, name, lvalue),
        Cmd::Write(expr, filename) => gen_write(ctx, expr, filename),
        Cmd::Let(lvalue, expr) => gen_let(ctx, lvalue, expr),
        Cmd::Assert(expr, message) => gen_assert(ctx, expr, message),
        Cmd::Print(string) => gen_print(ctx, string),
        Cmd::Show(expr) => gen_show(ctx, expr),
        Cmd::Time(cmd) => gen_time(ctx, cmd),
        Cmd::Fn {
            name,
            params,
            return_type,
            body,
        } => gen_fn(ctx, name, params, return_type, body),
        Cmd::Struct { name, fields } => gen_struct(ctx, name, fields),
    }
}

fn gen_read(ctx: &mut CodegenCtx, name: &str, lvalue: &TypedLValue<'_>) {
    todo!()
}

fn gen_write(ctx: &mut CodegenCtx, expr: &TypedExpr<'_, '_>, filename: &str) {
    todo!()
}

fn gen_let(ctx: &mut CodegenCtx, lvalue: &TypedLValue<'_>, expr: &TypedExpr<'_, '_>) {
    todo!()
}

fn gen_assert(ctx: &mut CodegenCtx, expr: &TypedExpr<'_, '_>, message: &str) {
    todo!()
}

fn gen_print(ctx: &mut CodegenCtx, string: &str) {
    todo!()
}

fn gen_show(ctx: &mut CodegenCtx, expr: &TypedExpr<'_, '_>) {
    todo!()
}

fn gen_time(ctx: &mut CodegenCtx, cmd: &TypedCmd<'_, '_>) {
    todo!()
}

fn gen_fn(
    ctx: &mut CodegenCtx,
    name: &str,
    params: Vec<&TypedBinding<'_, '_>>,
    return_type: &TypedType<'_, '_>,
    body: Vec<&TypedStmt<'_, '_>>,
) {
    todo!()
}

fn gen_struct(ctx: &mut CodegenCtx, name: &str, fields: Vec<&TypedStructField<'_, '_>>) {
    todo!()
}
