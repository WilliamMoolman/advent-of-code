use advent_of_code::utils::{Coord, Grid};
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let grid = Grid::from_input(input, |c| c);
    let count = grid
        .coordinates()
        .par_iter()
        .filter(|(item, c)| {
            *item == '@'
                && grid
                    .neighbours8(c)
                    .iter()
                    .filter(|c| grid.at(c) == '@')
                    .count()
                    < 4
        })
        .count();
    Some(count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = Grid::from_input(input, |c| c);
    let grid_items = grid
        .coordinates_iter()
        .map(|c| grid.neighbours8_sat(&c.1, |item| item == '@'))
        .collect_vec();
    let mut grid_with_nums = Grid {
        grid: grid_items,
        rows: grid.rows,
        cols: grid.cols,
    };
    let mut total_removed = 0;
    let mut to_go: Vec<Coord> = grid
        .coordinates_iter()
        .filter(|c| c.0 == '@')
        .map(|c| c.1)
        .collect();
    while let Some(coord) = to_go.pop() {
        if grid_with_nums.at(&coord) >= 4 || grid.at(&coord) != '@' {
            continue;
        }
        total_removed += 1;
        grid.set(&coord, '.');
        for n in grid.neighbours8_slice(&coord) {
            if let Some(c) = n {
                grid_with_nums.set(&c, grid_with_nums.at(&c) - 1);
                to_go.push(c);
            }
        }
    }
    Some(total_removed as u64)
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
