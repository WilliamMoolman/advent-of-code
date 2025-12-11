use std::collections::VecDeque;
use std::sync::atomic::AtomicU64;
use std::u64;

use indicatif::ProgressIterator;
use itertools::Itertools;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use smallvec::{SmallVec, smallvec};

advent_of_code::solution!(10);

struct Puzzle {
    lights: u64,
    buttons: Vec<u64>,
    buttons_full: Vec<Vec<usize>>,
    joltage: Vec<u64>,
}

impl Puzzle {
    fn new(lights: Vec<bool>, buttons: Vec<Vec<usize>>, joltage: Vec<u64>) -> Puzzle {
        // Button finangling
        // 1. Order biggest to smallest
        // 2. Remove duplicates
        //
        let mut buttons = buttons.clone();
        buttons.sort_by_key(|b| -(b.len() as i64));
        buttons = buttons.iter().unique().cloned().collect_vec();
        // println!("{lights:?} {buttons:?}");
        Puzzle {
            lights: lights
                .into_iter()
                .enumerate()
                .map(|(i, l)| if l { 1 << i } else { 0 })
                .sum(),
            buttons: buttons
                .iter()
                .map(|button| button.iter().map(|l| 1 << l).sum())
                .collect_vec(),
            buttons_full: buttons.clone(),
            joltage: joltage.clone(),
        }
    }

    fn solve_lights(&self) -> u64 {
        let mut q = VecDeque::new();
        q.push_back((0, 0));
        while let Some((status, n)) = q.pop_front() {
            if status == self.lights {
                return n;
            }
            for b in &self.buttons {
                q.push_back((status ^ b, n + 1));
            }
        }

        panic!();
    }

    fn solve_joltage_rec(
        &self,
        joltage_field_to_buttons: &Vec<Vec<usize>>,
        minnest_count: &AtomicU64,
        button_max: &Vec<u64>,
        current_jolts: SmallVec<[u64; 10]>,
        count: u64,
        depth: usize,
    ) -> u64 {
        if count > minnest_count.load(std::sync::atomic::Ordering::Relaxed) {
            return u64::MAX;
        }
        // Button by button. depth -> index of button
        if depth >= self.buttons_full.len() {
            return u64::MAX;
        }
        // Check if still solvable
        for i in 0..current_jolts.len() {
            if self.joltage[i] < current_jolts[i] {
                return u64::MAX;
            }
            if self.joltage[i] > current_jolts[i]
                && *joltage_field_to_buttons[i].last().unwrap_or(&0) < depth
            {
                return u64::MAX;
            }
        }
        let mut min_count = u64::MAX;
        'outer: for clicks in 0..=button_max[depth] {
            // Build joltage
            let mut next_jolts = current_jolts.clone();
            for b in &self.buttons_full[depth] {
                next_jolts[*b] += clicks;
            }
            // // Check overshot
            for i in 0..self.joltage.len() {
                if self.joltage[i] < next_jolts[i] {
                    break 'outer;
                }
            }

            // Return if end
            if depth == self.buttons_full.len() - 1 {
                let mut eq = true;
                for i in 0..next_jolts.len() {
                    // last button, actually set min count if possible
                    if self.joltage[i] != next_jolts[i] {
                        eq = false;
                        break;
                    }
                }
                if eq {
                    // println!("{}", count + clicks);
                    minnest_count.fetch_min(count + clicks, std::sync::atomic::Ordering::Relaxed);
                    return count + clicks;
                }
            }

            // If not end, get min
            min_count = min_count.min(self.solve_joltage_rec(
                joltage_field_to_buttons,
                minnest_count,
                button_max,
                next_jolts,
                count + clicks,
                depth + 1,
            ))
        }

        return min_count;
    }

    fn solve_joltage(&self) -> u64 {
        let button_max = self
            .buttons_full
            .iter()
            .map(|button| {
                self.joltage
                    .iter()
                    .enumerate()
                    .filter_map(|(i, j)| if button.contains(&i) { Some(j) } else { None })
                    .max()
                    .unwrap()
                    .clone()
            })
            .collect_vec();

        let joltage_field_to_buttons = (0..self.joltage.len())
            .map(|j| {
                self.buttons_full
                    .iter()
                    .enumerate()
                    .filter_map(move |(bidx, j_idxs)| {
                        if j_idxs.contains(&j) {
                            Some(bidx)
                        } else {
                            None
                        }
                    })
                    .collect_vec()
            })
            .collect_vec();

        // Parallize the work
        let minnest_count: AtomicU64 = AtomicU64::new(u64::MAX);

        let min_count = (0..=button_max[0])
            .into_par_iter()
            .map(|clicks| {
                let mut next_jolts = smallvec![0; self.joltage.len()];
                for b in &self.buttons_full[0] {
                    next_jolts[*b] += clicks;
                }
                self.solve_joltage_rec(
                    &joltage_field_to_buttons,
                    &minnest_count,
                    &button_max,
                    next_jolts,
                    clicks,
                    1,
                )
            })
            .min()
            .unwrap();

        if min_count == u64::MAX {
            panic!("no solution found");
        }
        return min_count;
    }
}
pub fn part_one(input: &str) -> Option<u64> {
    let puzzles = input
        .lines()
        .map(|line| {
            let mut split = line.split(" ");
            let lights = split
                .next()
                .unwrap()
                .chars()
                .filter(|c| *c != '[' && *c != ']')
                .map(|c| c == '#')
                .collect_vec();
            let mut buttons = vec![];
            while let Some(group) = split.next() {
                if group.starts_with('(') {
                    buttons.push(
                        group[1..group.len() - 1]
                            .split(',')
                            .map(|i| i.parse().unwrap())
                            .collect_vec(),
                    )
                }
            }

            Puzzle::new(lights, buttons, vec![])
        })
        .map(|p| p.solve_lights())
        .sum();

    Some(puzzles)
}

pub fn part_two(input: &str) -> Option<u64> {
    let puzzles = input
        .lines()
        .map(|line| {
            let mut split = line.split(" ");
            let _ = split.next();
            let mut buttons = vec![];
            let mut joltage = vec![];
            while let Some(group) = split.next() {
                if group.starts_with('(') {
                    buttons.push(
                        group[1..group.len() - 1]
                            .split(',')
                            .map(|i| i.parse().unwrap())
                            .collect_vec(),
                    )
                }
                joltage = String::from(&group[1..group.len() - 1])
                    .split(',')
                    .map(|i| i.parse::<u64>().unwrap())
                    .collect_vec();
            }

            Puzzle::new(vec![], buttons, joltage)
        })
        .collect_vec()
        .iter()
        .progress()
        .map(|p| p.solve_joltage())
        .sum();
    Some(puzzles)
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
