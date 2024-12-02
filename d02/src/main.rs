use std::env;
use std::fs;
use std::fmt;
use std::str::FromStr;
use std::num::ParseIntError;
use std::cmp;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input: String = fs::read_to_string(&args[1])
        .expect("Input file missing");

    println!("first = {}", first(&input));
    println!("second = {}", second(&input));
}

struct Triplet {
    red: u32,
    green: u32,
    blue: u32,
}

impl fmt::Display for Triplet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} red, {} green, {} blue; ", self.red, self.green, self.blue)
    }
}

impl FromStr for Triplet {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red: u32 = 0;
        let mut green: u32 = 0;
        let mut blue: u32 = 0;

        for pair in s.split(", ") {
            if let Some((val, cube)) = pair.split_once(" ") {
                match cube {
                    "red" => red = val.parse()?,
                    "green" => green = val.parse()?,
                    "blue" => blue = val.parse()?,
                    &_ => {},
                }
            }
        }
        Ok(Triplet { red, green, blue })
    }
}

fn first(input: &String) -> u32 {
    let max = Triplet { red: 12, green: 13, blue: 14, };

    input
        .lines()
        .map(line_to_game)
        .map(|(index, game)|
            (index, game
                .split("; ")
                .collect::<Vec<_>>()
            ))
        .map(|(index, grabs)|
            (index, grabs
                .iter()
                .filter_map(|grab| grab.parse::<Triplet>().ok())
                .collect::<Vec<_>>()
            ))
        .filter_map(|(index, triplets)|
            match triplets
                .iter()
                .all(|triplet|
                    triplet.red <= max.red &&
                    triplet.green <= max.green &&
                    triplet.blue <= max.blue
                ) {
                true => Some(index),
                false => None,
            }
        )
        .sum()
}

fn line_to_game(line: &str) -> (u32, &str) {
    let game_index_start = line
        .find(|c: char| c.is_digit(10))
        .expect("no digit in line -> input invalid");
    let game_index_length = &line[game_index_start..]
        .find(|c: char| !c.is_digit(10))
        .expect("no digit in line -> input invalid");

    let mut factor: u32 = 1;
    let mut game_index: u32 = 0;
    for digit in &line[game_index_start..game_index_start + game_index_length].chars().rev() {
        game_index += digit.to_digit(10).unwrap() * factor;
        factor *= 10;
    }

    return (game_index, &line[game_index_start + game_index_length + 2..]);
}

fn second(input: &String) -> u32 {
    input
        .lines()
        .map(line_to_game)
        .map(|(_, game)|
            game
                .split("; ")
                .collect::<Vec<_>>())
        .map(|grabs|
            grabs
                .iter()
                .filter_map(|grab| grab.parse::<Triplet>().ok())
                .collect::<Vec<_>>())
        .map(|triplets|
            triplets
                .iter()
                .fold(Triplet { red: 0, green: 0, blue: 0}, |a, b|
                    Triplet {
                        red: cmp::max(a.red, b.red),
                        green: cmp::max(a.green, b.green),
                        blue: cmp::max(a.blue, b.blue),
                    }))
        .map(|triplet|
             triplet.red * triplet.green * triplet.blue
        )
        .sum()
}
