use std::collections::HashMap;
use itertools::Itertools;

pub fn run(input: String) -> (usize, usize) {
    let string = input
        .replace("A", "E")
        .replace("T", "A")
        .replace("J", "B")
        .replace("Q", "C")
        .replace("K", "D");
    let ans_1: usize = calculate(&string);

    let string = string.replace("B", "0");
    let ans_2: usize = calculate(&string);

    (ans_1, ans_2)
}

fn calculate(string: &String) -> usize {
    string.lines()
        .map(to_hand)
        .sorted()
        .zip(1..)
        .map(|(hand, rank)| hand.bid * rank)
        .sum()
}

fn to_hand(s: &str) -> Hand {
    let split: Vec<&str> = s.split(" ").collect();
    let cards: &str = split[0].chars().as_str();
    let bid: usize = split[1].parse().unwrap();
    let hand_type: HandType = to_hand_type(cards);
    Hand { cards, hand_type, bid }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Hand<'a> {
    hand_type: HandType,
    cards: &'a str,
    bid: usize,
}

fn to_hand_type(cards: &str) -> HandType {
    let mut card_counts: HashMap<char, usize> = cards.chars().counts();

    let jokers_count = card_counts.remove(&'0').unwrap_or(0);
    if jokers_count == 5 {
        return HandType::FiveOfAKind;
    }
    let strongest_card_count = card_counts.iter_mut()
        .max_by(|(k1, v1), (k2, v2)| v1.cmp(v2).then(k1.cmp(k2)))
        .unwrap()
        .1;
    *strongest_card_count += jokers_count;

    let card_counts: Vec<&usize> = card_counts.values()
        .sorted_by(|a, b| a.cmp(b).reverse())
        .collect();

    return match card_counts[..] {
        [5] => Some(HandType::FiveOfAKind),
        [4, 1] => Some(HandType::FourOfAKind),
        [3, 2] => Some(HandType::FullHouse),
        [3, 1, 1] => Some(HandType::ThreeOfAKind),
        [2, 2, 1] => Some(HandType::TwoPair),
        [2, 1, 1, 1] => Some(HandType::OnePair),
        [1, 1, 1, 1, 1] => Some(HandType::HighCard),
        _ => None
    }.expect(&format!("Unknown hand type: {}, {:?}", cards, card_counts));
}