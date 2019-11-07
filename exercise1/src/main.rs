use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut number_of_tries: u32 = 0;

    //println!("The secret number is: {}", secret_number);

    loop {
        number_of_tries = number_of_tries + 1;

        println!("Please input your guess. Try number: {}", number_of_tries);

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You Win in {} tries", number_of_tries);
                break;
            }
        }
    }
}
