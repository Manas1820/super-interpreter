#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Identifier { value: String },
    String { value: String },
    Number { value: f64 },
    Boolean { value: bool },
    Nil,
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Identifier { value: _ } => write!(f, "null"),
            Literal::String { value } => write!(f, "{}", value),
            Literal::Number { value } => write!(f, "{:?}", value),
            Literal::Boolean { value } => write!(f, "{}", value),
            Literal::Nil => write!(f, "null"),
        }
    }
}
