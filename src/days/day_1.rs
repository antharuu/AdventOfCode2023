use std::collections::HashMap;

const INPUT_PATH: &str = "src/inputs/day_1.txt";
// const INPUT_PATH: &str = "src/inputs/day_1_test.txt";
// const INPUT_PATH: &str = "src/inputs/day_1_test_2.txt";

#[allow(dead_code)]
const LETTERS_AS_DIGITS: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

#[allow(dead_code)]
pub fn day_1_part_1() {
    let input = std::fs::read_to_string(INPUT_PATH).unwrap();
    let mut digits: Vec<u32> = Vec::new();
    for line in input.lines() {
        let digits_in_number = get_digits_in_number(line);
        digits.push(digits_in_number);
    }

    let mut sum: u32 = 0;
    for digit in digits {
        sum += digit;
    }

    println!("Sum: {}", sum);
}

#[allow(dead_code)]
pub fn day_1_part_2() {
    let input = std::fs::read_to_string(INPUT_PATH).unwrap();
    let mut digits: Vec<u32> = Vec::new();
    for line in input.lines() {
        let split_line: Vec<String> = split_into_digits_and_letters(line);
        let mut digits_in_string: Vec<u32> = Vec::new();
        for part in split_line {
            if part.chars().all(char::is_numeric) {
                let digit = part.parse::<u32>().unwrap();
                digits_in_string.push(digit);
            } else {
                let mut letters_as_digits = get_letters_as_digits(&part);
                digits_in_string.append(&mut letters_as_digits);
            }
        }
        let first_digit = digits_in_string[0];
        let last_digit = digits_in_string[digits_in_string.len() - 1];
        let full_digit = first_digit * 10 + last_digit;
        digits.push(full_digit);
    }

    let mut sum: u32 = 0;
    for digit in digits {
        sum += digit;
    }

    println!("Sum: {}", sum);
}

#[allow(dead_code)]
fn get_digit(input: &str, pattern: &str) -> u32 {
    let re = regex::Regex::new(pattern).unwrap();
    let caps = re.captures(input).unwrap();
    let digit = caps.name("digit").unwrap().as_str().parse::<u32>().unwrap();
    digit
}

#[allow(dead_code)]
fn get_first_digit(input: &str) -> u32 {
    get_digit(input, r"(?P<digit>\d).*")
}

#[allow(dead_code)]
fn get_last_digit(input: &str) -> u32 {
    get_digit(input, r".*(?P<digit>\d)")
}

#[allow(dead_code)]
fn get_digits_in_number(input: &str) -> u32 {
    let first_digit = get_first_digit(input);
    let last_digit = get_last_digit(input);
    let digits_in_number = first_digit * 10 + last_digit;

    digits_in_number
}

#[allow(dead_code)]
fn get_letters_as_digits(input: &str) -> Vec<u32> {
    let letters_as_digits: HashMap<&str, u32> = LETTERS_AS_DIGITS.iter().cloned().collect();
    let mut digits: Vec<u32> = Vec::new();

    // example: two1nine -> [2,9], eightwothree -> [8,2,3], abcone2threexyz -> [1,2], sgjshgjs -> []
    let mut input = input;
    while input.len() > 0 {
        let mut found_digit = false;
        for (letter, digit) in &letters_as_digits {
            if input.starts_with(letter) {
                digits.push(*digit);
                input = &input[letter.len()..];
                found_digit = true;
                break;
            }
        }
        if !found_digit {
            input = &input[1..];
        }
    }

    digits
}

#[allow(dead_code)]
fn split_into_digits_and_letters(input: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut index = 0;
    let chars: Vec<char> = input.chars().collect();

    while index < chars.len() {
        let mut longest_match = "";
        let mut match_len = 0;

        // Chercher le plus long mot correspondant
        for &(word, _) in &LETTERS_AS_DIGITS {
            if input[index..].starts_with(word) && word.len() > match_len {
                longest_match = word;
                match_len = word.len();
            }
        }

        if match_len > 0 {
            // Ajouter le mot correspondant au résultat
            result.push(longest_match.to_string());
        } else {
            // Traiter comme un caractère ordinaire
            result.push(chars[index].to_string());
        }

        // Incrémenter l'index de 1
        index += 1;
    }

    result
}