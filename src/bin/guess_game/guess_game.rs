use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn start_game() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(0..101);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(i) => i,
            Err(_) => {
                println!("need input number!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };

        println!("You guessed: {}", guess);
    }
}