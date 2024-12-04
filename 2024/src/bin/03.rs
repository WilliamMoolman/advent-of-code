advent_of_code::solution!(3);
use regex::Regex;

fn extract_mul(line: &str) -> Vec<(u32, u32)> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(line)
        .map(|cap| {
            // println!("{cap:?}");
            (
                cap.get(1).unwrap().as_str().parse::<u32>().unwrap(),
                cap.get(2).unwrap().as_str().parse::<u32>().unwrap(),
            )
        })
        .collect()
}

enum Command {
    DO,
    DONT,
    MUL(u32, u32),
}

use Command::*;

fn extract_mul_stops(line: &str) -> Vec<Command> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    re.captures_iter(line)
        .map(|cap| match &cap.get(0).unwrap().as_str()[..3] {
            "do(" => DO,
            "don" => DONT,
            "mul" => MUL(
                cap.get(1).unwrap().as_str().parse::<u32>().unwrap(),
                cap.get(2).unwrap().as_str().parse::<u32>().unwrap(),
            ),
            a => panic!("{cap:?} {a}"),
        })
        .collect()
}
pub fn part_one(input: &str) -> Option<u32> {
    let line = input.replace("\n", "");
    Some(extract_mul(&line).iter().map(|(a, b)| a * b).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    // let mut line = input.replace("\n", "");
    // let re = Regex::new(r"(don't\(\).*do\(\))").unwrap();
    // re.split(&line).for_each(|c| println!("{c}"));
    // line.replace(from, to)

    // Some(extract_mul(&line).iter().map(|(a, b)| a * b).sum())
    let line = input.replace("\n", "");
    let mut sum = 0;
    let mut go = true;
    for c in extract_mul_stops(&line) {
        match c {
            DO => go = true,
            DONT => go = false,
            MUL(a, b) => {
                if go {
                    sum += a * b
                }
            }
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
