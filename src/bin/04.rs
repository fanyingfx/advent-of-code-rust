use std::collections::{BTreeMap, HashMap, HashSet};

advent_of_code::solution!(4);

#[derive(Debug)]
pub struct Card {
    card_number: u32,
    winning_numbers: HashSet<u32>,
    my_numbers: HashSet<u32>,
}

impl Card {
    fn score(&self) -> u32 {
        let exp = self.winning_numbers.intersection(&self.my_numbers).count() as u32;
        match exp {
            0 => 0,
            _ => 2u32.pow(exp - 1),
        }
    }
    fn point(&self) -> u32 {
        self.winning_numbers.intersection(&self.my_numbers).count() as u32
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let cards = input.lines().filter_map(process_line);
    let res = cards.map(|card| card.score()).sum();
    Some(res)
}
pub fn process_line(line: &str) -> Option<Card> {
    let mut parts = line.split('|');
    let mut card_parts = parts.next()?.split(':');
    let card_number: u32 = card_parts
        .next()?
        .split(' ')
        .last()?
        .parse()
        .expect("card number should be a number string");
    let winning_numbers = card_parts
        .last()?
        .split(' ')
        .filter_map(|num| num.trim().parse().ok())
        .collect::<HashSet<u32>>();
    let my_numbers = parts
        .last()?
        .split(' ')
        .filter_map(|num| num.parse().ok())
        .collect::<HashSet<u32>>();

    Some(Card {
        card_number,
        winning_numbers,
        my_numbers,
    })
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards = input.lines().filter_map(process_line);
    let card_map = cards
        .clone()
        .map(|card| (card.card_number, card))
        .collect::<BTreeMap<u32, _>>();
    // dbg!(&cards_point);
    let card_counts = card_map.len() as u32;
    let mut dp:HashMap<u32, u32> = HashMap::new();
    fn calculate_point(card_number: u32, card_counts: u32, card_map: &BTreeMap<u32, Card>,dp:&mut HashMap<u32,u32>) -> u32 {
        if let Some(&val) = dp.get(&card_number){
            return  val;
        }
        let card = card_map.get(&card_number).unwrap();
        if card.point()==0 ||card.card_number == card_counts {
            dp.insert(card_number, 1);
            return 1;
        }
        let start = card.card_number + 1;
        let mut end = start + card.point() - 1;
        if end > card_counts {
            end = card_counts
        }
        let mut sum =1;
        for card_number in  start..=end{
            sum+=calculate_point(card_number, card_counts, card_map,dp);
        }
        dp.insert(card_number, sum);
        sum
        }
    for i in 1..=card_counts{
    calculate_point(i, card_counts, &card_map,&mut dp);
    }

    // dbg!(&dp);
    let mut res=0;
    for (_,value) in dp.into_iter(){
        res+=value;
    // res+=calculate_point(card_number, card_counts, &card_map,&mut dp);
    }
    Some(res)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(30));
    }
}
