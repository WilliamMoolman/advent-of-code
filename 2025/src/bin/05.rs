use itertools::Itertools;

advent_of_code::solution!(5);

#[derive(Debug)]
struct Window(u64, u64);

impl Window {
    fn from_lines(windows_lines: &[&str]) -> Vec<Window> {
        windows_lines
            .iter()
            .map(|l| {
                let (a, b) = l.split_once('-').unwrap();
                Window(a.parse().unwrap(), b.parse().unwrap())
            })
            .sorted_by_key(|x| x.0)
            .fold(Vec::new(), |mut acc, x| {
                match acc.last_mut() {
                    Some(Window(_, b)) if *b + 1 >= x.0 => *b = x.1.max(*b),
                    _ => acc.push(x),
                }
                acc
            })
    }
}
pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines().peekable();
    let range_strs: Vec<&str> = lines.peeking_take_while(|l| l.len() > 0).collect();
    lines.next();
    let ranges = Window::from_lines(&range_strs);

    let ingredients = lines
        .map(|ing| ing.parse::<u64>().unwrap())
        .filter(|&ing| ranges.iter().filter(|w| w.0 <= ing && w.1 >= ing).count() > 0)
        .count();

    Some(ingredients as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines().peekable();
    let range_strs: Vec<&str> = lines.peeking_take_while(|l| l.len() > 0).collect();
    let ranges = Window::from_lines(&range_strs);
    let sum = ranges.iter().map(|r| r.1 - r.0 + 1).sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
