advent_of_code::solution!(7);

fn _solve(input: &str) -> (Option<u64>, Option<u64>) {
    let mut lines = input.lines();
    let lineone = lines.next().unwrap();
    let idx_s = lineone.find('S').unwrap();
    let mut beams = vec![0; lineone.len()];
    let mut nsplits = 0;
    beams[idx_s] = 1;
    for line in lines {
        let bclone = beams.clone();
        for (i,nbeams) in bclone.iter().enumerate() {
            if line.chars().nth(i).unwrap() == '^' && bclone[i] > 0 {
                nsplits += 1;
                beams[i] = 0;
                beams[i - 1] += nbeams;
                beams[i + 1] += nbeams;
            }
        }
    }

    (Some(nsplits), Some(beams.iter().sum()))
}

pub fn part_one(input: &str) -> Option<u64> {
    _solve(input).0
}

pub fn part_two(input: &str) -> Option<u64> {
    _solve(input).1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
