advent_of_code::solution!(5);

fn _solve(input: &str, part1: bool) -> Option<u64> {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let ranges_part = parts[0];
    let numbers_part = parts[1];

    let mut ranges = Vec::new();
    for line in ranges_part.lines() {
        let bounds: Vec<&str> = line.split('-').collect();
        let start: u64 = bounds[0].parse().unwrap();
        let end: u64 = bounds[1].parse().unwrap();
        ranges.push((start, end));
    }
    let mut count = 0;
    if part1 {
        for line in numbers_part.lines() {
            let number: u64 = line.parse().unwrap();
            for (start, end) in &ranges {
                if number >= *start && number <= *end {
                    count += 1;
                    break;
                }
            }
        }
        Some(count)
    } else {
        ranges.sort();
        let mut total_len = ranges[0].1 - ranges[0].0 + 1;
        let mut current_end = ranges[0].1;
        for (start, end) in ranges.iter().skip(1) {
            if *start <= current_end + 1 {
                if *end > current_end {
                    total_len += end - current_end;
                    current_end = *end;
                }
            } else {
                total_len += end - start + 1;
                current_end = *end;
            }
        }
        Some(total_len)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    _solve(input, true)
}

pub fn part_two(input: &str) -> Option<u64> {
    _solve(input, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
