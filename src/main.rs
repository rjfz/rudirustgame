use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the fookin number");

    let super_secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
        println!("ENTER YE NUMBER");
    

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("I canny read it");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    println!("you decided to guess: {guess}");

    match guess.cmp(&super_secret_number) {
        Ordering::Less => println!("TOO SMALL BOZO."),
        Ordering::Greater => println!("TOO BIG"),
        Ordering::Equal => {
            println!("Lucker.");
             break;
         }
        }
    }
}
