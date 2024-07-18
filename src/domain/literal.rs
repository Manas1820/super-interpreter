#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub enum Literal {
    Identifier,
    String,
    Number,
    Boolean,
    Nil,
}
