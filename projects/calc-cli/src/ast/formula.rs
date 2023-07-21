#[derive(Debug)]
pub enum FormulaError {
    InvalidCharacter(char),
    InvalidOperator(char),
}

impl FormulaError {
    pub fn message(&self) -> String {
        match self {
            FormulaError::InvalidCharacter(cr) => format!("Invalid character: {cr}"),
            FormulaError::InvalidOperator(op) => format!("Invalid operator: {op}"),
        }
    }
}
