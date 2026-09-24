use crate::{
    parser::ast::{LValue, ParsedLValue},
    typechecker::{
        TypecheckCtx,
        ast::{TypeError, TypeErrorKind, TypeValue, TypedLValue},
    },
};

pub(super) fn typecheck_lvalue<'src, 'old, 'new>(
    ctx: &mut TypecheckCtx<'src, 'new>,
    lvalue: &'old ParsedLValue<'src>,
) -> Result<&'new TypedLValue<'src>, TypeError<'src, 'new>> {
    let loc = lvalue.offset;
    match &lvalue.value {
        LValue::Var(name) => Ok(TypedLValue::make_typed(ctx, loc, LValue::Var(name))),
        LValue::Array(arr, dimensions) => Ok(TypedLValue::make_typed(
            ctx,
            loc,
            LValue::Array(arr, dimensions.clone()),
        )),
    }
}

pub(super) fn bind_lvalue<'src, 'old, 'new>(
    ctx: &mut TypecheckCtx<'src, 'new>,
    lvalue: &'new TypedLValue<'src>,
    ty: &'new TypeValue<'src, 'new>,
) -> Result<(), TypeError<'src, 'new>> {
    match &lvalue.value {
        LValue::Var(name) => {
            ctx.bind(lvalue.offset, name, ty)?;
            Ok(())
        }
        LValue::Array(name, dimensions) => {
            // we can only bind an array lvalue to an array type
            let TypeValue::Array { dimension, .. } = ty else {
                return Err(TypeError {
                    offset: lvalue.offset,
                    value: TypeErrorKind::ArrayLValueOnNonArray { rhs: ty },
                });
            };

            // make sure the dimensions match
            if *dimension != dimensions.len() {
                return Err(TypeError {
                    offset: lvalue.offset,
                    value: TypeErrorKind::ArrayLValueDimensionMismatch {
                        expected: *dimension,
                        actual: dimensions.len(),
                    },
                });
            }

            // bind the array and all its dimensions
            ctx.bind(lvalue.offset, name, ty)?;
            for dim in dimensions {
                ctx.bind(lvalue.offset, dim, &TypeValue::Int)?;
            }
            Ok(())
        }
    }
}
