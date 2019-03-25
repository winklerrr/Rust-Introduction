use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("(The secret number is: {})", secret_number);

    loop {
        println!("Please input your guess:");

        let mut guess = String::new(); // all let variables are by default immutable
        io::stdin().read_line(&mut guess).expect("Failed to read line!"); // no expect => warning will be emitted by the compiler
        let guess: u32 = match guess.trim().parse() { // instead of .expect("Please type a number!");
            Ok(number) => number,
            Err(_) => {
                println!("Please type a number!");
                continue;
            },
        };

        print!("You guessed: {} - ", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("you win!");
                break;
            },
        }
    }
}
