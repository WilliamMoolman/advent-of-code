advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    let mut floor = 0;
    input.chars().for_each(|c| match c {
        '(' => floor += 1,
        ')' => floor -= 1,
        _ => panic!("Input contains non () input: {}", c),
    });
    Some(floor)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut floor: i32 = 0;
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("Input contains non () input: {}", c),
        }
        if floor < 0 {
            return Some(i + 1);
        }
    }
    panic!("No basement to the building!")
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
