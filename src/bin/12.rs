use std::{collections::HashMap, iter::repeat};

use itertools::Itertools;

advent_of_code::solution!(12);

#[derive(Debug, Clone)]
struct Record {
    schematic: Vec<char>,
    validation: Vec<u32>,
}

impl Record {
    fn from_line(line: &str) -> Record {
        let (schematic, validation) = line.split_once(" ").unwrap();
        let schematic = schematic.chars().collect();
        let validation = validation.split(',').map(|c| c.parse().unwrap()).collect();
        Record {
            schematic,
            validation,
        }
    }
    fn from_line_5x(line: &str) -> Record {
        let (schematic, validation) = line.split_once(" ").unwrap();
        let schematic: Vec<char> = schematic.chars().collect();
        let schematic5: Vec<char> = repeat(schematic).take(5).collect_vec().join(&'?');
        let validation: Vec<u32> = validation.split(',').map(|c| c.parse().unwrap()).collect();
        let validation5: Vec<u32> = repeat(validation).take(5).concat();
        Record {
            schematic: schematic5,
            validation: validation5,
        }
    }

    fn check_pattern_suffix(&self, last: usize) -> bool {
        for sch_idx in self.schematic.len() - last..self.schematic.len() {
            if self.schematic[sch_idx] == '#' {
                return false;
            }
        }
        true
    }
    fn check_pattern_prefix(&self, start_idx: usize, buffer: usize, validation_idx: usize) -> bool {
        for idx in 0..buffer {
            if self.schematic[start_idx + idx] == '#' {
                return false;
            }
        }
        for idx in 0..self.validation[validation_idx] as usize {
            if start_idx + buffer + idx >= self.schematic.len() {
                return false;
            }
            if self.schematic[start_idx + buffer + idx] == '.' {
                return false;
            }
        }
        true
    }

    fn get_max_buffer(&self) -> usize {
        self.schematic.len() - self.validation.iter().sum::<u32>() as usize
    }

    fn combinations(
        &self,
        start_idx: usize,
        max_buffer: usize,
        validation_idx: usize,
        memoize: &mut HashMap<(usize, usize, usize), u64>,
        depth: usize,
    ) -> u64 {
        if validation_idx == self.validation.len() {
            if self.check_pattern_suffix(max_buffer) {
                return 1;
            } else {
                return 0;
            };
        }
        let min_buffer = if validation_idx == 0 { 0 } else { 1 };
        let mut sum = 0;
        for buffsize in min_buffer..=min_buffer + max_buffer {
            if !self.check_pattern_prefix(start_idx, buffsize, validation_idx) {
                continue;
            }
            let next_start = start_idx + buffsize + self.validation[validation_idx] as usize;
            if max_buffer < buffsize {
                break;
            }
            let next_buffer = max_buffer - buffsize;
            let next_validation_idx = validation_idx + 1;
            if let Some(combinations) = memoize.get(&(next_start, next_buffer, next_validation_idx))
            {
                sum += combinations;
            } else {
                let combinations = self.combinations(
                    next_start,
                    next_buffer,
                    next_validation_idx,
                    memoize,
                    depth + 1,
                );
                memoize.insert((next_start, next_buffer, next_validation_idx), combinations);
                sum += combinations;
            }
        }
        sum
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let records: Vec<Record> = input.lines().map(Record::from_line).collect();
    let mut sum = 0;
    for record in records {
        let max_buffer = record.get_max_buffer();
        let mut memoize: HashMap<(usize, usize, usize), u64> = HashMap::new();
        let comb = record.combinations(0, max_buffer, 0, &mut memoize, 0);
        sum += comb;
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let records: Vec<Record> = input.lines().map(Record::from_line_5x).collect();
    let mut sum = 0;
    for record in records {
        let max_buffer = record.get_max_buffer();
        let mut memoize: HashMap<(usize, usize, usize), u64> = HashMap::new();
        sum += record.combinations(0, max_buffer, 0, &mut memoize, 0);
    }
    Some(sum)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
