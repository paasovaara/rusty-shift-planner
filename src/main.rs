use std::io;

fn main() {
    println!("Creating a random list!");
    /*
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
    */

    let input = read_input();
    print_output(input);
}

fn read_input() -> Vec<String> {
    println!("Reading input variables");
    let vec: Vec<String> = vec![
        "Markus".to_string(),
        "Batman".to_string(),
        "Joker".to_string(),
        "Alfred".to_string()
    ];
    vec
}

fn print_output(output: Vec<String>) {
    for out in &output {
        println!("{}", out);
    }
}