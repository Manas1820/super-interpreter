use super::{token::Token, Literal};

/*

The Expression struct should have the following methods

expression     → equality ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary
               | primary ;
primary        → NUMBER | STRING | "true" | "false" | "nil"
               | "(" expression ")" ;

*/

#[derive(Debug, Clone)]
pub enum Expression {
    Binary {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Grouping(Box<Expression>),
    Unary {
        operator: Token,
        right: Box<Expression>,
    },
    Literal(Literal),
}

impl Expression {
    pub fn new_binary(left: Box<Expression>, operator: Token, right: Box<Expression>) -> Self {
        Self::Binary {
            left,
            operator,
            right,
        }
    }

    pub fn new_grouping(expression: Expression) -> Self {
        Self::Grouping(Box::new(expression))
    }

    pub fn new_unary(operator: Token, right: Box<Expression>) -> Self {
        Self::Unary { operator, right }
    }

    pub fn new_literal(literal: Literal) -> Self {
        Self::Literal(literal)
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                write!(f, "({} {} {})", operator.lexeme, left, right)
            }
            Expression::Unary { operator, right } => {
                write!(f, "({} {})", operator.lexeme, right)
            }
            Expression::Literal(literal) => {
                write!(f, "{}", literal)
            }
            Expression::Grouping(literal) => {
                write!(f, "(group {})", literal)
            }
            _ => {
                write!(f, "null")
            }
        }
    }
}
