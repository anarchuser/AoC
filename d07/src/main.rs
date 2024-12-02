use std::env;
use std::fs;
use std::fmt;
use std::str::FromStr;
use std::num::ParseIntError;
use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use crate::Type::{FullHouse, HighCard, OnePair, Quadruple, Quintuple, Triple, TwoPairs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input: String = fs::read_to_string(&args[1])
        .expect("Input file missing");

    let mut players: Vec<Player> = input
        .lines()
        .filter_map(|s| s.parse::<Player>().ok())
        .collect();

    println!("first = {}", first(&mut players));
    println!("second = {}", second(&mut players));
}

struct Player {
    hand: String,
    bid: usize,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "hand: {}, bid: {}", self.hand, self.bid)
    }
}

impl FromStr for Player {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (raw_hand, raw_bid) = s.split_once(" ")
            .expect("invalid player");
        let hand = raw_hand.to_string();
        let bid = raw_bid.parse()
            .expect("invalid bid");
        Ok(Player { hand, bid })
    }
}

fn value_of(c: char) -> usize {
    match c {
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        '9' => 8,
        'T' => 9,
        'J' => 10,
        'Q' => 11,
        'K' => 12,
        'A' => 13,
        _ => 0,
    }
}

#[derive(Debug,Copy,Clone,PartialEq,Eq,Ord,PartialOrd)]
enum Type {
    HighCard,
    OnePair,
    TwoPairs,
    Triple,
    FullHouse,
    Quadruple,
    Quintuple,
}

fn cmp_hands(a: &str, b: &str) -> Ordering {
    match cmp_types(get_type(a), get_type(b)) {
        Ordering::Equal => cmp_labels(a, b),
        other => other,
    }
}

fn cmp_labels(a: &str, b: &str) -> Ordering {
    a.chars().map(value_of).collect::<Vec<_>>()
        .cmp(&b.chars().map(value_of).collect::<Vec<_>>())
}

fn cmp_types(a: Type, b: Type) -> Ordering {
    if a < b {
        Less
    } else if a > b {
        Greater
    } else {
        Equal
    }
}

fn get_type(hand: &str) -> Type {
    let mut map = vec![0usize; 14];
    hand.chars()
        .map(value_of)
        .for_each(|label| map[label] += 1);

    if map.contains(&5) {
        Quintuple
    } else if map.contains(&4) {
        Quadruple
    } else if map.contains(&3) && map.contains(&2) {
        FullHouse
    } else if map.contains(&3) {
        Triple
    } else if contains_n_times_n(2, 2, &map) {
        TwoPairs
    } else if map.contains(&2) {
        OnePair
    } else {
        HighCard
    }
}

fn contains_n_times_n(count: usize, times: usize, map: &Vec<usize>) -> bool {
    map.iter().filter(|&val| *val == count).count() == times
}

fn first(players: &mut Vec<Player>) -> usize {
    players
        .sort_by(|a, b|
            cmp_hands(&a.hand, &b.hand));
    players
        .iter()
        .enumerate()
        .map(|(rank, player)| (rank + 1) * player.bid)
        .sum()
}

fn second_value_of(c: char) -> usize {
    match c {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 1,
        'Q' => 11,
        'K' => 12,
        'A' => 13,
        _ => 0,
    }
}

fn second_cmp_hands(a: &str, b: &str) -> Ordering {
    match cmp_types(second_get_type(a), second_get_type(b)) {
        Equal => second_cmp_labels(a, b),
        other => other,
    }
}

fn second_cmp_labels(a: &str, b: &str) -> Ordering {
    a.chars().map(second_value_of).collect::<Vec<_>>()
        .cmp(&b.chars().map(second_value_of).collect::<Vec<_>>())
}

fn second_get_type(hand: &str) -> Type {
    let mut map = vec![0usize; 14];
    hand.chars()
        .map(second_value_of)
        .for_each(|label| map[label] += 1);
    let mut jokers = map[second_value_of('J')];
    // print!("{}: J = {} -> ", hand, jokers);

    if map.contains(&(5 - jokers)) {
        Quintuple
    } else if map.contains(&4)
        || jokers == 3
        || jokers == 2 && contains_n_times_n(2, 2, &map)
        || jokers == 1 && map.contains(&3)
    {
        Quadruple
    } else if map.contains(&3) && map.contains(&2)
        || jokers == 2 && contains_n_times_n(2, 2, &map)
        || jokers == 1 && (contains_n_times_n(2, 2, &map) || contains_n_times_n(3, 1, &map))
    {
        FullHouse
    } else if map.contains(&(3 - jokers)) {
        Triple
    } else if contains_n_times_n(2, 2, &map)
        || jokers == 2
        || jokers == 1 && (map.contains(&2))
    {
        TwoPairs
    } else if map.contains(&(2 - jokers)) {
        OnePair
    } else {
        HighCard
    }
}

fn second(players: &mut Vec<Player>) -> usize {
    players
        .sort_by(|a, b|
            second_cmp_hands(&a.hand, &b.hand));

    players
        .iter()
        .enumerate()
        .map(|(rank, player)| (rank + 1) * player.bid)
        .sum()
}