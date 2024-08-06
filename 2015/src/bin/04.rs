advent_of_code::solution!(4);
use md5;

pub fn part_one(input: &str) -> Option<usize> {
    let mut i: usize = 1;
    let key = input.trim();
    loop {
        if &format!("{:x}", md5::compute(format!("{key}{i}")))[..5] == "00000" {
            break;
        }
        i += 1;
    }
    Some(i)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut i: usize = 1;
    let key = input.trim();
    loop {
        if &format!("{:x}", md5::compute(format!("{key}{i}")))[..6] == "000000" {
            break;
        }
        i += 1;
    }
    Some(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(609043));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
