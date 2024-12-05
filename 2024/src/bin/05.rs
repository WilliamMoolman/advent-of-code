use std::{cmp::Ordering, collections::HashMap};

advent_of_code::solution!(5);

struct Printer {
    rules: HashMap<usize, Vec<usize>>,
    updates: Vec<Vec<usize>>,
}

impl Printer {
    fn new(rule_list: Vec<&str>, update_list: Vec<&str>) -> Printer {
        let mut rules: HashMap<usize, Vec<usize>> = HashMap::new();
        rule_list
            .iter()
            .map(|rule| {
                let (a, b) = rule.split_once("|").unwrap();
                (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap())
            })
            .for_each(|(a, b)| {
                if rules.contains_key(&b) {
                    rules.get_mut(&b).unwrap().push(a);
                } else {
                    rules.insert(b, vec![a]);
                }
            });
        let updates = update_list
            .iter()
            .map(|line| {
                line.split(",")
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect()
            })
            .collect();
        Printer { rules, updates }
    }

    fn get_good_updates(&self) -> Vec<Vec<usize>> {
        let empty_vec = Vec::new();
        self.updates
            .iter()
            .filter(|update| {
                update.iter().enumerate().all(|(i, a)| {
                    let preceeds = self.rules.get(a).unwrap_or(&empty_vec);
                    !update[i + 1..].iter().any(|b| preceeds.contains(b))
                })
            })
            .cloned()
            .collect()
    }

    fn get_bad_updates(&self) -> Vec<Vec<usize>> {
        let empty_vec = Vec::new();
        self.updates
            .iter()
            .filter(|update| {
                !update.iter().enumerate().all(|(i, a)| {
                    let preceeds = self.rules.get(a).unwrap_or(&empty_vec);
                    !update[i + 1..].iter().any(|b| preceeds.contains(b))
                })
            })
            .cloned()
            .collect()
    }

    fn fix_updates(&self, updates: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        updates
            .iter()
            .map(|update| {
                let mut new_update = update.clone();
                new_update.sort_by(|a, b| {
                    if self.rules.contains_key(b) && self.rules.get(b).unwrap().contains(a) {
                        Ordering::Greater
                    } else if self.rules.contains_key(a) && self.rules.get(a).unwrap().contains(b) {
                        Ordering::Less
                    } else {
                        Ordering::Equal
                    }
                });
                new_update
            })
            .collect()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let printer = Printer::new(
        input.lines().filter(|line| line.contains("|")).collect(),
        input.lines().filter(|line| line.contains(",")).collect(),
    );

    let updates = printer.get_good_updates();
    Some(
        updates
            .iter()
            .map(|line| line[(line.len() - 1) / 2] as u32)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let printer = Printer::new(
        input.lines().filter(|line| line.contains("|")).collect(),
        input.lines().filter(|line| line.contains(",")).collect(),
    );

    let updates = printer.fix_updates(printer.get_bad_updates());

    Some(
        updates
            .iter()
            .map(|line| line[(line.len() - 1) / 2] as u32)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
