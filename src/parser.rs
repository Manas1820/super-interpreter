use crate::domain::{token::Token, Expression, Literal, TokenType};

pub struct ParserError {
    pub message: String,
    pub token: Token,
}

impl ParserError {
    pub fn new(message: String, token: Token) -> Self {
        Self { message, token }
    }
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[line {}] Error: {}", self.token.line, self.message)
    }
}

pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize,
    pub errors: Vec<ParserError>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            errors: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Vec<Expression> {
        let mut expressions: Vec<Expression> = Vec::new();
        while !self.is_at_end() {
            expressions.push(self.expression());
        }
        expressions
    }

    // basic methods to help with parsing

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn advance_for_token_types(&mut self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.check_future_for_token(token_type) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    // will be used to check if the current token is of  the expected type
    // if it is, we will just peek  the token and not consume
    fn check_future_for_token(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }
}

/*

The Parser struct should have the following methods

Algorithm: Recursive Descent Parsing

*/

impl Parser {
    fn expression(&mut self) -> Expression {
        self.equality()
    }

    fn equality(&mut self) -> Expression {
        let mut expression: Expression = self.comparison();

        while self.advance_for_token_types(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator: Token = self.previous();
            let right: Expression = self.comparison();
            expression = Expression::new_binary(Box::new(expression), operator, Box::new(right));
        }

        expression
    }

    fn comparison(&mut self) -> Expression {
        let mut expression = self.term();
        while self.advance_for_token_types(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term();
            expression = Expression::new_binary(Box::new(expression), operator, Box::new(right));
        }
        expression
    }

    fn term(&mut self) -> Expression {
        let mut expression = self.factor();
        while self.advance_for_token_types(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor();
            expression = Expression::new_binary(Box::new(expression), operator, Box::new(right));
        }
        expression
    }

    fn factor(&mut self) -> Expression {
        let mut expression = self.unary();
        while self.advance_for_token_types(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary();
            expression = Expression::new_binary(Box::new(expression), operator, Box::new(right));
        }
        expression
    }

    fn unary(&mut self) -> Expression {
        if self.advance_for_token_types(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary();
            return Expression::new_unary(operator, Box::new(right));
        }

        self.primary()
            .unwrap_or(Expression::new_literal(Literal::Nil))
    }

    fn primary(&mut self) -> Option<Expression> {
        if self.advance_for_token_types(vec![
            TokenType::False,
            TokenType::True,
            TokenType::Nil,
            TokenType::Number,
            TokenType::String,
        ]) {
            match self.previous().literal.unwrap() {
                // if the literal is a number, we will return a number expression
                // with the value of the number
                // if the literal is a string, we will return a string expression
                // with the value of the string
                // if the literal is a boolean, we will return a boolean expression
                // with the value of the boolean
                // if the literal is nil, we will return a nil expression
                // with the value of the nil
                // if the literal is a identifier, we will return an expression
                // with the value of the identifier
                Literal::Number(value) => {
                    return Some(Expression::new_literal(Literal::Number(value)))
                }
                Literal::String(value) => {
                    return Some(Expression::new_literal(Literal::String(value)))
                }
                Literal::Boolean(value) => {
                    return Some(Expression::new_literal(Literal::Boolean(value)))
                }
                Literal::Nil => return Some(Expression::new_literal(Literal::Nil)),
                Literal::Identifier(value) => {
                    return Some(Expression::new_literal(Literal::Identifier(value)))
                }
            };
        }

        if self.advance_for_token_types(vec![TokenType::LeftParen]) {
            let expression = self.expression();
            if self.check_future_for_token(TokenType::RightParen) {
                self.advance();
                return Some(Expression::new_grouping(expression));
            }
            self.errors.push(ParserError {
                message: "Expect ')' after expression.".to_string(),
                token: self.peek(),
            });
        }

        self.errors.push(ParserError {
            message: "".to_string(),
            token: self.peek(),
        });

        None
    }
}
