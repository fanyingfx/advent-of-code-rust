advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines().map(|line|{
        line.split(':').last().unwrap().replace(' ', "")
    });
    let time = lines.next()?.parse::<f64>().ok()?;
    let distance = lines.next()?.parse::<f64>().ok()?;
    let delta = (time*time -4.0*distance).sqrt();
    let left = ((time-delta)/2.0).ceil() as u32;
    let right =((time+delta)/2.0).floor() as u32;
    // dbg!(left,right,right-left+1);


    Some(right-left+1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
