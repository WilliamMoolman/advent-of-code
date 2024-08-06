use std::{collections::HashMap, hash::Hash};

use itertools::Itertools;

advent_of_code::solution!(22);

struct Tetrimino {
    id: u32,
    cells: Vec<(u64, u64, u64)>,
}

impl Tetrimino {
    fn read_from_input(input: &str) -> Vec<Tetrimino> {
        let mut tetriminos = Vec::new();
        for (i, line) in input.lines().enumerate() {
            let mut cells = Vec::new();
            let (begin, end) = line.split_once('~').unwrap();
            let (start_x, start_y, start_z) = begin
                .split(',')
                .map(|v| v.parse::<u64>().unwrap())
                .collect_tuple()
                .unwrap();
            let (end_x, end_y, end_z) = end
                .split(',')
                .map(|v| v.parse::<u64>().unwrap())
                .collect_tuple()
                .unwrap();
            for x in start_x..=end_x {
                for y in start_y..=end_y {
                    for z in start_z..=end_z {
                        cells.push((x, y, z));
                    }
                }
            }
            tetriminos.push(Tetrimino {
                id: i as u32,
                cells,
            });
        }
        tetriminos
    }
    fn drop_all(tetriminos: &mut Vec<Tetrimino>) {
        let mut moved = true;
        while moved {
            moved = false;
            let mut moved_tetriminos = Vec::new();
            for tetrimino in tetriminos.iter() {
                let mut all_ok = true;

                for cell in &tetrimino.cells {
                    let (x, y, z) = cell;

                    if *z == 1 {
                        all_ok = false;
                        break;
                    }
                    if tetriminos
                        .iter()
                        .filter(|t| t.id != tetrimino.id)
                        .any(|t| t.cells.contains(&(*x, *y, z - 1)))
                    {
                        all_ok = false;
                        break;
                    }
                }

                if all_ok {
                    moved_tetriminos.push(tetrimino.id);
                    moved = true;
                }
            }
            for id in moved_tetriminos {
                tetriminos
                    .iter_mut()
                    .find(|t| t.id == id)
                    .unwrap()
                    .cells
                    .iter_mut()
                    .for_each(|c| c.2 -= 1);
            }
        }
    }
    fn get_supporting(
        tetriminos: &mut Vec<Tetrimino>,
    ) -> (HashMap<u32, Vec<u32>>, HashMap<u32, Vec<u32>>) {
        let mut supporting = HashMap::new();
        let mut supported = HashMap::new();
        for tetrimino in tetriminos.iter() {
            let mut above = Vec::new();
            for cell in &tetrimino.cells {
                let (x, y, z) = cell;
                tetriminos
                    .iter()
                    .filter(|t| t.id != tetrimino.id)
                    .filter(|t| t.cells.contains(&(*x, *y, z + 1)))
                    .for_each(|t: &Tetrimino| above.push(t.id));
            }
            supporting.insert(tetrimino.id, above);
            for id in above {
                supported.entry(id).or_insert(Vec::new()).push(tetrimino.id);
            }
        }

        for tetrimino in tetriminos.iter() {
            if let Some(supporting) = supporting.get(&tetrimino.id) {
                println!("{}: {:?}", tetrimino.id, tetrimino.cells);
                println!("{}: {:?}", tetrimino.id, supporting);
            }
        }
        (supporting, supported)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut tetriminos = Tetrimino::read_from_input(input);
    Tetrimino::drop_all(&mut tetriminos);
    let (supporting, supported) = Tetrimino::get_supporting(&mut tetriminos);

    let removable = tetriminos
        .iter()
        .filter(|t| {
            supporting.get(&t.id).unwrap().iter().all(|supported| {
                supporting
                    .iter()
                    .filter(|(_, v)| v.contains(&supported))
                    .count()
                    > 1
            })
        })
        .count();

    Some(removable as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut tetriminos = Tetrimino::read_from_input(input);
    Tetrimino::drop_all(&mut tetriminos);
    let (supporting, supported) = Tetrimino::get_supporting(&mut tetriminos);

    let chained = tetriminos
        .iter()
        .map(|t| {
            supporting.get(&t.id).unwrap().iter().all(|supported| {
                supporting
                    .iter()
                    .filter(|(_, v)| v.contains(&supported))
                    .count()
                    > 1
            })
        })
        .sum();

    Some(chained as u32)
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
