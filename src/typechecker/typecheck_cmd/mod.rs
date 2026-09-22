use crate::{
    parser::ast::{Cmd, ParsedBinding, ParsedCmd, ParsedExpr, ParsedStmt, ParsedType},
    typechecker::{
        ast::{TypeError, TypedCmd},
        typecheck_binding,
        typecheck_ctx::TypecheckCtx,
        typecheck_expr, typecheck_stmt, typecheck_type,
    },
};

pub(super) fn typecheck_cmd<'src, 'old, 'new>(
    ctx: &mut TypecheckCtx<'src, 'new>,
    cmd: &'old ParsedCmd<'src, 'old>,
) -> Result<&'new TypedCmd<'src, 'new>, TypeError<'src, 'new>> {
    let loc = cmd.offset;
    match &cmd.value {
        Cmd::Show(expr) => typecheck_show(ctx, loc, expr),
        Cmd::Struct { name, fields } => typecheck_struct(ctx, loc, name, fields),
        Cmd::Read(name, lvalue) => todo!(),
        Cmd::Write(expr, name) => typecheck_write(ctx, loc, expr, name),
        Cmd::Let(lvalue, expr) => todo!(),
        Cmd::Assert(expr, string) => typecheck_assert(ctx, loc, expr, string),
        Cmd::Print(string) => typecheck_print(ctx, loc, string),
        Cmd::Time(cmd) => typecheck_time(ctx, loc, cmd),
        Cmd::Fn {
            name,
            params,
            return_type,
            body,
        } => typecheck_fn(ctx, loc, name, params, return_type, body),
    }
}

fn typecheck_show<'src, 'old, 'new>(
    ctx: &mut TypecheckCtx<'src, 'new>,
    offset: usize,
    expr: &'old ParsedExpr<'src, 'old>,
) -> Result<&'new TypedCmd<'src, 'new>, TypeError<'src, 'new>> {
    let typed_expr = typecheck_expr::infer(ctx, expr)?;
    Ok(TypedCmd::new(ctx, offset, Cmd::Show(typed_expr)))
}

fn typecheck_struct<'src, 'old, 'new>(
    ctx: &mut TypecheckCtx<'src, 'new>,
    offset: usize,
    name: &'src str,
    fields: &Vec<(&'src str, &'old ParsedType<'src, 'old>)>,
) -> Result<&'new TypedCmd<'src, 'new>, TypeError<'src, 'new>> {
    let mut resolved_fields = Vec::new();
    let mut typed_fields = Vec::new();
    for field in fields {
        // resolve to a typevalue
        let resolved_tyval = typecheck_type::typevalue_of_type(ctx, field.1)?;
        resolved_fields.push((field.0, resolved_tyval));
        // copy the field name and type into the new arena allocator
        let field_type = typecheck_type::typecheck_type(ctx, field.1)?;
        typed_fields.push((field.0, field_type));
    }
    ctx.add_struct_info(name, resolved_fields);
    Ok(TypedCmd::new(
        ctx,
        offset,
        Cmd::Struct {
            name,
            fields: typed_fields,
        },
    ))
}

fn typecheck_write<'src, 'old, 'new>(
    ctx: &mut TypecheckCtx<'src, 'new>,
    offset: usize,
    expr: &'old ParsedExpr<'src, 'old>,
    name: &'src str,
) -> Result<&'new TypedCmd<'src, 'new>, TypeError<'src, 'new>> {
    let typed_expr = typecheck_expr::infer(ctx, expr)?;
    Ok(TypedCmd::new(ctx, offset, Cmd::Write(typed_expr, name)))
}

fn typecheck_assert<'src, 'old, 'new>(
    ctx: &mut TypecheckCtx<'src, 'new>,
    offset: usize,
    expr: &'old ParsedExpr<'src, 'old>,
    msg: &'src str,
) -> Result<&'new TypedCmd<'src, 'new>, TypeError<'src, 'new>> {
    let typed_expr = typecheck_expr::infer(ctx, expr)?;
    Ok(TypedCmd::new(ctx, offset, Cmd::Assert(typed_expr, msg)))
}

fn typecheck_print<'src, 'old, 'new>(
    ctx: &mut TypecheckCtx<'src, 'new>,
    offset: usize,
    msg: &'src str,
) -> Result<&'new TypedCmd<'src, 'new>, TypeError<'src, 'new>> {
    Ok(TypedCmd::new(ctx, offset, Cmd::Print(msg)))
}

fn typecheck_time<'src, 'old, 'new>(
    ctx: &mut TypecheckCtx<'src, 'new>,
    offset: usize,
    cmd: &'old ParsedCmd<'src, 'old>,
) -> Result<&'new TypedCmd<'src, 'new>, TypeError<'src, 'new>> {
    let typed_cmd = typecheck_cmd(ctx, cmd)?;
    Ok(TypedCmd::new(ctx, offset, Cmd::Time(typed_cmd)))
}

fn typecheck_fn<'src, 'old, 'new>(
    ctx: &mut TypecheckCtx<'src, 'new>,
    offset: usize,
    name: &'src str,
    params: &'old Vec<&'old ParsedBinding<'src, 'old>>,
    return_type: &'old ParsedType<'src, 'old>,
    body: &Vec<&'old ParsedStmt<'src, 'old>>,
) -> Result<&'new TypedCmd<'src, 'new>, TypeError<'src, 'new>> {
    // check and bind all parameters into the function body scope
    ctx.push_scope();
    let mut typed_params = Vec::new();
    for binding in params {
        let typed_param = typecheck_binding::typecheck_binding(ctx, binding)?;
        typed_params.push(typed_param);
        let typed_lvalue = typed_param.value.lvalue;
        let resolved_ty = typecheck_type::typevalue_of_type(ctx, typed_param.value.ty)?;
        ctx.bind(typed_lvalue, resolved_ty);
    }

    // construct ast node for return type
    let typed_return_type = typecheck_type::typecheck_type(ctx, return_type)?;
    let ret_tyval = typecheck_type::typevalue_of_type(ctx, return_type)?;

    // check each statement in the body
    let mut typed_body = Vec::new();
    for stmt in body {
        let typed_stmt = typecheck_stmt::typecheck_stmt(ctx, stmt, ret_tyval)?;
        typed_body.push(typed_stmt);
    }

    // goodbye function body scope
    ctx.pop_scope();

    Ok(TypedCmd::new(
        ctx,
        offset,
        Cmd::Fn {
            name,
            params: typed_params,
            return_type: typed_return_type,
            body: typed_body,
        },
    ))
}
