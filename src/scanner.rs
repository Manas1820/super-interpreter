use crate::domain::token::Token;
use crate::domain::token_type::TokenType;

#[derive(Debug, Clone)]
pub struct Scanner {
    pub source: String,
    pub tokens: Vec<Token>,
    pub start: usize,
    pub current: usize,
    pub line: u32,
    pub column: u32,
    pub errors: Option<ScannerError>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            column: 1,
            errors: None,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            Self::scan_token(self);
        }

        self.tokens.push(Token::new(
            TokenType::Eof,
            "".to_string(),
            None,
            self.line,
            self.column,
        ));
        self.tokens.clone()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token<'a>(&mut self) {
        let current_char = Self::advance(self);
        match current_char {
            '(' => Self::add_token(self, TokenType::LeftParen),
            ')' => Self::add_token(self, TokenType::RightParen),
            '{' => Self::add_token(self, TokenType::LeftBrace),
            '}' => Self::add_token(self, TokenType::RightBrace),
            ',' => Self::add_token(self, TokenType::Comma),
            '.' => Self::add_token(self, TokenType::Dot),
            '-' => Self::add_token(self, TokenType::Minus),
            '+' => Self::add_token(self, TokenType::Plus),
            ';' => Self::add_token(self, TokenType::Semicolon),
            '*' => Self::add_token(self, TokenType::Star),
            _ => {
                todo!("Implement the rest of the scan_token function")
            }
        }
    }

    fn advance(&mut self) -> char {
        let current_char = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        self.column += 1;

        current_char
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(token_type, text, None, self.line, self.column));
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ScannerError {
    pub message: String,
    pub line: u32,
    pub column: u32,
}

impl ScannerError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scanner_codecrafter_testcase() {
        let source = "(()".to_string();
        let mut scanner = Scanner::new(source);

        let tokens = scanner.scan_tokens();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].token_type, TokenType::LeftParen);
        assert_eq!(tokens[1].token_type, TokenType::LeftParen);
        assert_eq!(tokens[2].token_type, TokenType::RightParen);
        assert_eq!(tokens[3].token_type, TokenType::Eof);
    }

    #[test]
    fn test_scan_tokens() {
        let source = "(){},.-+;*".to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens.len(), 11);
        assert_eq!(tokens[0].token_type, TokenType::LeftParen);
        assert_eq!(tokens[1].token_type, TokenType::RightParen);
        assert_eq!(tokens[2].token_type, TokenType::LeftBrace);
        assert_eq!(tokens[3].token_type, TokenType::RightBrace);
        assert_eq!(tokens[4].token_type, TokenType::Comma);
        assert_eq!(tokens[5].token_type, TokenType::Dot);
        assert_eq!(tokens[6].token_type, TokenType::Minus);
        assert_eq!(tokens[7].token_type, TokenType::Plus);
        assert_eq!(tokens[8].token_type, TokenType::Semicolon);
        assert_eq!(tokens[9].token_type, TokenType::Star);
        assert_eq!(tokens[10].token_type, TokenType::Eof);
    }
}
