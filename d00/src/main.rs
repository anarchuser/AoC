use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input: String = fs::read_to_string(&args[1])
        .expect("Input file missing");

    println!("first = {}", first(&input));
    println!("second = {}", second(&input));
}

fn first(input: &String) -> u32 {
    0
}

fn second(input: &String) -> u32 {
    0
}