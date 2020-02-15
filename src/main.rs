extern crate rand;

use std::io;
use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() {
    println!("Creating a random list!");
    /*
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
    */

    let mut input = read_input();

    let mut random = thread_rng();
    input.as_mut_slice().shuffle(&mut random);
    //rand::thread_rng().shuffle(&mut input);

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