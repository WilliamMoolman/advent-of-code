use std::collections::VecDeque;

use advent_of_code::utils::LinesExt;

advent_of_code::solution!(10);

struct Map {
    map: Vec<Vec<u64>>,
    trailheads: Vec<(usize, usize)>,
    trailends: Vec<(usize, usize)>,
}

impl Map {
    fn from_input(input: &str) -> Map {
        let map: Vec<Vec<u64>> = input
            .lines()
            .to_char_grid()
            .iter()
            .map(|row| row.iter().map(|c| c.to_digit(10).unwrap() as u64).collect())
            .collect();
        let trailheads: Vec<(usize, usize)> = map
            .iter()
            .enumerate()
            .flat_map(|(r, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, height)| **height == 0)
                    .map(move |(c, _)| (r, c))
            })
            .collect();
        let trailends: Vec<(usize, usize)> = map
            .iter()
            .enumerate()
            .flat_map(|(r, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, height)| **height == 9)
                    .map(move |(c, _)| (r, c))
            })
            .collect();

        Map {
            map,
            trailheads,
            trailends,
        }
    }

    fn neighbours(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let (r, c) = pos;
        let (r_max, c_max) = (self.map.len(), self.map[0].len());
        let mut adjacent = vec![];
        if r != r_max - 1 {
            let new = (r + 1, c);
            if self.map[pos.0][pos.1] + 1 == self.map[new.0][new.1] {
                adjacent.push(new)
            }
        }
        if r != 0 {
            let new = (r - 1, c);
            if self.map[pos.0][pos.1] + 1 == self.map[new.0][new.1] {
                adjacent.push(new)
            }
        }
        if c != c_max - 1 {
            let new = (r, c + 1);
            if self.map[pos.0][pos.1] + 1 == self.map[new.0][new.1] {
                adjacent.push(new)
            }
        }
        if c != 0 {
            let new = (r, c - 1);
            if self.map[pos.0][pos.1] + 1 == self.map[new.0][new.1] {
                adjacent.push(new)
            }
        }
        adjacent
    }

    fn bfs(&self, start: (usize, usize)) -> Vec<Vec<bool>> {
        let mut stack = VecDeque::new();
        stack.push_back(start);
        let mut seen: Vec<Vec<bool>> = self
            .map
            .iter()
            .map(|row| row.iter().map(|_| false).collect())
            .collect();
        seen[start.0][start.1] = true;
        while let Some(pos) = stack.pop_front() {
            seen[pos.0][pos.1] = true;
            let adj: Vec<(usize, usize)> = self.neighbours(pos);
            for a in adj {
                if !seen[a.0][a.1] {
                    stack.push_back(a);
                }
            }
        }

        seen
    }

    fn num_routes(&self, start: (usize, usize)) -> u64 {
        if self.map[start.0][start.1] == 9 {
            1
        } else {
            self.neighbours(start)
                .iter()
                .map(|pos| self.num_routes(*pos))
                .sum::<u64>()
        }
    }

    fn find_scores(&self) -> u64 {
        self.trailheads
            .iter()
            .map(|(r, c)| {
                let seen = self.bfs((*r, *c));
                let count = self
                    .trailends
                    .iter()
                    .filter(|pos| seen[pos.0][pos.1])
                    .count() as u64;
                // println!("[{r}, {c}] => {count}");
                count
            })
            .sum()
    }
    fn find_ratings(&self) -> u64 {
        self.trailheads
            .iter()
            .map(|start| {
                let count = self.num_routes(*start);
                // println!("[{}, {}] => {count}", start.0, start.1);

                count
            })
            .sum()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let map = Map::from_input(input);
    Some(map.find_scores())
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = Map::from_input(input);
    Some(map.find_ratings())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
