advent_of_code::solution!(7);

#[derive(Debug)]
struct Equation {
    total: u64,
    values: Vec<u64>,
}

impl Equation {
    fn from_input(input: &Vec<&str>) -> Vec<Equation> {
        input
            .iter()
            .map(|line| {
                let (raw_total, raw_values) = line.split_once(": ").unwrap();
                let total = raw_total.parse().unwrap();
                let values = raw_values
                    .split(" ")
                    .map(|i| i.parse::<u64>().unwrap())
                    .collect();
                Equation { total, values }
            })
            .collect()
    }

    fn is_valid(&self, concat: bool) -> bool {
        if self.values.len() == 1 {
            self.total == self.values[0]
        } else if self.total < self.values[0] {
            false
        } else {
            let mut add_val = vec![self.values[0] + self.values[1]];
            add_val.extend(&self.values[2..]);
            let add_eq = Equation {
                total: self.total,
                values: add_val,
            };
            let mut mul_val = vec![self.values[0] * self.values[1]];
            mul_val.extend(&self.values[2..]);
            let mul_eq = Equation {
                total: self.total,
                values: mul_val,
            };
            let mut cct_val = vec![format!("{}{}", self.values[0], self.values[1])
                .parse::<u64>()
                .unwrap()];
            cct_val.extend(&self.values[2..]);
            let cct_eq = Equation {
                total: self.total,
                values: cct_val,
            };
            (concat && cct_eq.is_valid(concat))
                || mul_eq.is_valid(concat)
                || add_eq.is_valid(concat)
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let equations = Equation::from_input(&input.lines().collect());

    Some(
        equations
            .iter()
            .filter(|eq| eq.is_valid(false))
            .map(|eq| eq.total)
            .sum::<u64>() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations = Equation::from_input(&input.lines().collect());

    Some(
        equations
            .iter()
            .filter(|eq| eq.is_valid(true))
            .map(|eq| eq.total)
            .sum::<u64>() as u64,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
