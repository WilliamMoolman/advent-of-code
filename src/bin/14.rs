use std::collections::HashMap;

advent_of_code::solution!(14);

struct Grid(Vec<Vec<char>>);

impl Grid {
    fn roll_west(&mut self) {
        for row in self.0.iter_mut() {
            let mut start = 0;
            let mut rocks = 0;
            for i in 0..row.len() {
                match row[i] {
                    'O' => rocks += 1,
                    '.' => (),
                    '#' => {
                        for j in start..start + rocks {
                            row[j] = 'O';
                        }
                        for j in start + rocks..i {
                            row[j] = '.'
                        }
                        rocks = 0;
                        start = i + 1;
                    }
                    _ => panic!("Weird rock"),
                }
            }
            for j in start..start + rocks {
                row[j] = 'O';
            }
            for j in start + rocks..row.len() {
                row[j] = '.'
            }
        }
    }
    fn roll_east(&mut self) {
        let n = self.0[0].len() - 1;
        for row in self.0.iter_mut() {
            let mut start = 0;
            let mut rocks = 0;
            for i in 0..row.len() {
                match row[n - i] {
                    'O' => rocks += 1,
                    '.' => (),
                    '#' => {
                        for j in start..start + rocks {
                            row[n - j] = 'O';
                        }
                        for j in start + rocks..i {
                            row[n - j] = '.'
                        }
                        rocks = 0;
                        start = i + 1;
                    }
                    _ => panic!("Weird rock"),
                }
            }
            for j in start..start + rocks {
                row[n - j] = 'O';
            }
            for j in start + rocks..row.len() {
                row[n - j] = '.'
            }
        }
    }
    fn roll_north(&mut self) {
        for row_idx in 0..self.0[0].len() {
            let mut start = 0;
            let mut rocks = 0;
            for i in 0..self.0.len() {
                match self.0[i][row_idx] {
                    'O' => rocks += 1,
                    '.' => (),
                    '#' => {
                        for j in start..start + rocks {
                            self.0[j][row_idx] = 'O';
                        }
                        for j in start + rocks..i {
                            self.0[j][row_idx] = '.'
                        }
                        rocks = 0;
                        start = i + 1;
                    }
                    _ => panic!("Weird rock"),
                }
            }
            for j in start..start + rocks {
                self.0[j][row_idx] = 'O';
            }
            for j in start + rocks..self.0.len() {
                self.0[j][row_idx] = '.'
            }
        }
    }
    fn roll_south(&mut self) {
        let n = self.0.len() - 1;
        for row_idx in 0..self.0[0].len() {
            let mut start = 0;
            let mut rocks = 0;
            for i in 0..self.0.len() {
                match self.0[n - i][row_idx] {
                    'O' => rocks += 1,
                    '.' => (),
                    '#' => {
                        for j in start..start + rocks {
                            self.0[n - j][row_idx] = 'O';
                        }
                        for j in start + rocks..i {
                            self.0[n - j][row_idx] = '.'
                        }
                        rocks = 0;
                        start = i + 1;
                    }
                    _ => panic!("Weird rock"),
                }
            }
            for j in start..start + rocks {
                self.0[n - j][row_idx] = 'O';
            }
            for j in start + rocks..self.0.len() {
                self.0[n - j][row_idx] = '.'
            }
        }
    }

    fn score(&self) -> usize {
        let row_len = self.0.len();
        self.0
            .iter()
            .enumerate()
            .map(|(i, row)| (row_len - i) * row.iter().filter(|&&c| c == 'O').count())
            .sum()
    }

    fn cycle(&mut self) {
        self.roll_north();
        self.roll_west();
        self.roll_south();
        self.roll_east();
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let north_south = input.lines().map(|line| line.chars().collect()).collect();

    let mut grid = Grid(north_south);

    grid.roll_north();

    Some(grid.score())
}

pub fn part_two(input: &str) -> Option<usize> {
    let north_south = input.lines().map(|line| line.chars().collect()).collect();

    let mut grid = Grid(north_south);
    let mut cycles: HashMap<Vec<Vec<char>>, usize> = HashMap::new();
    let mut i = 0;
    let num_cycles = 1_000_000_000;
    // println!("0: {}", grid.score());
    while i < num_cycles {
        if cycles.contains_key(&grid.0) {
            let key = cycles.get(&grid.0).unwrap();
            let n = i - key;
            // println!("Stable configuration found! @ {key}->{i} ({n})! Fast forwarding...");
            cycles.clear();
            // println!("{i} -> {}", num_cycles - (num_cycles - i) % n);
            i = num_cycles - (num_cycles - i) % n;
        } else {
            cycles.insert(grid.0.clone(), i);
            grid.cycle();
            i += 1;
        }
        // println!("{i}: {}", grid.score());
    }
    Some(grid.score())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
