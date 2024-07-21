#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Identifier(String),
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Identifier(identifier) => write!(f, "null"),
            Literal::String(string) => write!(f, "{}", string),
            Literal::Number(number) => write!(f, "{:?}", number),
            Literal::Boolean(boolean) => write!(f, "{}", boolean),
            Literal::Nil => write!(f, "nil"),
        }
    }
}
