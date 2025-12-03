advent_of_code::solution!(3);

fn shared_impl(input: &str, rounds: usize) -> Option<u64> {
    let mut total = 0;
    for line in input.lines() {
        let digits: Vec<u32> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        let mut idx = 0;
        for i in 1..=rounds {
            let mut temp_max = digits[idx];
            let reserve_digits = rounds - i;
            for i in idx..digits.len()-reserve_digits {
                if digits[i] > temp_max {
                    idx = i;
                    temp_max = digits[i];
                }
            }
            idx += 1;
            total += temp_max as u64 * u64::pow(10, (rounds - i) as u32);
        }
    }
    Some(total)
}
pub fn part_one(input: &str) -> Option<u64> {
    shared_impl(input, 2)
}

pub fn part_two(input: &str) -> Option<u64> {
    shared_impl(input, 12)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
