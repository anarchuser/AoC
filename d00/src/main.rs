use std::env;
use std::fs;
use std::fmt;
use std::str::FromStr;
use std::num::ParseIntError;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input: String = fs::read_to_string(&args[1])
        .expect("Input file missing");

    let strct: Vec<Str> = input
        .lines()
        .filter_map(|s| s.parse::<Str>().ok())
        .collect();

    println!("first = {}", first());
    println!("second = {}", second());
}

struct Str {
}

impl fmt::Display for Str {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "test")
    }
}

impl FromStr for Str {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Str {  })
    }
}

fn first() -> usize {
    0
}

fn second() -> usize {
    0
}