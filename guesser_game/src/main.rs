use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the number!!!");
    println!("Please input your guess.");

    let secret_num = rand::thread_rng().gen_range(1..=10);
    // println!("Generated {secret_num}");
    loop {
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read user input");

        //Shadowing
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_)=>{
                println!("Yeda hai kya?");
                continue;
            }
        };

        println!("You guessed {guess}");
        match guess.cmp(&secret_num) {
            Ordering::Equal => {
                println!("You Won!");
                break;
            },
            Ordering::Greater => println!("Too Big!"),
            Ordering::Less => println!("Too Small!"),
        }
    }
}
