advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    let mut vec1: Vec<i32> = vec![];
    let mut vec2: Vec<i32> = vec![];
    input.lines().for_each(|line| {
        let (a, b) = line.split_once("   ").unwrap();
        vec1.push(a.parse().unwrap());
        vec2.push(b.parse().unwrap());
    });

    vec1.sort();
    vec2.sort();

    Some(vec1.iter().zip(vec2).map(|(a, b)| (a - b).abs()).sum())
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut vec1: Vec<i32> = vec![];
    let mut vec2: Vec<i32> = vec![];
    input.lines().for_each(|line| {
        let (a, b) = line.split_once("   ").unwrap();
        vec1.push(a.parse().unwrap());
        vec2.push(b.parse().unwrap());
    });

    Some(
        vec1.iter()
            .map(|&a| vec2.iter().filter(|&&b| a == b).count() as i32 * a)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
