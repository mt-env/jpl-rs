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
    pub ann: A,
    pub kind: ExprKind<'src, 'ast, A>,
}

pub enum ExprKind<'src, 'ast, A> {
    Int(i64),
    Float(f64),
    Bool(bool),
    Var(&'src str),
    ArrayLiteral(Vec<&'ast Spanned<Expr<'src, 'ast, A>>>),
}

impl<'src, 'ast> ParsedExpr<'src, 'ast> {
    pub(super) fn new(
        ctx: &ParserCtx<'src, 'ast>,
        offset: usize,
        expr: ExprKind<'src, 'ast, ()>,
    ) -> &'ast Self {
        ctx.alloc(Spanned {
            offset,
            value: Expr {
                ann: (),
                kind: expr,
            },
        })
    }
}

pub enum LValue<'src> {
    Var(&'src str),
}

impl<'src, 'ast> ParsedLValue<'src> {
    pub(super) fn new(
        ctx: &ParserCtx<'src, 'ast>,
        offset: usize,
        lvalue: LValue<'src>,
    ) -> &'ast Self {
        ctx.alloc(Spanned {
            offset,
            value: lvalue,
        })
    }
}

pub struct Spanned<T> {
    pub offset: usize,
    pub value: T,
}
