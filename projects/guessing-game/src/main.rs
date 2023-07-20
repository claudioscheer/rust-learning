mod random_number;
use std::cmp::Ordering;
use std::io::{stdin, stdout, Write};

fn main() {
    println!("Guess the random number.");
    let n = random_number::random_n();
    let mut guesses = 0;

    loop {
        guesses += 1;

        print!("Enter an integer: ");
        stdout().flush().unwrap();

        let mut user_input = String::new();
        let text_length = stdin()
            .read_line(&mut user_input)
            .expect("Failed to read from stdin.");
        assert_eq!(user_input.len(), text_length);

        let user_n: i32;
        match user_input.trim().parse::<i32>() {
            Ok(v) => {
                user_n = v;
            }
            Err(e) => {
                eprintln!("The value must be an integer. Err: {e}");
                continue;
            }
        }

        match user_n.cmp(&n) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win. you needed {guesses} guesses.");
                break;
            }
        }
    }
}
