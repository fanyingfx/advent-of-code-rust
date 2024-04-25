use std::{collections::HashMap, ptr::NonNull};

advent_of_code::solution!(8);
pub fn lcm(nums: &[u64]) -> u64{
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a:u64, b:u64) -> u64{
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}
pub fn part_one(input: &str) -> Option<u64> {
    let (instruction, nodes) = input.split_once("\n\n").unwrap();
    let nodes = nodes
        .lines()
        .map(|line| {
            let (key, value) = line.split_once('=').unwrap();
            let key = key.trim();
            let value = value.trim().replace(['(', ')', ' '], "");
            let (left, right) = value.split_once(',').unwrap();
            (key, (left.to_string(), right.to_string())) // Fix: Change the type of value to String
        })
        .collect::<HashMap<&str, _>>();
    let char_iter = instruction.chars().cycle();
    let mut count = 0;
    let mut current_node = nodes.get("AAA").unwrap();
    for ch in char_iter {
        count += 1;
        let node_name = match ch {
            'L' => current_node.0.to_string(),
            'R' => current_node.1.to_string(),
            _ => panic!(" Error instruction"),
        };
        // dbg!(&node_name);
        if node_name == "ZZZ" {
            return Some(count);
        }
        current_node = nodes.get(&*node_name).unwrap();
    }

    None
}

pub fn part_two(input: &str) -> Option<u64> {
    let (instruction, nodes) = input.split_once("\n\n").unwrap();
    let nodes = nodes
        .lines()
        .map(|line| {
            let (key, value) = line.split_once('=').unwrap();
            let key = key.trim();
            let value = value.trim().replace(['(', ')', ' '], "");
            let (left, right) = value.split_once(',').unwrap();
            (key, (left.to_string(), right.to_string())) // Fix: Change the type of value to String
        })
        .collect::<HashMap<&str, _>>();
    let char_iter = instruction.chars().cycle();
    // let mut count = 0;
    let current_nodes = nodes
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| nodes.get(k).unwrap());

    // count += 1;
    let steps=current_nodes.clone().map(|current_node| {
        let char_iter = char_iter.clone();
        let mut current_node=current_node;
        for (index, ch) in char_iter.enumerate() {
            let node_string = match ch {
                'L' => current_node.0.to_string(),
                'R' => current_node.1.to_string(),
                _ => panic!(" Error instruction"),
            };
            if node_string.ends_with('Z') {
                return (index + 1) as u64;
            }
            current_node=nodes.get(&*node_string).unwrap();
        }
        0
    }).collect::<Vec<u64>>();
    dbg!(&steps);
    let res = lcm(&steps);
    // let tem_node = current_node_list.clone().collect::<Vec<_>>();
    // dbg!(tem_node);
    // current_nodes = current_node_list.map(|k| {
    //     let node =nodes.get(&*k).unwrap();
    //     node
    // }).collect::<Vec<_>>();
    Some(res)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
