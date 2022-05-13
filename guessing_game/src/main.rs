use std::io;
use std::io::Write;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let mut secret: u32 = rand::thread_rng().gen_range(1..101);
    let mut guess: String;
    let mut guesses: i8 = 0;

    loop {
        guess = String::new();

        println!("Guess the number!");
        print!("> "); // Prompt

        io::stdout() // Ensure prompt is printed immediately
            .flush()
            .unwrap();

        io::stdin() // Capture input
            .read_line(&mut guess)
            .expect("Failed to read line");

        if guess.trim().eq("quit") { break; } // Quit on user prompt

        let guess: u32 = guess.trim().parse().expect("Expected a number."); // Shadow original guess var with u32

        println!("You guessed {}.", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Good guess!");
                guesses = 0;
                secret = rand::thread_rng().gen_range(1..101);
            },
        }

        if guesses + 1 > 5 {
            println!("The correct number was {}.", secret);
            println!("Better luck next time.\n");
            guesses = 0;
            secret = rand::thread_rng().gen_range(1..101);
        } else {
            guesses += 1;
        }
    }
}


