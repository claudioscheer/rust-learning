#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    InvalidCharacter,
    InvalidOperator,
}

#[derive(Debug)]
pub enum FormulaError {
    InvalidCharacter(char),
    InvalidOperator(char),
}

impl FormulaError {
    pub fn kind(&self) -> ErrorKind {
        match self {
            FormulaError::InvalidCharacter(_) => ErrorKind::InvalidCharacter,
            FormulaError::InvalidOperator(_) => ErrorKind::InvalidOperator,
        }
    }

    pub fn message(&self) -> String {
        match self {
            FormulaError::InvalidCharacter(cr) => format!("Invalid character: {cr}"),
            FormulaError::InvalidOperator(op) => format!("Invalid operator: {op}"),
        }
    }
}
