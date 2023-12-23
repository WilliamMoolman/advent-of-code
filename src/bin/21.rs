use core::fmt::Display;
use std::collections::HashMap;

advent_of_code::solution!(21);

#[derive(Clone)]
struct Board(Vec<Vec<bool>>);

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            for col in row {
                write!(f, "{}", if *col { '0' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Board {
    fn expand(&mut self) {
        println!("Expanding");
        let mut new = Board(vec![vec![false; self.0[0].len() * 3]; self.0.len() * 3]);
        for (y, row) in self.0.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                new.0[self.0.len() + y][self.0[0].len() + x] = *col;
            }
        }
        *self = new;
    }
    fn step(&self, rocks: &Board) -> Board {
        let mut new = Board(vec![vec![false; self.0[0].len()]; self.0.len()]);
        for (y, row) in self.0.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let (nx, ny) = (x as i64 + dx, y as i64 + dy);
                    if nx < 0 || ny < 0 {
                        continue;
                    }
                    let (nx, ny) = (nx as usize, ny as usize);
                    if nx >= row.len() || ny >= self.0.len() {
                        continue;
                    }
                    if *col && !rocks.0[ny][nx] {
                        new.0[ny][nx] = true;
                    }
                }
            }
        }
        new
    }

    fn step_idx(
        &self,
        rocks: &Board,
        board_x: i64,
        board_y: i64,
        boards: &mut HashMap<(i64, i64), Board>,
    ) -> Board {
        let mut new = Board(vec![vec![false; self.0[0].len()]; self.0.len()]);
        for (y, row) in self.0.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    if !*col {
                        continue;
                    }
                    let (nx, ny) = (x as i64 + dx, y as i64 + dy);
                    if nx < 0 {
                        let new_x: usize = (self.0[0].len() as i64 + nx) as usize;
                        let new_y = y;
                        if let Some(board) = boards.get_mut(&(board_x as i64 - 1, board_y as i64)) {
                            board.0[new_y][new_x as usize] = true;
                        } else {
                            let mut board = Board(vec![vec![false; self.0[0].len()]; self.0.len()]);
                            board.0[new_y][new_x as usize] = true;
                            boards.insert((x as i64 - 1, y as i64), board);
                        }
                        continue;
                    }
                    if ny < 0 {
                        let new_x = x;
                        let new_y: usize = (self.0.len() as i64 + ny) as usize;
                        if let Some(board) = boards.get_mut(&(board_x as i64, board_y as i64 - 1)) {
                            board.0[new_y as usize][new_x] = true;
                        } else {
                            let mut board = Board(vec![vec![false; self.0[0].len()]; self.0.len()]);
                            board.0[new_y as usize][new_x] = true;
                            boards.insert((x as i64, y as i64 - 1), board);
                        }
                        continue;
                    }
                    let (nx, ny) = (nx as usize, ny as usize);
                    if nx >= row.len() {
                        let new_x: usize = nx - self.0[0].len();
                        let new_y = y;
                        if let Some(board) = boards.get_mut(&(board_x as i64 + 1, board_y as i64)) {
                            board.0[new_y][new_x as usize] = true;
                        } else {
                            let mut board = Board(vec![vec![false; self.0[0].len()]; self.0.len()]);
                            board.0[new_y][new_x as usize] = true;
                            boards.insert((x as i64 + 1, y as i64), board);
                        }
                        continue;
                    }
                    if ny >= self.0.len() {
                        let new_x = x;
                        let new_y: usize = ny - self.0.len();
                        if let Some(board) = boards.get_mut(&(board_x as i64, board_y as i64 + 1)) {
                            board.0[new_y as usize][new_x] = true;
                        } else {
                            let mut board = Board(vec![vec![false; self.0[0].len()]; self.0.len()]);
                            board.0[new_y as usize][new_x] = true;
                            boards.insert((x as i64, y as i64 + 1), board);
                        }
                        continue;
                    }
                    if *col && !rocks.0[ny][nx] {
                        new.0[ny][nx] = true;
                    }
                }
            }
        }
        new
    }

    fn step_infinite(&self, rocks: &Board) -> Board {
        let mut new = Board(vec![vec![false; self.0[0].len()]; self.0.len()]);
        let mut y_offset: usize = 0;
        let mut x_offset: usize = 0;
        for (y, row) in self.0.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let (mut nx, mut ny) = (
                        x as i64 + dx + x_offset as i64,
                        y as i64 + dy + y_offset as i64,
                    );
                    if !*col {
                        continue;
                    }
                    if nx < 0 || ny < 0 {
                        new.expand();
                        y_offset += self.0.len();
                        x_offset += self.0[0].len();
                        nx += x_offset as i64;
                        ny += y_offset as i64;
                    }
                    let (mut nx, mut ny) = (nx as usize, ny as usize);
                    if nx >= new.0[0].len() || ny >= new.0.len() {
                        new.expand();
                        y_offset += self.0.len();
                        x_offset += self.0[0].len();
                        nx += x_offset;
                        ny += y_offset;
                    }
                    if *col && !rocks.0[ny % rocks.0.len()][nx % rocks.0[0].len()] {
                        new.0[ny][nx] = true;
                    }
                }
            }
        }
        new
    }

    fn count(&self) -> usize {
        self.0.iter().flatten().filter(|&&b| b).count()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let rocks = Board(
        input
            .lines()
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect(),
    );
    let mut steps = Board(
        input
            .lines()
            .map(|line| line.chars().map(|c| c == 'S').collect())
            .collect(),
    );

    let total_steps = if steps.0.len() == 11 { 6 } else { 64 };

    // println!("{}", steps);
    for _ in 0..total_steps {
        steps = steps.step(&rocks);
        // println!("{}", steps);
    }

    Some(steps.count() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let rocks = Board(
        input
            .lines()
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect(),
    );
    let mut steps = Board(
        input
            .lines()
            .map(|line| line.chars().map(|c| c == 'S').collect())
            .collect(),
    );

    let total_steps = if steps.0.len() == 11 { 5000 } else { 26501365 };

    let mut boards = HashMap::new();
    boards.insert((0, 0), steps.clone());
    // println!("{}", steps);
    for i in 0..total_steps {
        println!("{i}: {}", steps.count());
        steps = steps.st(&rocks);
        // println!("{}", steps);
    }

    Some(steps.count() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
