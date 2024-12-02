advent_of_code::solution!(2);

fn is_safe_asc(line: &Vec<u32>) -> bool {
    let mut prev = line[0];
    for &level in line[1..].iter() {
        if level < prev + 1 || level > prev + 3 {
            return false;
        }
        prev = level;
    }
    return true;
}

fn is_safe_desc(line: &Vec<u32>) -> bool {
    let mut prev = line[0];
    for &level in line[1..].iter() {
        if level + 1 > prev || level + 3 < prev {
            return false;
        }
        prev = level;
    }
    return true;
}

pub fn part_one(input: &str) -> Option<u32> {
    let num = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|d| d.parse().unwrap())
                .collect()
        })
        .filter(|levels| is_safe_asc(levels) || is_safe_desc(levels))
        .count();

    Some(num as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let num = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|d| d.parse().unwrap())
                .collect()
        })
        .filter(|levels| {
            is_safe_asc(levels)
                || is_safe_desc(levels)
                || (0..levels.len())
                    .map(|i| {
                        let mut level_copy = levels.clone();
                        level_copy.remove(i);
                        level_copy
                    })
                    .any(|levels| is_safe_asc(&levels) || is_safe_desc(&levels))
        })
        .count();

    Some(num as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
