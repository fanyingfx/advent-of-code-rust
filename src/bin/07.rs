use std::{cmp::Ordering, collections::HashMap};

advent_of_code::solution!(7);
#[derive(PartialEq, PartialOrd, Debug)]
enum CardType {
    High(String),
    OnePair(String),
    TwoPair(String),
    ThreeKind(String),
    FullHouse(String),
    FourKind(String),
    FiveKind(String),
}
const CARD_LABELS: &str = "23456789TJQKA";
const CARD_LABELSJ: &str = "J23456789TQKA";
impl CardType {
    fn compare(&self, other: &CardType) -> Ordering {
        use CardType::*;
        match (self, other) {
            (High(a), High(b))
            | (OnePair(a), OnePair(b))
            | (TwoPair(a), TwoPair(b))
            | (ThreeKind(a), ThreeKind(b))
            | (FullHouse(a), FullHouse(b))
            | (FourKind(a), FourKind(b))
            | (FiveKind(a), FiveKind(b)) => {
                let mut res = Ordering::Equal;
                for (ca, cb) in a.chars().zip(b.chars()) {
                    if ca != cb {
                        let oa = CARD_LABELS.find(ca).unwrap_or(0);
                        let ob = CARD_LABELS.find(cb).unwrap_or(0);
                        res = if oa > ob {
                            Ordering::Greater
                        } else {
                            Ordering::Less
                        };
                        break;
                    }
                }
                res
            }
            _ => {
                if self > other {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
        }
    }
    fn comparej(&self, other: &CardType) -> Ordering {
        use CardType::*;
        match (self, other) {
            (High(a), High(b))
            | (OnePair(a), OnePair(b))
            | (TwoPair(a), TwoPair(b))
            | (ThreeKind(a), ThreeKind(b))
            | (FullHouse(a), FullHouse(b))
            | (FourKind(a), FourKind(b))
            | (FiveKind(a), FiveKind(b)) => {
                let mut res = Ordering::Equal;
                for (ca, cb) in a.chars().zip(b.chars()) {
                    if ca != cb {
                        let oa = CARD_LABELSJ.find(ca).unwrap_or(0);
                        let ob = CARD_LABELSJ.find(cb).unwrap_or(0);
                        res = if oa > ob {
                            Ordering::Greater
                        } else {
                            Ordering::Less
                        };
                        break;
                    }
                }
                res
            }
            _ => {
                if self > other {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
        }
    }
}
fn parse_card_type(cards: String) -> CardType {
    use CardType::*;
    let mut counts = HashMap::new();
    for c in cards.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    let mut counts_vec: Vec<(char, usize)> = counts.into_iter().collect();
    counts_vec.sort_by(|a, b| b.1.cmp(&a.1));

    match counts_vec[0].1 {
        5 => FiveKind(cards),
        4 => FourKind(cards),
        3 => {
            if counts_vec.len() == 2 {
                FullHouse(cards)
            } else {
                ThreeKind(cards)
            }
        }
        2 => {
            if counts_vec.len() == 3 {
                TwoPair(cards)
            } else {
                OnePair(cards)
            }
        }
        _ => High(cards),
    }
}
fn parse_card_type_j(cards: String) -> CardType {
    use CardType::*;
    let mut counts = HashMap::new();
    let mut j_counts = 0;
    for c in cards.chars() {
        if c != 'J' {
            *counts.entry(c).or_insert(0) += 1;
        } else {
            j_counts += 1;
        }
    }
    let mut counts_vec: Vec<(char, usize)> = counts.into_iter().collect();
    counts_vec.sort_by(|a, b| b.1.cmp(&a.1));
    if j_counts == 5 {
        return FiveKind(cards);
    }
    let cnt = counts_vec[0].1 as u32 + j_counts;
    match cnt {
        5 => FiveKind(cards),
        4 => FourKind(cards),
        3 if counts_vec.len() == 2 => FullHouse(cards),
        3 => ThreeKind(cards),
        2 if counts_vec.len() == 3 => TwoPair(cards),
        2 => OnePair(cards),
        1 => High(cards),
        _ => panic!("Error Kind"),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands = input
        .lines()
        .map(|line| {
            let mut line_iter = line.split(' ');
            let cards = line_iter.next().unwrap();
            let cards = parse_card_type(cards.to_string());
            let bid = line_iter.next().unwrap().parse::<u32>().unwrap();
            (cards, bid)
        })
        .collect::<Vec<_>>();
    hands.sort_by(|a, b| a.0.compare(&b.0));
    let res = hands
        .iter()
        .enumerate()
        .map(|(index, (_, bid))| {
            let rank = index as u32 + 1;
            bid * rank
        })
        .sum::<u32>();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands = input
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(' ').unwrap();
            let cards = parse_card_type_j(cards.to_string());
            let bid = bid.parse::<u32>().unwrap();
            (cards, bid)
        })
        .collect::<Vec<_>>();
    hands.sort_by(|a, b| a.0.comparej(&b.0));
    let res = hands
        .iter()
        .enumerate()
        .map(|(index, (_, bid))| {
            let rank = index as u32 + 1;
            bid * rank
        })
        .sum::<u32>();

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
