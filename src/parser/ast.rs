use crate::{Allocator, Spanned, lexer::token::TokenKind, parser::ParserCtx};

pub type ParsedProgram<'src, 'ast> = Vec<&'ast ParsedCmd<'src, 'ast>>;
pub type ParsedCmd<'src, 'ast> = Spanned<Cmd<'src, 'ast, ()>>;
pub type ParsedExpr<'src, 'ast> = Spanned<Expr<'src, 'ast, ()>>;
pub type ParsedLValue<'src> = Spanned<LValue<'src>>;
pub type ParsedType<'src, 'ast> = Spanned<Type<'src, 'ast>>;
pub type ParsedStmt<'src, 'ast> = Spanned<Stmt<'src, 'ast, ()>>;
pub type ParsedBinding<'src, 'ast> = Spanned<Binding<'src, 'ast>>;
pub type ParsedStructField<'src, 'ast> = Spanned<StructField<'src, 'ast>>;
pub type ParsedLoopIterVar<'src, 'ast> = Spanned<LoopIterVar<'src, 'ast, ()>>;

impl<'src, 'ast> ParsedExpr<'src, 'ast> {
    pub(super) fn from(
        ctx: &ParserCtx<'src, 'ast>,
        offset: usize,
        kind: ExprKind<'src, 'ast, ()>,
    ) -> &'ast Self {
        ctx.alloc(ParsedExpr {
            offset,
            value: Expr { ann: (), kind },
        })
    }
}

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
        fields: Vec<&'ast Spanned<StructField<'src, 'ast>>>,
    },
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
    If(
        &'ast Spanned<Expr<'src, 'ast, A>>, // cond
        &'ast Spanned<Expr<'src, 'ast, A>>, // then
        &'ast Spanned<Expr<'src, 'ast, A>>, // else
    ),
    ArrayLoop(
        Vec<&'ast Spanned<LoopIterVar<'src, 'ast, A>>>, // bindings
        &'ast Spanned<Expr<'src, 'ast, A>>,             // body
    ),
    SumLoop(
        Vec<&'ast Spanned<LoopIterVar<'src, 'ast, A>>>, // bindings
        &'ast Spanned<Expr<'src, 'ast, A>>,             // body
    ),
    Unary(UnOp, &'ast Spanned<Expr<'src, 'ast, A>>),
    Binary(
        &'ast Spanned<Expr<'src, 'ast, A>>, // left
        BinOp,
        &'ast Spanned<Expr<'src, 'ast, A>>, // right
    ),
}

#[derive(Clone, Copy)]
pub enum UnOp {
    Neg,
    Not,
}

#[derive(Clone, Copy)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Eq,
    NotEq,
    Lt,
    Lte,
    Gt,
    Gte,
}

#[derive(Clone)]
pub enum LValue<'src> {
    Var(&'src str),
    Array(&'src str, Vec<&'src str>),
}

impl<'src, 'ast> ParsedLValue<'src> {
    pub(super) fn make_parsed(
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
    pub(super) fn make_parsed(
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

pub struct Binding<'src, 'ast> {
    pub lvalue: &'ast Spanned<LValue<'src>>,
    pub ty: &'ast Spanned<Type<'src, 'ast>>,
}

impl<'src, 'ast> ParsedBinding<'src, 'ast> {
    pub(super) fn make_parsed(
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

pub struct StructField<'src, 'ast> {
    pub name: &'src str,
    pub ty: &'ast Spanned<Type<'src, 'ast>>,
}

impl<'src, 'ast> ParsedStructField<'src, 'ast> {
    pub(super) fn make_parsed(
        ctx: &ParserCtx<'src, 'ast>,
        offset: usize,
        field: StructField<'src, 'ast>,
    ) -> &'ast Self {
        ctx.alloc(Spanned {
            offset,
            value: field,
        })
    }
}

pub struct LoopIterVar<'src, 'ast, A> {
    pub name: &'src str,
    pub expr: &'ast Spanned<Expr<'src, 'ast, A>>,
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
