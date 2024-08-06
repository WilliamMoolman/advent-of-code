advent_of_code::solution!(13);

#[derive(Debug)]
struct Pattern(Vec<Vec<char>>);

impl Pattern {
    fn get_horizontal(&self) -> Option<u64> {
        for (i, line) in self.0.iter().enumerate() {
            if i == self.0.len() - 1 {
                // println!("Skipping line at borders {}", i);
                continue;
            }
            let mut rotates = true;
            let mut j = 0;
            while i >= j && i + j < self.0.len() - 1 {
                if self.0[i - j] != self.0[i + j + 1] {
                    rotates = false;
                    // println!("Line {} does not match", i);
                    break; // Breaks
                }
                j += 1;
            }
            if rotates {
                return Some((i + 1) as u64);
            }
        }
        None
    }
    fn get_vertical(&self) -> Option<u64> {
        for i in 0..self.0[0].len() {
            // println!("I: {i}");
            if i == self.0[0].len() - 1 {
                // println!("  not suitable");
                continue;
            }
            let mut rotates = true;
            let mut j = 0;
            'outer: while i >= j && i + j < self.0[0].len() - 1 {
                // println!("  j: {j}");
                for k in 0..self.0.len() {
                    if self.0[k][i - j] != self.0[k][i + j + 1] {
                        // println!("    breaks on row:{k} reflection: {}||{}", i - j, i + j + 1);
                        rotates = false;
                        break 'outer; // Breaks
                    }
                }
                j += 1;
            }
            if rotates {
                // println!("REFLECTS!");
                return Some((i + 1) as u64);
            }
        }
        None
    }
    fn get_horizontal_smudge(&self) -> Option<u64> {
        for (i, line) in self.0.iter().enumerate() {
            if i == self.0.len() - 1 {
                // println!("Skipping line at borders {}", i);
                continue;
            }
            let mut errors = 0;
            let mut j = 0;
            'outer: while i >= j && i + j < self.0.len() - 1 {
                for k in 0..self.0[0].len() {
                    if self.0[i - j][k] != self.0[i + j + 1][k] {
                        errors += 1;
                        if errors > 1 {
                            // println!("Line {} does not match", i);
                            break 'outer; // Breaks
                        }
                    }
                }

                j += 1;
            }
            if errors == 1 {
                return Some((i + 1) as u64);
            }
        }
        None
    }
    fn get_vertical_smudge(&self) -> Option<u64> {
        for i in 0..self.0[0].len() {
            // println!("I: {i}");
            if i == self.0[0].len() - 1 {
                // println!("  not suitable");
                continue;
            }
            let mut errors = 0;
            let mut j = 0;
            'outer: while i >= j && i + j < self.0[0].len() - 1 {
                // println!("  j: {j}");
                for k in 0..self.0.len() {
                    if self.0[k][i - j] != self.0[k][i + j + 1] {
                        errors += 1;
                        if errors > 1 {
                            break 'outer;
                        }
                    }
                }
                j += 1;
            }
            if errors == 1 {
                // println!("REFLECTS!");
                return Some((i + 1) as u64);
            }
        }
        None
    }
}
pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let mut patterns = Vec::new();
    loop {
        let pattern_grid: Vec<Vec<char>> = lines
            .by_ref()
            .take_while(|l| l.len() > 0)
            .map(|l| l.chars().collect())
            .collect();
        if pattern_grid.len() == 0 {
            break;
        }
        // lines.next();
        patterns.push(Pattern(pattern_grid))
    }

    let sum = patterns
        .iter()
        .map(|p| {
            if let Some(row) = p.get_horizontal() {
                row * 100
            } else if let Some(col) = p.get_vertical() {
                col
            } else {
                for l in &p.0 {
                    println!("{}", l.iter().collect::<String>())
                }
                panic!("No pattern")
            }
        })
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let mut patterns = Vec::new();
    loop {
        let pattern_grid: Vec<Vec<char>> = lines
            .by_ref()
            .take_while(|l| l.len() > 0)
            .map(|l| l.chars().collect())
            .collect();
        if pattern_grid.len() == 0 {
            break;
        }
        // lines.next();
        patterns.push(Pattern(pattern_grid))
    }

    let sum = patterns
        .iter()
        .map(|p| {
            if let Some(row) = p.get_horizontal_smudge() {
                row * 100
            } else if let Some(col) = p.get_vertical_smudge() {
                col
            } else {
                for l in &p.0 {
                    println!("{}", l.iter().collect::<String>())
                }
                panic!("No pattern")
            }
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
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
