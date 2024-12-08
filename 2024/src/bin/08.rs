use std::collections::HashMap;

use advent_of_code::utils::LinesExt;

use itertools::Itertools;
use num::Integer;

advent_of_code::solution!(8);

struct World {
    antennae: HashMap<char, Vec<(usize, usize)>>,
    anti: Vec<Vec<bool>>,
}

impl World {
    fn build_antennae(map: &Vec<Vec<char>>) -> HashMap<char, Vec<(usize, usize)>> {
        let mut antennae: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
        map.iter().enumerate().for_each(|(r, row)| {
            row.iter().enumerate().for_each(|(c, freq)| {
                if let Some(positions) = antennae.get_mut(freq) {
                    positions.push((r, c));
                } else {
                    if freq != &'.' {
                        antennae.insert(*freq, vec![(r, c)]);
                    }
                }
            })
        });

        antennae
    }

    fn resonant(map: &Vec<Vec<char>>) -> World {
        let antennae = World::build_antennae(map);

        let r_bound = map.len();
        let c_bound = map[0].len();

        let mut anti = vec![vec![false; c_bound]; r_bound];

        antennae.iter().for_each(|(_freq, positions)| {
            positions.iter().combinations(2).for_each(|perm| {
                let a = perm[0];
                let ar = a.0 as i64;
                let ac = a.1 as i64;

                let b = perm[1];
                let br = b.0 as i64;
                let bc = b.1 as i64;

                let cr = br - ar;
                let cc = bc - ac;

                let x = (ar - cr, ac - cc);
                let y = (br + cr, bc + cc);

                for (r, c) in [x, y] {
                    if r >= 0 && r < r_bound as i64 && c >= 0 && c < c_bound as i64 {
                        anti[r as usize][c as usize] = true;
                    }
                }
            });
        });

        World { antennae, anti }
    }

    fn harmonics(map: &Vec<Vec<char>>) -> World {
        let antennae = World::build_antennae(map);

        let r_bound = map.len();
        let c_bound = map[0].len();

        let mut anti = vec![vec![false; c_bound]; r_bound];

        antennae.iter().for_each(|(_freq, positions)| {
            positions.iter().combinations(2).for_each(|perm| {
                let a = perm[0];
                let ar = a.0 as i64;
                let ac = a.1 as i64;

                let b = perm[1];
                let br = b.0 as i64;
                let bc = b.1 as i64;

                let cr = br - ar;
                let cc = bc - ac;

                let crn = cr / cr.gcd(&cc);
                let ccn = cc / cr.gcd(&cc);

                let mut r = br;
                let mut c = bc;
                while r >= 0 && r < r_bound as i64 && c >= 0 && c < c_bound as i64 {
                    anti[r as usize][c as usize] = true;
                    r += crn;
                    c += ccn;
                }
                let mut r = br;
                let mut c = bc;
                while r >= 0 && r < r_bound as i64 && c >= 0 && c < c_bound as i64 {
                    anti[r as usize][c as usize] = true;
                    r -= crn;
                    c -= ccn;
                }
            });
        });

        World { antennae, anti }
    }

    fn num_anti(&self) -> u64 {
        self.anti
            .iter()
            .map(|row| row.iter().filter(|x| **x).count() as u64)
            .sum()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let map = input.lines().to_char_grid();

    let world = World::resonant(&map);

    Some(world.num_anti())
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = input.lines().to_char_grid();

    let world = World::harmonics(&map);

    Some(world.num_anti())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
