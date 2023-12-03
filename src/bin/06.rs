use std::ops::RangeInclusive;

advent_of_code::solution!(6);

/*
turn off 674,321 through 793,388
toggle 749,672 through 973,965
turn on 943,30 through 990,907
 */
#[derive(Debug)]
struct Coord(usize, usize);

#[derive(Debug)]
enum Instruction {
    On(Coord, Coord),
    Off(Coord, Coord),
    Toggle(Coord, Coord),
}

impl Coord {
    fn from_string(s: &[&str]) -> (Coord, Coord) {
        let (x, y) = s[0].split_once(",").unwrap();
        let start = Coord(x.parse().unwrap(), y.parse().unwrap());
        let (x, y) = s[2].split_once(",").unwrap();
        let end = Coord(x.parse().unwrap(), y.parse().unwrap());
        (start, end)
    }

    fn range_with(&self, end: Coord) -> (RangeInclusive<usize>, RangeInclusive<usize>) {
        (self.0..=end.0, self.1..=end.1)
    }
}

impl Instruction {
    fn from_line(line: &str) -> Instruction {
        let fixed_line = line.replace("toggle", "toggle lights");
        let words: Vec<&str> = fixed_line.split_whitespace().collect();
        let (start, end) = Coord::from_string(&words[2..]);
        match words[0..2] {
            ["turn", "off"] => Instruction::Off(start, end),
            ["turn", "on"] => Instruction::On(start, end),
            ["toggle", "lights"] => Instruction::Toggle(start, end),
            _ => panic!("Invalid Instruction"),
        }
    }
}

fn mutate_lights(lights: &mut [[bool; 1000]], start: Coord, end: Coord, f: fn(bool) -> bool) {
    let (x_range, y_range) = start.range_with(end);
    for x in x_range {
        for y in y_range.clone() {
            lights[x][y] = f(lights[x][y]);
        }
    }
}

fn mutate_lights_2(lights: &mut Vec<Vec<u32>>, start: Coord, end: Coord, f: fn(u32) -> u32) {
    let (x_range, y_range) = start.range_with(end);
    for x in x_range {
        for y in y_range.clone() {
            lights[x][y] = f(lights[x][y]);
        }
    }
}

#[allow(dead_code, unused_variables)]
pub fn part_one(input: &str) -> Option<u32> {
    let mut lights = [[false; 1000]; 1000];
    input
        .lines()
        .map(Instruction::from_line)
        .for_each(|instruction| match instruction {
            Instruction::On(start, end) => mutate_lights(&mut lights, start, end, |_| true),
            Instruction::Off(start, end) => mutate_lights(&mut lights, start, end, |_| false),
            Instruction::Toggle(start, end) => mutate_lights(&mut lights, start, end, |x| !x),
        });
    Some(
        lights
            .into_iter()
            .map(|line| line.into_iter().filter(|x| *x).count() as u32)
            .sum(),
    )
}

#[allow(dead_code, unused_variables)]
pub fn part_two(input: &str) -> Option<u32> {
    let mut lights = vec![vec![0; 1000]; 1000];
    input
        .lines()
        .map(Instruction::from_line)
        .for_each(|instruction| match instruction {
            Instruction::On(start, end) => mutate_lights_2(&mut lights, start, end, |b| b + 1),
            Instruction::Off(start, end) => {
                mutate_lights_2(&mut lights, start, end, |b| b.saturating_sub(1))
            }
            Instruction::Toggle(start, end) => mutate_lights_2(&mut lights, start, end, |b| b + 2),
        });
    Some(
        lights
            .into_iter()
            .map(|line| line.into_iter().sum::<u32>())
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1000000));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
