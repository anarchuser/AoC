use std::env;
use std::fs;
use std::fmt;
use std::str::FromStr;
use std::num::ParseIntError;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input: String = fs::read_to_string(&args[1])
        .expect("Input file missing");

    let cards: Vec<Card> = input
        .lines()
        .filter_map(|s| s.parse::<Card>().ok())
        .collect();

    println!("first = {}", first(&cards));
    println!("second = {}", second(&cards));
}

struct Card {
    id: usize,
    winning_nums: Vec<usize>,
    your_nums: Vec<usize>,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Card {}: ", self.id)?;
        for winning_num in self.winning_nums.iter() {
            write!(f, "{winning_num} ")?;
        }
        write!(f, "|")?;
        for your_num in self.your_nums.iter() {
            write!(f, " {your_num}")?;
        }
        write!(f, "")
    }
}

impl FromStr for Card {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, tail) = extract_card_id(s);
        let (raw_winning_nums, raw_your_nums) = tail
            .split_once(" | ")
            .unwrap();
        let winning_nums: Vec<usize> = raw_winning_nums
            .split(" ")
            .filter_map(|s| s.parse().ok())
            .collect();
        let your_nums: Vec<usize> = raw_your_nums
            .split(" ")
            .filter_map(|s| s.parse().ok())
            .collect();
        Ok(Card { id, winning_nums, your_nums })
    }
}

fn extract_card_id(line: &str) -> (usize, &str) {
    let card_index_start = line
        .find(|c: char| c.is_digit(10))
        .expect("input invalid");
    let card_index_length = line[card_index_start..]
        .find(|c: char| !c.is_digit(10))
        .expect("input invalid");

    let card_id: usize = line[card_index_start..card_index_start + card_index_length].parse()
        .expect("input invalid");

    return (card_id, &line[card_index_start + card_index_length + 2..]);
}

fn first(cards: &Vec<Card>) -> usize {
    cards.iter()
        .map(count_winning_nums)
        .filter_map(|wins| match wins {
                0 => None,
                _ => Some(usize::pow(2, (wins - 1) as u32))
            })
        .sum()
}

fn count_winning_nums(card: &Card) -> usize {
    card.your_nums.iter()
        .filter(|your_num| card.winning_nums.contains(your_num))
        .count() as usize
}

fn second(cards: &Vec<Card>) -> usize {
    cards.iter()
        .map(|card| recursively_count_cards(cards, card.id))
        .sum::<usize>()
}

fn recursively_count_cards(cards: &Vec<Card>, card_id: usize) -> usize {
    1 + match count_winning_nums(&cards[card_id - 1]) {
        0 => 0,
        matches => (1..matches + 1)
            .map(|offset: usize| recursively_count_cards(cards, card_id + offset))
            .sum::<usize>()
    }
}