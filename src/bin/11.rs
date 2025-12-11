advent_of_code::solution!(11);

fn parse_flow_map(input: &str) -> std::collections::HashMap<&str, Vec<&str>> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();
            let input = parts[0];
            let outs: Vec<&str> = parts[1].split(" ").collect();
            (input, outs)
        })
        .collect()
}

fn paths<'a>(
    cache: &mut std::collections::HashMap<(&'a str, bool, bool), u64>,
    flow_map: &std::collections::HashMap<&'a str, Vec<&'a str>>,
    current: &'a str,
    mut dac: bool,
    mut fft: bool,
) -> u64 {
    if current == "out" {
        return if dac && fft { 1 } else { 0 };
    }
    if let Some(&cached) = cache.get(&(current, dac, fft)) {
        return cached;
    }
    if current == "dac" {
        dac = true;
    }
    if current == "fft" {
        fft = true;
    }
    let mut total = 0;
    if let Some(nexts) = flow_map.get(current) {
        for next in nexts {
            total += paths(cache, flow_map, next, dac, fft);
        }
    }
    cache.insert((current, dac, fft), total);
    total
}

pub fn part_one(input: &str) -> Option<u64> {
    // each lines reads "aaa: you hhh"
    // which means aaa leads to both you and hhh
    Some(paths(
        &mut std::collections::HashMap::new(),
        &parse_flow_map(input),
        "you",
        true,
        true,
    ))
}

pub fn part_two(input: &str) -> Option<u64> {
    let reaches = paths(
        &mut std::collections::HashMap::new(),
        &parse_flow_map(input),
        "svr",
        false,
        false,
    );

    Some(reaches)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }
}
