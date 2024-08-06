advent_of_code::solution!(15);
/*
Determine the ASCII code for the current character of the string.
Increase the current value by the ASCII code you just determined.
Set the current value to itself multiplied by 17.
Set the current value to the remainder of dividing itself by 256.
 */
fn hash(s: &str) -> u32 {
    let mut value = 0;
    for c in s.chars() {
        let code = c as u32;
        value += code;
        value *= 17;
        value %= 256;
    }
    value
}
pub fn part_one(input: &str) -> Option<u32> {
    Some(input.trim().split(',').map(|s| hash(s)).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut boxes: Vec<Vec<(String, u32)>> = vec![vec![]; 256];
    for s in input.trim().split(',') {
        let label = s
            .chars()
            .take_while(|c| c.is_alphabetic())
            .collect::<String>();
        let hash_value = hash(&label) as usize;
        let is_additive: bool = s.contains('=');
        if is_additive {
            let focal_strength = s.chars().last().unwrap().to_digit(10).unwrap();
            let mut contains = false;
            // Replace
            for (old_label, strength) in boxes[hash_value].iter_mut() {
                if *old_label == label {
                    *strength = focal_strength;
                    contains = true;
                    break;
                }
            }

            // Add
            if !contains {
                boxes[hash_value].push((label, focal_strength));
            }
        } else {
            // Remove
            boxes[hash_value].retain(|(old_label, _)| *old_label != label);
        }
    }

    let mut total_power: usize = 0;

    for (i, lightbox) in boxes.iter().enumerate() {
        for (j, (_, focal_strength)) in lightbox.iter().enumerate() {
            let power = (i + 1) * (j + 1) * (*focal_strength as usize);
            total_power += power;
        }
    }
    Some(total_power)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
