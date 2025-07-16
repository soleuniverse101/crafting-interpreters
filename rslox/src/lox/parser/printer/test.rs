use crate::lox::{
    parser::{
        expressions::{Binary, Expr, Grouping, Literal, LiteralValue, Unary},
        printer::AstPrinter,
    },
    token::{Token, TokenType},
};

#[test]
fn test_simple_syntax_tree() {
    let expression = Expr::Binary(Binary {
        left: Box::new(Unary::new(
            Token {
                _type: TokenType::Minus,
                lexeme: "-".to_owned(),
                line: 1,
            },
            Box::new(Literal::new(LiteralValue::Number(123.0))),
        )),
        operator: Token {
            _type: TokenType::Star,
            lexeme: "*".to_owned(),
            line: 1,
        },
        right: Box::new(Grouping::new(Box::new(Literal::new(LiteralValue::Number(
            45.67,
        ))))),
    });

    assert_eq!(AstPrinter::print(&expression), "(* (- 123) (group 45.67))");
}
