use std::collections::HashMap;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let res = input.lines().map(parse_line_one).sum::<u32>();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let res = input.lines().filter_map(parse_line_two).sum::<u32>();
    Some(res)
}
pub fn parse_line_two(input: &str) -> Option<u32> {
    let games = input
        .split(':')
        .last()
        .expect("should be more than 0 games")
        .split(';');
    // let (mut r, mut g, mut b) = (1, 1, 1);
    let mut counts =HashMap::<_,u32>::from([("red",1),("green",1),("blue",1)]);
    for game in games {
        let cubes = game.split(',');
        for cube in cubes {
            let mut cube=cube.trim().split(' ');
            let amount = cube.next().unwrap().parse::<u32>().unwrap();
            let color = cube.last().unwrap();
            let count = counts.entry(color).or_insert(0);
            *count = (*count).max(amount);

        };
    }
    // r * g * b
    Some(counts["red"] * counts["green"] * counts["blue"])

}
pub fn parse_line_one(input: &str) -> u32 {
    let (r, g, b) = (12, 13, 14);
    let mut parts = input.split(':');
    let game_id = match parts.next() {
        Some(game) => {
            if game.starts_with("Game") {
                game.split(' ')
                    .nth(1)
                    .expect("Should be a game id")
                    .parse::<u32>()
                    .expect("Shoule be a number")
            } else {
                0
            }
        }
        _ => 0,
    };
    // .expect("No game id").split(' ').nth(1).expect("No game id").parse::<u32>().expect("Not a number");
    let games = parts
        .next()
        .expect("should be more than 0 games")
        .split(';');
    for game in games {
        let cubes = game.split(',');
        for cube in cubes {
            match cube {
                _ if cube.ends_with("red") => {
                    if cube
                        .trim()
                        .split(' ')
                        .next()
                        .expect("shuould be a number string")
                        .parse::<u32>()
                        .unwrap()
                        > r
                    {
                        return 0;
                    }
                }
                _ if cube.ends_with("green") => {
                    if cube
                        .trim()
                        .split(' ')
                        .next()
                        .expect("shuould be a number string")
                        .parse::<u32>()
                        .unwrap()
                        > g
                    {
                        return 0;
                    }
                }
                _ if cube.ends_with("blue") => {
                    if cube
                        .trim()
                        .split(' ')
                        .next()
                        .expect("shuould be a number string")
                        .parse::<u32>()
                        .unwrap()
                        > b
                    {
                        return 0;
                    }
                }
                _ => (),
            };
            // dbg!(cube);
        }
    }
    game_id

    // (game_id, red, green, blue)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_parse_line_one() {
        let result = parse_line_one("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert_eq!(result, 1);
    }
    use rstest::rstest;
    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 48)]
    #[case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", 12)]
    #[case(
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        1560
    )]
    #[case(
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        630
    )]
    #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 36)]
    fn test_parse_line_two(#[case] line: &str, #[case] expeted: u32) {
        assert_eq!(parse_line_two(line), Some(expeted));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
