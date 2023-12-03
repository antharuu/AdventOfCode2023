const INPUT_PATH: &str = "src/inputs/day_2.txt";
// const INPUT_PATH: &str = "src/inputs/day_2_test.txt";

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

enum Color {
    Red,
    Blue,
    Green,
}

impl Color {
    fn as_str(&self) -> &str {
        match self {
            Color::Red => "red",
            Color::Blue => "blue",
            Color::Green => "green",
        }
    }
}

pub fn day_2_part_1() {
    let input = std::fs::read_to_string(INPUT_PATH).unwrap();
    let games: Vec<&str> = input.split("\n").collect();
    let mut total_possible_runs: u32 = 0;

    for (i, game) in games.iter().enumerate() {
        if play_game(game) {
            total_possible_runs += (i + 1) as u32;
        }
    }

    println!("Total possible runs: {}", total_possible_runs);
}

pub fn day_2_part_2() {
    let input = std::fs::read_to_string(INPUT_PATH).unwrap();
    let games: Vec<&str> = input.split("\n").collect();

    let mut total_power_of_cubes: u32 = 0;

    for game in games {
        let (red, green, blue) = get_fewest_cubes(game);
        let power_of_cubes = red * green * blue;

        total_power_of_cubes += power_of_cubes;
    }

    println!("Total power of cubes: {}", total_power_of_cubes);
}

fn play_game(game: &str) -> bool {
    // take the part after the ":"
    let runs_string: &str = game.split(": ").collect::<Vec<&str>>()[1];
    let runs: Vec<&str> = runs_string.split("; ").collect();

    let mut possible_runs = Vec::new();

    for (i, run) in runs.iter().enumerate() {
        let run_string: &str = run.trim();
        let (red, green, blue) = play_run(run_string);
        if is_possible(red, green, blue) {
            possible_runs.push(i);
        } else {
            return false;
        }
    }

    true
}

fn play_run(run: &str) -> (u32, u32, u32) {
    let colors: Vec<&str> = run.split(", ").collect();

    (
        get_cubes(&colors, Color::Red),
        get_cubes(&colors, Color::Green),
        get_cubes(&colors, Color::Blue)
    )
}

fn get_cubes(colors: &Vec<&str>, color: Color) -> u32 {
    for c in colors {
        if c.contains(color.as_str()) {
            let cube: u32 = c.split(" ").collect::<Vec<&str>>()[0].parse().unwrap();
            return cube;
        }
    }

    0
}

fn is_possible(red: u32, green: u32, blue: u32) -> bool {
    if red > MAX_RED || green > MAX_GREEN || blue > MAX_BLUE {
        return false;
    }

    if red == 0 && green == 0 && blue == 0 {
        return false;
    }

    true
}

fn get_fewest_cubes(game: &str) -> (u32, u32, u32) {
    let runs_string: &str = game.split(": ").collect::<Vec<&str>>()[1];
    let runs: Vec<&str> = runs_string.split("; ").collect();

    let (mut min_red, mut min_green, mut min_blue) = (0, 0, 0);
    for run in runs {
        let (red, green, blue) = play_run(run);

        if red > min_red {
            min_red = red;
        }

        if green > min_green {
            min_green = green;
        }

        if blue > min_blue {
            min_blue = blue;
        }
    }

    (min_red, min_green, min_blue)
}