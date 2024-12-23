use std::collections::HashSet;

use advent_of_code::utils::{Coord, Grid};

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u64> {
    let mut seen = HashSet::<Coord>::new();

    let grid = Grid::from_input(input, |c| c);

    let total: u64 = grid
        .coordinates()
        .iter()
        .map(|(item, coord)| {
            if seen.contains(coord) {
                return 0;
            }
            let mut group = vec![];
            let mut stack = vec![*coord];
            while let Some(c) = stack.pop() {
                if seen.contains(&c) {
                    continue;
                }
                seen.insert(c);
                group.push(c);
                let neighbours = grid.neighbours4(&c);
                for neighbour in neighbours {
                    if !seen.contains(&neighbour) && grid.at(&neighbour) == *item {
                        stack.push(neighbour);
                    }
                }
            }
            let perimeter: u64 = group
                .iter()
                .map(|c| {
                    let neighbours = grid.neighbours4(c);
                    neighbours
                        .iter()
                        .filter(|neigh| grid.at(neigh) != *item)
                        .count() as u64
                        + (if neighbours.len() != 4 {
                            4 - neighbours.len() as u64
                        } else {
                            0
                        })
                })
                .sum::<u64>();
            group.len() as u64 * perimeter
        })
        .sum();

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut seen = HashSet::<Coord>::new();

    let grid = Grid::from_input(input, |c| c);

    let total: u64 = grid
        .coordinates()
        .iter()
        .map(|(item, coord)| {
            if seen.contains(coord) {
                return 0;
            }
            let mut group = vec![];
            let mut stack = vec![*coord];
            while let Some(c) = stack.pop() {
                if seen.contains(&c) {
                    continue;
                }
                seen.insert(c);
                group.push(c);
                let neighbours = grid.neighbours4(&c);
                for neighbour in neighbours {
                    if !seen.contains(&neighbour) && grid.at(&neighbour) == *item {
                        stack.push(neighbour);
                    }
                }
            }
            let mut perimeter = 0;
            // top edge
            for r in 0..grid.rlim() {
                let mut edge = false;
                for c in 0..grid.clim() {
                    let crd = Coord(r, c);
                    let north = grid.n(&crd);
                    if group.contains(&crd) && (north.is_none() || !group.contains(&north.unwrap()))
                    {
                        if !edge {
                            // print!("[N: {r},{c}->");
                        }
                        edge = true;
                    } else {
                        if edge {
                            // print!("{r},{c}]");
                            perimeter += 1;
                        }
                        edge = false;
                    }
                }
                if edge {
                    // print!("{r},X]");
                    perimeter += 1;
                }
            }
            // south edge
            for r in 0..grid.rlim() {
                let mut edge = false;
                for c in 0..grid.clim() {
                    let crd = Coord(r, c);
                    let south = grid.s(&crd);
                    if group.contains(&crd) && (south.is_none() || !group.contains(&south.unwrap()))
                    {
                        if !edge {
                            // print!("[S: {r},{c}->");
                        }
                        edge = true;
                    } else {
                        if edge {
                            perimeter += 1;
                        }
                        edge = false;
                    }
                }
                if edge {
                    // print!("{r},X]");
                    perimeter += 1;
                }
            }
            // east
            for c in 0..grid.clim() {
                let mut edge = false;
                for r in 0..grid.rlim() {
                    let crd = Coord(r, c);
                    let east = grid.e(&crd);
                    if group.contains(&crd) && (east.is_none() || !group.contains(&east.unwrap())) {
                        edge = true;
                    } else {
                        if edge {
                            perimeter += 1;
                        }
                        edge = false;
                    }
                }
                if edge {
                    // print!("{r},X]");
                    perimeter += 1;
                }
            }
            // west edge
            for c in 0..grid.clim() {
                let mut edge = false;
                for r in 0..grid.rlim() {
                    let crd = Coord(r, c);
                    let west = grid.w(&crd);
                    if group.contains(&crd) && (west.is_none() || !group.contains(&west.unwrap())) {
                        edge = true;
                    } else {
                        if edge {
                            perimeter += 1;
                        }
                        edge = false;
                    }
                }
                if edge {
                    // print!("{r},X]");
                    perimeter += 1;
                }
            }
            // println!(
            //     "{item}: {} x {} = {}",
            //     group.len(),
            //     perimeter,
            //     group.len() as u64 * perimeter
            // );
            group.len() as u64 * perimeter
        })
        .sum();

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
