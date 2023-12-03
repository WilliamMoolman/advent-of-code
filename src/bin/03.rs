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

fn extract_num(lines: &Vec<&str>, i: usize, c_idx: usize) -> u32 {
    let mut nforward: Vec<char> = vec![lines[i].chars().nth(c_idx).unwrap()];
    for j in (c_idx + 1)..lines[0].len() {
        let c = lines[i].chars().nth(j).unwrap();
        if c.is_digit(10) {
            nforward.push(c);
        } else {
            break;
        }
    }
    let mut n_reverse: Vec<char> = Vec::new();
    for j in (0..c_idx).rev() {
        let c = lines[i].chars().nth(j).unwrap();
        if c.is_digit(10) {
            n_reverse.push(c);
        } else {
            break;
        }
    }
    n_reverse.reverse();
    n_reverse.extend(nforward);
    String::from(n_reverse.into_iter().collect::<String>())
        .parse()
        .unwrap()
}

fn extract_nums(lines: &Vec<&str>, i: usize, c_idx: usize, nums: &mut Vec<u32>) {
    let l = if c_idx != 0 {
        lines[i].chars().nth(c_idx - 1).unwrap()
    } else {
        'x'
    };
    let m = lines[i].chars().nth(c_idx).unwrap();
    let r = if c_idx != lines[0].len() - 1 {
        lines[i].chars().nth(c_idx + 1).unwrap()
    } else {
        'x'
    };

    match (l.is_digit(10), m.is_digit(10), r.is_digit(10)) {
        (_, true, _) => nums.push(extract_num(lines, i, c_idx)),
        (true, false, true) => {
            nums.push(extract_num(lines, i, c_idx - 1));
            nums.push(extract_num(lines, i, c_idx + 1))
        }
        (true, _, false) => nums.push(extract_num(lines, i, c_idx - 1)),
        (false, _, true) => nums.push(extract_num(lines, i, c_idx + 1)),

        (false, false, false) => {}
    }
}
fn handle_gear(lines: &Vec<&str>, i: usize, c_idx: usize) -> Option<u32> {
    let mut nums = Vec::new();
    if i != 0 {
        extract_nums(lines, i - 1, c_idx, &mut nums)
    }
    extract_nums(lines, i, c_idx, &mut nums);
    if i != lines.len() - 1 {
        extract_nums(lines, i + 1, c_idx, &mut nums);
    }

    if nums.len() == 2 {
        return Some(nums[0] * nums[1]);
    }

    None
}
pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let mut sum = 0;
    for i in 0..lines.len() {
        for (c_idx, c) in lines[i].chars().enumerate() {
            if c == '*' {
                if let Some(ratio) = handle_gear(&lines, i, c_idx) {
                    sum += ratio;
                }
            }
        }
    }
    Some(sum)
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
        assert_eq!(result, Some(467835));
    }
}
