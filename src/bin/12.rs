advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u64> {
    let mut shapes_strs: Vec<&str>= input.split("\n\n").collect();
    let trees_str = shapes_strs.pop()?;

    let sizes: Vec<usize> = shapes_strs.into_iter().map(|s| s.chars().filter(|c| *c == '#').count()).collect();

    let area_demands: Vec<(usize, Vec<usize>)> = trees_str
        .split_terminator("\n")
        .map(|line|{
            let (area_str, demands_str) = line.split_once(": ").unwrap();

            let (a1, a2) = area_str.split_once("x").unwrap();
            let area = a1.parse::<usize>().unwrap() * a2.parse::<usize>().unwrap();

            let demands = demands_str.split_whitespace().map(|d| d.parse::<usize>().unwrap()).collect();

            (area, demands)
        })
        .collect();

    let res = area_demands.iter()
        .filter(|(area, demands)| {
            let demand_size: usize = sizes.iter().zip(demands.iter()).map(|(s, d)| s*d).sum();
            *area >= demand_size + 3
        })
        .count();

    Some(res as u64)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
