use itertools::Itertools;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let splits = 0;
    let acc = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| c == 'S')
        .collect_vec();

    let width = acc.len();

    let (splits, _acc) = lines.fold((splits, acc), |(splits, acc), line| {
        let mut next_acc = vec![false; width];
        let mut next_splits = splits;
        line.char_indices().for_each(|(i, c)| {
            if acc[i] {
                if c == '^' {
                    next_splits += 1;
                    if i != 0 {
                        next_acc[i - 1] = true;
                    }
                    if i != width - 1 {
                        next_acc[i + 1] = true;
                    }
                } else {
                    next_acc[i] = true;
                }
            }
        });
        (next_splits, next_acc)
    });

    Some(splits)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines().peekable();
    let first_line = lines.peek().unwrap();
    let width = first_line.len();
    let starting_point = first_line.find('S').unwrap();

    let acc = vec![1; width];

    let acc = lines.rev().fold(acc, |acc, line| {
        let mut next_acc = vec![0; width];
        line.char_indices().for_each(|(i, c)| {
            if c == '^' {
                if i != 0 {
                    next_acc[i] += acc[i - 1];
                }
                if i != width - 1 {
                    next_acc[i] += acc[i + 1];
                }
            } else {
                next_acc[i] = acc[i]
            }
        });
        next_acc
    });

    Some(acc[starting_point])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result: Option<u64> = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
