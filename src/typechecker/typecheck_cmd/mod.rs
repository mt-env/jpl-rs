use crate::{
    parser::ast::{
        Cmd, ParsedBinding, ParsedCmd, ParsedExpr, ParsedLValue, ParsedStmt, ParsedStructField,
        ParsedType, StructField,
    },
    typechecker::{
        ast::{TypeError, TypeValue, TypedCmd, TypedStructField},
        typecheck_binding,
        typecheck_ctx::TypecheckCtx,
        typecheck_expr, typecheck_lvalue, typecheck_stmt, typecheck_type,
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
        Cmd::Read(name, lvalue) => typecheck_read(ctx, loc, name, lvalue),
        Cmd::Write(expr, name) => typecheck_write(ctx, loc, expr, name),
        Cmd::Let(lvalue, expr) => typecheck_let(ctx, loc, lvalue, expr),
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
    fields: &Vec<&'old ParsedStructField<'src, 'old>>,
) -> Result<&'new TypedCmd<'src, 'new>, TypeError<'src, 'new>> {
    let mut resolved_fields = Vec::new();
    let mut typed_fields = Vec::new();
    for field in fields {
        // resolve to a typevalue
        let resolved_tyval = typecheck_type::typevalue_of_type(ctx, field.value.ty)?;
        resolved_fields.push((field.value.name, resolved_tyval));
        // copy the field name and type into the new arena allocator
        let field_type = typecheck_type::typecheck_type(ctx, field.value.ty)?;
        typed_fields.push(TypedStructField::make_typed(
            ctx,
            offset,
            StructField {
                name: field.value.name,
                ty: field_type,
            },
        ));
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

fn typecheck_read<'src, 'old, 'new>(
    ctx: &mut TypecheckCtx<'src, 'new>,
    offset: usize,
    name: &'src str,
    lvalue: &'old ParsedLValue<'src>,
) -> Result<&'new TypedCmd<'src, 'new>, TypeError<'src, 'new>> {
    let typed_lvalue = typecheck_lvalue::typecheck_lvalue(ctx, lvalue)?;
    let rgba2d = TypeValue::new(
        ctx,
        TypeValue::Array {
            element_type: TypeValue::new(ctx, TypeValue::Struct { name: "rgba" }),
            dimension: 2,
        },
    );
    typecheck_lvalue::bind_lvalue(ctx, typed_lvalue, rgba2d)?;
    Ok(TypedCmd::new(ctx, offset, Cmd::Read(name, typed_lvalue)))
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

fn typecheck_let<'src, 'old, 'new>(
    ctx: &mut TypecheckCtx<'src, 'new>,
    offset: usize,
    lvalue: &'old ParsedLValue<'src>,
    expr: &'old ParsedExpr<'src, 'old>,
) -> Result<&'new TypedCmd<'src, 'new>, TypeError<'src, 'new>> {
    let typed_lvalue = typecheck_lvalue::typecheck_lvalue(ctx, lvalue)?;
    let typed_expr = typecheck_expr::infer(ctx, expr)?;
    typecheck_lvalue::bind_lvalue(ctx, typed_lvalue, typed_expr.value.ann)?;
    Ok(TypedCmd::new(
        ctx,
        offset,
        Cmd::Let(typed_lvalue, typed_expr),
    ))
}

fn typecheck_assert<'src, 'old, 'new>(
    ctx: &mut TypecheckCtx<'src, 'new>,
    offset: usize,
    expr: &'old ParsedExpr<'src, 'old>,
    msg: &'src str,
) -> Result<&'new TypedCmd<'src, 'new>, TypeError<'src, 'new>> {
    let typed_expr = typecheck_expr::check(ctx, expr, &TypeValue::Bool)?;
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
    let mut param_types = Vec::new();
    for binding in params {
        let typed_param = typecheck_binding::typecheck_binding(ctx, binding)?;
        typed_params.push(typed_param);
        let typed_lvalue = typed_param.value.lvalue;
        let resolved_ty = typecheck_type::typevalue_of_type(ctx, typed_param.value.ty)?;
        param_types.push(resolved_ty);
        typecheck_lvalue::bind_lvalue(ctx, typed_lvalue, resolved_ty)?;
    }

    // construct ast node for return type
    let typed_return_type = typecheck_type::typecheck_type(ctx, return_type)?;
    let ret_tyval = typecheck_type::typevalue_of_type(ctx, return_type)?;

    // add fn info to global env - has to be done before checking body for recursion
    ctx.add_fn_info(name, param_types, ret_tyval)?;

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
