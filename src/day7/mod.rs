use std::cmp::Ordering;

use crate::common::file_io::read_file_to_string;

const FIVE_OF_A_KIND: u32 = 7;
const FOUR_OF_A_KIND: u32 = 6;
const FULL_HOUSE: u32 = 5;
const THREE_OF_A_KIND: u32 = 4;
const TWO_PAIR: u32 = 3;
const ONE_PAIR: u32 = 2;
const HIGH_CARD: u32 = 1;

pub fn puz1() {
    let input = read_file_to_string("input/day7-input");
    let mut cards = parse_cards(&input);
    println!("{cards:?}");
    cards.sort_by(|a, b| a.value.cmp(&b.value).then(a.compare_cards(b)));

    println!("{cards:?}");

    let mut answer = 0;
    for (index, card) in cards.iter().enumerate() {
        answer += ((index as u32 + 1) * card.bid);
    }
    println!("{answer}");
}

fn card_value(card: char) -> u32 {
    match card {
        '0'..='9' => String::from(card).parse().unwrap(),
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => 0,
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Hand {
    hand: Vec<char>,
    bid: u32,
    value: u32,
}

impl Hand {
    pub fn compare_cards(&self, b: &Hand) -> Ordering {
        let b_hand = &b.hand;
        for (index, card) in self.hand.iter().enumerate() {
            match card_value(*card).cmp(&card_value(b_hand[index])) {
                Ordering::Equal => continue,
                Ordering::Less => return Ordering::Less,
                Ordering::Greater => return Ordering::Greater,
            }
        }
        Ordering::Equal
    }
}

fn find_kind(input: &Vec<char>) -> u32 {
    //copy the input
    let mut sorted_copy = Vec::from(['0', '0', '0', '0', '0']);

    sorted_copy.clone_from_slice(input.as_slice());

    sorted_copy.sort_by(|a, b| card_value(*a).cmp(&card_value(*b)));

    let result = HIGH_CARD;

    if sorted_copy.iter().all(|card| card.eq(&sorted_copy[0])) {
        return FIVE_OF_A_KIND;
    } else if (sorted_copy[0..4]
        .iter()
        .all(|card| card.eq(&sorted_copy[0])))
        || (sorted_copy[1..5]
            .iter()
            .all(|card| card.eq(&sorted_copy[1])))
    {
        return FOUR_OF_A_KIND;
    } else if (sorted_copy[0..3]
        .iter()
        .all(|card| card.eq(&sorted_copy[0]))
        && (sorted_copy[3].eq(&sorted_copy[4])))
        || (sorted_copy[2..=4]
            .iter()
            .all(|card| card.eq(&sorted_copy[2]))
            && (sorted_copy[0].eq(&sorted_copy[1])))
    {
        return FULL_HOUSE;
    } else if (sorted_copy[0..3]
        .iter()
        .all(|card| card.eq(&sorted_copy[0])))
        || (sorted_copy[1..4]
            .iter()
            .all(|card| card.eq(&sorted_copy[1])))
        || (sorted_copy[2..5]
            .iter()
            .all(|card| card.eq(&sorted_copy[2])))
    {
        return THREE_OF_A_KIND;
    } else if (sorted_copy[0].eq(&sorted_copy[1]) && sorted_copy[2].eq(&sorted_copy[3]))
        || (sorted_copy[0].eq(&sorted_copy[1]) && sorted_copy[3].eq(&sorted_copy[4]))
        || (sorted_copy[1].eq(&sorted_copy[2]) && sorted_copy[3].eq(&sorted_copy[4]))
    {
        return TWO_PAIR;
    } else if (sorted_copy[0].eq(&sorted_copy[1]))
        || (sorted_copy[1].eq(&sorted_copy[2]))
        || (sorted_copy[2].eq(&sorted_copy[3]))
        || (sorted_copy[3].eq(&sorted_copy[4]))
    {
        return ONE_PAIR;
    }

    result
}

fn parse_cards(input: &str) -> Vec<Hand> {
    let lines: Vec<&str> = input.split("\n").map(|s| s.trim()).collect();
    let mut hands = Vec::new();
    for line in lines {
        let split: Vec<&str> = line.split(" ").collect();

        // calculate value
        let hand = String::from(split[0]).chars().collect();
        let kind = find_kind(&hand);
        hands.push(Hand {
            hand: hand,
            bid: split[1].parse().unwrap(),
            value: kind,
        });
    }
    hands
}

mod test {
    use crate::day7::{
        parse_cards, FIVE_OF_A_KIND, FOUR_OF_A_KIND, FULL_HOUSE, ONE_PAIR, THREE_OF_A_KIND,
        TWO_PAIR,
    };

    use super::find_kind;

    #[test]
    pub fn test_5of() {
        assert_eq!(
            FIVE_OF_A_KIND,
            find_kind(&Vec::from(['0', '0', '0', '0', '0']))
        );
    }

    #[test]
    pub fn test_4of() {
        assert_eq!(
            FOUR_OF_A_KIND,
            find_kind(&Vec::from(['0', '0', '0', '0', '1']))
        );
        assert_eq!(
            FOUR_OF_A_KIND,
            find_kind(&Vec::from(['0', '1', '1', '1', '1']))
        );
    }

    #[test]
    pub fn test_full_house() {
        assert_eq!(FULL_HOUSE, find_kind(&Vec::from(['0', '0', '0', '1', '1'])));
        assert_eq!(FULL_HOUSE, find_kind(&Vec::from(['0', '0', '1', '1', '1'])));
    }

    #[test]
    pub fn test_3of() {
        assert_eq!(
            THREE_OF_A_KIND,
            find_kind(&Vec::from(['0', '0', '0', '1', '2']))
        );
        assert_eq!(
            THREE_OF_A_KIND,
            find_kind(&Vec::from(['1', '0', '0', '0', '2']))
        );
        assert_eq!(
            THREE_OF_A_KIND,
            find_kind(&Vec::from(['3', '2', '3', '0', '3']))
        );
    }

    #[test]
    pub fn test_1pair() {
        assert_eq!(ONE_PAIR, find_kind(&Vec::from(['0', '0', '1', '2', '3'])));
        assert_eq!(ONE_PAIR, find_kind(&Vec::from(['0', '1', '1', '2', '3'])));
        assert_eq!(ONE_PAIR, find_kind(&Vec::from(['0', '1', '2', '2', '3'])));
        assert_eq!(ONE_PAIR, find_kind(&Vec::from(['0', '1', '2', '3', '3'])));
    }

    #[test]
    pub fn test_2pair() {
        assert_eq!(TWO_PAIR, find_kind(&Vec::from(['0', '0', '1', '1', '2'])));
        assert_eq!(TWO_PAIR, find_kind(&Vec::from(['0', '0', '1', '2', '2'])));
        assert_eq!(TWO_PAIR, find_kind(&Vec::from(['0', '1', '1', '2', '2'])));
    }

    #[test]
    pub fn test1() {
        let test_data = test_data();

        let mut cards = parse_cards(&test_data);
        println!("{cards:?}");
        cards.sort_by(|a, b| a.value.cmp(&b.value).then(a.compare_cards(b)));

        println!("{cards:?}");

        let mut answer = 0;
        for (index, card) in cards.iter().enumerate() {
            answer += ((index as u32 + 1) * card.bid);
        }
        assert_eq!(6440, answer);
    }

    fn test_data() -> String {
        r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
            .to_string()
    }
}
