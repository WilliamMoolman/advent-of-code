use std::collections::HashMap;

advent_of_code::solution!(11);

struct Stones {
    memoize: HashMap<(u64, usize), usize>,
}

impl Stones {
    fn new() -> Stones {
        Stones {
            memoize: HashMap::new(),
        }
    }
    fn apply_rules(&mut self, n: u64, blinks: usize) -> usize {
        if blinks == 0 {
            return 1;
        }
        if let Some(out) = self.memoize.get(&(n, blinks)) {
            return *out;
        }

        let digits = n.to_string();
        let states;
        if n == 0 {
            states = vec![1];
        } else if digits.len() % 2 == 0 {
            states = vec![
                digits[..digits.len() / 2].parse().unwrap(),
                digits[digits.len() / 2..].parse().unwrap(),
            ];
        } else {
            states = vec![n * 2024];
        }

        let out = states
            .iter()
            .map(|n2| self.apply_rules(*n2, blinks - 1))
            .sum::<usize>();

        self.memoize.insert((n, blinks), out);

        out
    }
}

fn apply_rules(n: u64) -> Vec<u64> {
    let digits = n.to_string();
    if n == 0 {
        vec![1]
    } else if digits.len() % 2 == 0 {
        vec![
            digits[..digits.len() / 2].parse().unwrap(),
            digits[digits.len() / 2..].parse().unwrap(),
        ]
    } else {
        vec![n * 2024]
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    // let mut starting: Vec<u64> = input.split(" ").map(|n| n.parse().unwrap()).collect();
    // for _ in 0..25 {
    //     starting = starting.iter().flat_map(|n| apply_rules(*n)).collect();
    // }

    // Some(starting.len() as u64)
    let starting: Vec<u64> = input.split(" ").map(|n| n.parse().unwrap()).collect();
    let mut stones = Stones::new();

    Some(
        starting
            .iter()
            .map(|n| stones.apply_rules(*n, 25) as u64)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let starting: Vec<u64> = input.split(" ").map(|n| n.parse().unwrap()).collect();
    let mut stones = Stones::new();

    Some(
        starting
            .iter()
            .map(|n| stones.apply_rules(*n, 75) as u64)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
