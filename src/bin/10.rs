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

struct Pos<const STEP: usize>(usize, usize, usize, usize);

impl<const STEP: usize> Pos<STEP> {
    fn next(&mut self, next_move: char) {
        match next_move {
            'N' => self.n(),
            'S' => self.s(),
            'E' => self.e(),
            'W' => self.w(),
            _ => panic!("No such move!"),
        };
    }

    fn char_at(&self, board: &Vec<Vec<char>>) -> char {
        board[self.0][self.1]
    }

    fn n(&mut self) -> bool {
        if self.0 >= STEP {
            self.0 -= STEP;
            true
        } else {
            false
        }
    }
    fn s(&mut self) -> bool {
        if self.0 < self.3 - STEP {
            self.0 += STEP;
            true
        } else {
            false
        }
    }
    fn e(&mut self) -> bool {
        if self.1 < self.3 - STEP {
            self.1 += STEP;
            true
        } else {
            false
        }
    }
    fn w(&mut self) -> bool {
        if self.1 >= STEP {
            self.1 -= STEP;
            true
        } else {
            false
        }
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
    let mut start = Pos::<1>(start.0, start.1, board.len(), board[0].len());
    let mut count = 0;

    loop {
        start.next(next_step);
        count += 1;
        // println!("{}, {}", next_step, start.char_at(board));
        next_step = match (next_step, start.char_at(board)) {
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
            (_, 'S') => return count,
            (_, _) => panic!("Weird step"),
        };
    }
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
2. perform fill on edges --> does not work!
3. cound non zeroed chars */
fn walk_board_fill(board: &mut Vec<Vec<char>>) -> u32 {
    let start = get_start(board);
    let mut next_step = get_start_surrounding(board, start, 2);
    let mut start = Pos::<2>(start.0, start.1, board.len(), board[0].len());
    let mut count = 0;

    match next_step {
        'N' => board[start.0 - 1][start.1] = '*',
        'E' => board[start.0][start.1 + 1] = '*',
        'W' => board[start.0][start.1 - 1] = '*',
        'S' => board[start.0 + 1][start.1] = '*',
        _ => (),
    };

    loop {
        board[start.0][start.1] = '*';
        start.next(next_step);
        count += 1;
        // println!(
        //     "{}, {} ({},{})",
        //     next_step,
        //     start.char_at(board),
        //     start.0,
        //     start.1
        // );
        next_step = match (next_step, start.char_at(board)) {
            ('N', '|') => {
                board[start.0 - 1][start.1] = '*';
                'N'
            }
            ('N', '7') => {
                board[start.0][start.1 - 1] = '*';
                'W'
            }
            ('N', 'F') => {
                board[start.0][start.1 + 1] = '*';
                'E'
            }
            ('E', 'J') => {
                board[start.0 - 1][start.1] = '*';
                'N'
            }
            ('E', '-') => {
                board[start.0][start.1 + 1] = '*';
                'E'
            }
            ('E', '7') => {
                board[start.0 + 1][start.1] = '*';
                'S'
            }
            ('W', '-') => {
                board[start.0][start.1 - 1] = '*';
                'W'
            }
            ('W', 'F') => {
                board[start.0 + 1][start.1] = '*';
                'S'
            }
            ('W', 'L') => {
                board[start.0 - 1][start.1] = '*';
                'N'
            }
            ('S', '|') => {
                board[start.0 + 1][start.1] = '*';
                'S'
            }
            ('S', 'J') => {
                board[start.0][start.1 - 1] = '*';
                'W'
            }
            ('S', 'L') => {
                board[start.0][start.1 + 1] = '*';
                'E'
            }
            // (_, 'S') => return count,
            (_, '*') => return count,
            (_, _) => panic!("Weird step"),
        };
    }
}

fn flood_fill(board: &mut Vec<Vec<char>>, x: usize, y: usize, x_lim: usize, y_lim: usize) {
    if board[x][y] == 'O' || board[x][y] == '*' {
        return;
    }
    board[x][y] = 'O';
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
            if c != ' ' && c != '*' && c != 'O' {
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
                line.chars().flat_map(|c| [c, ' ']).collect::<Vec<char>>(),
                repeat(' ').take(line.len() * 2).collect(),
            ]
        })
        .collect::<Vec<Vec<char>>>();
    // print_board(&board);
    walk_board_fill(&mut board);
    // println!();
    // println!();
    // print_board(&board);
    fill_outside(&mut board);
    // println!();
    // println!();
    // print_board(&board);

    Some(count_inside(&board))
}

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
