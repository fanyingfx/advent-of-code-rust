advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    // let mut sum = 0;
    let output = input
        .lines()
        .map(|line| {
            let mut it = line.chars().filter_map(|character| character.to_digit(10));
            let first = it.next().expect("shold be a number");
            match it.last() {
                Some(num) => first * 10 + num,
                None => first * 10 + first,
            }
        })
        .sum::<u32>();

    Some(output)
}
fn process_line(line: &str) -> Option<u32> {
    let mut index = 0;
    let line_iter = std::iter::from_fn(move || {
        let reduced_line = &line[index..];
        let result = match reduced_line {
            _ if reduced_line.starts_with("one") => Some('1'),
            _ if reduced_line.starts_with("two") => Some('2'),
            _ if reduced_line.starts_with("three") => Some('3'),
            _ if reduced_line.starts_with("four") => Some('4'),
            _ if reduced_line.starts_with("five") => Some('5'),
            _ if reduced_line.starts_with("six") => Some('6'),
            _ if reduced_line.starts_with("seven") => Some('7'),
            _ if reduced_line.starts_with("eight") => Some('8'),
            _ if reduced_line.starts_with("nine") => Some('9'),
            _ => reduced_line.chars().next(),
        };
        index += 1;
        result
    });
    // The rest of your function...
    let mut it = line_iter.filter_map(|character| character.to_digit(10));
    let first = it.next().expect("shold be a number");
    let last = it.last().unwrap_or(first);
    Some(first * 10 + last)
}

pub fn part_two(input: &str) -> Option<u32> {
    let output = input.lines().filter_map(process_line).sum::<u32>();

    Some(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    use rstest::rstest;
    // 29, 83, 13, 24, 42, 14, 76
    // 29, 83, 13, 24, 42, 14, 76
    #[rstest]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]
    fn line_test(#[case] line: &str, #[case] expected: u32) {
        assert_eq!(process_line(line), Some(expected));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
