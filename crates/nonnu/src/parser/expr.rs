use crate::lexer::SyntaxKind;

use super::Parser;

pub fn expr(p: &mut Parser) {
    expr_binding_power(p, 0);
}

fn expr_binding_power(p: &mut Parser, minimum_binding_power: u8) {
    let checkpoint = p.checkpoint();

    match p.peek() {
        Some(SyntaxKind::Number) | Some(SyntaxKind::Ident) => p.bump(),
        Some(SyntaxKind::Minus) => {
            let op = PrefixOp::Neg;
            let ((), right_binding_power) = op.binding_power();

            p.bump();

            p.start_node_at(checkpoint, SyntaxKind::PrefixExpr);
            expr_binding_power(p, right_binding_power);
            p.finish_node();
        }
        Some(SyntaxKind::LParen) => {
            p.bump();
            expr_binding_power(p, 0);

            assert_eq!(p.peek(), Some(SyntaxKind::RParen));
            p.bump();
        }
        _ => {}
    }

    loop {
        let op = match p.peek() {
            Some(SyntaxKind::Plus) => InfixOp::Add,
            Some(SyntaxKind::Minus) => InfixOp::Sub,
            Some(SyntaxKind::Star) => InfixOp::Mul,
            Some(SyntaxKind::Slash) => InfixOp::Div,
            _ => return,
        };

        let (left_binding_power, right_binding_power) = op.binding_power();

        if left_binding_power < minimum_binding_power {
            return;
        }

        p.bump();

        p.start_node_at(checkpoint, SyntaxKind::BinaryExpr);
        expr_binding_power(p, right_binding_power);
        p.finish_node();
    }
}

enum InfixOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl InfixOp {
    fn binding_power(&self) -> (u8, u8) {
        match self {
            Self::Add | Self::Sub => (1, 2),
            Self::Mul | Self::Div => (3, 4),
        }
    }
}

enum PrefixOp {
    Neg,
}

impl PrefixOp {
    fn binding_power(&self) -> ((), u8) {
        match self {
            Self::Neg => ((), 5),
        }
    }
}
