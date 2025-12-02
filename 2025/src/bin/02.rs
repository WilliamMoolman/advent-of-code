use itertools::Itertools;

advent_of_code::solution!(2);

fn get_repeated(range: &str, n: usize) -> Vec<u64> {
    let (begin_str, end_str) = range.split_once('-').unwrap();
    let begin_len = begin_str.len();
    let end_len = end_str.len();
    let begin_part: String = if begin_len % n == 0 {
        begin_str[0..begin_len / n].to_owned()
    } else {
        10_u64.pow(begin_len as u32 / n as u32).to_string()
    };
    let end_part: String = if end_len % n == 0 {
        end_str[0..end_len / n].to_owned()
    } else {
        "9".repeat(end_len / n)
    };
    let lower: u64 = begin_str.parse().unwrap();
    let upper: u64 = end_str.parse().unwrap();
    (begin_part.parse::<u64>().unwrap()..=end_part.parse::<u64>().unwrap())
        .into_iter()
        .map(|part| part.to_string().repeat(n as usize).parse::<u64>().unwrap())
        .filter(move |&i| i >= lower && i <= upper)
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let sum = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .into_iter()
        .map(|range| get_repeated(range, 2).iter().sum::<u64>())
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let sum = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .into_iter()
        .map(|range| {
            let (_, end) = range.split_once('-').unwrap();
            (2..=end.len())
                .into_iter()
                .flat_map(|n| get_repeated(range, n))
                .unique()
                .sum::<u64>()
        })
        .sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
