use advent_of_code::utils::{Coord, Grid, get_nums_from_line};
use itertools::Itertools;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines().rev().peekable();
    let operands = lines.next().unwrap().split_ascii_whitespace().collect_vec();
    let mut acc = operands
        .iter()
        .map(|c| match c {
            &"+" => 0,
            &"*" => 1,
            _ => panic!(),
        })
        .collect_vec();
    lines.for_each(|line| {
        let nums = get_nums_from_line(line);
        for i in 0..acc.len() {
            match operands[i] {
                "+" => acc[i] += nums[i],
                "*" => acc[i] *= nums[i],
                _ => panic!(),
            }
        }
    });
    Some(acc.iter().sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines().peekable();
    let rest = lines
        .peeking_take_while(|s| !s.starts_with('*') && !s.starts_with('+'))
        .collect_vec();
    let max_len = rest.iter().map(|l| l.len()).max().unwrap();
    let rest = rest
        .iter()
        .map(|l| {
            let mut s = String::from(*l);
            s.push_str(&" ".repeat(max_len - l.len()));
            s
        })
        .join("\n");
    let operands = lines.next().unwrap().split_ascii_whitespace().collect_vec();
    let grid = Grid::from_input(&rest, |c| c);

    let mut acc = operands
        .iter()
        .map(|c| match c {
            &"+" => 0,
            &"*" => 1,
            _ => panic!(),
        })
        .collect_vec();
    let mut column = operands.len() - 1;
    for c in (0..grid.clim()).rev() {
        let n_str = (0..grid.rlim()).map(|r| grid.at(&Coord(r, c))).join("");
        if n_str.trim().is_empty() {
            column -= 1;
            continue;
        }
        let n: u64 = n_str.trim().parse().unwrap();

        match operands[column] {
            "+" => acc[column] += n,
            "*" => acc[column] *= n,
            _ => panic!(),
        }
    }
    Some(acc.iter().sum())
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
