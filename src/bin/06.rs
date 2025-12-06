advent_of_code::solution!(6);

fn column_reduction<I>(op: char, values: I) -> u64
where
    I: Iterator<Item = u64>,
{
    match op {
        '+' => values.sum(),
        '*' => values.product(),
        other => panic!("unsupported operator: {other}"),
    }
}

pub fn part_one(input: &str) -> Option<u64> {
     let mut lines: Vec<&str> = input
        .lines()
        .collect();
    let ops_line = lines.pop()?;
    let operators: Vec<char> = ops_line
        .split_whitespace()
        .filter_map(|s| s.chars().next())
        .collect();
    let matrix: Vec<Vec<u64>> = lines
        .into_iter()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<u64>().expect("invalid number"))
                .collect()
        })
        .collect();

    let mut result: u64 = 0;
    let len = matrix[0].len();

    for col in 0..len {
        let col_values: Vec<u64> = matrix.iter().map(|row| row[col]).collect();
        let op = operators[col];
        result += column_reduction(op, col_values.into_iter());
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut rlines = input.lines().rev();
    let operators: Vec<char> = rlines
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|s| s.chars().next())
        .collect();
    for line in rlines {
        matrix.push(line.chars().collect());
    };
    let cols = matrix[0].len();
    let mut matrix_numbers: Vec<Vec<u64>> = Vec::new();

    let mut numbers = Vec::new();
    for c in 0..=cols {
        let seperator_or_end = c == cols || matrix.iter().all(|r| r[c].is_whitespace());
        if seperator_or_end {
            matrix_numbers.push(numbers);
            numbers = Vec::new();
            continue;
        }
        let mut anum = 0;
        let mut ir: u32 = 0;
        for r in matrix.iter() {
            if !r[c].is_whitespace() {
                let d = r[c].to_digit(10).unwrap();
                anum += (d as u64) * 10_u64.pow(ir);
                ir += 1;
            }
        }
        numbers.push(anum);
    }

    let res = operators
        .iter()
        .zip(matrix_numbers.iter())
        .map(|(&op, nums)| column_reduction(op, nums.iter().copied()))
        .sum();
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
