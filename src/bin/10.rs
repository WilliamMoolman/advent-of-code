use std::iter::repeat;

advent_of_code::solution!(10);

/*
| is a vertical pipe connecting north and south.
- is a horizontal pipe connecting east and west.
L is a 90-degree bend connecting north and east.
J is a 90-degree bend connecting north and west.
7 is a 90-degree bend connecting south and west.
F is a 90-degree bend connecting south and east.
. is ground; there is no pipe in this tile.
S is the starting position of the animal;
*/

struct Pos<const STEP: usize>(usize, usize);

impl<const STEP: usize> Pos<STEP> {
    fn next(&mut self, step: char, board: &Vec<Vec<char>>) -> Option<char> {
        match step {
            'N' => self.n(),
            'S' => self.s(),
            'E' => self.e(),
            'W' => self.w(),
            _ => panic!("No such move!"),
        };
        let next_step = match (step, self.char_at(board)) {
            ('N', '|') => 'N',
            ('N', '7') => 'W',
            ('N', 'F') => 'E',
            ('E', 'J') => 'N',
            ('E', '-') => 'E',
            ('E', '7') => 'S',
            ('W', '-') => 'W',
            ('W', 'F') => 'S',
            ('W', 'L') => 'N',
            ('S', '|') => 'S',
            ('S', 'J') => 'W',
            ('S', 'L') => 'E',
            (_, 'S') => return None,
            (_, '*') => return None,
            (_, _) => panic!("Weird step"),
        };
        Some(next_step)
    }

    fn char_at(&self, board: &Vec<Vec<char>>) -> char {
        board[self.0][self.1]
    }

    fn n(&mut self) {
        self.0 -= STEP;
    }
    fn s(&mut self) {
        self.0 += STEP;
    }
    fn e(&mut self) {
        self.1 += STEP;
    }
    fn w(&mut self) {
        self.1 -= STEP;
    }
}

fn get_start(board: &Vec<Vec<char>>) -> (usize, usize) {
    for (i, line) in board.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == 'S' {
                return (i, j);
            }
        }
    }
    panic!("No S on board!");
}

fn get_start_surrounding(board: &Vec<Vec<char>>, start: (usize, usize), step_size: usize) -> char {
    // Not stable
    if board[start.0 - step_size][start.1] == '|'
        || board[start.0 - step_size][start.1] == '7'
        || board[start.0 - step_size][start.1] == 'F'
    {
        'N'
    } else if board[start.0][start.1 - step_size] == '-'
        || board[start.0][start.1 - step_size] == 'F'
        || board[start.0][start.1 - step_size] == 'L'
    {
        'W'
    } else if board[start.0 + step_size][start.1] == '|'
        || board[start.0 + step_size][start.1] == 'J'
        || board[start.0 + step_size][start.1] == 'L'
    {
        'S'
    } else if board[start.0][start.1 + step_size] == '-'
        || board[start.0][start.1 + step_size] == '7'
        || board[start.0][start.1 + step_size] == 'J'
    {
        'E'
    } else {
        panic!("No surroundings!");
    }
}

fn walk_board(board: &Vec<Vec<char>>) -> u32 {
    let start = get_start(board);
    let mut next_step = get_start_surrounding(board, start, 1);
    let mut pos = Pos::<1>(start.0, start.1);
    let mut count = 0;

    while let Some(_next_step) = pos.next(next_step, board) {
        next_step = _next_step;
        count += 1;
    }
    return count + 1;
}

pub fn part_one(input: &str) -> Option<u32> {
    let board = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    Some(walk_board(&board) / 2)
}

/*
1. expand board to allow squeezing
1. mask out path
2. perform fill on edges
3. count non filled/path/padded chars */

fn interpolate(board: &mut Vec<Vec<char>>, step: char, pos: &Pos<2>) {
    match step {
        'N' => board[pos.0 - 1][pos.1] = '*',
        'E' => board[pos.0][pos.1 + 1] = '*',
        'W' => board[pos.0][pos.1 - 1] = '*',
        'S' => board[pos.0 + 1][pos.1] = '*',
        _ => (),
    };
}
fn walk_board_fill(board: &mut Vec<Vec<char>>) -> u32 {
    let start = get_start(board);
    let mut step = get_start_surrounding(board, start, 2);
    let mut pos = Pos::<2>(start.0, start.1);
    let mut count = 0;
    let mut prev_pos = start;

    interpolate(board, step, &pos);

    while let Some(next_step) = pos.next(step, board) {
        board[prev_pos.0][prev_pos.1] = '*';
        prev_pos = (pos.0, pos.1);
        step = next_step;
        interpolate(board, next_step, &pos);
        count += 1;
    }
    board[prev_pos.0][prev_pos.1] = '*';
    return count + 1;
}

fn flood_fill(board: &mut Vec<Vec<char>>, x: usize, y: usize, x_lim: usize, y_lim: usize) {
    if board[x][y] == '*' || board[x][y] == ' ' {
        return;
    }
    board[x][y] = ' ';
    if x > 0 {
        flood_fill(board, x - 1, y, x_lim, y_lim);
    }
    if x < x_lim - 1 {
        flood_fill(board, x + 1, y, x_lim, y_lim);
    }
    if y > 0 {
        flood_fill(board, x, y - 1, x_lim, y_lim);
    }
    if y < y_lim - 1 {
        flood_fill(board, x, y + 1, x_lim, y_lim);
    }
}
fn fill_outside(board: &mut Vec<Vec<char>>) {
    let x_lim = board.len();
    let y_lim = board[0].len();
    for y in 0..y_lim {
        flood_fill(board, 0, y, x_lim, y_lim); // Top
        flood_fill(board, x_lim - 1, y, x_lim, y_lim); // Bottom
    }
    for x in 0..x_lim {
        flood_fill(board, x, 0, x_lim, y_lim); // Left
        flood_fill(board, x, y_lim - 1, x_lim, y_lim); //Right
    }
}

fn count_inside(board: &Vec<Vec<char>>) -> u32 {
    let mut count = 0;
    for line in board.iter() {
        for &c in line {
            if c != '~' && c != '*' && c != ' ' {
                count += 1;
            }
        }
    }
    count
}
pub fn part_two(input: &str) -> Option<u32> {
    let mut board = input
        .lines()
        .flat_map(|line| {
            [
                line.chars().flat_map(|c| [c, '~']).collect::<Vec<char>>(),
                repeat('~').take(line.len() * 2).collect(),
            ]
        })
        .collect::<Vec<Vec<char>>>();

    walk_board_fill(&mut board);

    fill_outside(&mut board);

    Some(count_inside(&board))
}

#[allow(dead_code)]
fn print_board(board: &Vec<Vec<char>>) {
    for line in board.iter() {
        println!("{}", line.iter().collect::<String>());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
