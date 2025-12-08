advent_of_code::solution!(8);

use std::collections::HashSet;

fn merge_ntimes(
    nloops: u32,
    total_boxes: usize,
    boxes: &[(i64, i64, i64)],
    junction_groups: &mut Vec<HashSet<usize>>,
    distance_pairs: &mut Vec<(f64, usize, usize)>,
) -> u64 {
    for _ in 0..nloops {
        let (_, i1, i2) = distance_pairs.pop().unwrap();
        let c1i = junction_groups
            .iter()
            .position(|c| c.contains(&i1))
            .unwrap();
        let c2i = junction_groups
            .iter()
            .position(|c| c.contains(&i2))
            .unwrap();
        if c1i == c2i {
            // both in same circuit, skip
            continue;
        } else {
            // merge circuits
            let [c1_g, c2_g] = junction_groups.get_disjoint_mut([c1i, c2i]).unwrap();
            c1_g.extend(c2_g.iter());
            if c1_g.len() == total_boxes {
                return (boxes[i1].0 * boxes[i2].0) as u64
            }
            junction_groups.remove(c2i);
        }
    }
    0
}

fn dist(a: (i64, i64, i64), b: (i64, i64, i64)) -> f64 {
    let d = (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2) + (a.2 - b.2).pow(2);
    (d as f64).sqrt()
}

fn _solve(input: &str, part2: bool) -> Option<u64> {
    let boxes: Vec<_> = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num| num.parse().unwrap())
                .collect::<Vec<i64>>()
        })
        .map(|nums| (nums[0], nums[1], nums[2]))
        .collect();

    let total_boxes = boxes.len();
    let n_connect = if total_boxes < 30 {
        10
    } else {
        1000
    };

    let mut distance_pairs: Vec<(f64, usize, usize)> = Vec::new();
    for i in 0..boxes.len() {
        for j in i + 1..boxes.len() {
            let d = dist(boxes[i], boxes[j]);
            distance_pairs.push((d, i, j));
        }
    }
    // sort descending because we want to use pop to get smallest distances
    distance_pairs.sort_by(|a, b| b.0.total_cmp(&a.0));
    let mut junction_groups: Vec<HashSet<usize>> = (0..total_boxes)
        .map(|i| {
            let mut set = HashSet::new();
            set.insert(i);
            set
        })
        .collect();
    merge_ntimes(n_connect, total_boxes, &boxes, &mut junction_groups, &mut distance_pairs);
    junction_groups.sort_by_key(|c| c.len());
    let res1 = junction_groups.iter().rev().take(3).map(|c| c.len() as u64);
    let res = res1.product();
    if !part2 {
        Some(res)
    } else {
        Some(merge_ntimes(u32::MAX, total_boxes, &boxes, &mut junction_groups, &mut distance_pairs))
    }
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
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
