use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input: String = fs::read_to_string(&args[1])
        .expect("Input file missing");

    // read input into map
    let map: Vec<Vec<char>> = input
        .lines()
        .map(|row| row
            .chars()
            .collect())
        .collect();

    println!("first = {}", first(&map));
    println!("second = {}", second(&map));
}

struct Part {
    number: String,
    x: usize,
    y: usize,
}

fn first(map: &Vec<Vec<char>>) -> usize {
    let mut parts: Vec<Part> = Vec::new();

    let max_x: usize = map[0].len();
    let max_y: usize = map.len();

    // part 1: get list of all numbers with coordinate + length
    let mut cur_part: Part;
    let mut cur_on_part = false;

    for (y, row) in map.iter().enumerate() {
        cur_part = Part { number: "".to_string(), x: 0, y };

        for (x, c) in row.iter().enumerate() {
            let now_on_part: bool = c.is_digit(10);

            // found new part number
            if !cur_on_part && now_on_part {
                cur_on_part = true;
                cur_part.x = x;
            }

            // past the end of current part
            if cur_on_part && !now_on_part {
                cur_on_part = false;
                parts.push(cur_part);
                cur_part = Part { number: "".to_string(), x: 0, y };
            }

            // if we're on a char, add it to the current part
            if now_on_part {
                cur_part.number.push(c.clone());
            }
        }
        // we've reached eol while on a number
        if cur_on_part {
            parts.push(cur_part);
        }
        cur_on_part = false;
    }

    // part 2: filter and sum up all valid parts
    parts
        .iter()
        .filter(|part|
            get_border(&part, max_x, max_y)
                .iter()
                .any(|(x, y)| map[*y][*x] != '.' && !map[*y][*x].is_digit(10))
        )
        .map(|part| part.number.parse::<usize>().unwrap())
        .sum()
}

fn get_border(part: &Part, max_x: usize, max_y: usize) -> Vec<(usize, usize)> {
    let mut border: Vec<(usize, usize)> = Vec::new();

    let on_left_edge = part.x == 0;
    let on_right_edge = part.x + part.number.len() + 1 >= max_x;
    let on_top_edge = part.y == 0;
    let on_bottom_edge = part.y + 1 >= max_y;

    // add field left of part if possible
    if !on_left_edge {
        border.push((part.x - 1, part.y));
    }
    if !on_right_edge {
        border.push((part.x + part.number.len(), part.y));
    }
    
    // add upper and lower border if possible
    let x_start = if on_left_edge { 0 } else { part.x - 1 };
    let x_end = if on_right_edge { max_x } else { part.x + part.number.len() + 1 };

    if !on_top_edge {
        for x in x_start..x_end {
            border.push((x, part.y - 1));
        }
    }

    if !on_bottom_edge {
        for x in x_start..x_end {
            border.push((x, part.y + 1));
        }
    }

    return border;
}

fn second(map: &Vec<Vec<char>>) -> usize {
    let mut gears: Vec<(usize, usize)> = Vec::new();

    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '*' {
                gears.push((x, y));
            }
        }
    }

    gears
        .iter()
        .filter_map(|gear| try_get_gear_ratio(*gear, map))
        .sum()
}

fn try_get_gear_ratio(gear: (usize, usize), map: &Vec<Vec<char>>) -> Option<usize> {
    let mut numbers: Vec<usize> = Vec::new();
    let (gear_x, gear_y) = gear;
    let max_x: usize = map[0].len();
    let max_y: usize = map.len();

    // check left
    {
        let mut x: usize = gear_x;
        let mut acc: usize = 0;
        let mut fac: usize = 1;
        while x > 0 {
            x -= 1;
            match map[gear_y][x].to_digit(10) {
                Some(digit) => {
                    acc += digit as usize * fac;
                    fac *= 10;
                },
                None => break,
            }
        }
        if acc > 0 {
            numbers.push(acc);
        }
    }

    // check right
    {
        let mut x: usize = gear_x;
        let mut acc: String = "".to_string();
        while x < max_x - 1 {
            x += 1;
            let c = map[gear_y][x];
            if c.is_digit(10) {
                acc.push(c);
            } else {
                break;
            }
        }
        if !acc.is_empty() {
            numbers.push(acc.parse().unwrap());
        }
    }

    // check top
    if gear_y > 0{
        let y: usize = gear_y - 1;
        if map[y][gear_x].is_digit(10) {
            // digit directly above * => only one number above possible
            let mut x: usize = gear_x;
            while x < max_x && map[y][x].is_digit(10) {
                x += 1;
            }
            let mut acc: usize = 0;
            let mut fac: usize = 1;
            while x > 0 {
                x -= 1;
                match map[y][x].to_digit(10) {
                    Some(digit) => {
                        acc += digit as usize * fac;
                        fac *= 10;
                    },
                    None => break,
                }
            }
            if acc > 0 {
                numbers.push(acc);
            }
        } else {
            // no digit above => two numbers possible
            // check left
            {
                let mut x: usize = gear_x;
                let mut acc: usize = 0;
                let mut fac: usize = 1;
                while x > 0 {
                    x -= 1;
                    match map[y][x].to_digit(10) {
                        Some(digit) => {
                            acc += digit as usize * fac;
                            fac *= 10;
                        },
                        None => break,
                    }
                }
                if acc > 0 {
                    numbers.push(acc);
                }
            }

            // check right
            {
                let mut x: usize = gear_x;
                let mut acc: String = "".to_string();
                while x < max_x - 1 {
                    x += 1;
                    let c = map[y][x];
                    if c.is_digit(10) {
                        acc.push(c);
                    } else {
                        break;
                    }
                }
                if !acc.is_empty() {
                    numbers.push(acc.parse().unwrap());
                }
            }
        }
    }

    // check bottom
    if gear_y < max_y - 1 {
        let y: usize = gear_y + 1;
        if map[y][gear_x].is_digit(10) {
            // digit directly above * => only one number above possible
            let mut x: usize = gear_x;
            while x < max_x && map[y][x].is_digit(10) {
                x += 1;
            }
            let mut acc: usize = 0;
            let mut fac: usize = 1;
            while x > 0 {
                x -= 1;
                match map[y][x].to_digit(10) {
                    Some(digit) => {
                        acc += digit as usize * fac;
                        fac *= 10;
                    },
                    None => break,
                }
            }
            if acc > 0 {
                numbers.push(acc);
            }
        } else {
            // no digit above => two numbers possible
            // check left
            {
                let mut x: usize = gear_x;
                let mut acc: usize = 0;
                let mut fac: usize = 1;
                while x > 0 {
                    x -= 1;
                    match map[y][x].to_digit(10) {
                        Some(digit) => {
                            acc += digit as usize * fac;
                            fac *= 10;
                        },
                        None => break,
                    }
                }
                if acc > 0 {
                    numbers.push(acc);
                }
            }

            // check right
            {
                let mut x: usize = gear_x;
                let mut acc: String = "".to_string();
                while x < max_x - 1 {
                    x += 1;
                    let c = map[y][x];
                    if c.is_digit(10) {
                        acc.push(c);
                    } else {
                        break;
                    }
                }
                if !acc.is_empty() {
                    numbers.push(acc.parse().unwrap());
                }
            }
        }
    }

    // print!("row {gear_y} / column {gear_x}:\t");
    // for number in numbers.iter() {
    //     print!("{number} ");
    // }
    // println!();
    
    if numbers.len() == 2 {
        Some(numbers
            .iter()
            .fold(1, |a, b| a * b))
    } else {
        None
    }
}
