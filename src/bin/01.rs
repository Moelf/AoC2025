advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut dial: i32 = 50;
    let mut res: u64 = 0;
    // Each line looks like "L68" or "R23"
    // which means turn left or right and move that many steps

    for line in input.lines() {
        let (turn, steps) = line.split_at(1);
        let steps: i32 = steps.parse().unwrap();
        match turn {
            "L" => dial -= steps,
            "R" => dial += steps,
            _ => panic!("huh?"),
        }

        // dial runs 0 - 99, so need to wrap around
        dial = dial.rem_euclid(100);
        res += (dial == 0) as u64;
    }
    return Some(res);
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut dial: i32 = 50;
    let mut res: u64 = 0;
    // Each line looks like "L68" or "R23"
    // which means turn left or right and move that many steps

    for line in input.lines() {
        let (turn, steps) = line.split_at(1);
        let steps: i32 = steps.parse().unwrap();

        let step = match turn {
            "L" => -1,
            "R" => 1,
            _ => panic!("huh?"),
        };
        for _ in 0..steps {
            dial += step;
            // if click zero, increment res
            dial = dial.rem_euclid(100);
            res += (dial == 0) as u64;
        }

    }
    return Some(res);
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
        assert_eq!(result, Some(6));
    }
}
