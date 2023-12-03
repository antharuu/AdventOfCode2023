#![allow(dead_code)]

const INPUT_PATH: &str = "src/inputs/day_3.txt";
// const INPUT_PATH: &str = "src/inputs/day_3_test.txt";


#[derive(Clone)]
#[derive(PartialEq)]
struct Vector {
    x: i32,
    y: i32,
}

#[derive(Clone)]
struct Number {
    value: i32,
    position: Vector,
}

struct Symbol {
    position: Vector,
}

struct Gear {
    position: Vector,
    numbers: Vec<i32>,
}

enum Tile {
    NumberTile(Number),
    SymbolTile(Symbol),
    GearTile(Gear),
}

struct Map {
    tiles: Vec<Tile>,
}

impl Map {
    pub fn get_tile_at_position(&self, position: Vector) -> Option<&Tile> { // Added pub to make function visible. The function return a reference instead of owned Tile
        for tile in &self.tiles { // &self.tiles because you only need to borrow self.tiles not own it
            match tile {
                Tile::NumberTile(number) => {
                    if number.position == position {
                        return Some(&tile); // &tile because you're only borrowing tile
                    }
                }
                Tile::SymbolTile(symbol) => {
                    if symbol.position == position {
                        return Some(&tile); // &tile because you're only borrowing tile
                    }
                }
                Tile::GearTile(gear) => {
                    if gear.position == position {
                        return Some(&tile); // &tile because you're only borrowing tile
                    }
                }
            }
        }
        None
    }

    fn get_adjacent_positions(position: Vector) -> Vec<Vector> {
        let mut adjacent_positions: Vec<Vector> = Vec::new();

        adjacent_positions.push(Vector { x: position.x - 1, y: position.y });       // LEFT
        adjacent_positions.push(Vector { x: position.x + 1, y: position.y });       // RIGHT
        adjacent_positions.push(Vector { x: position.x, y: position.y - 1 });       // UP
        adjacent_positions.push(Vector { x: position.x, y: position.y + 1 });       // DOWN
        adjacent_positions.push(Vector { x: position.x - 1, y: position.y - 1 });   // UP LEFT
        adjacent_positions.push(Vector { x: position.x + 1, y: position.y - 1 });   // UP RIGHT
        adjacent_positions.push(Vector { x: position.x - 1, y: position.y + 1 });   // DOWN LEFT
        adjacent_positions.push(Vector { x: position.x + 1, y: position.y + 1 });   // DOWN RIGHT

        adjacent_positions
    }

    fn is_adjacent_to_symbol(&self, position: Vector) -> bool {
        let adjacent_positions = Map::get_adjacent_positions(position);

        for adjacent_position in adjacent_positions {
            if let Some(tile) = self.get_tile_at_position(adjacent_position) {
                match tile {
                    Tile::SymbolTile(_) => {
                        return true;
                    }
                    Tile::GearTile(_) => {
                        return true;
                    }
                    _ => {}
                }
            }
        }

        false
    }

    fn is_adjacent_to_too_number(&self, position: Vector) -> (bool, Vec<i32>) {
        let adjacent_positions = Map::get_adjacent_positions(position);
        let mut adjacent_numbers = Vec::new();

        for adjacent_position in adjacent_positions {
            if let Some(tile) = self.get_tile_at_position(adjacent_position) {
                match tile {
                    Tile::NumberTile(number) => {
                        let full_number = get_full_number(number.position.clone(), self.clone());
                        if !adjacent_numbers.contains(&full_number) {
                            adjacent_numbers.push(full_number);
                        }
                    }
                    _ => {}
                }
            }
        }

        (adjacent_numbers.len() > 1, adjacent_numbers)
    }
}

impl Number {
    fn is_adjacent_to_symbol(&self, map: &Map) -> bool {
        map.is_adjacent_to_symbol(self.position.clone())
    }
}

impl Gear {
    fn is_adjacent_to_too_number(&self, map: &Map) -> (bool, Vec<i32>) {
        map.is_adjacent_to_too_number(self.position.clone())
    }
}

fn add_to_map(map: &mut Map, character: char, x: usize, y: usize) {
    let position = Vector { x: x as i32, y: y as i32 };
    if character.is_numeric() {
        let number = Number { value: character.to_digit(10).unwrap() as i32, position };
        map.tiles.push(Tile::NumberTile(number));
    } else if character == '*' {
        let gear = Gear { position, numbers: [].to_vec() };
        map.tiles.push(Tile::GearTile(gear));
    } else if character != '.' {
        let symbol = Symbol { position };
        map.tiles.push(Tile::SymbolTile(symbol));
    }
}

fn vec_to_num(vect: Vec<i32>) -> i32 {
    let mut num = 0;
    for i in vect {
        num = num * 10 + i;
    }
    num
}

fn get_full_number(position: Vector, map: &Map) -> i32 {
    let first_position = get_first_digit_position(position.clone(), &map);
    let parts_of_number: Vec<i32> = get_number_parts(first_position.clone(), &map);
    vec_to_num(parts_of_number)
}

fn get_first_digit_position(position: Vector, map: &Map) -> Vector {
    // while you can go and it's a number, go left
    let mut current_position = position.clone();

    let mut current_tile = match map.get_tile_at_position(current_position.clone()) {
        Some(tile) => tile,
        None => return Vector {
            x: current_position.x + 1,
            y: current_position.y,
        }, // return early if tile is None
    };

    while let Tile::NumberTile(_) = current_tile {
        current_position.x -= 1;

        if let Some(tile) = map.get_tile_at_position(current_position.clone()) {
            current_tile = tile;
        } else {
            break; // break the loop if tile is None
        }
    }

    Vector {
        x: current_position.x + 1,
        y: current_position.y,
    }
}

fn get_number_parts(position: Vector, map: &Map) -> Vec<i32> {
    let mut current_position = position.clone();
    let mut current_tile = match map.get_tile_at_position(current_position.clone()) {
        Some(tile) => tile,
        None => return [].to_vec(), // return early if tile is None
    };

    let mut parts_of_number: Vec<i32> = Vec::new();

    while let Tile::NumberTile(number) = current_tile {
        parts_of_number.push(number.value);

        current_position.x += 1;

        if let Some(tile) = map.get_tile_at_position(current_position.clone()) {
            current_tile = tile;
        } else {
            break; // break the loop if tile is None
        }
    }

    parts_of_number
}

pub fn day_3_part_1() {
    let input = std::fs::read_to_string(INPUT_PATH).unwrap();

    let lines = input.lines().enumerate();
    let mut map = Map { tiles: Vec::new() };
    let mut number_groups: Vec<Vec<Number>> = Vec::new();
    for (y, line) in lines {
        let chars = line.chars().enumerate();
        let mut num_group = Vec::new();
        for (x, character) in chars {
            add_to_map(&mut map, character, x, y);
            if character.is_numeric() {
                let number = Number {
                    value: character.to_digit(10).unwrap() as i32,
                    position: Vector { x: x as i32, y: y as i32 },
                };
                num_group.push(number);
            } else {
                if num_group.len() >= 1 {
                    number_groups.push(num_group.clone());
                    num_group.clear();
                }
            }
        }

        if num_group.len() >= 1 {
            number_groups.push(num_group.clone());
            num_group.clear();
        }
    }

    let adjacent_numbers_groups: Vec<Vec<Number>> = number_groups.iter().filter(|group| {
        let mut adjacent_numbers = Vec::new();
        for number in group.iter() {
            if number.is_adjacent_to_symbol(&map) {
                adjacent_numbers.push(number.clone());
            }
        }
        adjacent_numbers.len() > 0
    }).cloned().collect();

    let mut sum_of_adjacent_numbers: i32 = 0;

    for group in adjacent_numbers_groups {
        let mut group_value = 0;
        for number in group {
            group_value = group_value * 10 + number.value;
        }

        sum_of_adjacent_numbers += group_value;
    }

    println!("Sum of adjacent numbers: {}", sum_of_adjacent_numbers);
}

pub fn day_3_part_2() {
    let input = std::fs::read_to_string(INPUT_PATH).unwrap();

    let lines = input.lines().enumerate();
    let mut map = Map { tiles: Vec::new() };
    let mut gears: Vec<Gear> = Vec::new();
    for (y, line) in lines {
        let chars = line.chars().enumerate();
        let mut num_group = Vec::new();
        for (x, character) in chars {
            add_to_map(&mut map, character, x, y);
            if character.is_numeric() {
                num_group.push(character.to_digit(10).unwrap() as i32);
            } else {
                if character == '*' {
                    let gear = Gear {
                        position: Vector { x: x as i32, y: y as i32 },
                        numbers: [].to_vec(),
                    };
                    gears.push(gear);
                }
                end_group(&mut num_group);
            }
        }

        end_group(&mut num_group);
    }

    let mut ratios: Vec<i32> = Vec::new();

    for gear in &mut gears {
        let (is_adjacent, numbers) = gear.is_adjacent_to_too_number(&map);
        if is_adjacent {
            let mult = numbers.iter().fold(1, |acc, x| acc * x);
            ratios.push(mult);
        }
    }

    let mut sum_of_ratios = 0;
    for ratio in ratios {
        sum_of_ratios += ratio;
    }

    println!("Sum of ratios: {}", sum_of_ratios);
}

fn end_group(mut num_group: &mut Vec<i32>) {
    if num_group.len() >= 1 {
        num_group.clear();
    }
}