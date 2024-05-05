use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{guess} is not a number, please retry");
                continue;
            },
        };
        println!("You guessed: {guess}");

        let whatfrommatch = match guess.cmp(&secret_number) {
            Ordering::Less => "too small!",
            Ordering::Greater => "too big!",
            Ordering::Equal => {
                println!("Equal, you win!");
                break;
            },
        };

        println!("You get it {whatfrommatch}");
    }
}
