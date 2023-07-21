mod sya;

use std::io::{self, Write};

fn main() -> io::Result<()> {
    print!("Type the formula: ");
    io::stdout().flush()?;

    let mut formula = String::new();
    io::stdin().read_line(&mut formula)?;

    let postfix = sya::parse::infix_to_postfix(&formula.trim()).unwrap();
    println!("The postfix formula is: {:?}", postfix);

    let result = sya::evaluate::evaluate_postfix(&postfix).unwrap();
    println!("The result is: {result}");

    return Ok(());
}
