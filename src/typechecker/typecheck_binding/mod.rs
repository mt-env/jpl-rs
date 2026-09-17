use crate::{
    parser::ast::{Binding, ParsedBinding},
    typechecker::{
        TypecheckCtx,
        ast::{TypeError, TypedBinding},
        typecheck_lvalue, typecheck_type,
    },
};

pub(super) fn typecheck_binding<'src, 'old, 'new>(
    ctx: &mut TypecheckCtx<'src, 'new>,
    binding: &'old ParsedBinding<'src, 'old>,
) -> Result<&'new TypedBinding<'src, 'new>, TypeError<'src, 'new>> {
    let loc = binding.offset;
    let typed_lvalue = typecheck_lvalue::typecheck_lvalue(ctx, binding.value.lvalue)?;
    let typed_type = typecheck_type::typecheck_type(ctx, binding.value.ty)?;

    Ok(TypedBinding::make_typed(
        ctx,
        loc,
        Binding {
            lvalue: typed_lvalue,
            ty: typed_type,
        },
    ))
}
