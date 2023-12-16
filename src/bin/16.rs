advent_of_code::solution!(16);

#[derive(Debug)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

impl Direction {
    fn increment(&self, row: usize, col: usize) -> Option<(usize, usize)> {
        match self {
            Up => {
                if row == 0 {
                    None
                } else {
                    Some((row - 1, col))
                }
            }
            Left => {
                if col == 0 {
                    None
                } else {
                    Some((row, col - 1))
                }
            }
            Right => Some((row, col + 1)),
            Down => Some((row + 1, col)),
        }
    }
    fn reflect(&self, mirror: char) -> Direction {
        match (self, mirror) {
            (Up, '/') => Right,
            (Up, '\\') => Left,
            (Left, '/') => Down,
            (Left, '\\') => Up,
            (Right, '/') => Up,
            (Right, '\\') => Down,
            (Down, '/') => Left,
            (Down, '\\') => Right,
            (_, _) => panic!(),
        }
    }
    fn split(&self, splitter: char) -> (Direction, Option<Direction>) {
        match (self, splitter) {
            (Up, '|') => (Up, None),
            (Left, '-') => (Left, None),
            (Right, '-') => (Right, None),
            (Down, '|') => (Down, None),
            (Up, '-') | (Down, '-') => (Left, Some(Right)),
            (Left, '|') | (Right, '|') => (Up, Some(Down)),
            (_, _) => panic!(),
        }
    }
}

use std::collections::VecDeque;

use Direction::*;

#[derive(Clone)]
struct History(bool, bool, bool, bool);

impl History {
    fn new() -> History {
        History(false, false, false, false)
    }
    fn has_had(&self, direction: &Direction) -> bool {
        match direction {
            Up => self.0,
            Left => self.1,
            Right => self.2,
            Down => self.3,
        }
    }
    fn visit(&mut self, direction: &Direction) {
        match direction {
            Up => self.0 = true,
            Left => self.1 = true,
            Right => self.2 = true,
            Down => self.3 = true,
        }
    }
    fn any(&self) -> bool {
        self.0 || self.1 || self.2 || self.3
    }
}

fn lightbeam(
    lightgrid: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<History>>,
    row: usize,
    col: usize,
    direction: Direction,
) {
    let mut queue = VecDeque::new();
    queue.push_back((row, col, direction));
    while !queue.is_empty() {
        let (row, col, direction) = queue.pop_front().unwrap();
        // println!("{row} {col} {direction:?}");
        if row > lightgrid.len() - 1 {
            continue;
        }
        if col > lightgrid[0].len() - 1 {
            continue;
        }
        if visited[row][col].has_had(&direction) {
            continue;
        }
        visited[row][col].visit(&direction);
        match lightgrid[row][col] {
            '.' => {
                if let Some((next_row, next_col)) = direction.increment(row, col) {
                    // println!("PUSHED");
                    queue.push_back((next_row, next_col, direction));
                }
            }
            '/' | '\\' => {
                let new_dir = direction.reflect(lightgrid[row][col]);
                if let Some((next_row, next_col)) = new_dir.increment(row, col) {
                    queue.push_back((next_row, next_col, new_dir));
                }
            }
            '|' | '-' => {
                let (dir_one, dir_two) = direction.split(lightgrid[row][col]);
                if let Some((next_row, next_col)) = dir_one.increment(row, col) {
                    queue.push_back((next_row, next_col, dir_one));
                }
                if let Some(dir_two) = dir_two {
                    if let Some((next_row, next_col)) = dir_two.increment(row, col) {
                        queue.push_back((next_row, next_col, dir_two));
                    }
                }
            }
            _ => panic!(),
        }
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let lightgrid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let total_light = get_lights(&lightgrid, 0, 0, Right);

    Some(total_light)
}

fn get_lights(lightgrid: &Vec<Vec<char>>, row: usize, col: usize, direction: Direction) -> u32 {
    let mut visited = vec![vec![History::new(); lightgrid[0].len()]; lightgrid.len()];
    lightbeam(&lightgrid, &mut visited, row, col, direction);

    let mut total_light = 0;
    for row in visited {
        for c in row {
            if c.any() {
                total_light += 1;
            }
        }
    }
    return total_light;
}

pub fn part_two(input: &str) -> Option<u32> {
    let lightgrid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut lights = Vec::new();
    for c in 0..lightgrid[0].len() {
        lights.push(get_lights(&lightgrid, 0, c, Down));
    }
    for r in 0..lightgrid.len() {
        lights.push(get_lights(&lightgrid, r, 0, Right));
        lights.push(get_lights(&lightgrid, r, lightgrid[0].len() - 1, Left));
    }
    for c in 0..lightgrid[0].len() {
        lights.push(get_lights(&lightgrid, lightgrid.len() - 1, c, Up));
    }

    let max_light = *lights.iter().max().unwrap();
    Some(max_light)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
