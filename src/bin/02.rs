advent_of_code::solution!(2);
use regex::Regex;

struct Game {
    id: u32,
    blue: u32,
    green: u32,
    red: u32,
}

fn get_re_max(re: Regex, haystack: &str) -> u32 {
    re.captures_iter(haystack)
        .map(|c| c.extract::<1>().1[0])
        .map(|c| c.parse::<u32>().unwrap())
        .max()
        .unwrap()
}

impl Game {
    fn new(line: &str) -> Game {
        let re_id = Regex::new(r"Game (\d+):").unwrap();
        let re_blue = Regex::new(r" (\d+) blue").unwrap();
        let re_red = Regex::new(r" (\d+) red").unwrap();
        let re_green = Regex::new(r" (\d+) green").unwrap();

        let id: u32 = re_id
            .captures(line)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .unwrap();
        let blue: u32 = get_re_max(re_blue, line);
        let green: u32 = get_re_max(re_green, line);
        let red: u32 = get_re_max(re_red, line);
        Game {
            id,
            blue,
            green,
            red,
        }
    }
    fn le(&self, red: u32, green: u32, blue: u32) -> bool {
        self.blue <= blue && self.red <= red && self.green <= green
    }

    fn power(&self) -> u32 {
        self.blue * self.red * self.green
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| Game::new(line))
            .filter(|g| g.le(12, 13, 14))
            .map(|g| g.id)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| Game::new(line))
            .map(|g| g.power())
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
