extern crate rand;

//use std::io;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::{thread, time::Duration};
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

    let mut input = read_file_content("names.txt".to_string());

    let mut random = thread_rng();
    input.shuffle(&mut random);
//let print_closure = |name: &String| {
//
    /*let print_closure = (name: &String) {
        println!("Next one is...");
        thread::sleep(Duration::from_secs(1));
        name
    };*/
    print_output(input);
    //print_output(input, &print_closure);
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

fn print_output(output: Vec<String>/*, printer: &dyn Fn(&String) -> &String*/) {
    //fn print_output(output: Vec<String>, printer: &dyn Fn(&String) -> &String) {
    for out in &output {
        thread::sleep(Duration::from_secs(1));
        println!("{}", out)
        //
        // println!("{}", printer(out));
    }
}

fn read_file_content(path: String) -> Vec<String> {
    let f = File::open(path).expect("Unable to open file");
    let reader = BufReader::new(f);
    let mut lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line_content = line.expect("Unable to read line");
        //println!("Line: {}", line_content);
        lines.push(line_content.trim().to_string());
    }
    lines
}