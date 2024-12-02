use std::env;
use std::fs;
use std::fmt;
use std::str::FromStr;
use std::num::ParseIntError;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input: String = fs::read_to_string(&args[1])
        .expect("Input file missing");

    let map: Map = input.parse::<Map>().expect("invalid input");

    println!("{map}");

    println!("first = {}", first(&map));
    println!("second = {}", second(&map));
}

struct Map {
    instructions: String,
    map: Vec<(usize, usize)>,
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.instructions)?;
        for (idx, entry) in self.map.iter().enumerate() {
            if entry.0 != 0 || entry.1 != 0 {
                writeln!(f, "{} = ({}, {})", idx_to_node(idx), idx_to_node(entry.0), idx_to_node(entry.1))?;
                writeln!(f, "{} = ({}, {})", idx, entry.0, entry.1)?;
            }
        }
        writeln!(f, "")
    }
}

impl FromStr for Map {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (instructions, raw_map) = s.split_once("\n\n").expect("invalid input");
        let mut map: Vec<(usize, usize)> = vec![(0, 0); node_to_idx("ZZZ") + 1];
        raw_map
            .lines()
            .map(|line| line.split_once(" = (").unwrap())
            .map(|(a, tail)| (node_to_idx(a), tail.split_once(", ").unwrap()))
            .map(|(a, (b, tail))| (a, (b, tail.split_once(")").unwrap().0)))
            .for_each(|(idx, (left, right))| map[idx] = (node_to_idx(left), node_to_idx(right)));

        Ok(Map { instructions: instructions.to_string(), map })
    }
}

impl Map {
    fn steps_from_to(&self, start: usize, end: usize) -> usize {
        let mut trav = start;
        let mut count = 0;
        loop {
            for instruction in self.instructions.chars() {
                match instruction {
                    'L' => trav = self.map[trav].0,
                    'R' => trav = self.map[trav].1,
                    _ => { panic!("invalid direction") },
                }
                count += 1;
                if trav == end {
                    return count;
                }
            }
        }
    }

    fn steps_from_till_xxZ(&self, start: usize) -> usize {
        let mut trav = start;
        let mut count = 0;
        loop {
            for instruction in self.instructions.chars() {
                match instruction {
                    'L' => trav = self.map[trav].0,
                    'R' => trav = self.map[trav].1,
                    _ => {},
                }
                count += 1;
                if usize_to_char(trav % 27) == 'Z' {
                    println!("{} -> {} = {count} steps", idx_to_node(start), idx_to_node(trav));
                    return count;
                }
            }
        }
    }
}

fn node_to_idx(node: &str) -> usize {
    let mut acc = 0;
    let mut fac = 1;
    for c in node.chars().rev() {
        acc += char_to_usize(c) * fac;
        fac *= char_to_usize('Z') + 1;
    }
    acc
}
fn char_to_usize(c: char) -> usize {
    (c as u8 - b'A' + 1) as usize
}

fn idx_to_node(idx: usize) -> String {
    let mut acc: String = String::default();
    let mut dec = idx;
    while dec > 0 {
        acc.push(usize_to_char(dec % 27));
        dec /= 27;
    }
    acc.chars().rev().collect()
}
fn usize_to_char(num: usize) -> char {
    (b'A' + num as u8 - 1) as char
}

fn gcd(mut a:usize, mut b:usize) -> usize{
    if a == b {
        return a;
    }
    if b > a {
        return gcd(b, a)
    }
    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    a
}

fn lcm(a:usize, b:usize) -> usize{
    a * b / gcd(a,b)
}

fn first(map: &Map) -> usize {
    map.steps_from_to(node_to_idx("AAA"), node_to_idx("ZZZ"))
}

fn second(map: &Map) -> usize {
    map.map.iter()
        .enumerate()
        .filter_map(|(idx, node)| if node.0 > 0 && node.1 > 0 { Some(idx) } else { None})
        .filter(|a| a % 27 == 1)
        .map(|node| map.steps_from_till_xxZ(node))
        .fold(1, |acc, steps| lcm(acc, steps))
}