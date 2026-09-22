use crate::{
    Spanned,
    parser::ast::{Binding, Cmd, Expr, ExprKind, LValue, Stmt, Type},
    typechecker::typecheck_ctx::TypecheckCtx,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TypeValue<'src, 'ast> {
    Int,
    Bool,
    Float,
    Array {
        element_type: &'ast Self,
        dimension: usize,
    },
    Struct {
        name: &'src str,
    },
    Void,
}

pub type TypedProgram<'src, 'ast> = Vec<&'ast TypedCmd<'src, 'ast>>;
pub type TypedCmd<'src, 'ast> = Spanned<Cmd<'src, 'ast, &'ast TypeValue<'src, 'ast>>>;
pub type TypedExpr<'src, 'ast> = Spanned<Expr<'src, 'ast, &'ast TypeValue<'src, 'ast>>>;
pub type TypedLValue<'src> = Spanned<LValue<'src>>;
pub type TypedType<'src, 'ast> = Spanned<Type<'src, 'ast>>;
pub type TypedStmt<'src, 'ast> = Spanned<Stmt<'src, 'ast, &'ast TypeValue<'src, 'ast>>>;
pub type TypedBinding<'src, 'ast> = Spanned<Binding<'src, 'ast>>;

pub type TypeError<'src, 'ast> = Spanned<TypeErrorKind<'src, 'ast>>;

impl<'src, 'ast> TypedCmd<'src, 'ast> {
    pub(super) fn new(
        ctx: &TypecheckCtx<'src, 'ast>,
        offset: usize,
        cmd: Cmd<'src, 'ast, &'ast TypeValue<'src, 'ast>>,
    ) -> &'ast Self {
        ctx.alloc(Spanned { offset, value: cmd })
    }
}

impl<'src, 'ast> TypedExpr<'src, 'ast> {
    pub(super) fn new(
        ctx: &TypecheckCtx<'src, 'ast>,
        offset: usize,
        kind: ExprKind<'src, 'ast, &'ast TypeValue<'src, 'ast>>,
        ty: &'ast TypeValue<'src, 'ast>,
    ) -> &'ast Self {
        ctx.alloc(Spanned {
            offset,
            value: Expr { ann: ty, kind },
        })
    }
}

impl<'src, 'ast> TypedLValue<'src> {
    pub(super) fn make_typed(
        ctx: &TypecheckCtx<'src, 'ast>,
        offset: usize,
        lvalue: LValue<'src>,
    ) -> &'ast Self {
        ctx.alloc(Spanned {
            offset,
            value: lvalue,
        })
    }
}

impl<'src, 'ast> TypedType<'src, 'ast> {
    pub(super) fn make_typed(
        ctx: &TypecheckCtx<'src, 'ast>,
        offset: usize,
        ty: Type<'src, 'ast>,
    ) -> &'ast Self {
        ctx.alloc(Spanned { offset, value: ty })
    }
}

impl<'src, 'ast> TypedStmt<'src, 'ast> {
    pub(super) fn new(
        ctx: &TypecheckCtx<'src, 'ast>,
        offset: usize,
        stmt: Stmt<'src, 'ast, &'ast TypeValue<'src, 'ast>>,
    ) -> &'ast Self {
        ctx.alloc(Spanned {
            offset,
            value: stmt,
        })
    }
}

impl<'src, 'ast> TypedBinding<'src, 'ast> {
    pub(super) fn make_typed(
        ctx: &TypecheckCtx<'src, 'ast>,
        offset: usize,
        binding: Binding<'src, 'ast>,
    ) -> &'ast Self {
        ctx.alloc(Spanned {
            offset,
            value: binding,
        })
    }
}

pub enum TypeErrorKind<'src, 'ast> {
    ExpectType {
        expected: &'ast TypeValue<'src, 'ast>,
        found: &'ast TypeValue<'src, 'ast>,
    },
    ExpectTypes {
        expected: &'ast [&'ast TypeValue<'src, 'ast>],
        found: &'ast TypeValue<'src, 'ast>,
    },
    EmptyArrayLiteral,
    UnknownStruct(&'src str),
    StructFieldCountMismatch {
        struct_name: &'src str,
        expected: usize,
        actual: usize,
    },
    DotOnNonStruct,
    UnknownStructField {
        struct_name: &'src str,
        field_name: &'src str,
    },
    ArrayIndexOnNonArray,
    ArrayIndexDimensionMismatch {
        expected: usize,
        actual: usize,
    },
    ArrayLValueOnNonArray {
        rhs: &'ast TypeValue<'src, 'ast>,
    },
    ArrayLValueDimensionMismatch {
        expected: usize,
        actual: usize,
    },
}

impl<'src, 'ast> TypeValue<'src, 'ast> {
    pub(super) fn new(ctx: &TypecheckCtx<'src, 'ast>, value: Self) -> &'ast Self {
        ctx.alloc(value)
    }
}
