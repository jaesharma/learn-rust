use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Secret number: {}", secret_number);
    loop {
        println!("Guess the number: ");
        let mut num = String::new();
        io::stdin()
            .read_line(&mut num)
            .expect("Error while reading guessed number");
        let num: u32 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match num.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small".red()),
            Ordering::Greater => println!("{}", "Too big".red()),
            Ordering::Equal => {
                println!("{}", "You win".green());
                break;
            }
        }
    }
}
