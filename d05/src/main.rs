use std::env;
use std::fs;
use std::fmt;
use std::str::FromStr;
use std::num::ParseIntError;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input: String = fs::read_to_string(&args[1])
        .expect("Input file missing");
    let almanac: Almanac = input
        .parse::<Almanac>()
        .expect("invalid input");

    println!("first = {}", first(&almanac));
    println!("second = {}", second(&almanac));

}

struct Almanac {
    seeds: Vec<usize>,
    seed_ranges: Vec<Range>,
    seed_to_soil: Map,
    soil_to_fertilizer: Map,
    fertilizer_to_water: Map,
    water_to_light: Map,
    light_to_temperature: Map,
    temperature_to_humidity: Map,
    humidity_to_location: Map,
}

struct Map {
    ranges: Vec<Range>,
}

struct Range {
    dst_start: usize,
    src_start: usize,
    length: usize,
}

impl fmt::Display for Almanac {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "seeds:")?;
        for seed in self.seeds.iter() {
            write!(f, " {seed}")?
        }
        write!(f, "\n\nseed-to-soil map:\n{}", self.seed_to_soil)?;
        write!(f, "\nsoil-to-fertilizer map:\n{}", self.soil_to_fertilizer)?;
        write!(f, "\nfertilizer-to-water map:\n{}", self.fertilizer_to_water)?;
        write!(f, "\nwater-to-light map:\n{}", self.water_to_light)?;
        write!(f, "\nlight-to-temperature map:\n{}", self.light_to_temperature)?;
        write!(f, "\ntemperature-to-humidity map:\n{}", self.temperature_to_humidity)?;
        write!(f, "\nhumidity-to-location map:\n{}", self.humidity_to_location)
    }
}

impl FromStr for Almanac {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let seeds: Vec<usize> = s
            .split_once(": ").expect("invalid seeds").1
            .split_once('\n').expect("invalid list of seeds").0
            .split_whitespace()
            .filter_map(|s| s.parse::<usize>().ok())
            .collect();
        let seed_ranges: Vec<Range> = seeds.chunks(2).into_iter()
            .map(|chunk| Range { dst_start: chunk[0], src_start: chunk[0], length: chunk[1] })
            .collect();
        let seed_to_soil: Map = s
            .split_once("seed-to-soil map:\n").expect("invalid seed-to-soil start").1
            .split_once("\n\nsoil-to-fertilizer").expect("invalid seed-to-soil end").0
            .parse::<Map>()
            .unwrap();
        let soil_to_fertilizer: Map = s
            .split_once("soil-to-fertilizer map:\n").expect("invalid soil-to-fertilizer start").1
            .split_once("\n\nfertilizer-to-water").expect("invalid soil-to-fertilizer end").0
            .parse::<Map>()
            .unwrap();
        let fertilizer_to_water: Map = s
            .split_once("fertilizer-to-water map:\n").expect("invalid fertilizer-to-water start").1
            .split_once("\n\nwater-to-light").expect("invalid fertilizer-to-water end").0
            .parse::<Map>()
            .unwrap();
        let water_to_light: Map = s
            .split_once("water-to-light map:\n").expect("invalid water-to-light start").1
            .split_once("\n\nlight-to-temperature").expect("invalid water-to-light end").0
            .parse::<Map>()
            .unwrap();
        let light_to_temperature: Map = s
            .split_once("light-to-temperature map:\n").expect("invalid light-to-temperature start").1
            .split_once("\n\ntemperature-to-humidity").expect("invalid light-to-temperature end").0
            .parse::<Map>()
            .unwrap();
        let temperature_to_humidity: Map = s
            .split_once("temperature-to-humidity map:\n").expect("invalid temperature-to-humidity start").1
            .split_once("\n\nhumidity-to-location").expect("invalid temperature-to-humidity end").0
            .parse::<Map>()
            .unwrap();
        let humidity_to_location: Map = s
            .split_once("humidity-to-location map:\n").expect("invalid humidity-to-location start").1
            .parse::<Map>()
            .unwrap();
        Ok(Almanac { seeds
            , seed_ranges
            , seed_to_soil
            , soil_to_fertilizer
            , fertilizer_to_water
            , water_to_light
            , light_to_temperature
            , temperature_to_humidity
            , humidity_to_location
        })
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Ok(for range in self.ranges.iter() {
            write!(f, "{range}")?
        })
    }
}

impl FromStr for Map {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Map { ranges:
            s.lines()
                .filter_map(| s| s.parse::<Range>().ok())
                .collect()
        })
    }
}

impl Map {
    fn map(&self, val: usize) -> usize {
        match self.ranges.iter()
            .filter_map(|range| range.map(&val))
            .next() {
            None => val,
            Some(mapped) => mapped,
        }
    }
    fn map_rev(&self, val: usize) -> usize {
        match self.ranges.iter()
            .filter_map(|range| range.map_rev(&val))
            .next() {
            None => val,
            Some(mapped) => mapped,
        }
    }
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{} {} {}", self.dst_start, self.src_start, self.length)
    }
}

impl FromStr for Range {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (raw_dst_start, tail) = s.split_once(" ").unwrap();
        let (raw_src_start, raw_length) = tail.split_once(" ").unwrap();
        let dst_start = raw_dst_start.parse().unwrap();
        let src_start = raw_src_start.parse().unwrap();
        let length = raw_length.parse().unwrap();
        Ok(Range { dst_start, src_start, length })
    }
}

impl Range {
    fn map(&self, val: &usize) -> Option<usize> {
        match (self.src_start..self.src_start + self.length).contains(val) {
            true => Some(self.dst_start + (val - self.src_start)),
            false => None,
        }
    }
    fn map_rev(&self, val: &usize) -> Option<usize> {
        match (self.dst_start..self.dst_start + self.length).contains(val) {
            true => Some(self.src_start + (val - self.dst_start)),
            false => None,
        }
    }
}

fn first(almanac: &Almanac) -> usize {
    almanac.seeds.iter()
        .map(|&seed| almanac.seed_to_soil.map(seed))
        .map(|soil| almanac.soil_to_fertilizer.map(soil))
        .map(|fertilizer| almanac.fertilizer_to_water.map(fertilizer))
        .map(|water| almanac.water_to_light.map(water))
        .map(|light| almanac.light_to_temperature.map(light))
        .map(|temperature| almanac.temperature_to_humidity.map(temperature))
        .map(|humidity| almanac.humidity_to_location.map(humidity))
        .fold(usize::MAX, usize::min)
}

fn second(almanac: &Almanac) -> usize {
    (0..)
        .into_iter()
        .map(|location| (location, almanac.humidity_to_location.map_rev(location)))
        .map(|(location, humidity)| (location, almanac.temperature_to_humidity.map_rev(humidity)))
        .map(|(location, temperature)| (location, almanac.light_to_temperature.map_rev(temperature)))
        .map(|(location, light)| (location, almanac.water_to_light.map_rev(light)))
        .map(|(location, water)| (location, almanac.fertilizer_to_water.map_rev(water)))
        .map(|(location, fertilizer)| (location, almanac.soil_to_fertilizer.map_rev(fertilizer)))
        .map(|(location, soil)| (location, almanac.seed_to_soil.map_rev(soil)))
        .find(|(_, seed)|
            almanac.seed_ranges.iter()
                .any(|range| range
                    .map_rev(seed)
                    .is_some()))
        .unwrap()
        .0
}