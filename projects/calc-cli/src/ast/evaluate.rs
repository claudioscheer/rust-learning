use super::formula::FormulaError;

fn add(a: String, b: String) -> String {
    return (a.parse::<i32>().unwrap() + b.parse::<i32>().unwrap()).to_string();
}

fn sub(a: String, b: String) -> String {
    return (a.parse::<i32>().unwrap() - b.parse::<i32>().unwrap()).to_string();
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
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(operate(a, b, token)?);
            }
            ' ' => {}
            _ => return Err(FormulaError::InvalidCharacter(token)),
        }
    }

    return Ok(stack.pop().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add("1".to_string(), "2".to_string()), "3");
    }

    #[test]
    fn test_sub() {
        assert_eq!(sub("1".to_string(), "2".to_string()), "-1");
    }

    #[test]
    fn test_evaluate_postfix() -> Result<(), FormulaError> {
        assert_eq!(evaluate_postfix("3 4 +")?, "7");
        assert_eq!(evaluate_postfix("3 4 + 9 -")?, "-2");
        assert_eq!(evaluate_postfix("3 4 + 9 - 5 + 4 - 2 -")?, "-3");

        return Ok(());
    }
}
