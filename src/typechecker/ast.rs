use crate::{
    Spanned,
    parser::ast::{Binding, Cmd, Expr, LValue, Stmt, Type},
};

pub enum TypeValue<'src, 'ast> {
    Int,
    Bool,
    Float,
    Array {
        element_type: &'ast TypeValue<'src, 'ast>,
        dimension: usize,
    },
    Struct {
        name: &'src str,
    },
    Void,
}

pub type TypedProgram<'src, 'ast> = Vec<&'ast TypedCmd<'src, 'ast>>;
pub type TypedCmd<'src, 'ast> = Spanned<Cmd<'src, 'ast, TypeValue<'src, 'ast>>>;
pub type TypedExpr<'src, 'ast> = Spanned<Expr<'src, 'ast, TypeValue<'src, 'ast>>>;
pub type TypedLValue<'src> = Spanned<LValue<'src>>;
pub type TypedType<'src, 'ast> = Spanned<Type<'src, 'ast>>;
pub type TypedStmt<'src, 'ast> = Spanned<Stmt<'src, 'ast, TypeValue<'src, 'ast>>>;
pub type TypedBinding<'src, 'ast> = Spanned<Binding<'src, 'ast>>;

pub type TypeError<'src, 'ast> = Spanned<TypeErrorKind<'src, 'ast>>;

impl<'src, 'ast> TypedCmd<'src, 'ast> {
    pub fn new(offset: usize, cmd: Cmd<'src, 'ast, TypeValue<'src, 'ast>>) -> Self {
        Spanned { offset, value: cmd }
    }
}

impl<'src, 'ast> TypedExpr<'src, 'ast> {
    pub fn new(offset: usize, expr: Expr<'src, 'ast, TypeValue<'src, 'ast>>) -> Self {
        Spanned {
            offset,
            value: expr,
        }
    }
}

impl<'src, 'ast> TypedStmt<'src, 'ast> {
    pub fn new(offset: usize, stmt: Stmt<'src, 'ast, TypeValue<'src, 'ast>>) -> Self {
        Spanned {
            offset,
            value: stmt,
        }
    }
}

pub enum TypeErrorKind<'src, 'ast> {
    UnexpectedType {
        expected: &'ast [TypeValue<'src, 'ast>],
        found: &'ast TypeValue<'src, 'ast>,
    },
}
