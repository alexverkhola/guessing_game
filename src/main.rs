extern crate rand;



use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1, 100);

    println!("Guess the number");

    println!("Please, enter your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

    let guess: u32 = guess.trim().parse()
        .expect("Pleace type a number");

    println!("You guessed {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("To small, real number is {}", &secret_number),
        Ordering::Greater => println!("To big real number is {}", &secret_number),
        Ordering::Equal => println!("You win"),

    }
}
