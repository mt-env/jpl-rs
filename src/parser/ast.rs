use crate::{Spanned, lexer::token::TokenKind, parser::ParserCtx};

pub type ParsedProgram<'src, 'ast> = Vec<&'ast ParsedCmd<'src, 'ast>>;
pub type ParsedCmd<'src, 'ast> = Spanned<Cmd<'src, 'ast, ()>>;
pub type ParsedExpr<'src, 'ast> = Spanned<Expr<'src, 'ast, ()>>;
pub type ParsedLValue<'src> = Spanned<LValue<'src>>;
pub type ParsedType<'src, 'ast> = Spanned<Type<'src, 'ast>>;
pub type ParsedStmt<'src, 'ast> = Spanned<Stmt<'src, 'ast, ()>>;
pub type ParsedBinding<'src, 'ast> = Spanned<Binding<'src, 'ast>>;

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
    Fn {
        name: &'src str,
        params: Vec<&'ast Spanned<Binding<'src, 'ast>>>,
        return_type: &'ast Spanned<Type<'src, 'ast>>,
        body: Vec<&'ast Spanned<Stmt<'src, 'ast, A>>>,
    },
    Struct {
        name: &'src str,
        fields: Vec<(&'src str, &'ast Spanned<Type<'src, 'ast>>)>,
    },
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
    Void,
    ArrayLiteral(Vec<&'ast Spanned<Expr<'src, 'ast, A>>>),
    StructLiteral(&'src str, Vec<&'ast Spanned<Expr<'src, 'ast, A>>>),
    Dot(&'ast Spanned<Expr<'src, 'ast, A>>, &'src str),
    ArrayIndex(
        &'ast Spanned<Expr<'src, 'ast, A>>,
        &'ast Vec<&'ast Spanned<Expr<'src, 'ast, A>>>,
    ),
    Call(&'src str, Vec<&'ast Spanned<Expr<'src, 'ast, A>>>),
    If {
        cond: &'ast Spanned<Expr<'src, 'ast, A>>,
        then_b: &'ast Spanned<Expr<'src, 'ast, A>>,
        else_b: &'ast Spanned<Expr<'src, 'ast, A>>,
    },
    ArrayLoop {
        bindings: Vec<(&'src str, &'ast Spanned<Expr<'src, 'ast, A>>)>,
        body: &'ast Spanned<Expr<'src, 'ast, A>>,
    },
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
    Array(&'src str, Vec<&'src str>),
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

pub enum Type<'src, 'ast> {
    Int,
    Float,
    Bool,
    Array {
        element_type: &'ast Spanned<Self>,
        dimension: usize,
    },
    Struct {
        name: &'src str,
    },
    Void,
}

impl<'src, 'ast> ParsedType<'src, 'ast> {
    pub(super) fn new(
        ctx: &ParserCtx<'src, 'ast>,
        offset: usize,
        ty: Type<'src, 'ast>,
    ) -> &'ast Self {
        ctx.alloc(Spanned { offset, value: ty })
    }
}

pub enum Stmt<'src, 'ast, A> {
    Let(
        &'ast Spanned<LValue<'src>>,
        &'ast Spanned<Expr<'src, 'ast, A>>,
    ),
    Assert(&'ast Spanned<Expr<'src, 'ast, A>>, &'src str),
    Return(&'ast Spanned<Expr<'src, 'ast, A>>),
}

impl<'src, 'ast> ParsedStmt<'src, 'ast> {
    pub(super) fn new(
        ctx: &ParserCtx<'src, 'ast>,
        offset: usize,
        stmt: Stmt<'src, 'ast, ()>,
    ) -> &'ast Self {
        ctx.alloc(Spanned {
            offset,
            value: stmt,
        })
    }
}

pub struct Binding<'src, 'ast> {
    pub lvalue: &'ast Spanned<LValue<'src>>,
    pub ty: &'ast Spanned<Type<'src, 'ast>>,
}

impl<'src, 'ast> ParsedBinding<'src, 'ast> {
    pub(super) fn new(
        ctx: &ParserCtx<'src, 'ast>,
        offset: usize,
        binding: Binding<'src, 'ast>,
    ) -> &'ast Self {
        ctx.alloc(Spanned {
            offset,
            value: binding,
        })
    }
}

pub enum ParseErrorKind<'src> {
    InvalidIntLiteral(&'src str),
    InvalidFloatLiteral(&'src str),
    UnexpectedToken {
        expected: Vec<TokenKind>,
        found: TokenKind,
        value: &'src str,
    },
}

pub type ParseError<'src> = Spanned<ParseErrorKind<'src>>;

impl<'src> ParseError<'src> {
    #[must_use]
    pub const fn new(offset: usize, kind: ParseErrorKind<'src>) -> Self {
        Spanned {
            offset,
            value: kind,
        }
    }
}
