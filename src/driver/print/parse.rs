use crate::parser::ast::{
    BinOp, Cmd, ExprKind, LValue, ParsedCmd, ParsedExpr, ParsedLValue, ParsedStmt, ParsedType,
    Stmt, Type, UnOp,
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
                write!(f, "(FnCmd {name} ((")?;
                for (i, param) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{} {}", param.value.lvalue, param.value.ty)?;
                }
                write!(f, ")) {return_type}")?;
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
            ExprKind::Void => write!(f, "(VoidExpr)"),
            ExprKind::ArrayLiteral(arr) => {
                write!(f, "(ArrayLiteralExpr")?;
                for expr in arr {
                    write!(f, " {expr}")?;
                }
                write!(f, ")")
            }
            ExprKind::StructLiteral(name, fields) => {
                write!(f, "(StructLiteralExpr {name}")?;
                for field in fields {
                    write!(f, " {field}")?;
                }
                write!(f, ")")
            }
            ExprKind::Dot(expr, field) => write!(f, "(DotExpr {expr} {field})"),
            ExprKind::ArrayIndex(expr, indices) => {
                write!(f, "(ArrayIndexExpr {expr}")?;
                for index in indices.iter() {
                    write!(f, " {index}")?;
                }
                write!(f, ")")
            }
            ExprKind::Call(name, args) => {
                write!(f, "(CallExpr {name}")?;
                for arg in args {
                    write!(f, " {arg}")?;
                }
                write!(f, ")")
            }
            ExprKind::If(cond, then_b, else_b) => write!(f, "(IfExpr {cond} {then_b} {else_b})"),
            ExprKind::ArrayLoop(loop_vars, body) => {
                write!(f, "(ArrayLoopExpr")?;
                for (var, range) in loop_vars {
                    write!(f, " {var} {range}")?;
                }
                write!(f, " {body})")
            }
            ExprKind::SumLoop(loop_vars, body) => {
                write!(f, "(SumLoopExpr")?;
                for (var, range) in loop_vars {
                    write!(f, " {var} {range}")?;
                }
                write!(f, " {body})")
            }
            ExprKind::Unary(op, expr) => write!(f, "(UnopExpr {op} {expr})"),
            ExprKind::Binary(left, op, right) => write!(f, "(BinopExpr {left} {op} {right})"),
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

impl std::fmt::Display for ParsedStmt<'_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.value {
            Stmt::Let(lvalue, expr) => write!(f, "(LetStmt {lvalue} {expr})"),
            Stmt::Assert(expr, str) => write!(f, "(AssertStmt {expr} {str})"),
            Stmt::Return(expr) => write!(f, "(ReturnStmt {expr})"),
        }
    }
}

impl std::fmt::Display for UnOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnOp::Not => write!(f, "!"),
            UnOp::Neg => write!(f, "-"),
        }
    }
}

impl std::fmt::Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinOp::Add => write!(f, "+"),
            BinOp::Sub => write!(f, "-"),
            BinOp::Mul => write!(f, "*"),
            BinOp::Div => write!(f, "/"),
            BinOp::Mod => write!(f, "%"),
            BinOp::And => write!(f, "&&"),
            BinOp::Or => write!(f, "||"),
            BinOp::Eq => write!(f, "=="),
            BinOp::NotEq => write!(f, "!="),
            BinOp::Lt => write!(f, "<"),
            BinOp::Gt => write!(f, ">"),
            BinOp::Lte => write!(f, "<="),
            BinOp::Gte => write!(f, ">="),
        }
    }
}
