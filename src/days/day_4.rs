#![allow(dead_code)]

// const INPUT_PATH: &str = "src/inputs/day_4.txt";
const INPUT_PATH: &str = "src/inputs/day_4_test.txt";
// const INPUT_PATH: &str = "src/inputs/day_4_test_2.txt";

#[derive(Clone)]
pub struct Card {
    id: i32,
    raw_line: String,
    hand: Vec<i32>,
    winning_cards: Vec<i32>,
    copy: i32,
}

fn get_numbers_from_line(line: &str) -> Vec<i32> {
    // trim
    let line = line.trim();
    // split by space
    let line = line.split(" ").collect::<Vec<&str>>();
    // remove empty strings
    let line = line.iter().filter(|x| !x.is_empty()).collect::<Vec<&&str>>();
    // convert to i32
    let line = line.iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    line
}

impl Card {
    fn new(id: i32, raw_line: &str, is_copy: i32) -> Card {
        let base_line = raw_line.split(":").collect::<Vec<&str>>()[1];
        let line = base_line.split("|").collect::<Vec<&str>>();
        let hand = get_numbers_from_line(line[0]);
        let winning_cards = get_numbers_from_line(line[1]);

        Card {
            id,
            raw_line: raw_line.to_string(),
            hand,
            winning_cards,
            copy: is_copy,
        }
    }

    fn get_winning_cards(&self) -> Vec<i32> {
        let mut winning_cards = Vec::new();
        for card in &self.hand {
            if self.winning_cards.contains(card) {
                winning_cards.push(*card);
            }
        }

        winning_cards
    }

    fn get_points(&self) -> i32 {
        let winning_cards = self.get_winning_cards();
        // for each winning card gain points like: 1: 1, 2: 2, 3: 4, 4: 8, 5: 16, 6: 32, 7: 64, 8: 128

        let mut points = 0;
        for _i in winning_cards.iter().enumerate() {
            if points == 0 {
                points = 1;
            } else {
                points *= 2;
            }
        }

        points
    }
}


struct Card2
{
    id: usize,
    winning: Vec<usize>,
    hand: Vec<usize>,
    matches: usize,
    points: usize,
}

impl std::str::FromStr for Card2
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let mut parts = s.split(':');
        let id = parts.next().unwrap().split(' ').next_back().unwrap().parse().unwrap();
        let mut sets = parts.next().unwrap().split('|');
        let winning = sets.next().unwrap()
            .trim()
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.parse().unwrap())
            .collect::<Vec<usize>>();
        let hand = sets.next().unwrap()
            .trim()
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.parse().unwrap())
            .collect::<Vec<usize>>();
        let matches = hand.iter().filter(|&x| winning.contains(x)).count();
        let points = 2_usize.pow(matches as u32) / 2;

        Ok(Card2 {
            id,
            winning,
            hand,
            matches,
            points,
        })
    }
}

fn get_indexes(length: i32, start_from: i32) -> Vec<i32> {
    let mut indexes = Vec::new();
    for i in 1..length + 1 {
        indexes.push(start_from + i);
    }

    indexes
}

pub fn day_4_part_1() {
    let input = std::fs::read_to_string(INPUT_PATH).unwrap();

    let mut total_points = 0;
    let mut index = 0;
    for line in input.lines() {
        index += 1;
        let card = Card::new(index, line, 0);
        let points = card.get_points();
        total_points += points;
    }

    println!("Total points: {}", total_points);
}

pub fn day_4_part_2() {
    let cards = include_str!("../../src/inputs/day_4.txt")
        .lines()
        .map(|x| x.parse::<Card2>().unwrap())
        .collect::<Vec<Card2>>();

    let mut counters: Vec<usize> = vec![1; cards.len()];

    for i in 0..counters.len() {
        for j in i..i + cards[i].matches {
            if j + 1 < counters.len() {
                counters[j + 1] += counters[i];
            }
        }
    }

    let result1: usize = cards.iter().map(|card| card.points).sum();
    let result2: usize = counters.iter().sum();

    println!("4\t{result1:<20}\t{result2:<20}");
}

// #[cfg(test)]
// mod tests {
//     /*
//         Expected to have:
//         Card: 1 x 1
//         Card: 2 x 2
//         Card: 3 x 4
//         Card: 4 x 8
//         Card: 5 x 14
//         Card: 6 x 1
//     */
//
//     use crate::days::day_4::Card;
//
//     #[test]
//     fn good_amount_of_cards_1() {
//         let cards: Vec<Card> = super::day_4_part_2();
//         let card_1: Vec<Card> = cards.iter().filter(|x| x.id == 1).cloned().collect();
//
//         assert_eq!(card_1.len(), 1);
//     }
//
//     #[test]
//     fn good_amount_of_cards_2() {
//         let cards: Vec<Card> = super::day_4_part_2();
//         let card_2: Vec<Card> = cards.iter().filter(|x| x.id == 2).cloned().collect();
//
//         assert_eq!(card_2.len(), 2);
//     }
//
//     #[test]
//     fn good_amount_of_cards_3() {
//         let cards: Vec<Card> = super::day_4_part_2();
//         let card_3: Vec<Card> = cards.iter().filter(|x| x.id == 3).cloned().collect();
//
//         assert_eq!(card_3.len(), 4);
//     }
//
//     #[test]
//     fn good_amount_of_cards_4() {
//         let cards: Vec<Card> = super::day_4_part_2();
//         let card_4: Vec<Card> = cards.iter().filter(|x| x.id == 4).cloned().collect();
//
//         assert_eq!(card_4.len(), 8);
//     }
//
//     #[test]
//     fn good_amount_of_cards_5() {
//         let cards: Vec<Card> = super::day_4_part_2();
//         let card_5: Vec<Card> = cards.iter().filter(|x| x.id == 5).cloned().collect();
//
//         assert_eq!(card_5.len(), 14);
//     }
//
//     #[test]
//     fn good_amount_of_cards_6() {
//         let cards: Vec<Card> = super::day_4_part_2();
//         let card_6: Vec<Card> = cards.iter().filter(|x| x.id == 6).cloned().collect();
//
//         assert_eq!(card_6.len(), 1);
//     }
// }