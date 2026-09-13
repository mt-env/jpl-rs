use crate::{
    parser::ast::{Cmd, ParsedCmd, ParsedExpr, ParsedType},
    typechecker::{
        ast::{TypeError, TypedCmd},
        typecheck_ctx::TypecheckCtx,
        typecheck_expr, typecheck_type,
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
        _ => todo!(),
    }
}

fn typecheck_show<'src, 'old, 'new>(
    ctx: &TypecheckCtx<'src, 'new>,
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
