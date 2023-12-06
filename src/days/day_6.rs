#![allow(dead_code)]

fn get_input(test_input: bool) -> &'static str {
    match test_input {
        true => include_str!("../../src/inputs/day_6_test.txt"),
        false => include_str!("../../src/inputs/day_6.txt")
    }
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

fn parse_line(input: &str) -> Vec<u64> {
    input
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

fn vec_to_num(vec: Vec<u64>) -> u64 {
    let mut num = String::new();
    for digit in vec {
        num.push_str(&digit.to_string());
    }

    num.parse::<u64>().unwrap()
}

fn parse_races(input: &str) -> Vec<Race> {
    let times = parse_line(input
        .lines()
        .nth(0)
        .unwrap());

    let distances = parse_line(input
        .lines()
        .nth(1)
        .unwrap());

    // verif if times and distances have the same length, else return an error
    if times.len() != distances.len() {
        panic!("Error: times and distances have not the same length");
    }

    let mut races: Vec<Race> = Vec::new();
    let mut index = 0;
    for _time in times.clone().into_iter() {
        races.push(Race {
            time: times[index],
            distance: distances[index],
        });
        index += 1;
    }

    races
}

fn parse_races_2(input: &str) -> Race {
    let times = parse_line(input
        .lines()
        .nth(0)
        .unwrap());

    let distances = parse_line(input
        .lines()
        .nth(1)
        .unwrap());

    println!("{:?}", times);
    println!("{:?}", distances);

    Race {
        time: vec_to_num(times),
        distance: vec_to_num(distances),
    }
}

fn get_nb_way_to_win(races: Vec<Race>) -> u64 {
    let mut total_ways_to_win = 1;

    for race in races {
        total_ways_to_win *=  get_nb_ways_for_race(race);
    }

    total_ways_to_win
}

fn get_nb_ways_for_race(race: Race) -> u64 {
    let race_time = race.time;
    let race_distance = race.distance;
    let mut num_ways_to_win = 0;
    for press_time in 0..race_time {
        let move_time = race_time - press_time;
        let speed = press_time.clone();
        let moved_distance = speed * move_time;
        if moved_distance > race_distance {
            num_ways_to_win += 1;
        }
    }

    num_ways_to_win
}


pub fn day_6_part_1() {
    let races = parse_races(get_input(false));
    let nb_way_to_win = get_nb_way_to_win(races);
    println!("Number of ways to win: {}", nb_way_to_win);
}

pub fn day_6_part_2() {
    let race = parse_races_2(get_input(false));
    let nb_way_to_win = get_nb_ways_for_race(race);
    println!("Number of ways to win: {}", nb_way_to_win);
}

#[cfg(test)]
mod tests {
    /*
        Time:      7  15   30
        Distance:  9  40  200
    */

    use crate::days::day_6::{get_input, parse_races};

    #[test]
    fn test_parse() {
        let races = parse_races(get_input(true));

        assert_eq!(races[0].time, 7);
        assert_eq!(races[0].distance, 9);

        assert_eq!(races[1].time, 15);
        assert_eq!(races[1].distance, 40);

        assert_eq!(races[2].time, 30);
        assert_eq!(races[2].distance, 200);
    }
}