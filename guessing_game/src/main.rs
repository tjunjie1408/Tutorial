use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is {}", secret_number);

    let mut min = 1;
    let mut max = 100;

    loop {
        println!("Please input your guess. (Range: {} - {})", min, max);

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            //Result handling for read_line:
            //---OK (usize of bytes read)
            //---Err (e) (std::io::Error)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; //shadowing

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                if guess >= min {
                    min = guess + 1;
                }
            }
            Ordering::Greater => {
                println!("Too big!");
                if guess <= max {
                    max = guess - 1;
                }
            }
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
