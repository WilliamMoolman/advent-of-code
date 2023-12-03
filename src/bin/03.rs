advent_of_code::solution!(3);

fn contains_symbol(char_slice: Vec<char>) -> bool {
    let mut contains = false;
    for c in char_slice {
        contains |= c != '.' && !c.is_digit(10)
    }
    contains
}

fn symbol_surrounding(lines: &Vec<Vec<char>>, i: usize, start: usize, end: usize) -> bool {
    let start_idx = if start == 0 { 0 } else { start - 1 };
    let end_idx = (end + 1).min(lines[0].len());
    let mut add = false;

    // Check above
    if i != 0 {
        add |= contains_symbol(lines[i - 1][start_idx..end_idx].to_vec());
    }
    // Check mid
    add |= contains_symbol(lines[i][start_idx..end_idx].to_vec());

    // Check after
    if i != lines.len() - 1 {
        add |= contains_symbol(lines[i + 1][start_idx..end_idx].to_vec());
    }

    add
}

fn extract_num(line: &Vec<char>, c_idx: usize) -> (u32, usize) {
    let mut right_digits = vec![line[c_idx]];
    for j in (c_idx + 1)..line.len() {
        let c = line[j];
        if c.is_digit(10) {
            right_digits.push(c);
        } else {
            break;
        }
    }
    let mut left_digits: Vec<char> = Vec::new();
    for j in (0..c_idx).rev() {
        let c = line[j];
        if c.is_digit(10) {
            left_digits.push(c);
        } else {
            break;
        }
    }
    left_digits.reverse();
    left_digits.extend(right_digits);
    let num = String::from(left_digits.into_iter().collect::<String>())
        .parse()
        .unwrap();
    (num, num.to_owned().to_string().len())
}

fn extract_nums(lines: &Vec<Vec<char>>, i: usize, c_idx: usize, nums: &mut Vec<u32>) {
    let l = if c_idx != 0 { lines[i][c_idx - 1] } else { 'x' };
    let m = lines[i][c_idx];
    let r = if c_idx != lines[0].len() - 1 {
        lines[i][c_idx + 1]
    } else {
        'x'
    };

    match (l.is_digit(10), m.is_digit(10), r.is_digit(10)) {
        (_, true, _) => nums.push(extract_num(&lines[i], c_idx).0),
        (left, false, right) => {
            if left {
                nums.push(extract_num(&lines[i], c_idx - 1).0)
            };
            if right {
                nums.push(extract_num(&lines[i], c_idx + 1).0)
            };
        }
    }
}

fn handle_gear(lines: &Vec<Vec<char>>, i: usize, c_idx: usize) -> Option<u32> {
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

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut sum: u32 = 0;
    for i in 0..lines.len() {
        let mut c_idx = 0;
        while c_idx < lines[i].len() {
            let c = lines[i][c_idx];

            if c.is_digit(10) {
                let (num, length) = extract_num(&lines[i], c_idx);
                if symbol_surrounding(&lines, i, c_idx, c_idx + length) {
                    sum += num;
                }

                c_idx += length;
            } else {
                c_idx += 1;
            }
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut sum = 0;
    for i in 0..lines.len() {
        for (c_idx, c) in lines[i].iter().enumerate() {
            if c == &'*' {
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
