#![allow(dead_code)]

fn get_input(test_input: bool) -> &'static str {
    match test_input {
        true => include_str!("../../src/inputs/day_5_test.txt"),
        false => include_str!("../../src/inputs/day_5.txt")
    }
}

#[derive(Debug)]
struct Location {
    start_destination: u64,
    start_source: u64,
    length: u64,
}

#[derive(Debug)]
struct Map {
    from: String,
    to: String,
    value: Vec<Location>,
}

#[derive(Debug)]
struct Seeds {
    value: Vec<u64>,
    ranges: Vec<(u64, u64)>,
}

#[derive(Debug)]
struct Maps {
    seeds: Seeds,
    maps: Vec<Map>,
}

impl std::str::FromStr for Maps {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split("\n\r").collect::<Vec<&str>>();

        let seeds: Seeds = parts[0].parse::<Seeds>().unwrap();

        let mut maps: Vec<Map> = Vec::new();
        for part in parts[1..].iter() {
            let map = part.parse::<Map>().unwrap();
            maps.push(map);
        }

        Ok(Maps {
            seeds,
            maps,
        })
    }
}

use std::error::Error;

impl std::str::FromStr for Seeds {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.split(":")
            .nth(1)
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        let mut ranges: Vec<(u64, u64)> = Vec::new();
        for i in 0..value.len() {
            if i % 2 == 0 {
                ranges.push((value[i], value[i + 1]));
            }
        }

        Ok(Seeds {
            value,
            ranges,
        })
    }
}

impl std::str::FromStr for Map {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name = s.split(":")
            .nth(0)
            .unwrap()
            .split_whitespace()
            .nth(0)
            .unwrap()
            .split("-to-");

        let value_in_vec: Vec<Vec<u64>> = s.split(":")
            .nth(1)
            .unwrap()
            .trim()
            .split("\r\n")
            .map(|x| x.split_whitespace().map(|y| y.parse::<u64>().unwrap()).collect::<Vec<u64>>())
            .collect::<Vec<Vec<u64>>>();

        let value = value_in_vec.iter().map(|x| {
            Location {
                start_destination: x[0],
                start_source: x[1],
                length: x[2],
            }
        }).collect::<Vec<Location>>();

        let from = name.clone().nth(0).unwrap().to_string();
        let to = name.clone().nth(1).unwrap().to_string();

        Ok(Map {
            from,
            to,
            value,
        })
    }
}

fn transform_card(seed: u64, map: &Map) -> u64 {
    for location in map.value.iter() {
        let destination = location.start_destination;
        let source = location.start_source;
        let length = location.length;


        let source_range = source..(source + length);
        if source_range.contains(&seed) {
            let offset = seed - location.start_source;
            return destination + offset;
        }
    }

    seed
}

fn generate_seed(seeds: &Seeds) -> Vec<u64> {
    let mut seed_list: Vec<u64> = Vec::new();
    for range in seeds.ranges.iter() {
        let start = range.0;
        let length = range.1;

        for i in 0..length {
            seed_list.push(start + i);
        }
    }

    seed_list
}

pub fn day_5_part_1() {
    let maps = get_input(true).parse::<Maps>().unwrap();

    let mut lowest_seed: u64 = u64::MAX;

    for seed in maps.seeds.value {
        let mut transformed_value = Some(seed);

        for map in maps.maps.iter() {
            if let Some(value) = transformed_value {
                transformed_value = Some(transform_card(value, map));
            }
        }

        if transformed_value.unwrap() < lowest_seed {
            lowest_seed = transformed_value.unwrap();
        }
    }

    println!("Lowest seed: {}", lowest_seed);
}

pub fn day_5_part_2() { // Take a while to run (about 30 to 60 minutes)
    let maps = get_input(false).parse::<Maps>().unwrap();

    let mut lowest_seed: u64 = u64::MAX;
    let all_seeds = generate_seed(&maps.seeds);
    println!("Seed created: {}", all_seeds.len());

    let mut i = 0;
    let total = all_seeds.len();
    for seed in all_seeds {
        i += 1;
        let mut transformed_value = Some(seed);

        for map in maps.maps.iter() {
            if let Some(value) = transformed_value {
                transformed_value = Some(transform_card(value, map));
            }
        }

        if let Some(final_value) = transformed_value {
            if final_value < lowest_seed {
                lowest_seed = final_value;
            }
        }

        // print every 1000 seeds
        if i % 1000 == 0 {
            println!("Seed: {} / {}", i, total);
        }
    }

    println!("Lowest seed: {}", lowest_seed);
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4
    */

    #[test]
    fn test_seed() {
        let maps = get_input(true).parse::<Maps>().unwrap();

        assert_eq!(maps.seeds.value, vec![79, 14, 55, 13]);
    }

    #[test]
    fn test_seed_to_soil_map() {
        let maps = get_input(true).parse::<Maps>().unwrap();

        assert_eq!(maps.maps[0].from, "seed");
        assert_eq!(maps.maps[0].to, "soil");

        assert_eq!(maps.maps[0].value[0].start_destination, 50);
        assert_eq!(maps.maps[0].value[0].start_source, 98);
        assert_eq!(maps.maps[0].value[0].length, 2);

        assert_eq!(maps.maps[0].value[1].start_destination, 52);
        assert_eq!(maps.maps[0].value[1].start_source, 50);
        assert_eq!(maps.maps[0].value[1].length, 48);
    }

    #[test]
    fn test_soil_to_fertilizer_map() {
        let maps = get_input(true).parse::<Maps>().unwrap();

        assert_eq!(maps.maps[1].from, "soil");
        assert_eq!(maps.maps[1].to, "fertilizer");

        assert_eq!(maps.maps[1].value[0].start_destination, 0);
        assert_eq!(maps.maps[1].value[0].start_source, 15);
        assert_eq!(maps.maps[1].value[0].length, 37);

        assert_eq!(maps.maps[1].value[1].start_destination, 37);
        assert_eq!(maps.maps[1].value[1].start_source, 52);
        assert_eq!(maps.maps[1].value[1].length, 2);

        assert_eq!(maps.maps[1].value[2].start_destination, 39);
        assert_eq!(maps.maps[1].value[2].start_source, 0);
        assert_eq!(maps.maps[1].value[2].length, 15);
    }

    #[test]
    fn test_fertilizer_to_water_map() {
        let maps = get_input(true).parse::<Maps>().unwrap();

        assert_eq!(maps.maps[2].from, "fertilizer");
        assert_eq!(maps.maps[2].to, "water");

        assert_eq!(maps.maps[2].value[0].start_destination, 49);
        assert_eq!(maps.maps[2].value[0].start_source, 53);
        assert_eq!(maps.maps[2].value[0].length, 8);

        assert_eq!(maps.maps[2].value[1].start_destination, 0);
        assert_eq!(maps.maps[2].value[1].start_source, 11);
        assert_eq!(maps.maps[2].value[1].length, 42);

        assert_eq!(maps.maps[2].value[2].start_destination, 42);
        assert_eq!(maps.maps[2].value[2].start_source, 0);
        assert_eq!(maps.maps[2].value[2].length, 7);

        assert_eq!(maps.maps[2].value[3].start_destination, 57);
        assert_eq!(maps.maps[2].value[3].start_source, 7);
        assert_eq!(maps.maps[2].value[3].length, 4);
    }

    #[test]
    fn test_water_to_light_map() {
        let maps = get_input(true).parse::<Maps>().unwrap();

        assert_eq!(maps.maps[3].from, "water");
        assert_eq!(maps.maps[3].to, "light");

        assert_eq!(maps.maps[3].value[0].start_destination, 88);
        assert_eq!(maps.maps[3].value[0].start_source, 18);
        assert_eq!(maps.maps[3].value[0].length, 7);

        assert_eq!(maps.maps[3].value[1].start_destination, 18);
        assert_eq!(maps.maps[3].value[1].start_source, 25);
        assert_eq!(maps.maps[3].value[1].length, 70);
    }

    #[test]
    fn test_light_to_temperature_map() {
        let maps = get_input(true).parse::<Maps>().unwrap();

        assert_eq!(maps.maps[4].from, "light");
        assert_eq!(maps.maps[4].to, "temperature");

        assert_eq!(maps.maps[4].value[0].start_destination, 45);
        assert_eq!(maps.maps[4].value[0].start_source, 77);
        assert_eq!(maps.maps[4].value[0].length, 23);

        assert_eq!(maps.maps[4].value[1].start_destination, 81);
        assert_eq!(maps.maps[4].value[1].start_source, 45);
        assert_eq!(maps.maps[4].value[1].length, 19);

        assert_eq!(maps.maps[4].value[2].start_destination, 68);
        assert_eq!(maps.maps[4].value[2].start_source, 64);
        assert_eq!(maps.maps[4].value[2].length, 13);
    }

    #[test]
    fn test_temperature_to_humidity_map() {
        let maps = get_input(true).parse::<Maps>().unwrap();

        assert_eq!(maps.maps[5].from, "temperature");
        assert_eq!(maps.maps[5].to, "humidity");

        assert_eq!(maps.maps[5].value[0].start_destination, 0);
        assert_eq!(maps.maps[5].value[0].start_source, 69);
        assert_eq!(maps.maps[5].value[0].length, 1);

        assert_eq!(maps.maps[5].value[1].start_destination, 1);
        assert_eq!(maps.maps[5].value[1].start_source, 0);
        assert_eq!(maps.maps[5].value[1].length, 69);
    }

    #[test]
    fn test_humidity_to_location_map() {
        let maps = get_input(true).parse::<Maps>().unwrap();

        assert_eq!(maps.maps[6].from, "humidity");
        assert_eq!(maps.maps[6].to, "location");

        assert_eq!(maps.maps[6].value[0].start_destination, 60);
        assert_eq!(maps.maps[6].value[0].start_source, 56);
        assert_eq!(maps.maps[6].value[0].length, 37);

        assert_eq!(maps.maps[6].value[1].start_destination, 56);
        assert_eq!(maps.maps[6].value[1].start_source, 93);
        assert_eq!(maps.maps[6].value[1].length, 4);
    }
}