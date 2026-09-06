use crate::parser::ast::{
    Cmd, ExprKind, LValue, ParsedBinding, ParsedCmd, ParsedExpr, ParsedLValue, ParsedStmt,
    ParsedType, Stmt, Type,
};

pub fn print_sexp(ast: Vec<&ParsedCmd<'_, '_>>) {
    for cmd in ast {
        println!("{cmd}");
    }
}

impl std::fmt::Display for ParsedCmd<'_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.value {
            Cmd::Read(var, lvalue) => write!(f, "(ReadCmd {var} {lvalue})"),
            Cmd::Write(expr, var) => write!(f, "(WriteCmd {expr} {var})"),
            Cmd::Let(lvalue, expr) => write!(f, "(LetCmd {lvalue} {expr})"),
            Cmd::Assert(expr, msg) => write!(f, "(AssertCmd {expr} {msg})"),
            Cmd::Print(msg) => write!(f, "(PrintCmd {msg})"),
            Cmd::Show(expr) => write!(f, "(ShowCmd {expr})"),
            Cmd::Time(cmd) => write!(f, "(TimeCmd {cmd})"),
            Cmd::Fn {
                name,
                params,
                return_type,
                body,
            } => {
                write!(f, "(FnCmd {name} (")?;
                for (i, param) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{param}")?;
                }
                write!(f, ") {return_type}")?;
                for stmt in body {
                    write!(f, " {stmt}")?;
                }
                write!(f, ")")
            }
            Cmd::Struct { name, fields } => {
                write!(f, "(StructCmd {name}")?;
                for (field_name, field_type) in fields {
                    write!(f, " {field_name} {field_type}")?;
                }
                write!(f, ")")
            }
        }
    }
}

impl std::fmt::Display for ParsedExpr<'_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.value.kind {
            ExprKind::Int(i) => write!(f, "(IntExpr {i})"),
            ExprKind::Float(fl) => write!(f, "(FloatExpr {})", *fl as i64),
            ExprKind::Bool(b) => {
                if *b {
                    write!(f, "(TrueExpr)")
                } else {
                    write!(f, "(FalseExpr)")
                }
            }
            ExprKind::Var(var) => write!(f, "(VarExpr {var})"),
            ExprKind::ArrayLiteral(arr) => {
                write!(f, "(ArrayLiteralExpr")?;
                for expr in arr {
                    write!(f, " {expr}")?;
                }
                write!(f, ")")
            }
        }
    }
}

impl std::fmt::Display for ParsedLValue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.value {
            LValue::Var(var) => write!(f, "(VarLValue {var})"),
            LValue::Array(var, dims) => {
                write!(f, "(ArrayLValue {var}")?;
                for dim in dims {
                    write!(f, " {dim}")?;
                }
                write!(f, ")")
            }
        }
    }
}

impl std::fmt::Display for ParsedType<'_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.value {
            Type::Int => write!(f, "(IntType)"),
            Type::Float => write!(f, "(FloatType)"),
            Type::Bool => write!(f, "(BoolType)"),
            Type::Void => write!(f, "(VoidType)"),
            Type::Array {
                element_type,
                dimension,
            } => write!(f, "(ArrayType {element_type} {dimension})"),
            Type::Struct { name } => write!(f, "(StructType {name})"),
        }
    }
}

impl std::fmt::Display for ParsedBinding<'_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {})", self.value.lvalue, self.value.ty)
    }
}

impl std::fmt::Display for ParsedStmt<'_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.value {
            Stmt::Let(lvalue, expr) => write!(f, "(LetStmt {lvalue} {expr})"),
            Stmt::Assert(expr, str) => write!(f, "(AssertStmt {expr} {str})"),
            Stmt::Return(expr) => write!(f, "(ReturnStmt {expr})"),
        }
    }
}
