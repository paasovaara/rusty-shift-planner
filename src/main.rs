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
    //print_output(&input);

    //let print_closure = |name: &String| {
//
    //let print_closure = (name: &String) {
    /*let print_closure = |name: &String| {
        println!("Next one is...");
        thread::sleep(Duration::from_secs(1));
        name
    };*/

    print_output_via_decorator(input, delay);
}

fn delay(name: &String) -> &String {
    println!("Next one is...");
    thread::sleep(Duration::from_secs(1));
    name
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

fn print_output(output: &Vec<String>/*, printer: &dyn Fn(&String) -> &String*/) {
    //fn print_output(output: Vec<String>, printer: &dyn Fn(&String) -> &String) {
    for out in output {
        thread::sleep(Duration::from_secs(1));
        println!("{}", out)
        //
        // println!("{}", printer(out));
    }
}
// Function overloading not supported, consider using traits
// https://stackoverflow.com/questions/42236166/is-it-possible-to-overload-a-function-with-different-numbers-of-arguments-using
//fn print_output_via_decorator(output: Vec<String>, decorator: &dyn Fn(&String) -> &String) {
fn print_output_via_decorator(output: Vec<String>, decorator: fn(&String) -> &String) {
    for out in &output {
        println!("{}", decorator(out));
    }
}

fn read_file_content(path: String) -> Vec<String> {
    //TODO Error handling
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