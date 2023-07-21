// https://en.wikipedia.org/wiki/Shunting_yard_algorithm
// https://faculty.cs.niu.edu/~hutchins/csci241/eval.htm

use super::formula::{FormulaError, Node, Operator};
use std::ops::Not;

fn parse_string_to_f64(value: &String) -> Result<Node, FormulaError> {
    match value.parse::<f64>() {
        Ok(n) => Ok(Node::Value(n)),
        Err(_) => return Err(FormulaError::ParseError(value.to_owned())),
    }
}

pub fn infix_to_postfix(formula: &str) -> Result<Vec<Node>, FormulaError> {
    let mut output: Vec<Node> = Vec::new();
    let mut operators: Vec<Node> = Vec::new();
    let mut number = String::new();

    for token in formula.chars() {
        match token {
            '0'..='9' | '.' => number.push(token),
            '+' | '-' => {
                if number.is_empty().not() {
                    output.push(parse_string_to_f64(&number)?);
                    number.clear();
                }

                while let Some(op) = operators.pop() {
                    output.push(op);
                }

                match token {
                    '+' => operators.push(Node::Operator(Operator::Plus)),
                    '-' => operators.push(Node::Operator(Operator::Minus)),
                    _ => {}
                }
            }
            ' ' => {
                if number.is_empty().not() {
                    output.push(parse_string_to_f64(&number)?);
                    number.clear();
                }
            }
            _ => return Err(FormulaError::InvalidCharacter(token)),
        }
    }

    if number.is_empty().not() {
        output.push(parse_string_to_f64(&number)?);
    }

    while let Some(op) = operators.pop() {
        output.push(op);
    }

    return Ok(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infix_to_postfix() -> Result<(), FormulaError> {
        assert_eq!(
            infix_to_postfix("3 + 4 - 1")?,
            vec![
                Node::Value(3.0),
                Node::Value(4.0),
                Node::Operator(Operator::Plus),
                Node::Value(1.0),
                Node::Operator(Operator::Minus)
            ]
        );
        assert_eq!(
            infix_to_postfix("30 + 45")?,
            vec![
                Node::Value(30.0),
                Node::Value(45.0),
                Node::Operator(Operator::Plus),
            ]
        );
        assert_eq!(
            infix_to_postfix("30.5 + 45 - 1")?,
            vec![
                Node::Value(30.5),
                Node::Value(45.0),
                Node::Operator(Operator::Plus),
                Node::Value(1.),
                Node::Operator(Operator::Minus),
            ]
        );

        Ok(())
    }

    #[test]
    fn test_infix_to_postfix_err() {
        let res = infix_to_postfix("1 + 2b");

        assert!(res.is_err_and(|e| e.message() == "Invalid character: b"));
    }
}
