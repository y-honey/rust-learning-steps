use std::io;

fn main() {
    println!("Guess the number!");
    println!("input number: ");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Fail to read line");
    println!("you guessed: {guess}");
}
