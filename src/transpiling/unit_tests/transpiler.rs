use strum::IntoEnumIterator;

use super::ast::{
    expression::{Expression, ExpressionKind},
    operators::{BinaryOp, InplaceOp, UnaryOp},
    statements::{Statement, Statements},
};
use super::{Transpiler, Formatter, c_translations::CSyntax};

#[test]
fn binary_ops() {
    let x = Box::new(Expression {
        kind: ExpressionKind::Variable("x".to_string()),
    });

    for op in BinaryOp::iter().filter(|op| *op!=BinaryOp::ShiftCircularLeft && *op!=BinaryOp::ShiftCircularRight) {
        let ast = vec![Statement::Expression(Expression {
            kind: ExpressionKind::Binary(
                op.clone(),
                x.clone(),
                Box::new(Expression {
                    kind: ExpressionKind::Literal,
                }),
            ),
        })];
    
        let translation = Transpiler::transpile(&ast);
    
        assert_eq!(translation, Formatter::main(format!("(x){}(1)\n", op.to_c_syntax())));
    }
}

#[test]
fn unary_ops() {
    let x = Box::new(Expression {
        kind: ExpressionKind::Variable("x".to_string()),
    });

    for op in UnaryOp::iter() {
        let ast = vec![Statement::Expression(Expression {
            kind: ExpressionKind::Unary(
                op.clone(),
                x.clone(),
            ),
        })];

        let translation = Transpiler::transpile(&ast);

        assert_eq!(translation, Formatter::main(format!("{}(x)\n", op.to_c_syntax())));

    }
}