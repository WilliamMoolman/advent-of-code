advent_of_code::solution!(9);

fn difference_of_numbers(numbers: &Vec<i64>) -> Vec<i64> {
    let first = &numbers[0..numbers.len() - 1];
    let second = &numbers[1..numbers.len()];
    let diff = second
        .iter()
        .zip(first.iter())
        .map(|(a, b)| a - b)
        .collect();
    diff
}

fn next_in_sequence(numbers: Vec<i64>) -> i64 {
    if numbers.iter().all(|n| *n == 0) {
        return 0;
    }
    let diff = difference_of_numbers(&numbers);
    let next = numbers[numbers.len() - 1] + next_in_sequence(diff);
    next
}

fn prev_in_sequence(numbers: Vec<i64>) -> i64 {
    if numbers.iter().all(|n| *n == 0) {
        return 0;
    }
    let diff = difference_of_numbers(&numbers);
    let next = numbers[0] - prev_in_sequence(diff);
    next
}

pub fn part_one(input: &str) -> Option<i64> {
    Some(
        input
            .lines()
            .map(|l| {
                l.split_ascii_whitespace()
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect()
            })
            .map(next_in_sequence)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    Some(
        input
            .lines()
            .map(|l| {
                l.split_ascii_whitespace()
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect()
            })
            .map(prev_in_sequence)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
