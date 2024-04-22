
advent_of_code::solution!(5);
// use std::ops::Range;
use std::ops::RangeInclusive;

use indicatif::ProgressIterator;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use indicatif::ParallelProgressIterator;
#[derive(Debug)]
struct MapRange {
    source_start: u64,
    destination_start: u64,
    range_length: u64,
}
#[derive(Debug)]
struct ItemMap {
    map_ranges: Vec<MapRange>,
}
impl MapRange {
    fn get_destination(&self, source: u64) -> Option<u64> {
        let range = self.source_start..=self.source_start + self.range_length;
        if range.contains(&source) {
            let res = self.destination_start + (source - self.source_start);
            return Some(res);
        }
        None
    }
}
impl ItemMap {
    fn parse_map(line: &str) -> Self {
        let map_lines = line.split(":\n").last().unwrap();
        let map_ranges = map_lines
            .trim()
            .split('\n')
            .map(|map_line| {
                // dbg!(map_line);
                let tuple3 = map_line
                    .split(' ')
                    .filter_map(|num| num.parse::<u64>().ok())
                    .collect::<Vec<_>>();
                if tuple3.len() != 3 {
                    panic!("{} should only have 3 values", map_lines)
                }
                let (source_start, destination_start, range_length) =
                    (tuple3[1], tuple3[0], tuple3[2]);
                MapRange {
                    source_start,
                    destination_start,
                    range_length,
                }
            })
            .collect::<Vec<_>>();
        ItemMap { map_ranges }
    }
    fn get_destination(&self, source: u64) -> u64 {
        for map_range in self.map_ranges.iter() {
            if let Some(destination) = map_range.get_destination(source) {
                return destination;
            }
        }
        source
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.split("\n\n");
    let seeds = lines
        .next()?
        .split(": ")
        .last()?
        .split(' ')
        .filter_map(|x| x.parse::<u64>().ok())
        .collect::<Vec<_>>();
    // let s=splits.into_iter().collect::<Vec<String>>();
    // let line = lines.next()?;
    let maps = lines.map(ItemMap::parse_map).collect::<Vec<_>>();

    fn get_location(maps: &[ItemMap], source: u64) -> u64 {
        maps.iter().fold(source, |acc, m| {
            // dbg!(acc,m,m.get_destination(acc));

            m.get_destination(acc)
        })
    }
    seeds.iter().map(|x| get_location(&maps, *x)).min()
    // Some(min)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.split("\n\n");
    let seeds = lines
        .next()?
        .split(": ")
        .last()?
        .split(' ')
        // .filter_map(|x| x.parse::<u64>().ok())
        .collect::<Vec<_>>()
        .chunks(2)
        .filter_map(|chunk| {
            if chunk.len() == 2 {
                let (start,end):(u64,u64)=(chunk[0].parse().unwrap(), chunk[1].parse().unwrap());
                Some(RangeInclusive::new(start, start+end))
            } else {
                None
            }
        })
        .collect::<Vec<RangeInclusive<u64>>>();
    // let s=splits.into_iter().collect::<Vec<String>>();
    // let line = lines.next()?;
    let maps = lines.map(ItemMap::parse_map).collect::<Vec<_>>();

    fn get_location(maps: &[ItemMap], source: u64) -> u64 {
        maps.iter().fold(source, |acc, m| {

            m.get_destination(acc)
        })
    }
    // dbg!(&seeds,&maps);

    seeds
        .into_par_iter()
        .progress() // Remove this line
        .filter_map(|range| {
            range.clone().map(|x| {
                get_location(&maps, x)
            }).min()
        }).min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
