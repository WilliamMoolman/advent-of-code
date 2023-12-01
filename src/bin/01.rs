advent_of_code::solution!(1);

use regex::Regex;

fn get_first_last_num(line: &str) -> u32 {
    let re = Regex::new(r"(\d)").unwrap();
    let digits: Vec<&str> = re
        .captures_iter(line)
        .map(|c| c.extract::<1>().1[0])
        .collect();
    let num_str = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
    num_str.parse().unwrap()
}

fn get_first_last_num_with_words(line: &str) -> u32 {
    let re = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();

    // Account for overlapping matches. Default regex crate does not support look-ahead
    let l = String::from(line)
        .replace("oneight", "oneeight")
        .replace("twone", "twoone")
        .replace("threeight", "threeeight")
        .replace("fiveight", "fiveeight")
        .replace("sevenine", "sevennine")
        .replace("eightwo", "eighttwo")
        .replace("eighthree", "eightthree")
        .replace("nineight", "nineeight");
    let digits: Vec<u32> = re
        .captures_iter(&l)
        .map(|c| c.extract::<1>().1[0])
        .map(|c| match c {
            "one" | "1" => 1,
            "two" | "2" => 2,
            "three" | "3" => 3,
            "four" | "4" => 4,
            "five" | "5" => 5,
            "six" | "6" => 6,
            "seven" | "7" => 7,
            "eight" | "8" => 8,
            "nine" | "9" => 9,
            _ => panic!("Regex should not have matched {}!", c),
        })
        .collect();
    let num_str = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
    num_str.parse().unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(|line| get_first_last_num(line)).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| get_first_last_num_with_words(line))
            .sum(),
    )
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
        assert_eq!(result, Some(281));
    }
}
