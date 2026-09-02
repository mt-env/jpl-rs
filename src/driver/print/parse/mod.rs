use crate::parser::ast::{Cmd, ExprKind, LValue, ParsedCmd, ParsedExpr, ParsedLValue};

pub fn print_sexp<'src, 'ast>(ast: Vec<&ParsedCmd<'src, 'ast>>) {
    for cmd in ast {
        println!("{cmd}")
    }
}

impl std::fmt::Display for ParsedCmd<'_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.value {
            Cmd::Read(var, lvalue) => write!(f, "(ReadCmd {var} {lvalue})"),
            Cmd::Write(expr, var) => write!(f, "(WriteCmd {expr} {var})"),
            Cmd::Let(lvalue, expr) => write!(f, "(LetCmd {lvalue} {expr})"),
            Cmd::Assert(expr, msg) => write!(f, "(AssertCmd {expr} {msg})"),
            Cmd::Print(msg) => write!(f, "(PrintCmd {msg})"),
            Cmd::Show(expr) => write!(f, "(ShowCmd {expr})"),
            Cmd::Time(cmd) => write!(f, "(TimeCmd {cmd})"),
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
                let mut arr_str = String::from("(ArrayLiteralExpr");
                for expr in arr {
                    arr_str.push_str(&format!(" {expr}"));
                }
                arr_str.push(')');
                write!(f, "{arr_str}")
            }
        }
    }
}

impl std::fmt::Display for ParsedLValue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.value {
            LValue::Var(var) => write!(f, "(VarLValue {var})"),
        }
    }
}
