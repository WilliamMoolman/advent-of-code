advent_of_code::solution!(2);
use regex::Regex;

fn wrapping(l: u32, w: u32, h: u32) -> u32 {
    let (s1, s2, s3) = (l * w, w * h, h * l);
    let surface = 2 * s1 + 2 * s2 + 2 * s3;
    let extra = s1.min(s2).min(s3);
    surface + extra
}

fn ribbon(l: u32, w: u32, h: u32) -> u32 {
    let (f1, f2, f3) = (2 * l + 2 * w, 2 * w + 2 * h, 2 * h + 2 * l);
    let min_perimeter = f1.min(f2).min(f3);
    min_perimeter + l * w * h
}

fn input_items(input: &str) -> Vec<(u32, u32, u32)> {
    let re = Regex::new(r"(\d+)x(\d+)x(\d+)").unwrap();
    input
        .lines()
        .map(|line| {
            let sides = re.captures(line).unwrap();
            (
                sides[1].parse().unwrap(),
                sides[2].parse().unwrap(),
                sides[3].parse().unwrap(),
            )
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input_items(input)
            .iter()
            .map(|(l, w, h)| wrapping(*l, *w, *h))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input_items(input)
            .iter()
            .map(|(l, w, h)| ribbon(*l, *w, *h))
            .sum(),
    )
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
