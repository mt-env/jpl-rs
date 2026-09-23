use crate::{
    parser::ast::{ParsedExpr, ParsedLValue, ParsedStmt, Stmt},
    typechecker::{
        TypecheckCtx,
        ast::{TypeError, TypeValue, TypedStmt},
        typecheck_expr, typecheck_lvalue,
    },
};

pub(super) fn typecheck_stmt<'src, 'old, 'new>(
    ctx: &mut TypecheckCtx<'src, 'new>,
    stmt: &'old ParsedStmt<'src, 'old>,
    expected_ret_tyval: &'new TypeValue<'src, 'new>,
) -> Result<&'new TypedStmt<'src, 'new>, TypeError<'src, 'new>> {
    let loc = stmt.offset;
    match stmt.value {
        Stmt::Let(lvalue, expr) => typecheck_let(ctx, loc, lvalue, expr),
        Stmt::Assert(expr, msg) => typecheck_assert(ctx, loc, expr, msg),
        Stmt::Return(expr) => typecheck_return(ctx, loc, expr, expected_ret_tyval),
    }
}

fn typecheck_let<'src, 'old, 'new>(
    ctx: &mut TypecheckCtx<'src, 'new>,
    offset: usize,
    lvalue: &'old ParsedLValue<'src>,
    expr: &'old ParsedExpr<'src, 'old>,
) -> Result<&'new TypedStmt<'src, 'new>, TypeError<'src, 'new>> {
    let resolved_expr = typecheck_expr::infer(ctx, expr)?;
    let resolved_lvalue = typecheck_lvalue::typecheck_lvalue(ctx, lvalue)?;
    let expr_tyval = resolved_expr.value.ann;
    typecheck_lvalue::bind_lvalue(ctx, resolved_lvalue, expr_tyval)?;
    Ok(TypedStmt::new(
        ctx,
        offset,
        Stmt::Let(resolved_lvalue, resolved_expr),
    ))
}

fn typecheck_assert<'src, 'old, 'new>(
    ctx: &mut TypecheckCtx<'src, 'new>,
    offset: usize,
    expr: &'old ParsedExpr<'src, 'old>,
    msg: &'src str,
) -> Result<&'new TypedStmt<'src, 'new>, TypeError<'src, 'new>> {
    let typed_expr = typecheck_expr::infer(ctx, expr)?;
    Ok(TypedStmt::new(ctx, offset, Stmt::Assert(typed_expr, msg)))
}

fn typecheck_return<'src, 'old, 'new>(
    ctx: &mut TypecheckCtx<'src, 'new>,
    offset: usize,
    expr: &'old ParsedExpr<'src, 'old>,
    expected_ret_tyval: &'new TypeValue<'src, 'new>,
) -> Result<&'new TypedStmt<'src, 'new>, TypeError<'src, 'new>> {
    let resolved_expr = typecheck_expr::check(ctx, expr, expected_ret_tyval)?;
    Ok(TypedStmt::new(ctx, offset, Stmt::Return(resolved_expr)))
}
