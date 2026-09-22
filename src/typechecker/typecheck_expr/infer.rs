use crate::{
    parser::ast::{BinOp, ExprKind, ParsedExpr, UnOp},
    typechecker::{
        ast::{TypeError, TypeErrorKind, TypeValue, TypedExpr},
        typecheck_ctx::{NameInfo, TypecheckCtx},
        typecheck_expr,
    },
};

pub(super) fn infer<'src, 'old, 'new>(
    ctx: &mut TypecheckCtx<'src, 'new>,
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
        ExprKind::StructLiteral(name, fields) => infer_struct_literal(ctx, loc, name, fields),
        ExprKind::Dot(struct_expr, field) => infer_dot(ctx, loc, struct_expr, field),
        ExprKind::ArrayIndex(arr, indices) => infer_array_index(ctx, loc, arr, indices),
        ExprKind::Call(_, _) => todo!(),
        ExprKind::If(cond, thenb, elseb) => infer_if(ctx, loc, cond, thenb, elseb),
        ExprKind::ArrayLoop(bindings, body) => infer_array_loop(ctx, loc, bindings, body),
        ExprKind::SumLoop(bindings, body) => infer_sum_loop(ctx, loc, bindings, body),
        ExprKind::Unary(op, inner) => infer_unop(ctx, loc, *op, inner),
        ExprKind::Binary(left, op, right) => infer_binop(ctx, loc, left, *op, right),
    }
}

fn infer_int<'src, 'old, 'new>(
    ctx: &mut TypecheckCtx<'src, 'new>,
    offset: usize,
    val: i64,
) -> &'new TypedExpr<'src, 'new> {
    TypedExpr::new(ctx, offset, ExprKind::Int(val), &TypeValue::Int)
}

fn infer_float<'src, 'old, 'new>(
    ctx: &mut TypecheckCtx<'src, 'new>,
    offset: usize,
    val: f64,
) -> &'new TypedExpr<'src, 'new> {
    TypedExpr::new(ctx, offset, ExprKind::Float(val), &TypeValue::Float)
}

fn infer_bool<'src, 'old, 'new>(
    ctx: &mut TypecheckCtx<'src, 'new>,
    offset: usize,
    val: bool,
) -> &'new TypedExpr<'src, 'new> {
    TypedExpr::new(ctx, offset, ExprKind::Bool(val), &TypeValue::Bool)
}

fn infer_void<'src, 'old, 'new>(
    ctx: &mut TypecheckCtx<'src, 'new>,
    offset: usize,
) -> &'new TypedExpr<'src, 'new> {
    TypedExpr::new(ctx, offset, ExprKind::Void, &TypeValue::Void)
}

fn infer_array_literal<'src, 'old, 'new>(
    ctx: &mut TypecheckCtx<'src, 'new>,
    offset: usize,
    elements: &Vec<&'old ParsedExpr<'src, 'old>>,
) -> Result<&'new TypedExpr<'src, 'new>, TypeError<'src, 'new>> {
    let mut typed_exprs = Vec::new();
    let mut parsed_exprs = elements.into_iter();

    // empty array is a type error
    let Some(first_element) = parsed_exprs.next() else {
        return Err(TypeError {
            offset,
            value: TypeErrorKind::EmptyArrayLiteral,
        });
    };

    // check the rest
    let typed_first_element = infer(ctx, first_element)?;
    let element_type = typed_first_element.value.ann;
    typed_exprs.push(typed_first_element);
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

fn infer_struct_literal<'src, 'old, 'new>(
    ctx: &mut TypecheckCtx<'src, 'new>,
    offset: usize,
    struct_name: &'src str,
    fields: &Vec<&'old ParsedExpr<'src, 'old>>,
) -> Result<&'new TypedExpr<'src, 'new>, TypeError<'src, 'new>> {
    let Some(NameInfo::Struct(struct_def)) = ctx.lookup(struct_name) else {
        return Err(TypeError {
            offset,
            value: TypeErrorKind::UnknownStruct(struct_name),
        });
    };

    let mut resolved_fields = Vec::new();
    if struct_def.len() != fields.len() {
        return Err(TypeError {
            offset,
            value: TypeErrorKind::StructFieldCountMismatch {
                struct_name,
                expected: struct_def.len(),
                actual: fields.len(),
            },
        });
    }

    for ((_, field_type), field_expr) in struct_def.iter().zip(fields.iter()) {
        let resolved_field = typecheck_expr::check(ctx, field_expr, field_type)?;
        resolved_fields.push(resolved_field);
    }

    Ok(TypedExpr::new(
        ctx,
        offset,
        ExprKind::StructLiteral(struct_name, resolved_fields),
        TypeValue::new(ctx, TypeValue::Struct { name: struct_name }),
    ))
}

fn infer_dot<'src, 'old, 'new>(
    ctx: &mut TypecheckCtx<'src, 'new>,
    offset: usize,
    struct_expr: &'old ParsedExpr<'src, 'old>,
    field_name: &'src str,
) -> Result<&'new TypedExpr<'src, 'new>, TypeError<'src, 'new>> {
    // make sure lhs is a struct
    let typed_struct_expr = infer(ctx, struct_expr)?;
    let TypeValue::Struct { name: struct_name } = typed_struct_expr.value.ann else {
        return Err(TypeError {
            offset,
            value: TypeErrorKind::DotOnNonStruct,
        });
    };

    let Some(NameInfo::Struct(struct_def)) = ctx.lookup(struct_name) else {
        return Err(TypeError {
            offset,
            value: TypeErrorKind::UnknownStruct(struct_name),
        });
    };

    // find field
    for (field, field_type) in struct_def.iter() {
        if *field == field_name {
            return Ok(TypedExpr::new(
                ctx,
                offset,
                ExprKind::Dot(typed_struct_expr, field),
                field_type,
            ));
        }
    }

    Err(TypeError {
        offset,
        value: TypeErrorKind::UnknownStructField {
            struct_name,
            field_name,
        },
    })
}

fn infer_array_index<'src, 'old, 'new>(
    ctx: &mut TypecheckCtx<'src, 'new>,
    offset: usize,
    arr: &'old ParsedExpr<'src, 'old>,
    indices: &Vec<&'old ParsedExpr<'src, 'old>>,
) -> Result<&'new TypedExpr<'src, 'new>, TypeError<'src, 'new>> {
    let typed_arr = infer(ctx, arr)?;
    let TypeValue::Array {
        element_type,
        dimension,
    } = typed_arr.value.ann
    else {
        return Err(TypeError {
            offset,
            value: TypeErrorKind::ArrayIndexOnNonArray,
        });
    };

    if indices.len() != *dimension {
        return Err(TypeError {
            offset,
            value: TypeErrorKind::ArrayIndexDimensionMismatch {
                expected: *dimension,
                actual: indices.len(),
            },
        });
    }

    let mut typed_indices = Vec::new();
    for index in indices.iter() {
        typed_indices.push(typecheck_expr::check(ctx, index, &TypeValue::Int)?);
    }

    Ok(TypedExpr::new(
        ctx,
        offset,
        ExprKind::ArrayIndex(typed_arr, ctx.alloc(typed_indices)),
        *element_type,
    ))
}

fn infer_if<'src, 'old, 'new>(
    ctx: &mut TypecheckCtx<'src, 'new>,
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

fn infer_array_loop<'src, 'old, 'new>(
    ctx: &mut TypecheckCtx<'src, 'new>,
    offset: usize,
    bindings: &Vec<(&'src str, &'old ParsedExpr<'src, 'old>)>,
    body: &'old ParsedExpr<'src, 'old>,
) -> Result<&'new TypedExpr<'src, 'new>, TypeError<'src, 'new>> {
    infer_loop_expr(ctx, offset, bindings, body, ExprKind::ArrayLoop)
}

fn infer_sum_loop<'src, 'old, 'new>(
    ctx: &mut TypecheckCtx<'src, 'new>,
    offset: usize,
    bindings: &Vec<(&'src str, &'old ParsedExpr<'src, 'old>)>,
    body: &'old ParsedExpr<'src, 'old>,
) -> Result<&'new TypedExpr<'src, 'new>, TypeError<'src, 'new>> {
    infer_loop_expr(ctx, offset, bindings, body, ExprKind::SumLoop)
}

fn infer_loop_expr<'src, 'old, 'new>(
    ctx: &mut TypecheckCtx<'src, 'new>,
    offset: usize,
    bindings: &Vec<(&'src str, &'old ParsedExpr<'src, 'old>)>,
    body: &'old ParsedExpr<'src, 'old>,
    make: impl Fn(
        Vec<(&'src str, &'new TypedExpr<'src, 'new>)>,
        &'new TypedExpr<'src, 'new>,
    ) -> ExprKind<'src, 'new, &'new TypeValue<'src, 'new>>,
) -> Result<&'new TypedExpr<'src, 'new>, TypeError<'src, 'new>> {
    ctx.push_scope();

    // each binding must be an int, and is then added to the scope
    let mut typed_bindings = Vec::new();
    for binding in bindings.iter() {
        typed_bindings.push((
            binding.0,
            typecheck_expr::check(ctx, binding.1, &TypeValue::Int)?,
        ));
        ctx.bind(binding.0, &TypeValue::Int);
    }

    let typed_body = infer(ctx, body)?;

    ctx.pop_scope();

    Ok(TypedExpr::new(
        ctx,
        offset,
        make(typed_bindings, typed_body),
        typed_body.value.ann,
    ))
}

fn infer_unop<'src, 'old, 'new>(
    ctx: &mut TypecheckCtx<'src, 'new>,
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
    ctx: &mut TypecheckCtx<'src, 'new>,
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
    ctx: &mut TypecheckCtx<'src, 'new>,
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
    ctx: &mut TypecheckCtx<'src, 'new>,
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
    ctx: &mut TypecheckCtx<'src, 'new>,
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
