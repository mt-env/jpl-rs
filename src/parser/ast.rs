use crate::parser::ParserCtx;

pub type ParsedProgram<'src, 'ast> = Vec<&'ast ParsedCmd<'src, 'ast>>;
pub type ParsedCmd<'src, 'ast> = Spanned<Cmd<'src, 'ast, ()>>;
pub type ParsedExpr<'src, 'ast> = Spanned<Expr<'src, 'ast, ()>>;
pub type ParsedLValue<'src> = Spanned<LValue<'src>>;

pub enum Cmd<'src, 'ast, A> {
    Read(&'src str, &'ast Spanned<LValue<'src>>),
    Write(&'ast Spanned<Expr<'src, 'ast, A>>, &'src str),
    Let(
        &'ast Spanned<LValue<'src>>,
        &'ast Spanned<Expr<'src, 'ast, A>>,
    ),
    Assert(&'ast Spanned<Expr<'src, 'ast, A>>, &'src str),
    Print(&'src str),
    Show(&'ast Spanned<Expr<'src, 'ast, A>>),
    Time(&'ast Spanned<Self>),
}

impl<'src, 'ast> ParsedCmd<'src, 'ast> {
    pub(super) fn new(
        ctx: &ParserCtx<'src, 'ast>,
        offset: usize,
        cmd: Cmd<'src, 'ast, ()>,
    ) -> &'ast Self {
        ctx.alloc(Spanned { offset, value: cmd })
    }
}

pub struct Expr<'src, 'ast, A> {
    ann: A,
    kind: ExprKind<'src, 'ast>,
}

pub enum ExprKind<'src, 'ast> {
    Int(i64),
    Float(f64),
    Bool(bool),
    Var(&'src str),
    ArrayLiteral(Vec<&'ast Self>),
}

pub enum LValue<'src> {
    Var(&'src str),
}

pub struct Spanned<T> {
    offset: usize,
    value: T,
}
