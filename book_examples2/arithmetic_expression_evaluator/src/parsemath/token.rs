#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Add,
    Substract,
    Multiply,
    Divide,
    Caret,
    LeftParen,
    RightParen,
    Num(f64),
    EOF,
}
