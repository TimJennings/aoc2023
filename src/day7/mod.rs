use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

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
    let cards = parse_cards(&input);
    let answer = sort_and_score(cards);
    println!("{answer}");
}

pub fn puz2() {
    let mut input = read_file_to_string("input/day7-input");
    // replace all the jacks with * so we can use the same code for puz1 and puz2
    input = input.replace('J', "*");
    let cards = parse_cards(&input);
    let answer = sort_and_score(cards);
    println!("{answer}");
}

fn sort_and_score(mut cards: Vec<Hand>) -> u32 {
    cards.sort_by(|a, b| a.value.cmp(&b.value).then(a.compare_cards(b)));

    let mut answer = 0;
    for (index, card) in cards.iter().enumerate() {
        answer += ((index as u32 + 1) * card.bid);
    }
    answer
}

fn card_value(card: char) -> u32 {
    match card {
        '0'..='9' => String::from(card).parse().unwrap(),
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        '*' => 0, //wildcard
        _ => 0,
    }
}

#[derive(Debug)]
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

fn find_kind2(input: &Vec<char>) -> u32 {
    let mut same_set: HashMap<char, u32> = HashMap::new();
    let mut jokers = 0;

    for char in input {
        if char.eq(&'*') {
            jokers += 1;
        } else {
            match same_set.get(char) {
                Some(count) => same_set.insert(*char, count + 1),
                None => same_set.insert(*char, 1),
            };
        }
    }

    // apply the jokers then map onto kinds
    let mut counts: Vec<u32> = same_set.iter().map(|(k, v)| *v).collect();

    // special case all jokers
    if counts.len() == 0 {
        return FIVE_OF_A_KIND;
    }

    // sort highest first
    counts.sort_by(|a, b| b.cmp(a));

    // use the jokers on the largest set
    let mut first = counts.first_mut().unwrap();
    *first += jokers;

    // map onto kinds
    match counts.as_slice() {
        [5] => FIVE_OF_A_KIND,
        [4, 1] => FOUR_OF_A_KIND,
        [3, 1, 1] => THREE_OF_A_KIND,
        [3, 2] => FULL_HOUSE,
        [2, 2, 1] => TWO_PAIR,
        [2, 1, 1, 1] => ONE_PAIR,
        _ => HIGH_CARD,
    }
}

fn parse_cards(input: &str) -> Vec<Hand> {
    let lines: Vec<&str> = input.split("\n").map(|s| s.trim()).collect();
    let mut hands = Vec::new();
    for line in lines {
        let split: Vec<&str> = line.split(" ").collect();

        // calculate value
        let hand = String::from(split[0]).chars().collect();
        let kind = find_kind2(&hand);
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
        find_kind2, parse_cards, sort_and_score, FIVE_OF_A_KIND, FOUR_OF_A_KIND, FULL_HOUSE,
        ONE_PAIR, THREE_OF_A_KIND, TWO_PAIR,
    };

    #[test]
    pub fn test_5of() {
        assert_eq!(
            FIVE_OF_A_KIND,
            find_kind2(&Vec::from(['*', '*', '*', '*', '*']))
        );
        assert_eq!(
            FIVE_OF_A_KIND,
            find_kind2(&Vec::from(['0', '0', '0', '0', '0']))
        );

        assert_eq!(
            FIVE_OF_A_KIND,
            find_kind2(&Vec::from(['0', '0', '*', '0', '0']))
        );
    }

    #[test]
    pub fn test_4of() {
        assert_eq!(
            FOUR_OF_A_KIND,
            find_kind2(&Vec::from(['0', '*', '0', '0', '1']))
        );
        assert_eq!(
            FOUR_OF_A_KIND,
            find_kind2(&Vec::from(['0', '*', '1', '1', '1']))
        );

        assert_eq!(
            FOUR_OF_A_KIND,
            find_kind2(&Vec::from(['0', '0', '0', '0', '1']))
        );
        assert_eq!(
            FOUR_OF_A_KIND,
            find_kind2(&Vec::from(['0', '1', '1', '1', '1']))
        );
    }

    #[test]
    pub fn test_full_house() {
        assert_eq!(
            FULL_HOUSE,
            find_kind2(&Vec::from(['0', '0', '0', '1', '1']))
        );
        assert_eq!(
            FULL_HOUSE,
            find_kind2(&Vec::from(['0', '0', '1', '1', '1']))
        );
        assert_eq!(
            FULL_HOUSE,
            find_kind2(&Vec::from(['*', '0', '0', '1', '1']))
        );
        assert_eq!(
            FULL_HOUSE,
            find_kind2(&Vec::from(['0', '0', '*', '1', '1']))
        );
    }

    #[test]
    pub fn test_3of() {
        assert_eq!(
            THREE_OF_A_KIND,
            find_kind2(&Vec::from(['0', '0', '0', '1', '2']))
        );
        assert_eq!(
            THREE_OF_A_KIND,
            find_kind2(&Vec::from(['1', '0', '0', '0', '2']))
        );
        assert_eq!(
            THREE_OF_A_KIND,
            find_kind2(&Vec::from(['3', '2', '3', '0', '3']))
        );

        assert_eq!(
            THREE_OF_A_KIND,
            find_kind2(&Vec::from(['*', '1', '1', '2', '3']))
        );
        assert_eq!(
            THREE_OF_A_KIND,
            find_kind2(&Vec::from(['2', '1', '*', '1', '3']))
        );
        assert_eq!(
            THREE_OF_A_KIND,
            find_kind2(&Vec::from(['3', '2', '*', '1', '3']))
        );
    }

    #[test]
    pub fn test_1pair() {
        assert_eq!(ONE_PAIR, find_kind2(&Vec::from(['0', '0', '1', '2', '3'])));
        assert_eq!(ONE_PAIR, find_kind2(&Vec::from(['0', '1', '1', '2', '3'])));
        assert_eq!(ONE_PAIR, find_kind2(&Vec::from(['0', '1', '2', '2', '3'])));
        assert_eq!(ONE_PAIR, find_kind2(&Vec::from(['0', '1', '2', '3', '3'])));
    }

    #[test]
    pub fn test_2pair() {
        assert_eq!(TWO_PAIR, find_kind2(&Vec::from(['0', '0', '1', '1', '2'])));
        assert_eq!(TWO_PAIR, find_kind2(&Vec::from(['0', '0', '1', '2', '2'])));
        assert_eq!(TWO_PAIR, find_kind2(&Vec::from(['0', '1', '1', '2', '2'])));
    }

    #[test]
    pub fn test1() {
        let test_data = test_data();

        let cards = parse_cards(&test_data);
        let answer = sort_and_score(cards);
        assert_eq!(6440, answer);
    }

    #[test]
    pub fn test2() {
        let mut test_data = test_data();
        // replace all the jacks with * so we can use the same code for puz1 and puz2
        test_data = test_data.replace('J', "*");
        let cards = parse_cards(&test_data);
        let answer = sort_and_score(cards);
        println!("{answer}");

        assert_eq!(5905, answer);
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
