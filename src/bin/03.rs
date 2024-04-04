use std::collections::HashSet;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let nums = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let not_symbol = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.'];
    let matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let (w, h) = (matrix[0].len(), matrix.len());
    let mut sum = 0;

    for (i, row) in matrix.iter().enumerate() {
        let mut start_index = 0;
        let mut end_index = 0;

        while end_index < w {
            if nums.contains(&row[start_index]) {
                while end_index + 1 < w && nums.contains(&row[end_index + 1]) {
                    end_index += 1
                }
                if end_index == w {
                    end_index = w - 1;
                }
                let mut with_symbol = false;
                let number = row[start_index..=end_index].iter().collect::<String>();

                let current_number = number.parse::<u32>().expect("should be a num");
                let new_s = if start_index > 0 {
                    start_index - 1
                } else {
                    start_index
                };
                let new_e = if end_index < w - 1 {
                    end_index + 1
                } else {
                    end_index
                };
                // dbg!(number,current_number);
                if i > 0 {
                    for ix in new_s..=new_e {
                        if ix < w && !not_symbol.contains(&matrix[i - 1][ix]) {
                            with_symbol = true;
                        }
                    }
                }
                if i < h - 1 {
                    for ix in new_s..=new_e {
                        if ix < w && !not_symbol.contains(&matrix[i + 1][ix]) {
                            with_symbol = true;
                        }
                    }
                }
                if start_index > 0 && !not_symbol.contains(&row[start_index - 1]) {
                    with_symbol = true;
                }
                if end_index < w - 1 && !not_symbol.contains(&row[end_index + 1]) {
                    with_symbol = true;
                }
                if with_symbol {
                    sum += current_number;
                }
                // dbg!(current_number,row,with_symbol);
            }
            end_index += 1;
            start_index = end_index;
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    // let not_symbol = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.'];
    let mut res=0;
    let matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let (w, h) = (matrix[0].len(), matrix.len());
    for (i, row) in matrix.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if c != &'*' {
                continue;
            }
            // let mut vec: Vec<u32> = Vec::new();
            let directions = [
                (-1, -1),
                (0, -1),
                (1, -1),
                (-1, 0),
                (1, 0),
                (-1, 1),
                (0, 1),
                (1, 1),
            ];
            let mut num_set = HashSet::new();
            for (dr, dc) in &directions {
                let new_row = i as i32 + dr ;
                let new_col = j as i32 + dc;
                if new_row >=0 && new_col>=0{
                    let new_row= new_row as usize;
                    let new_col= new_col as usize;

                    if matrix[new_row][new_col].is_ascii_digit(){
                        let mut left = new_col;
                        let mut right = new_col;
                        while left >0 && matrix[new_row][left-1].is_ascii_digit() {
                            left -=1;
                        }
                        while right <w-1 && matrix[new_row][right+1].is_ascii_digit() {
                            right+=1
                        }
                        dbg!(matrix[new_row][left..=right].iter().collect::<String>());

                        num_set.insert((new_row,left,right));
                    }
                }

            }
            // dbg!(&num_set);
            if num_set.len()==2{
                // dbg!("two values");
                res+=num_set.iter().fold(1, |acc,x| {
                    let v= matrix[x.0][x.1..=x.2].iter().collect::<String>();
                    let v = v.parse::<u32>().unwrap();
                    acc*v
                }
            )
            }
        }
    }
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
