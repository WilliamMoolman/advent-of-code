advent_of_code::solution!(1);

use regex::Regex;

fn get_first_last_num(line: &str) -> u32 {
    let re = Regex::new(r"(\d)").unwrap();
    let digits: Vec<&str> = re.captures_iter(line).map(|c| c.extract::<1>().1[0]).collect();
    let num_str = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
    num_str.parse().unwrap()
}
pub fn part_one(input: &str) -> Option<u32> {
    Some(
    input.lines().map(|line| {
        get_first_last_num(line)
    }).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
