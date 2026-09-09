use crate::{
    Spanned,
    parser::ast::{Binding, Cmd, Expr, LValue, Stmt, Type},
};

pub enum TypeValue {}

pub type TypedProgram<'src, 'ast> = Vec<&'ast TypedCmd<'src, 'ast>>;
pub type TypedCmd<'src, 'ast> = Spanned<Cmd<'src, 'ast, TypeValue>>;
pub type TypedExpr<'src, 'ast> = Spanned<Expr<'src, 'ast, TypeValue>>;
pub type TypedLValue<'src> = Spanned<LValue<'src>>;
pub type TypedType<'src, 'ast> = Spanned<Type<'src, 'ast>>;
pub type TypedStmt<'src, 'ast> = Spanned<Stmt<'src, 'ast, TypeValue>>;
pub type TypedBinding<'src, 'ast> = Spanned<Binding<'src, 'ast>>;

impl<'src, 'ast> TypedCmd<'src, 'ast> {
    pub fn new(offset: usize, cmd: Cmd<'src, 'ast, TypeValue>) -> Self {
        Spanned { offset, value: cmd }
    }
}

impl<'src, 'ast> TypedExpr<'src, 'ast> {
    pub fn new(offset: usize, expr: Expr<'src, 'ast, TypeValue>) -> Self {
        Spanned {
            offset,
            value: expr,
        }
    }
}

impl<'src, 'ast> TypedStmt<'src, 'ast> {
    pub fn new(offset: usize, stmt: Stmt<'src, 'ast, TypeValue>) -> Self {
        Spanned {
            offset,
            value: stmt,
        }
    }
}
