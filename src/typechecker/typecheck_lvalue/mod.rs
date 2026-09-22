use crate::{
    parser::ast::{LValue, ParsedLValue},
    typechecker::{
        TypecheckCtx,
        ast::{TypeError, TypedLValue},
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
