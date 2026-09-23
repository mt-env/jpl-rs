use crate::{
    parser::ast::{Cmd, ExprKind, Stmt},
    typechecker::ast::{TypeValue, TypedCmd, TypedExpr, TypedProgram, TypedStmt},
};

pub fn print_typed_program(program: TypedProgram) {
    for cmd in program {
        println!("{cmd}");
    }
}

impl std::fmt::Display for TypedCmd<'_, '_> {
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
                for field in fields {
                    let field_name = field.value.name;
                    let field_type = field.value.ty;
                    write!(f, " {field_name} {field_type}")?;
                }
                write!(f, ")")
            }
        }
    }
}

impl std::fmt::Display for TypedExpr<'_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ty_value = self.value.ann;
        match &self.value.kind {
            ExprKind::Int(i) => write!(f, "(IntExpr {ty_value} {i})"),
            ExprKind::Float(fl) => write!(f, "(FloatExpr {ty_value} {})", *fl as i64),
            ExprKind::Bool(b) => {
                if *b {
                    write!(f, "(TrueExpr {ty_value})")
                } else {
                    write!(f, "(FalseExpr {ty_value})")
                }
            }
            ExprKind::Var(var) => todo!(),
            ExprKind::Void => write!(f, "(VoidExpr {ty_value})"),
            ExprKind::ArrayLiteral(arr) => {
                write!(f, "(ArrayLiteralExpr {ty_value}")?;
                for expr in arr {
                    write!(f, " {expr}")?;
                }
                write!(f, ")")
            }
            ExprKind::StructLiteral(name, fields) => {
                write!(f, "(StructLiteralExpr {ty_value} {name}")?;
                for field in fields {
                    write!(f, " {field}")?;
                }
                write!(f, ")")
            }
            ExprKind::Dot(expr, field) => write!(f, "(DotExpr {ty_value} {expr} {field})"),
            ExprKind::ArrayIndex(expr, indices) => {
                write!(f, "(ArrayIndexExpr {ty_value} {expr}")?;
                for index in indices.iter() {
                    write!(f, " {index}")?;
                }
                write!(f, ")")
            }
            ExprKind::Call(name, args) => todo!(),
            ExprKind::If(cond, then_b, else_b) => {
                write!(f, "(IfExpr {ty_value} {cond} {then_b} {else_b})")
            }
            ExprKind::ArrayLoop(loop_vars, body) => todo!(),
            ExprKind::SumLoop(loop_vars, body) => todo!(),
            ExprKind::Unary(op, expr) => write!(f, "(UnopExpr {ty_value} {op} {expr})"),
            ExprKind::Binary(left, op, right) => {
                write!(f, "(BinopExpr {ty_value} {left} {op} {right})")
            }
        }
    }
}

impl std::fmt::Display for TypedStmt<'_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.value {
            Stmt::Let(lvalue, expr) => write!(f, "(LetStmt {lvalue} {expr})"),
            Stmt::Assert(expr, str) => write!(f, "(AssertStmt {expr} {str})"),
            Stmt::Return(expr) => write!(f, "(ReturnStmt {expr})"),
        }
    }
}

impl std::fmt::Display for TypeValue<'_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeValue::Int => write!(f, "(IntType)"),
            TypeValue::Float => write!(f, "(FloatType)"),
            TypeValue::Bool => write!(f, "(BoolType)"),
            TypeValue::Void => write!(f, "(VoidType)"),
            TypeValue::Array {
                element_type,
                dimension,
            } => write!(f, "(ArrayType {element_type} {dimension})"),
            TypeValue::Struct { name } => write!(f, "(StructType {name})"),
        }
    }
}
