advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let sum = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .map(|digits| {
            let mut max = 0;
            let mut max_idx = 0;
            let mut max2 = 0;
            for i in 0..digits.len() - 1 {
                if digits[i] > max {
                    max = digits[i];
                    max_idx = i;
                }
            }
            for i in max_idx + 1..digits.len() {
                if digits[i] > max2 {
                    max2 = digits[i];
                }
            }
            format!("{max}{max2}").parse::<u64>().unwrap()
        })
        .sum();

    Some(sum)
}

fn nth_max(digits: &[u32], start: usize, n: usize) -> usize {
    let mut max = 0;
    let mut max_idx = 0;
    for i in start..digits.len() - n {
        if digits[i] > max {
            max = digits[i];
            max_idx = i;
        }
    }
    return max_idx;
}

pub fn part_two(input: &str) -> Option<u64> {
    let sum = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .map(|digits| {
            let mut jolts = vec![];
            let mut last = 0;
            for i in (0..12).rev() {
                let idx = nth_max(&digits, last, i);
                jolts.push(digits[idx].to_string());
                last = idx + 1;
            }

            let number = jolts.join("");

            number.parse::<u64>().unwrap()
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
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
