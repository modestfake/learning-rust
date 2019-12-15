use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("===== GUESS THE NUMBER! =====");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut attempt = 0;

    loop {
        if attempt == 0 {
            println!("👩‍✈️ Please input your guess.");
        } else {
            println!("👩‍✈️ Please try again.");
        }

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        attempt = attempt + 1;

        println!("\n{}. You guessed: {}", attempt, guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("🧐  Too small!\n"),
            Ordering::Greater => println!("😮 Too big!\n"),
            Ordering::Equal => {
                println!("🎉 You win!\nAttempts: {}", attempt);
                break;
            }
        }
    }
}
