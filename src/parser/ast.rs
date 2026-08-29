enum Cmd<'src, 'ast> {
    Read(&'src str, &'ast LValue<'src>),
    Write(&'ast Expr<'src, 'ast>, &'src str),
    Let(&'ast LValue<'src>, &'ast Expr<'src, 'ast>),
    Assert(&'ast Expr<'src, 'ast>, &'src str),
    Print(&'src str),
    Show(&'ast Expr<'src, 'ast>),
    Time(&'ast Cmd<'src, 'ast>),
}

enum Expr<'src, 'ast> {
    Int(i64),
    Float(f64),
    Bool(bool),
    Var(&'src str),
    ArrayLiteral(Vec<&'ast Expr<'src, 'ast>>),
}

enum LValue<'src> {
    Var(&'src str),
}
