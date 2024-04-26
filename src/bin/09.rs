advent_of_code::solution!(9);
fn make_prediction(history: &[i32]) -> i32 {
    if history.iter().all(|x| x == &0) {
        return 0;
    }
    let last_v = history.last().unwrap();
    let rec = history.windows(2).map(|s| s[1] - s[0]).collect::<Vec<_>>();

    last_v + make_prediction(&rec)

}
fn make_prediction_prev(history: &[i32]) -> i32 {
    if history.iter().all(|x| x == &0) {
        return 0;
    }
    let last_v = history.first().unwrap();
    let rec = history.windows(2).map(|s| s[1] - s[0]).collect::<Vec<_>>();

    last_v - make_prediction_prev(&rec)

}
pub fn part_one(input: &str) -> Option<i32> {
    let historys = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    // dbg!(historys);
    let res = historys.iter().map(|history|make_prediction(history)).sum::<i32>();

    Some(res)
}

pub fn part_two(input: &str) -> Option<i32> {
    let historys = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    // dbg!(historys);
    let res = historys.iter().map(|history|make_prediction_prev(history)).sum::<i32>();

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
