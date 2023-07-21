use super::formula::FormulaError;

fn add(a: String, b: String) -> String {
    return String::from("0");
}

fn sub(a: String, b: String) -> String {
    return String::from("0");
}

fn operate(a: String, b: String, operator: char) -> Result<String, FormulaError> {
    match operator {
        '+' => return Ok(add(a, b)),
        '-' => return Ok(sub(a, b)),
        _ => return Err(FormulaError::InvalidOperator(operator)),
    }
}

pub fn evaluate_postfix(formula: &str) -> Result<String, FormulaError> {
    let mut stack: Vec<String> = Vec::new();

    for token in formula.chars() {
        match token {
            '0'..='9' => stack.push(token.to_string()),
            '+' | '-' => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(operate(a, b, token)?);
            }
            ' ' => {}
            _ => return Err(FormulaError::InvalidCharacter(token)),
        }
    }

    return Ok(stack.pop().unwrap());
}
