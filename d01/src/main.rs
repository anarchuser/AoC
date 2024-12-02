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
    input
        .split("\n")
        .map(|line| {
            line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
            })
        .map(|digits|
            match digits.last() {
                Some(last) => digits[0] * 10 + last,
                None => 0,
            })
        .sum()
}

fn second(input: &String) -> u32 {
    input
        .lines()
        .map(digits_from_line)
        .map(|digits|
            match digits.last() {
                Some(last) => digits[0] * 10 + last,
                None => 0,
            })
        .sum()
}

fn digits_from_line(line: &str) -> Vec<u32> {
    let mut digits: Vec<u32> = Vec::new();
    for (pos, c) in line.chars().enumerate() {
        if let Some(digit) = c.to_digit(10) {
            digits.push(digit);
        } else if let Some(digit) = digit_from_word(&line[pos..]) {
            digits.push(digit);
        }
    }
    return digits;
}

fn digit_from_word(slice: &str) -> Option<u32> {
    for (digit, word) in DIGITS.iter().enumerate() {
        if slice.starts_with(word) {
            return Some(digit as u32);
        }
    }
    return None;
}

const DIGITS: [&str; 10] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
];
