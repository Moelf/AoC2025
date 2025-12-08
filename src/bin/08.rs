advent_of_code::solution!(8);

use std::collections::HashSet;

const N_CONNECT: u32 = 10;

fn dist(a: (u64, u64, u64), b: (u64, u64, u64)) -> f64 {
    let d = (a.0 - b.0).pow(2)
        + (a.1 - b.1).pow(2)
        + (a.2 - b.2).pow(2);
    (d as f64).sqrt()
}

pub fn part_one(input: &str) -> Option<u64> {
    // each line looks like 162,817,812
    // want to parse into a vec
    let boxes: Vec<(u64, u64, u64)> = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .map(|nums: Vec<u64>| (nums[0], nums[1], nums[2]))
        .collect();

    let distance_pairs: Vec<(f64, usize, usize)>
    let junction_groups: Vec<HashSet<(u64, u64, u64)>> = Vec::new();

    None
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
