advent_of_code::solution!(5);
use fancy_regex::Regex;

fn is_vowel(c: &char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}

fn has_3_vowels(line: &&str) -> bool {
    line.chars().filter(|c| is_vowel(c)).count() >= 3
}

fn has_duplicates(line: &&str) -> bool {
    let mut cs = line.chars();
    let mut c_prev = cs.next().unwrap();
    for c in cs {
        if c == c_prev {
            return true;
        }
        c_prev = c;
    }
    return false;
}

fn banned_groups(line: &&str) -> bool {
    !(line.contains("ab") || line.contains("cd") || line.contains("pq") || line.contains("xy"))
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .filter(|line| has_3_vowels(line))
            .filter(|line| has_duplicates(line))
            .filter(banned_groups)
            .count(),
    )
}

fn double_duplicate(line: &&str) -> bool {
    let re = Regex::new(r"(\w\w)\w*\1").unwrap();
    re.is_match(line).unwrap()
}
fn aba(line: &&str) -> bool {
    let re = Regex::new(r"(\w)\w\1").unwrap();
    re.is_match(line).unwrap()
}

// Sadly very slow
pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .filter(|line| double_duplicate(line))
            .filter(|line| aba(line))
            .count(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
