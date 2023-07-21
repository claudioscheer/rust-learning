#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
}

#[derive(Debug, PartialEq)]
pub enum Node {
    Value(f64),
    Operator(Operator),
}

#[derive(Debug)]
pub enum FormulaError {
    InvalidCharacter(char),
    InvalidOperator(Operator),
    ParseError(String),
}

impl FormulaError {
    pub fn message(&self) -> String {
        match self {
            FormulaError::InvalidCharacter(cr) => format!("Invalid character: {cr}"),
            FormulaError::InvalidOperator(op) => format!("Invalid operator: {:?}", op),
            FormulaError::ParseError(value) => format!("Error to parse value {value}"),
        }
    }
}
