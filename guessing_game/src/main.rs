use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let secret = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Guess a number");

        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            // below signifies what is returned if the read_line function errors
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            },
        };
    
        println!("You guessed {}, and the correct answer is {}", guess, secret);
    
        match guess.cmp(&secret) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("Congrats!");
                break;
            }
        }
    }
}
