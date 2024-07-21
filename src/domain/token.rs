use super::literal::Literal;
use super::token_type::TokenType;

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: u32,
    pub column: u32,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<Literal>,
        line: u32,
        column: u32,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
            column,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(literal) = &self.literal {
            match literal {
                Literal::Boolean(_) => {
                    return write!(f, "{} {} null", self.token_type, self.lexeme)
                }
                Literal::Identifier(_) => {
                    return write!(f, "{} {} null", self.token_type, self.lexeme)
                }
                Literal::Nil => return write!(f, "{} {} null", self.token_type, literal),
                _ => return write!(f, "{} {} {}", self.token_type, self.lexeme, literal),
            }
        } else {
            write!(f, "{} {} null", self.token_type, self.lexeme)
        }
    }
}
