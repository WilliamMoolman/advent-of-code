advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut dial = 50;
    let mut zeroes = 0;
    input.lines().for_each(|line| {
        let mut chars = line.chars();
        let dir = chars.next().unwrap();
        let mag: i32 = chars.collect::<String>().parse().unwrap();
        dial = match dir {
            'L' => (dial + 100 - mag) % 100,
            'R' => (dial + mag) % 100,
            _ => panic!(),
        };
        if dial == 0 {
            zeroes += 1;
        }
    });
    Some(zeroes)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut dial = 50;
    let mut zeroes: u64 = 0;
    input.lines().for_each(|line| {
        let mut chars = line.chars();
        let dir = chars.next().unwrap();
        let mut mag: i32 = chars.collect::<String>().parse().unwrap();
        if mag > 100 {
            zeroes += mag as u64 / 100;
            mag %= 100;
        }
        dial = match dir {
            'L' => {
                if mag >= dial && dial != 0 {
                    zeroes += 1
                };
                (dial + 100 - mag) % 100
            }
            'R' => {
                if mag >= (100 - dial) && dial != 0 {
                    zeroes += 1
                };
                (dial + mag) % 100
            }
            _ => panic!(),
        };
    });
    Some(zeroes)
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
        assert_eq!(result, Some(6));
    }
}
