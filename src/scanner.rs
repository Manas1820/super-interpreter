use crate::domain::token::Token;
use crate::domain::token_type::TokenType;

/*
    The Scanner is responsible for converting the source code into a sequence of tokens.
    The Scanner will read the source code character by character and convert it into tokens.

    Reference - https://craftinginterpreters.com/scanning.html#recognizing-lexemes
*/

#[derive(Debug, Clone)]
pub struct Scanner {
    pub source: Vec<char>,
    pub tokens: Vec<Token>,
    pub start: usize,
    pub current: usize,
    pub line: u32,
    pub column: u32,
    pub errors: Vec<ScannerError>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            column: 0,
            errors: Vec::new(),
        }
    }

    pub fn scan_tokens(&mut self) {
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
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token<'a>(&mut self) {
        let current_char = Self::advance(self);
        dbg!(self.current.clone());
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
            '!' => {
                if Self::advance_peek(self, '=') {
                    Self::add_token(self, TokenType::BangEqual);
                } else {
                    Self::add_token(self, TokenType::Bang);
                }
            }
            '=' => {
                if Self::advance_peek(self, '=') {
                    Self::add_token(self, TokenType::EqualEqual);
                } else {
                    Self::add_token(self, TokenType::Equal);
                }
            }
            '<' => {
                if Self::advance_peek(self, '=') {
                    Self::add_token(self, TokenType::LessEqual);
                } else {
                    Self::add_token(self, TokenType::Less);
                }
            }
            '>' => {
                if Self::advance_peek(self, '=') {
                    Self::add_token(self, TokenType::GreaterEqual);
                } else {
                    Self::add_token(self, TokenType::Greater);
                }
            }
            '/' => {
                if Self::advance_peek(self, '/') {
                    // A comment goes until the end of the line.
                    while self.peek() != '\n' && !self.is_at_end() {
                        Self::advance(self);
                    }
                    self.line += 1;
                } else {
                    Self::add_token(self, TokenType::Slash);
                }
            }
            ' ' | '\r' | '\t' => {
                // Ignore whitespace.
            }
            '\n' => {
                self.line += 1;
                self.column = 0;
            }
            _ => {
                self.errors.push(ScannerError {
                    message: format!("Unexpected character: {}", current_char),
                    line: self.line,
                    column: self.column,
                });
            }
        }
    }

    fn advance(&mut self) -> char {
        let current_char = self.source[self.current];
        self.current += 1;
        self.column += 1;

        current_char
    }

    fn advance_peek(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        let next_char = self.source[self.current];
        if next_char != expected {
            return false;
        }

        self.current += 1;
        self.column += 1;

        true
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.source[self.current]
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = self.source[self.start..self.current].iter().collect();
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

impl std::fmt::Display for ScannerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[line {}] Error: {}", self.line, self.message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scanner_codecrafter_testcase() {
        let source = "(()".to_string();
        let mut scanner = Scanner::new(source);

        scanner.scan_tokens();
        assert_eq!(scanner.tokens.len(), 4);
        assert_eq!(scanner.tokens[0].token_type, TokenType::LeftParen);
        assert_eq!(scanner.tokens[1].token_type, TokenType::LeftParen);
        assert_eq!(scanner.tokens[2].token_type, TokenType::RightParen);
        assert_eq!(scanner.tokens[3].token_type, TokenType::Eof);
    }

    #[test]
    fn test_scan_tokens() {
        let source = "(){},.-+;*".to_string();
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 11);
        assert_eq!(scanner.tokens[0].token_type, TokenType::LeftParen);
        assert_eq!(scanner.tokens[1].token_type, TokenType::RightParen);
        assert_eq!(scanner.tokens[2].token_type, TokenType::LeftBrace);
        assert_eq!(scanner.tokens[3].token_type, TokenType::RightBrace);
        assert_eq!(scanner.tokens[4].token_type, TokenType::Comma);
        assert_eq!(scanner.tokens[5].token_type, TokenType::Dot);
        assert_eq!(scanner.tokens[6].token_type, TokenType::Minus);
        assert_eq!(scanner.tokens[7].token_type, TokenType::Plus);
        assert_eq!(scanner.tokens[8].token_type, TokenType::Semicolon);
        assert_eq!(scanner.tokens[9].token_type, TokenType::Star);
        assert_eq!(scanner.tokens[10].token_type, TokenType::Eof);
    }
}
