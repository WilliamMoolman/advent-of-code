advent_of_code::solution!(3);

fn contains_symbol(c: char) -> bool {
    c != '.' && !c.is_digit(10)
}

fn check_number(lines: &Vec<&str>, i: usize, start: usize, end: usize) -> Option<u32> {
    let start_idx = if start == 0 { 0 } else { start - 1 };
    let end_idx = (end + 1).min(lines[0].len());
    let mut add = false;

    // Check above
    if i != 0 {
        add |= lines[i - 1][start_idx..end_idx].contains(contains_symbol);
    }
    // Check mid
    add |= lines[i][start_idx..end_idx].contains(contains_symbol);

    // Check after
    if i != lines.len() - 1 {
        add |= lines[i + 1][start_idx..end_idx].contains(contains_symbol);
    }

    if add {
        let x = lines[i][start..end].parse::<u32>().unwrap();
        return Some(x);
    }
    None
}
pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let mut num_start = 0;
    let mut num_length = 0;
    let mut sum: u32 = 0;
    for i in 0..lines.len() {
        for (c_idx, c) in lines[i].chars().enumerate() {
            match (c.is_digit(10), num_length) {
                (true, 0) => {
                    num_start = c_idx;
                    num_length = 1
                } // New number
                (true, _) => num_length += 1, // Mid number
                (false, 0) => continue,       // Not in number
                (false, _) => {
                    // println!(
                    //     "{}: {}/{} - {}",
                    //     i,
                    //     num_start,
                    //     num_length,
                    //     &lines[i][num_start..(num_start + num_length)]
                    // );
                    // End of number
                    if let Some(num) = check_number(&lines, i, num_start, num_start + num_length) {
                        sum += num;
                    }
                    num_length = 0;
                }
            }
        }

        if num_length != 0 {
            if let Some(num) = check_number(&lines, i, num_start, num_start + num_length) {
                sum += num;
            }
            num_length = 0;
        }
    }
    Some(sum)
}

#[allow(dead_code, unused_variables)]
pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
