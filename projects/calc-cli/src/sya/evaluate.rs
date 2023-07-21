use super::formula::{FormulaError, Node, Operator};

fn add(a: f64, b: f64) -> f64 {
    return a + b;
}

fn sub(a: f64, b: f64) -> f64 {
    return a - b;
}

fn operate(a: f64, b: f64, operator: Operator) -> f64 {
    match operator {
        Operator::Plus => return add(a, b),
        Operator::Minus => return sub(a, b),
    }
}

pub fn evaluate_postfix(nodes: &Vec<Node>) -> Result<f64, FormulaError> {
    let mut stack: Vec<f64> = Vec::new();

    for node in nodes {
        match node {
            Node::Value(n) => stack.push(n.to_owned()),
            Node::Operator(op) => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(operate(a, b, *op));
            }
        }
    }

    return Ok(stack.pop().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1., 2.), 3.);
    }

    #[test]
    fn test_sub() {
        assert_eq!(sub(1., 2.), -1.);
    }

    #[test]
    fn test_evaluate_postfix() -> Result<(), FormulaError> {
        assert_eq!(
            evaluate_postfix(&vec![
                Node::Value(3.),
                Node::Value(4.),
                Node::Operator(Operator::Plus)
            ])?,
            7.
        );
        assert_eq!(
            evaluate_postfix(&vec![
                Node::Value(3.),
                Node::Value(4.),
                Node::Operator(Operator::Plus),
                Node::Value(11.),
                Node::Operator(Operator::Minus)
            ])?,
            -4.
        );

        return Ok(());
    }
}
