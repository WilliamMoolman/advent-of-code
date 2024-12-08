advent_of_code::solution!(6);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    OutBounds,
    Blocked,
    Visited(usize),
    Empty,
}
use std::collections::HashSet;

use Tile::*;

impl Tile {
    fn from_char(c: char) -> Tile {
        match c {
            '.' => Empty,
            '^' => Visited(0),
            '#' => Blocked,
            _ => panic!(),
        }
    }
}

#[derive(Clone)]
struct Guard {
    r: usize,
    c: usize,   // floor[x][y]
    dir: usize, // 0 N, 1 E, 2 S, 3 W
}

impl Guard {
    fn next_r(&self) -> usize {
        match self.dir {
            0 => self.r - 1,
            2 => self.r + 1,
            _ => self.r,
        }
    }
    fn next_c(&self) -> usize {
        match self.dir {
            3 => self.c - 1,
            1 => self.c + 1,
            _ => self.c,
        }
    }
    fn rotate(&mut self) {
        self.dir = (self.dir + 1) % 4;
    }
    fn walk(&mut self) {
        self.r = self.next_r();
        self.c = self.next_c();
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let raw: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut floor = vec![vec![OutBounds; raw[0].len() + 2]];
    floor.extend(raw.iter().map(|row| {
        let mut nr = vec![OutBounds];
        nr.extend(row.iter().map(|&c| Tile::from_char(c)));
        nr.push(OutBounds);
        nr
    }));
    floor.push(vec![OutBounds; raw[0].len() + 2]);

    let mut guard = Guard { r: 0, c: 0, dir: 0 };
    'outer: for (r, row) in floor.iter().enumerate() {
        for (c, &tile) in row.iter().enumerate() {
            if tile == Visited(0) {
                guard.r = r;
                guard.c = c;
                break 'outer;
            }
        }
    }

    loop {
        if floor[guard.next_r()][guard.next_c()] == OutBounds {
            break;
        } else if floor[guard.next_r()][guard.next_c()] == Blocked {
            guard.rotate();
        } else {
            guard.walk();
            floor[guard.r][guard.c] = Visited(0);
        }
    }

    Some(
        floor
            .iter()
            .map(|row| row.iter().filter(|&&tile| tile == Visited(0)).count())
            .sum::<usize>() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let raw: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut floor = vec![vec![OutBounds; raw[0].len() + 2]];
    floor.extend(raw.iter().map(|row| {
        let mut nr = vec![OutBounds];
        nr.extend(row.iter().map(|&c| Tile::from_char(c)));
        nr.push(OutBounds);
        nr
    }));
    floor.push(vec![OutBounds; raw[0].len() + 2]);

    let mut original_guard = Guard { r: 0, c: 0, dir: 0 };
    'outer: for (r, row) in floor.iter().enumerate() {
        for (c, &tile) in row.iter().enumerate() {
            if tile == Visited(0) {
                original_guard.r = r;
                original_guard.c = c;
                break 'outer;
            }
        }
    }

    let mut original_path = HashSet::new();

    let mut guard = original_guard.clone();
    loop {
        if floor[guard.next_r()][guard.next_c()] == OutBounds {
            break;
        } else if floor[guard.next_r()][guard.next_c()] == Blocked {
            guard.rotate();
        } else {
            guard.walk();
            original_path.insert((guard.r, guard.c));
        }
    }

    let mut valid_loops = 0;

    for (ob_r, ob_c) in original_path {
        let mut new_floor = floor.clone();
        new_floor[ob_r][ob_c] = Blocked;
        let mut new_guard = original_guard.clone();
        let mut looping = false;
        loop {
            if new_floor[new_guard.next_r()][new_guard.next_c()] == OutBounds {
                break;
            } else if new_floor[new_guard.next_r()][new_guard.next_c()] == Blocked {
                new_guard.rotate();
            } else if new_floor[new_guard.next_r()][new_guard.next_c()] == Visited(new_guard.dir) {
                looping = true;
                break;
            } else {
                new_guard.walk();
                new_floor[new_guard.r][new_guard.c] = Visited(new_guard.dir);
            }
        }
        if looping {
            valid_loops += 1;
        }
    }

    Some(valid_loops)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
