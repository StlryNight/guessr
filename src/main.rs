use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Number guessing game!");
    let secret_number = rand::rng().random_range(0..=15);
    println!("---------------------");
    println!("I'm thinking of a number from 0 to 15");
    println!("---------------------");
    loop {
        println!("Guess the number!");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("To low"),
            Ordering::Greater => println!("Too High"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}