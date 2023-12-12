use memoize::memoize;
use std::iter::repeat;

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

    fn schematic_as(&self, mask: usize) -> Vec<char> {
        Record::schematic_as_static(&self.schematic, mask)
    }

    fn schematic_as_static(schematic: &Vec<char>, mask: usize) -> Vec<char> {
        let mut n = -1;
        schematic
            .iter()
            .map(|c| match c {
                '?' => {
                    n += 1;
                    if mask & (1 << n) == 0 {
                        '.'
                    } else {
                        '#'
                    }
                }
                _ => *c,
            })
            .collect()
    }

    fn does_schematic_fit(&self, schematic: &Vec<char>) -> bool {
        let hashtags: Vec<usize> = schematic
            .iter()
            .group_by(|&&c| c == '#')
            .into_iter()
            .filter_map(|(v, tags)| if v { Some(tags.count()) } else { None })
            .collect();
        if self.validation.len() != hashtags.len() {
            return false;
        }
        for (i, &valid_size) in self.validation.iter().enumerate() {
            if valid_size as usize != hashtags[i] {
                return false;
            }
        }
        return true;
    }

    fn how_much_does_schematic_fit(schematic: &Vec<char>, validation: &[u32]) -> Option<usize> {
        let hashtags: Vec<usize> = schematic
            .iter()
            .group_by(|&&c| c == '#')
            .into_iter()
            .filter_map(|(v, tags)| if v { Some(tags.count()) } else { None })
            .collect();
        if validation.len() < hashtags.len() {
            return None;
        }
        for (i, &tag_size) in hashtags.iter().enumerate() {
            if validation[i] as usize != tag_size {
                return None;
            }
        }

        return Some(validation.len() - hashtags.len());
    }

    fn schematic_as_grouped(&self, mask: usize) -> Vec<Vec<char>> {
        let mut n = -1;
        let mut groups: Vec<Vec<char>> = Vec::new();
        let mut schematic_iter = self.schematic.iter();
        loop {
            let _ = schematic_iter.by_ref().take_while(|&&c| c == '.');
            let group = schematic_iter
                .by_ref()
                .take_while(|&&c| c != '.')
                .copied()
                .collect_vec();
            if group.len() == 0 {
                break;
            }
            groups.push(group);
        }
        groups
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let records: Vec<Record> = input.lines().map(Record::from_line).collect();
    let mut combinations = 0;
    for record in records.iter() {
        // println!("{record:?}");
        let num_question = record.schematic.iter().filter(|&&c| c == '?').count();
        // println!("?x{num_question}");
        for i in 0..2usize.pow(num_question as u32) {
            let new_schem = record.schematic_as(i);
            let works = record.does_schematic_fit(&new_schem);

            if works {
                combinations += 1;
            }
        }
    }
    Some(combinations)
}

fn get_counts_for_group(schematic: &Vec<char>, validation_remaining: &[u32]) -> Vec<u64> {
    // Need to remove this part!!!! grows to like >2^40
    // let num_question = current_group.iter().filter(|&&c| c == '?').count();
    // for i in 0..2usize.pow(num_question as u32) {
    //     let new_schem = Record::schematic_as_static(current_group, i);
    //     if let Some(num) = Record::how_much_does_schematic_fit(&new_schem, validation_remaining) {
    //         counts[num] += 1;
    //     }
    // }
    //
    println!("GROUP: {}", schematic.iter().collect::<String>());
    let min_version: Vec<char> = schematic
        .iter()
        .map(|&c| match c {
            '?' => '.',
            _ => c,
        })
        .collect();
    let max_version: Vec<char> = schematic
        .iter()
        .map(|&c| match c {
            '?' => '#',
            _ => c,
        })
        .collect();

    let mut min: Vec<usize> = Vec::new();
    let mut max: Vec<usize> = Vec::new();

    let mut idx = 0;
    while idx < min_version.len() {
        if min_version[idx] == '.' {
            min.push(0);
            idx += 1;
            continue;
        }
        let mut j = 0;
        while idx + j < min_version.len() && min_version[idx + j] == '#' {
            j += 1;
        }
        for _ in 0..j {
            min.push(j);
        }
        idx += j;
    }
    let mut idx = 0;
    while idx < max_version.len() {
        if max_version[idx] == '.' {
            max.push(0);
            idx += 1;
            continue;
        }
        let mut j = 0;
        while idx + j < max_version.len() && max_version[idx + j] == '#' {
            j += 1;
        }
        for _ in 0..j {
            max.push(j);
        }
        idx += j;
    }

    println!(
        "MIN:   {}",
        min.iter().map(|d| d.to_string()).collect::<String>()
    );
    println!(
        "MAX:   {}",
        max.iter().map(|d| d.to_string()).collect::<String>()
    );
    // Call this the flattening
    let mut counts = vec![0; validation_remaining.len() + 1];
    // let s_idx = 0;
    // let v_idx = 0;

    // Need to remove this part!!!! grows to like >2^40

    let num_question = schematic.iter().filter(|&&c| c == '?').count();
    for i in 0..2usize.pow(num_question as u32) {
        let new_schem = Record::schematic_as_static(schematic, i);
        if let Some(num) = Record::how_much_does_schematic_fit(&new_schem, validation_remaining) {
            counts[num] += 1;
        }
    }
    counts
}

fn get_combinations(schematic_groups: &[Vec<char>], validation_remaining: &[u32]) -> u64 {
    if schematic_groups.len() == 0 {
        return 0;
    }
    let current_group = &schematic_groups[0];
    let counts = get_counts_for_group(current_group, validation_remaining);

    //
    let mut total = counts[0];
    for i in 1..counts.len() {
        if counts[i] == 0 {
            continue;
        }
        total += counts[i] * get_combinations(&schematic_groups[1..], &validation_remaining[i..]);
    }
    total
}

fn rec_get_combinations(schematic: &[char], validation: &[u32]) -> u64 {
    if schematic[0] == '?' {
        return rec_get_combinations(schematic, validation);
    }
    let mut n = -1;
    let mut groups: Vec<Vec<char>> = Vec::new();
    let mut schematic_iter = self.schematic.iter();
    loop {
        let _ = schematic_iter.by_ref().take_while(|&&c| c == '.');
        let group = schematic_iter
            .by_ref()
            .take_while(|&&c| c != '.')
            .copied()
            .collect_vec();
        if group.len() == 0 {
            break;
        }
        groups.push(group);
    }
    groups
}

pub fn part_two(input: &str) -> Option<u64> {
    let records: Vec<Record> = input.lines().map(Record::from_line_5x).collect();
    let mut combinations = 0;
    for record in records.iter() {
        println!("{record:?} ==>");
        // let num_question = record.schematic.iter().filter(|&&c| c == '?').count();
        // let groups: Vec<Vec<char>> = record.schematic_as_grouped(0);
        // let total = get_combinations(&groups, &record.validation[..]);
        // println!("<== Total: {total}");
        // combinations += total;
        let num_question = record.schematic.iter().filter(|&&c| c == '?').count();

        for i in 0..2usize.pow(num_question as u32) {
            let new_schem = record.schematic_as(i);
            let works = record.does_schematic_fit(&new_schem);

            if works {
                combinations += 1;
            }
        }

        // return None;
        // println!("?x{num_question}");
        // println!("Iterating over {:e}", 2usize.pow(num_question as u32));
        // for i in 0..2usize.pow(num_question as u32) {
        //     let new_schem = record.schematic_as(i);
        //     let works = record.does_schematic_fit(&new_schem);

        //     if works {
        //         combinations += 1;
        //     }
        // }
    }
    Some(combinations)
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
