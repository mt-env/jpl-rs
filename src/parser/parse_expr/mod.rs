use crate::{
    lexer::token::{Token, TokenKind},
    parser::{
        ParserCtx,
        ast::{BinOp, ExprKind, ParseError, ParsedExpr},
    },
};

mod parse_primary;

#[derive(PartialEq, PartialOrd)]
enum Prec {
    Prefix = 0,
    Logic = 1,
    Cmp = 2,
    Add = 3,
    Mult = 4,
    Unary = 5,
    OOB = 6,
}

impl Prec {
    fn of_binop(binop: BinOp) -> Self {
        match binop {
            BinOp::Mul | BinOp::Div | BinOp::Mod => Prec::Mult,
            BinOp::Add | BinOp::Sub => Prec::Add,
            BinOp::Lt | BinOp::Lte | BinOp::Gt | BinOp::Gte | BinOp::Eq | BinOp::NotEq => Prec::Cmp,
            BinOp::And | BinOp::Or => Prec::Logic,
        }
    }

    fn next(&self) -> Self {
        match self {
            Prec::Prefix => Prec::Logic,
            Prec::Logic => Prec::Cmp,
            Prec::Cmp => Prec::Add,
            Prec::Add => Prec::Mult,
            Prec::Mult => Prec::Unary,
            Prec::Unary => Prec::OOB,
            Prec::OOB => Prec::OOB,
        }
    }
}

pub(super) fn parse_expr<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
) -> Result<&'ast ParsedExpr<'src, 'ast>, ParseError<'src>> {
    parse_expr_precedence(ctx, Prec::Prefix)
}

fn parse_expr_precedence<'src, 'ast>(
    ctx: &mut ParserCtx<'src, 'ast>,
    min_prec: Prec,
) -> Result<&'ast ParsedExpr<'src, 'ast>, ParseError<'src>> {
    let mut left = if let Some(operator) = ctx.try_peek_unop() {
        let Token { offset, .. } = ctx.expect(TokenKind::Op)?;
        let operand = parse_expr_precedence(ctx, Prec::Unary)?;
        ParsedExpr::new(ctx, offset, ExprKind::Unary(operator, operand))
    } else {
        parse_primary::parse_primary(ctx)?
    };

    loop {
        let Some(binop) = ctx.try_peek_binop() else {
            break;
        };
        ctx.expect(TokenKind::Op)?;

        let prec = Prec::of_binop(binop);
        if prec > min_prec {
            break;
        }

        let right = parse_expr_precedence(ctx, prec.next())?;
        left = ParsedExpr::new(ctx, left.offset, ExprKind::Binary(left, binop, right));
    }

    Ok(left)
}
