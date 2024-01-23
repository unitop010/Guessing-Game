extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101);
    
    //print secret number
    //println!("The secret number is: {}", secret_number);
    
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // let guess: u32 = match guess.trim().parse(){
        //     Ok(num) => num,
        //     Err(_) => continue,
        // };

        //print Invalid input
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. You must enter a number.");
                // return;
                continue;
            }
        };

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("To small!"),
            Ordering::Greater => println!("To big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        //without declaring Ordering
        // match guess.cmp(&secret_number){
        //     std::cmp::Ordering::Less => println!("Too small!"),
        //     std::cmp::Ordering::Greater => println!("Too big!"),
        //     std::cmp::Ordering::Equal => println!("You win!"),
        // }
    }
}
