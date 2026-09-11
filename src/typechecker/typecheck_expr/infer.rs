use crate::{
    parser::ast::{BinOp, ExprKind, ParsedExpr, UnOp},
    typechecker::{
        ast::{TypeError, TypeValue, TypedExpr},
        typecheck_ctx::TypecheckCtx,
        typecheck_expr,
    },
};

pub(super) fn infer<'src, 'old, 'new>(
    ctx: &TypecheckCtx<'src, 'new>,
    expr: &'old ParsedExpr<'src, 'old>,
) -> Result<&'new TypedExpr<'src, 'new>, TypeError<'src, 'new>> {
    let loc = expr.offset;
    match &expr.value.kind {
        ExprKind::Int(val) => Ok(infer_int(ctx, loc, *val)),
        ExprKind::Float(val) => Ok(infer_float(ctx, loc, *val)),
        ExprKind::Bool(val) => Ok(infer_bool(ctx, loc, *val)),
        ExprKind::Var(_) => todo!(),
        ExprKind::Void => Ok(infer_void(ctx, loc)),
        ExprKind::ArrayLiteral(elements) => infer_array_literal(ctx, loc, elements),
        ExprKind::StructLiteral(_, _) => todo!(),
        ExprKind::Dot(_, _) => todo!(),
        ExprKind::ArrayIndex(_, _) => todo!(),
        ExprKind::Call(_, _) => todo!(),
        ExprKind::If(cond, thenb, elseb) => infer_if(ctx, loc, cond, thenb, elseb),
        ExprKind::ArrayLoop(_, _) => todo!(),
        ExprKind::SumLoop(_, _) => todo!(),
        ExprKind::Unary(op, inner) => infer_unop(ctx, loc, *op, inner),
        ExprKind::Binary(left, op, right) => infer_binop(ctx, loc, left, *op, right),
    }
}

fn infer_int<'src, 'old, 'new>(
    ctx: &TypecheckCtx<'src, 'new>,
    offset: usize,
    val: i64,
) -> &'new TypedExpr<'src, 'new> {
    TypedExpr::new(ctx, offset, ExprKind::Int(val), &TypeValue::Int)
}

fn infer_float<'src, 'old, 'new>(
    ctx: &TypecheckCtx<'src, 'new>,
    offset: usize,
    val: f64,
) -> &'new TypedExpr<'src, 'new> {
    TypedExpr::new(ctx, offset, ExprKind::Float(val), &TypeValue::Float)
}

fn infer_bool<'src, 'old, 'new>(
    ctx: &TypecheckCtx<'src, 'new>,
    offset: usize,
    val: bool,
) -> &'new TypedExpr<'src, 'new> {
    TypedExpr::new(ctx, offset, ExprKind::Bool(val), &TypeValue::Bool)
}

fn infer_void<'src, 'old, 'new>(
    ctx: &TypecheckCtx<'src, 'new>,
    offset: usize,
) -> &'new TypedExpr<'src, 'new> {
    TypedExpr::new(ctx, offset, ExprKind::Void, &TypeValue::Void)
}

fn infer_array_literal<'src, 'old, 'new>(
    ctx: &TypecheckCtx<'src, 'new>,
    offset: usize,
    elements: &Vec<&'old ParsedExpr<'src, 'old>>,
) -> Result<&'new TypedExpr<'src, 'new>, TypeError<'src, 'new>> {
    let mut typed_exprs = Vec::new();
    let mut parsed_exprs = elements.into_iter();

    // empty array is a type error
    let Some(first_element) = parsed_exprs.next() else {
        todo!()
    };

    // check the rest
    let typed_first_element = infer(ctx, first_element)?;
    let element_type = typed_first_element.value.ann;
    while let Some(element) = parsed_exprs.next() {
        typed_exprs.push(typecheck_expr::check(ctx, element, element_type)?);
    }

    // return the element
    Ok(TypedExpr::new(
        ctx,
        offset,
        ExprKind::ArrayLiteral(typed_exprs),
        TypeValue::new(
            ctx,
            TypeValue::Array {
                element_type,
                dimension: 1,
            },
        ),
    ))
}

fn infer_if<'src, 'old, 'new>(
    ctx: &TypecheckCtx<'src, 'new>,
    offset: usize,
    cond: &'old ParsedExpr<'src, 'old>,
    thenb: &'old ParsedExpr<'src, 'old>,
    elseb: &'old ParsedExpr<'src, 'old>,
) -> Result<&'new TypedExpr<'src, 'new>, TypeError<'src, 'new>> {
    let typed_cond = typecheck_expr::check(ctx, cond, &TypeValue::Bool)?;
    let typed_thenb = infer(ctx, thenb)?;
    let ternary_type = typed_thenb.value.ann;
    let typed_elseb = typecheck_expr::check(ctx, elseb, ternary_type)?;
    Ok(TypedExpr::new(
        ctx,
        offset,
        ExprKind::If(typed_cond, typed_thenb, typed_elseb),
        ternary_type,
    ))
}

fn infer_unop<'src, 'old, 'new>(
    ctx: &TypecheckCtx<'src, 'new>,
    offset: usize,
    op: UnOp,
    inner: &'old ParsedExpr<'src, 'old>,
) -> Result<&'new TypedExpr<'src, 'new>, TypeError<'src, 'new>> {
    match op {
        UnOp::Neg => {
            let typed_inner = typecheck_expr::check_num(ctx, inner)?;
            let inner_type = typed_inner.value.ann;
            Ok(TypedExpr::new(
                ctx,
                offset,
                ExprKind::Unary(UnOp::Neg, typed_inner),
                inner_type,
            ))
        }
        UnOp::Not => {
            let typed_inner = typecheck_expr::check(ctx, inner, &TypeValue::Bool)?;
            Ok(TypedExpr::new(
                ctx,
                offset,
                ExprKind::Unary(UnOp::Not, typed_inner),
                &TypeValue::Bool,
            ))
        }
    }
}

fn infer_binop<'src, 'old, 'new>(
    ctx: &TypecheckCtx<'src, 'new>,
    offset: usize,
    left: &'old ParsedExpr<'src, 'old>,
    op: BinOp,
    right: &'old ParsedExpr<'src, 'old>,
) -> Result<&'new TypedExpr<'src, 'new>, TypeError<'src, 'new>> {
    match op {
        BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Mod => {
            infer_arith_binop(ctx, offset, left, op, right)
        }
        BinOp::And | BinOp::Or => infer_logic_binop(ctx, offset, left, op, right),
        BinOp::Eq | BinOp::NotEq | BinOp::Lt | BinOp::Lte | BinOp::Gt | BinOp::Gte => {
            infer_cmp_binop(ctx, offset, left, op, right)
        }
    }
}

fn infer_arith_binop<'src, 'old, 'new>(
    ctx: &TypecheckCtx<'src, 'new>,
    offset: usize,
    left: &'old ParsedExpr<'src, 'old>,
    op: BinOp,
    right: &'old ParsedExpr<'src, 'old>,
) -> Result<&'new TypedExpr<'src, 'new>, TypeError<'src, 'new>> {
    let typed_left = typecheck_expr::check_num(ctx, left)?;
    let left_type = typed_left.value.ann;
    let typed_right = typecheck_expr::check(ctx, right, left_type)?;
    Ok(TypedExpr::new(
        ctx,
        offset,
        ExprKind::Binary(typed_left, op, typed_right),
        left_type,
    ))
}

fn infer_logic_binop<'src, 'old, 'new>(
    ctx: &TypecheckCtx<'src, 'new>,
    offset: usize,
    left: &'old ParsedExpr<'src, 'old>,
    op: BinOp,
    right: &'old ParsedExpr<'src, 'old>,
) -> Result<&'new TypedExpr<'src, 'new>, TypeError<'src, 'new>> {
    let typed_left = typecheck_expr::check(ctx, left, &TypeValue::Bool)?;
    let typed_right = typecheck_expr::check(ctx, right, &TypeValue::Bool)?;
    Ok(TypedExpr::new(
        ctx,
        offset,
        ExprKind::Binary(typed_left, op, typed_right),
        &TypeValue::Bool,
    ))
}

fn infer_cmp_binop<'src, 'old, 'new>(
    ctx: &TypecheckCtx<'src, 'new>,
    offset: usize,
    left: &'old ParsedExpr<'src, 'old>,
    op: BinOp,
    right: &'old ParsedExpr<'src, 'old>,
) -> Result<&'new TypedExpr<'src, 'new>, TypeError<'src, 'new>> {
    let typed_left = typecheck_expr::check_primitive(ctx, left)?;
    let left_type = typed_left.value.ann;
    let typed_right = typecheck_expr::check(ctx, right, left_type)?;
    Ok(TypedExpr::new(
        ctx,
        offset,
        ExprKind::Binary(typed_left, op, typed_right),
        &TypeValue::Bool,
    ))
}
