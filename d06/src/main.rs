fn main() {
    {
        let races: Vec<Race> = vec![
            Race { time: 61, distance: 430 },
            Race { time: 67, distance: 1036 },
            Race { time: 75, distance: 1307 },
            Race { time: 71, distance: 1150 },
        ];
        println!("first = {}", calc(&races));
    }
    {
        let races: Vec<Race> = vec![
            Race { time: 61677571, distance: 430103613071150 },
        ];
        println!("second = {}", calc(&races));
    }
}

struct Race {
    time: usize,
    distance: usize,
}

fn distance_per_accel(time_accel: usize, time_total: usize) -> usize {
    time_accel * (time_total - time_accel)
}

fn calc(races: &Vec<Race>) -> usize {
    races.iter()
        .map(|race| (0..race.time)
            .map(|accel| distance_per_accel(accel, race.time))
            .filter(|&distance| distance > race.distance)
            .count()
        )
        .product()
}