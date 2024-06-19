use rand::Rng;
use std::io;

fn to_number(value: String) -> Result<i32, String> {
    match value.trim().parse() {
        Ok(num) => Ok(num),
        Err(_) => Err(String::from("Incorrect number")),
    }
}
fn check(answer: i32, guess: i32) -> bool{
    return if answer == guess {
        println!("You won!");
        true
    } else {
        let a: [i32; 3] = [answer / 100, (answer / 10) % 10, answer % 10];
        let b: [i32; 3] = [guess / 100, (guess / 10) % 10, guess % 10];

        let mut same = 0;
        let mut almost = 0;
        for i in 0..3 {
            if a[i] == b[i] {
                same += 1;
            } else {
                for j in 0..3 {
                    if i != j && a[i] == b[j] {
                        almost += 1;
                        break;
                    }
                }
            }
        }

        println!(
            "There are {} on correct position{}",
            same,
            if same > 1 { "s" } else { "" }
        );
        println!(
            "There are {} that are almost on correct position {}",
            almost,
            if almost > 1 { "s" } else { "" }
        );
        false
    }
}

fn main() {
    println!("Welcome to guessing number!");

    let mut rng = rand::thread_rng();

    let random_number: i32 = rng.gen_range(100..1000);

    let mut guessed: bool = false;
    while !guessed {
        println!("What's your guess between 100 and 999?");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");

        let current_guess = match to_number(input) {
            Ok(current_guess) => {
                if current_guess < 100 || current_guess > 999 {
                    println!("Enter a number between 100 and 999!");
                    continue;
                } else {
                    current_guess
                }
            }
            Err(err) => {
                println!("{}", err);
                continue; // Continue the loop on error
            }
        };

        guessed = check(random_number, current_guess);
    }
}
