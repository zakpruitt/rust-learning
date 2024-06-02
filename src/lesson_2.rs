use std::io;

fn guessing_game() {
    println!("Guess the number!");
    print!("Please input your guess: ");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

    println!("You guessed: {guess}");
}

pub fn execute_lesson() {
    guessing_game();
}