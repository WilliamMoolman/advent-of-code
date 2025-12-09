use std::{sync::atomic::AtomicU64, vec};

use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let points: Vec<(u64, u64)> = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(',').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect_vec();

    let mut max_size = 0;
    for i in 0..points.len() - 1 {
        for j in i..points.len() {
            let size =
                (points[i].0.abs_diff(points[j].0) + 1) * (points[i].1.abs_diff(points[j].1) + 1);
            if size >= max_size {
                max_size = size;
            }
        }
    }

    Some(max_size)
}

pub fn part_two(input: &str) -> Option<u64> {
    let points: Vec<(u64, u64)> = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(',').unwrap();
            (a.parse::<u64>().unwrap(), b.parse().unwrap())
        })
        .collect_vec();
    let rows = points.iter().map(|p| p.0).max().unwrap() as usize + 1;
    let cols = points.iter().map(|p| p.1).max().unwrap() as usize + 1;
    let mut grid = vec![vec![false; cols]; rows * 2];
    for i in 0..points.len() - 1 {
        let a = points[i];
        let b = points[i + 1];
        if a.0 == b.0 {
            for col in a.1.min(b.1)..=a.1.max(b.1) {
                grid[(a.0 * 2) as usize][col as usize] = true;
            }
        } else {
            for row in (a.0.min(b.0) * 2)..=(a.0.max(b.0) * 2) {
                grid[row as usize][a.1 as usize] = true;
            }
        }
    }
    let a = points[points.len() - 1];
    let b = points[0];
    if a.0 == b.0 {
        for col in a.1.min(b.1)..=a.1.max(b.1) {
            grid[(a.0 * 2) as usize][col as usize] = true;
        }
    } else {
        for row in (a.0.min(b.0) * 2)..=(a.0.max(b.0) * 2) {
            grid[row as usize][a.1 as usize] = true;
        }
    }

    for i in 0..(rows * 2) {
        let mut in_wall = false;
        let mut out = true;
        for j in 0..cols {
            if grid[i][j] {
                in_wall = true;
                continue;
            }
            if in_wall {
                out = !out;
                in_wall = false;
            }
            grid[i][j] = !out;
        }
    }

    let new_grid = grid.chunks(2).map(|c| c[1].clone()).collect_vec();

    println!("Generated grid!");
    let mut combinations = vec![];
    for i in 0..points.len() - 1 {
        for j in i..points.len() {
            combinations.push((i, j));
        }
    }
    let mut max_size = AtomicU64::new(0);
    combinations.par_iter().progress().for_each(|(i, j)| {
        let size =
            (points[*i].0.abs_diff(points[*j].0) + 1) * (points[*i].1.abs_diff(points[*j].1) + 1);
        if size >= max_size.load(std::sync::atomic::Ordering::Relaxed) {
            // Check ok
            let xs = points[*i].0.min(points[*j].0)..=points[*i].0.max(points[*j].0);
            let ys = points[*i].1.min(points[*j].1)..=points[*i].1.max(points[*j].1);
            if xs
                .cartesian_product(ys)
                .all(|(x, y)| new_grid[x as usize][y as usize])
            {
                max_size.fetch_max(size, std::sync::atomic::Ordering::Relaxed);
            }
        }
    });

    Some(*max_size.get_mut())
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
