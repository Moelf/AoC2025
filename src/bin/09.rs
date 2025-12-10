advent_of_code::solution!(9);

fn _sol(input: &str, p2: bool) -> (u64, u64) {
    let xys: Vec<(i64, i64)> = input
        .lines()
        .map(|line| {
            let (xs, ys) = line.split_once(',').unwrap();
            (xs.parse().unwrap(), ys.parse().unwrap())
        })
        .collect();
    let mut largest_rect_area = 0;
    let mut largest_contained_area = 0;
    let mut wrap_range: Vec<usize> = (0..xys.len()).collect();
    wrap_range.push(0);

    for i in 0..xys.len() {
        'outer_loop: for j in i + 1..xys.len() {
            let (mut x, mut y) = xys[i];
            let (mut u, mut v) = xys[j];
            let width = (x - u).abs() + 1;
            let height = (y - v).abs() + 1;
            let area = width * height;
            largest_rect_area = largest_rect_area.max(area as u64);

            // for part 2
            if !p2 || area as u64 <= largest_contained_area {
                continue 'outer_loop;
            }
            // x,y is bottom left
            // u,v is top right
            if x > u {
                (x, u) = (u, x);
            };
            if y > v {
                (y, v) = (v, y);
            };
            for outer in 0..xys.len() {
                let wrap_idx1 = wrap_range[outer];
                let wrap_idx2 = wrap_range[outer + 1];
                let (mut p, mut q) = xys[wrap_idx1];
                let (mut r, mut s) = xys[wrap_idx2];
                // p,q is bottom/left
                // r,s is top/right
                // the polygon segment must be either horizontal or vertical
                if p > r {
                    (p, r) = (r, p);
                };
                if q > s {
                    (q, s) = (s, q);
                };
                // check if polygon segment crosses rectangle
                // p < u: segment left is left of rectangle right edge
                // r > x: segment right is right of rectangle left edge
                //
                // q < v: segment bottom is below rectangle top edge
                // s > y: segment top is above rectangle bottom edge
                if (p < u && r > x) && (q < v && s > y) {
                    continue 'outer_loop;
                }
            }
            largest_contained_area = largest_contained_area.max(area as u64);
        }
    }
    (largest_rect_area, largest_contained_area)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (p1, _) = _sol(input, false);
    Some(p1)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, p2) = _sol(input, true);
    Some(p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
