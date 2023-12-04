use std::{collections::HashMap, fmt::format};

use regex::Regex;

use crate::common::file_io::read_file_to_vec;

pub fn puz1() {
    let test_grid: Vec<String> = read_file_to_vec("input/day4-input");

    let cards = parse_cards(test_grid);

    let total = cards.total();

    println!("Day3 puz1 total: {total}");
}

pub fn puz2() {
    let test_grid: Vec<String> = read_file_to_vec("input/day4-input");

    let cards = parse_cards(test_grid);

    let total = cards.recursive_card_count();

    println!("Day3 puz2 number of cards: {total}");
}

#[derive(Debug)]
struct Scratchcards {
    cards: HashMap<u32, Scratchcard>,
}

impl Scratchcards {
    pub fn recursive_card_count(&self) -> u32 {
        let mut card_count: u32 = 0;

        for card in &self.cards {
            card_count = card_count + self.desend_card(&card.1);
        }

        card_count
    }

    fn desend_card(&self, card: &Scratchcard) -> u32 {
        let mut cards = 1;
        // how many wins on this card
        let wins = card.total_wins;

        // clone in a number of cards equal to wins and recurse
        let mut draw = 0;
        while draw < wins {
            let next_card = self
                .cards
                .get(&(card.id + draw + 1))
                .expect(format!("failed to draw id {}", card.id + draw + 1).as_str());
            // recurse into the won card
            cards = cards + self.desend_card(&next_card);
            draw = draw + 1;
        }
        cards
    }

    pub fn total(&self) -> u32 {
        let mut total: u32 = 0;

        for card in self.cards.iter() {
            let card = card.1;
            let mut card_score = 0;
            let mut next_score = 1;

            for drawn_number in &card.drawn_numbers {
                if card.winning_numbers.contains(&drawn_number) {
                    // score points and increment next score
                    println!(
                        "Card {} drew {} and scored {}",
                        card.id, drawn_number, next_score
                    );
                    card_score = next_score;
                    next_score = next_score * 2;
                }
            }

            println!("Card {} scored {}", card.id, card_score);
            total = total + card_score;
        }

        total
    }
}

#[derive(Debug)]
struct Scratchcard {
    id: u32,
    winning_numbers: Vec<u32>,
    drawn_numbers: Vec<u32>,
    total_wins: u32,
}

fn parse_cards(input: Vec<String>) -> Scratchcards {
    Scratchcards {
        cards: input
            .iter()
            .map(|line| line.trim())
            .map(|line| parse_card(line))
            .map(|card| (card.id, card))
            .collect(),
    }
}

fn parse_card(line: &str) -> Scratchcard {
    let matcher = Regex::new("^Card *([0-9]*):(.*)$");
    let re = matcher.unwrap();
    let captures = re.captures(line).expect("failed to get captures");
    let card_number = captures.get(1).expect("failed to get game id").as_str();

    let cards: Vec<&str> = captures
        .get(2)
        .expect("failed to get cards")
        .as_str()
        .split("|")
        .collect();
    let winning_cards = cards[0];
    let drawn_cards = cards[1];

    let winning_cards: Vec<u32> = winning_cards
        .trim()
        .split_whitespace()
        .map(|card| card.parse().expect("failed to parse number"))
        .collect();

    let drawn_cards: Vec<u32> = drawn_cards
        .trim()
        .split_whitespace()
        .map(|card| card.parse().expect("failed to parse number"))
        .collect();

    let mut win_count = 0;

    for drawn_number in &drawn_cards {
        if winning_cards.contains(&drawn_number) {
            // score points and increment next score
            win_count = win_count + 1;
        }
    }

    Scratchcard {
        id: card_number.parse().expect("failed to parse card number"),
        winning_numbers: winning_cards,
        drawn_numbers: drawn_cards,
        total_wins: win_count,
    }
}

mod tests {
    use crate::{common::file_io::read_file_to_vec, day4::parse_cards};

    #[test]
    fn test_total_points() {
        let test_grid: Vec<String> = create_test_input();

        let cards = parse_cards(test_grid);
        println!("{cards:?}");
        let total = cards.total();

        assert_eq!(13, total);
    }

    #[test]
    fn test_full_data_points() {
        let test_grid: Vec<String> = read_file_to_vec("input/day4-input");

        let cards = parse_cards(test_grid);

        let total = cards.total();

        assert_eq!(21558, total);
    }

    #[test]
    fn test_recursive_card_count_test_input() {
        let test_grid: Vec<String> = create_test_input();

        let cards = parse_cards(test_grid);
        let total = cards.recursive_card_count();

        assert_eq!(30, total);
    }

    #[test]
    fn test_recursive_card_count_full_input() {
        let test_grid: Vec<String> = read_file_to_vec("input/day4-input");

        let cards = parse_cards(test_grid);
        let total = cards.recursive_card_count();

        assert_eq!(10425665, total);
    }

    fn create_test_input() -> Vec<String> {
        vec![
            String::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
            String::from("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"),
            String::from("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"),
            String::from("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"),
            String::from("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"),
            String::from("Card     6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"),
        ]
    }
}
