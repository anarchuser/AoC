use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input: String = fs::read_to_string(&args[1])
        .expect("Input file missing");

    let mut ids: (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|a| a.split_once("   ").unwrap())
        .map(|(a, b)| (
            a.parse::<i32>().unwrap(),
            b.parse::<i32>().unwrap()
        ))
        .collect::<(Vec<i32>, Vec<i32>)>()
        ;
    
    ids.0.sort();
    ids.1.sort();
    
    println!("first = {}", first(&ids));
    println!("second = {}", second(&ids));
}

fn first((a, b): &(Vec<i32>, Vec<i32>)) -> i32 {
    a
        .iter()
        .zip(b
            .iter())
        .map(|(a, b)| i32::abs(a - b))
        .sum()
}

fn second((a, b): &(Vec<i32>, Vec<i32>)) -> i32 {
    a
        .iter()
        .map(|x| *x * b
            .iter()
            .filter(|y| x == *y)
            .count()
            as i32
        )
        .sum::<i32>()
}