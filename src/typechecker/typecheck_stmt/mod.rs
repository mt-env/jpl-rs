use crate::{
    parser::ast::{ParsedExpr, ParsedLValue, ParsedStmt, Stmt},
    typechecker::{
        TypecheckCtx,
        ast::{TypeError, TypedStmt},
    },
};

pub(super) fn typecheck_stmt<'src, 'old, 'new>(
    ctx: &mut TypecheckCtx<'src, 'new>,
    stmt: &'old ParsedStmt<'src, 'old>,
) -> Result<&'new TypedStmt<'src, 'new>, TypeError<'src, 'new>> {
    let loc = stmt.offset;
    match stmt.value {
        Stmt::Let(lvalue, expr) => typecheck_let(ctx, loc, lvalue, expr),
        Stmt::Assert(expr, msg) => typecheck_assert(ctx, loc, expr, msg),
        Stmt::Return(expr) => typecheck_return(ctx, loc, expr),
    }
}

fn typecheck_let<'src, 'old, 'new>(
    ctx: &mut TypecheckCtx<'src, 'new>,
    offset: usize,
    lvalue: &'old ParsedLValue<'src>,
    expr: &'old ParsedExpr<'src, 'old>,
) -> Result<&'new TypedStmt<'src, 'new>, TypeError<'src, 'new>> {
    todo!()
}

fn typecheck_assert<'src, 'old, 'new>(
    ctx: &mut TypecheckCtx<'src, 'new>,
    offset: usize,
    expr: &'old ParsedExpr<'src, 'old>,
    msg: &'src str,
) -> Result<&'new TypedStmt<'src, 'new>, TypeError<'src, 'new>> {
    todo!()
}

fn typecheck_return<'src, 'old, 'new>(
    ctx: &mut TypecheckCtx<'src, 'new>,
    offset: usize,
    expr: &'old ParsedExpr<'src, 'old>,
) -> Result<&'new TypedStmt<'src, 'new>, TypeError<'src, 'new>> {
    todo!()
}
