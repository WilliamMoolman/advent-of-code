advent_of_code::solution!(3);
use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<usize> {
    let mut visited = HashSet::new();
    let mut sx = 0;
    let mut sy = 0;
    visited.insert((sx, sy));

    input.chars().for_each(|c| {
        match c {
            '^' => sy += 1,
            'v' => sy -= 1,
            '<' => sx -= 1,
            '>' => sx += 1,
            _ => panic!("Rogue input: {}", c),
        };
        visited.insert((sx, sy));
    });
    Some(visited.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut visited = HashSet::new();
    let mut sx = 0;
    let mut sy = 0;
    let mut rx = 0;
    let mut ry = 0;
    visited.insert((sx, sy));
    let mut robo_turn = false;

    input.chars().for_each(|c| {
        robo_turn = !robo_turn;
        if robo_turn {
            match c {
                '^' => ry += 1,
                'v' => ry -= 1,
                '<' => rx -= 1,
                '>' => rx += 1,
                _ => panic!("Rogue input: {}", c),
            };
            visited.insert((rx, ry));
        } else {
            match c {
                '^' => sy += 1,
                'v' => sy -= 1,
                '<' => sx -= 1,
                '>' => sx += 1,
                _ => panic!("Rogue input: {}", c),
            };
            visited.insert((sx, sy));
        }
    });
    Some(visited.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
