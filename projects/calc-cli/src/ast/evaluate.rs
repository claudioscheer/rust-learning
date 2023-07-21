use super::formula::FormulaError;

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn sub(a: i32, b: i32) -> i32 {
    return a - b;
}

fn operate(a: i32, b: i32, operator: char) -> Result<i32, FormulaError> {
    match operator {
        '+' => return Ok(add(a, b)),
        '-' => return Ok(sub(a, b)),
        _ => return Err(FormulaError::InvalidOperator(operator)),
    }
}

pub fn evaluate_postfix(formula: &str) -> Result<i32, FormulaError> {
    let mut stack: Vec<i32> = Vec::new();

    for token in formula.chars() {
        match token {
            '0'..='9' => {
                let n = token
                    .to_digit(10)
                    .ok_or_else(|| FormulaError::ParseError(token))? as i32;
                stack.push(n);
            }
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
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_sub() {
        assert_eq!(sub(1, 2), -1);
    }

    #[test]
    fn test_evaluate_postfix() -> Result<(), FormulaError> {
        assert_eq!(evaluate_postfix("3 4 +")?, 7);
        assert_eq!(evaluate_postfix("3 4 + 9 -")?, -2);
        assert_eq!(evaluate_postfix("3 4 + 9 - 5 + 4 - 2 -")?, -3);

        return Ok(());
    }
}
