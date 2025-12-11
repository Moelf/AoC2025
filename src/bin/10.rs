advent_of_code::solution!(10);

use z3::{Config, Context, Optimize, SatResult, ast::Int};

fn press_button(target: &mut [bool], button: &Vec<usize>) {
    for &index in button {
        target[index] = !target[index];
    }
}

fn bfs(target: Vec<bool>, buttons: Vec<Vec<usize>>) -> u64 {
    use std::collections::{HashSet, VecDeque};

    let mut queue: VecDeque<(Vec<bool>, u64)> = VecDeque::new();
    let mut visited: HashSet<Vec<bool>> = HashSet::new();

    let initial_state = vec![false; target.len()];
    queue.push_back((initial_state.clone(), 0));
    visited.insert(initial_state);

    while let Some((current_state, steps)) = queue.pop_front() {
        if current_state == target {
            return steps;
        }

        for button in &buttons {
            let mut next_state = current_state.clone();
            press_button(&mut next_state, button);
            if !visited.contains(&next_state) {
                visited.insert(next_state.clone());
                queue.push_back((next_state, steps + 1));
            }
        }
    }

    u64::MAX // If no solution is found
}

pub fn part_one(input: &str) -> Option<u64> {
    // each line looks like
    // [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    // we want to make an boolean array of length 4 to represent the states
    // the target state is [false, true, true, false] because # means light on
    // (3) means toggle light at index 3
    // (1,3) means toggle lights at index 0 and 3

    let mut targets: Vec<Vec<bool>> = Vec::new();
    let mut buttons: Vec<Vec<Vec<usize>>> = Vec::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        let state_str = parts[0];
        let mut target: Vec<bool> = Vec::new();
        for c in state_str.chars() {
            match c {
                '#' => target.push(true),
                '.' => target.push(false),
                _ => (),
            }
        }
        targets.push(target);

        let mut button_list: Vec<Vec<usize>> = Vec::new();
        for part in &parts[1..parts.len() - 1] {
            let indices_str = part.trim_matches(|c| c == '(' || c == ')');
            let indices: Vec<usize> = indices_str
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            button_list.push(indices);
        }
        buttons.push(button_list);
    }

    let mut res = 0;
    for (i, target) in targets.iter().enumerate() {
        let button_list = &buttons[i];
        res += bfs(target.clone(), button_list.clone());
    }
    Some(res)
}

fn z3solve(buttons: Vec<Vec<usize>>, joltages: Vec<u64>) -> u64 {
    let opt = Optimize::new();

    let presses: Vec<Int> = (0..buttons.len())
        .map(|i| Int::new_const(format!("press{}", i)))
        .collect();

    for press in &presses {
        opt.assert(&press.ge(Int::from_i64(0)));
    }

    for (i, joltage) in joltages.iter().enumerate() {
        let mut terms: Vec<Int> = Vec::new();
        for (j, button) in buttons.iter().enumerate() {
            if button.contains(&i) {
                terms.push(presses[j].clone());
            }
        }

        // sum the terms; if empty, sum is 0
        let sum_expr = if terms.is_empty() {
            Int::from_u64(0)
        } else {
            terms.into_iter().reduce(|acc, v| acc+v).unwrap()
        };

        let rhs = Int::from_u64(*joltage);
        opt.assert(&sum_expr.eq(&rhs));
    }

    let sum_presses = if presses.is_empty() {
        Int::from_i64(0)
    } else {
        presses
            .iter()
            .cloned()
            .reduce(|acc, v| acc + v)
            .unwrap()
    };
    opt.minimize(&sum_presses);

    let mut part2: u64 = 0;
    match opt.check(&[]) {
        SatResult::Sat => {
            let model = opt.get_model().expect("model should exist");

            for press in &presses {
                let val = model
                    .get_const_interp(press)
                    .expect("press must be in model")
                    .as_i64()
                    .expect("press is an Int");
                part2 += val as u64;
            };
        }
        SatResult::Unsat => panic!("unsatisfiable"),
        SatResult::Unknown => panic!("solver returned unknown"),
    }
    part2
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut targets: Vec<Vec<u64>> = Vec::new();
    let mut buttons: Vec<Vec<Vec<usize>>> = Vec::new();
    // [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    // now we care about the numbers in {} as target
    for line in input.lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        let state_str = parts[parts.len() - 1];
        let mut target: Vec<u64> = Vec::new();
        let numbers_str = state_str.trim_matches(|c| c == '{' || c == '}');
        for num_str in numbers_str.split(',') {
            let num = num_str.parse::<u64>().unwrap();
            target.push(num);
        }
        targets.push(target);

        let mut button_list: Vec<Vec<usize>> = Vec::new();
        for part in &parts[1..parts.len() - 1] {
            let indices_str = part.trim_matches(|c| c == '(' || c == ')');
            let indices: Vec<usize> = indices_str
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            button_list.push(indices);
        }
        buttons.push(button_list);
    }

    let mut res = 0;
    for (i, target) in targets.iter().enumerate() {
        res += z3solve(buttons[i].clone(), target.clone());
    }
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
