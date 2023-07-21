// https://en.wikipedia.org/wiki/Shunting_yard_algorithm
// https://faculty.cs.niu.edu/~hutchins/csci241/eval.htm

use super::formula::FormulaError;

pub fn infix_to_postfix(formula: &str) -> Result<String, FormulaError> {
    let mut output: Vec<char> = Vec::new();
    let mut operators: Vec<char> = Vec::new();

    for token in formula.chars() {
        match token {
            '0'..='9' => output.push(token),
            '+' | '-' => {
                while !operators.is_empty() {
                    output.push(operators.pop().unwrap())
                }
                operators.push(token);
            }
            ' ' => {}
            _ => return Err(FormulaError::InvalidCharacter(token)),
        }
    }

    while let Some(op) = operators.pop() {
        output.push(op);
    }

    let output_string: Vec<String> = output.iter().map(|c| c.to_string()).collect();
    return Ok(output_string.join(" "));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infix_to_postfix() -> Result<(), FormulaError> {
        assert_eq!(infix_to_postfix("3 + 4")?, "3 4 +",);
        assert_eq!(infix_to_postfix("3 + 4 - 1")?, "3 4 + 1 -",);
        assert_eq!(infix_to_postfix("1 + 2 - 3")?, "1 2 + 3 -",);
        assert_eq!(infix_to_postfix("1 + 2 + 3")?, "1 2 + 3 +",);

        Ok(())
    }

    #[test]
    fn test_infix_to_postfix_err() {
        let res = infix_to_postfix("1 + 2b");

        assert!(res.is_err_and(|e| e.message() == "Invalid character: b"));
    }
}
