use crate::{
    lexer::token::{Token, TokenKind},
    parser::{
        ParserCtx,
        ast::{BinOp, ExprKind, ParseError, ParsedExpr},
    },
};

mod parse_primary;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
enum Prec {
    Prefix = 0,
    Logic = 1,
    Cmp = 2,
    Add = 3,
    Mult = 4,
    Unary = 5,
    OoB = 6,
}

impl Prec {
    const fn of_binop(binop: BinOp) -> Self {
        match binop {
            BinOp::Mul | BinOp::Div | BinOp::Mod => Self::Mult,
            BinOp::Add | BinOp::Sub => Self::Add,
            BinOp::Lt | BinOp::Lte | BinOp::Gt | BinOp::Gte | BinOp::Eq | BinOp::NotEq => Self::Cmp,
            BinOp::And | BinOp::Or => Self::Logic,
        }
    }

    const fn next(self) -> Self {
        match self {
            Self::Prefix => Self::Logic,
            Self::Logic => Self::Cmp,
            Self::Cmp => Self::Add,
            Self::Add => Self::Mult,
            Self::Mult => Self::Unary,
            Self::Unary | Self::OoB => Self::OoB,
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

    while let Some(binop) = ctx.try_peek_binop() {
        let prec = Prec::of_binop(binop);
        if prec < min_prec {
            break;
        }

        ctx.expect(TokenKind::Op)?;

        let right = parse_expr_precedence(ctx, prec.next())?;
        left = ParsedExpr::new(ctx, left.offset, ExprKind::Binary(left, binop, right));
    }

    Ok(left)
}
