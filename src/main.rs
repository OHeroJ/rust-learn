use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Hello, world!");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("the secret number is : {}", secret_number);
    println!("please input your guess");

    loop {
        println!("please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
