advent_of_code::solution!(4);

fn _solve(input: &str, remove: bool) -> Option<u64> {
    let mut count = 0;
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut r = 0;
    let mut c = 0;
    while r < rows {
        while c < cols {
            if grid[r][c] == '@' {
                let mut adjacent_count = 0;
                for dr in -1..=1 {
                    for dc in -1..=1 {
                        if dr == 0 && dc == 0{
                            continue;
                        }
                        let nr = r as isize + dr;
                        let nc = c as isize + dc;
                        if nr < 0 || nr >= rows as isize {
                            continue;
                        }
                        if nc < 0 || nc >= cols as isize {
                            continue;
                        }
                        if grid[nr as usize][nc as usize] == '@'{
                            adjacent_count += 1;
                        }
                    }
                }
                if adjacent_count < 4 {
                    count += 1;
                    if remove{
                        grid[r][c] = '.';
                        r = match r {
                            0 => 0,
                            _ => (r - 1).max(0),
                        };
                        c = match c {
                            0 => 0,
                            _ => (c - 1).max(0),
                        };
                        continue;
                    }
                }
            }
            c += 1;
        }
        c = 0;
        r += 1
    }

    Some(count)
}
pub fn part_one(input: &str) -> Option<u64> {
    _solve(input, false)
}

pub fn part_two(input: &str) -> Option<u64> {
    _solve(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
