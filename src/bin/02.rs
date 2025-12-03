advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let ranges: Vec<&str> = input.trim().split(',').collect();
    let mut res: u64 = 0;
    for r in ranges {
        let (start, end) = r.split_once('-').unwrap();
        let start: u64 = start.parse().unwrap();
        let end: u64 = end.parse().unwrap();
        for n in start..=end {
            let s = n.to_string();
            let len = s.len();
            if len.is_multiple_of(2) {
                let half = len / 2;
                if s[..half] == s[half..] {
                    res += n;
                }
            }
        }
    }
    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges: Vec<&str> = input.trim().split(',').collect();
    let mut res: u64 = 0;
    for r in ranges {
        let (start, end) = r.split_once('-').unwrap();
        let start: u64 = start.parse().unwrap();
        let end: u64 = end.parse().unwrap();
        for n in start..=end {
            let s = n.to_string();
            let len = s.len();
            let max_pattern_len = len / 2;
            for pattern_len in 1..=max_pattern_len {
                if len.is_multiple_of(pattern_len)
                    && s == s[0..pattern_len].repeat(len / pattern_len)
                {
                    res += n;
                    break;
                }
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
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
