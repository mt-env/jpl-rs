use crate::{
    Spanned,
    parser::ast::{ParsedType, Type},
    typechecker::{
        TypecheckCtx,
        ast::{TypeError, TypeValue, TypedType},
    },
};

pub(super) fn typecheck_type<'src, 'old, 'new>(
    ctx: &mut TypecheckCtx<'src, 'new>,
    ty: &'old ParsedType<'src, 'old>,
) -> Result<&'new TypedType<'src, 'new>, TypeError<'src, 'new>> {
    let Spanned { offset, value } = ty;
    let copied_value = match value {
        Type::Int => Type::Int,
        Type::Float => Type::Float,
        Type::Bool => Type::Bool,
        Type::Array {
            element_type,
            dimension,
        } => {
            let typed_element_type = typecheck_type(ctx, element_type)?;
            Type::Array {
                element_type: typed_element_type,
                dimension: *dimension,
            }
        }
        Type::Struct { name } => Type::Struct { name },
        Type::Void => Type::Void,
    };
    Ok(TypedType::make_typed(ctx, *offset, copied_value))
}

pub(super) fn typevalue_of_type<'src, 'old, 'new>(
    ctx: &mut TypecheckCtx<'src, 'new>,
    ty: &'old ParsedType<'src, 'old>,
) -> Result<&'new TypeValue<'src, 'new>, TypeError<'src, 'new>> {
    let typevalue = match ty.value {
        Type::Int => TypeValue::Int,
        Type::Float => TypeValue::Float,
        Type::Bool => TypeValue::Bool,
        Type::Array {
            element_type,
            dimension,
        } => {
            let element_typevalue = typevalue_of_type(ctx, element_type)?;
            TypeValue::Array {
                element_type: element_typevalue,
                dimension: dimension,
            }
        }
        Type::Struct { name } => TypeValue::Struct { name },
        Type::Void => TypeValue::Void,
    };
    Ok(TypeValue::new(ctx, typevalue))
}
