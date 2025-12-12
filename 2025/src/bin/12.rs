use std::fmt::Debug;

use indicatif::ProgressIterator;
use itertools::Itertools;

advent_of_code::solution!(12);

struct Piece {
    // TODO: remove duplicates?
    shapes: [[[bool; 3]; 3]; 4],
}

impl Debug for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Piece:\n")?;
        for shape in self.shapes {
            for row in shape {
                f.write_fmt(format_args!(
                    "{}\n",
                    row.map(|b| if b { '#' } else { '.' })
                        .iter()
                        .collect::<String>()
                ))?;
            }
            f.write_str("------\n")?;
        }

        Ok(())
    }
}

impl Piece {
    fn rotate(shape: [[bool; 3]; 3]) -> [[bool; 3]; 3] {
        [
            [shape[2][0], shape[1][0], shape[0][0]],
            [shape[2][1], shape[1][1], shape[0][1]],
            [shape[2][2], shape[1][2], shape[0][2]],
        ]
    }
    fn new(shape: [[bool; 3]; 3]) -> Piece {
        let shape1 = Piece::rotate(shape);
        let shape2 = Piece::rotate(shape1);
        let shape3 = Piece::rotate(shape2);
        Piece {
            shapes: [shape, shape1, shape2, shape3],
        }
    }

    fn squares(&self) -> usize {
        self.shapes[0]
            .iter()
            .flat_map(|s| s)
            .filter(|b| **b)
            .count()
    }
}
#[derive(Debug)]
struct Puzzle<'a> {
    width: usize,
    height: usize,
    piece_counts: Vec<usize>,
    pieces: &'a Vec<Piece>,
}

impl<'a> Puzzle<'a> {
    fn fit(
        &self,
        board: &Vec<Vec<u8>>,
        shape: &[[bool; 3]; 3],
        shape_idx: u8,
        r: usize,
        c: usize,
    ) -> Option<Vec<Vec<u8>>> {
        // Needed to pack tight. No need for upshift, guaranteed that previous rows filled
        let l_shift = match shape[0] {
            [true, _, _] => 0,
            [false, true, _] => 1,
            [false, false, true] => 2,
            [false, false, false] => panic!("Shouldn't have empty row!"),
        };
        if l_shift > c {
            return None;
        }
        if self.width - c + l_shift < 3 || self.height - r < 3 {
            return None;
        }
        for r_idx in 0..3 {
            for c_idx in 0..3 {
                if !shape[r_idx][c_idx] {
                    continue;
                }
                if board[r + r_idx][c + c_idx - l_shift] > 0 {
                    return None;
                }
            }
        }

        let mut board_next = board.clone();
        for r_idx in 0..3 {
            for c_idx in 0..3 {
                if !shape[r_idx][c_idx] {
                    continue;
                }
                board_next[r + r_idx][c + c_idx - l_shift] = shape_idx;
            }
        }
        Some(board_next)
    }

    fn solve(&self) -> bool {
        let diff = (self.width * self.height) as i64
            - self
                .pieces
                .iter()
                .zip(&self.piece_counts)
                .map(|(p, c)| (p.squares() * c) as i64)
                .sum::<i64>();
        if diff < 0 {
            return false;
        }

        let board = vec![vec![0; self.width]; self.height];

        self.solve_rec(diff as usize, &self.piece_counts, &board)
    }

    fn solve_rec(&self, diff: usize, piece_counts: &Vec<usize>, board: &Vec<Vec<u8>>) -> bool {
        let next_spot = board
            .iter()
            .enumerate()
            .filter_map(|(r, row)| {
                if let Some((c, _)) = row
                    .iter()
                    .enumerate()
                    .filter(|(_c, item)| **item == 0)
                    .next()
                {
                    Some((r, c))
                } else {
                    None
                }
            })
            .next();

        // Used up all pieces
        if piece_counts.iter().all(|c| *c == 0) {
            return true;
        }

        // Board is filled, still pieces
        if next_spot.is_none() {
            return false;
        }

        let (next_r, next_c) = next_spot.unwrap();

        // Place next piece (if possible)
        for (p_idx, count) in piece_counts.iter().enumerate() {
            if *count == 0 {
                continue;
            }
            for rotation in &self.pieces[p_idx].shapes {
                if let Some(board_next) = self.fit(board, rotation, p_idx as u8 + 1, next_r, next_c)
                {
                    let mut piece_counts_next = piece_counts.clone();
                    piece_counts_next[p_idx] -= 1;
                    if self.solve_rec(diff, &piece_counts_next, &board_next) {
                        return true;
                    }
                }
            }
        }
        if diff > 0 {
            let mut board_next = board.clone();
            board_next[next_r][next_c] = 9;
            return self.solve_rec(diff - 1, piece_counts, &board_next);
        }

        false
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut input = input.lines().peekable();
    let pieces = input
        .peeking_take_while(|line| !line.contains('x'))
        .chunks(5)
        .into_iter()
        .map(|mut chunk| {
            chunk.next();
            Piece::new([
                *chunk
                    .next()
                    .unwrap()
                    .chars()
                    .map(|c| c == '#')
                    .collect_vec()
                    .first_chunk::<3>()
                    .unwrap(),
                *chunk
                    .next()
                    .unwrap()
                    .chars()
                    .map(|c| c == '#')
                    .collect_vec()
                    .first_chunk::<3>()
                    .unwrap(),
                *chunk
                    .next()
                    .unwrap()
                    .chars()
                    .map(|c| c == '#')
                    .collect_vec()
                    .first_chunk::<3>()
                    .unwrap(),
            ])
        })
        .collect_vec();

    let places = input
        .map(|line| {
            let (space_str, rest) = line.split_once(": ").unwrap();

            let (w, h) = space_str.split_once('x').unwrap();
            let idxs = rest
                .split(' ')
                .map(|idx| idx.parse::<usize>().unwrap())
                .collect_vec();
            Puzzle {
                width: w.parse().unwrap(),
                height: h.parse().unwrap(),
                piece_counts: idxs,
                pieces: &pieces,
            }
        })
        .collect_vec();

    Some(
        places
            .iter()
            .progress()
            .filter_map(|p| if p.solve() { Some(()) } else { None })
            .count() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
