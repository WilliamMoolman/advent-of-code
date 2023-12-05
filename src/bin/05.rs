use std::collections::HashMap;

advent_of_code::solution!(5);

struct Offset {
    start: i64,
    end: i64,
    offset: i64,
}

impl Offset {
    fn convert(&self, value: i64) -> Option<i64> {
        if value >= self.start && value < self.end {
            Some(self.offset - self.start + value)
        } else {
            None
        }
    }
}

struct Converter {
    offsets: Vec<Offset>,
}

impl Converter {
    fn new() -> Converter {
        Converter { offsets: vec![] }
    }
    fn add_from_line(&mut self, line: &str) {
        let mut l_tokens = line.split_ascii_whitespace();
        let offset = l_tokens.next().unwrap().parse().unwrap();
        let start = l_tokens.next().unwrap().parse().unwrap();
        let length: i64 = l_tokens.next().unwrap().parse().unwrap();
        let o = Offset {
            start,
            end: start + length,
            offset,
        };
        self.offsets.push(o);
    }

    fn convert(&self, value: i64) -> i64 {
        for offset in self.offsets.iter() {
            if let Some(new_value) = offset.convert(value) {
                return new_value;
            }
        }
        value
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut hm: HashMap<String, Converter> = HashMap::new();
    let mut lines = input.lines();
    let mut first_line = lines.next().unwrap().split_ascii_whitespace();
    first_line.next();
    let seeds: Vec<i64> = first_line
        .map(|x| x.parse().expect(&format!("These should be nums {x}")))
        .collect();
    lines.next();
    while let Some(line) = lines.next() {
        let (name, _) = line.split_once(' ').unwrap();
        let mut converter = Converter::new();
        while let Some(line) = lines.next() {
            if line.len() < 1 {
                break;
            }
            converter.add_from_line(line);
        }
        hm.insert(name.to_string(), converter);
    }

    seeds
        .iter()
        .map(|&seed| hm.get("seed-to-soil").unwrap().convert(seed))
        .map(|soil| hm.get("soil-to-fertilizer").unwrap().convert(soil))
        .map(|fertilizer| hm.get("fertilizer-to-water").unwrap().convert(fertilizer))
        .map(|water| hm.get("water-to-light").unwrap().convert(water))
        .map(|light| hm.get("light-to-temperature").unwrap().convert(light))
        .map(|temperature| {
            hm.get("temperature-to-humidity")
                .unwrap()
                .convert(temperature)
        })
        .map(|humidity| hm.get("humidity-to-location").unwrap().convert(humidity))
        .min()
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut hm: HashMap<String, Converter> = HashMap::new();
    let mut lines = input.lines();
    let mut first_line = lines.next().unwrap().split_ascii_whitespace();
    first_line.next();
    let seeds: Vec<i64> = first_line
        .map(|x| x.parse().expect(&format!("These should be nums {x}")))
        .collect();
    lines.next();
    while let Some(line) = lines.next() {
        let (name, _) = line.split_once(' ').unwrap();
        let mut converter = Converter::new();
        while let Some(line) = lines.next() {
            if line.len() < 1 {
                break;
            }
            converter.add_from_line(line);
        }
        hm.insert(name.to_string(), converter);
    }

    seeds
        .chunks(2)
        .flat_map(|chunk: &[i64]| chunk[0]..chunk[0] + chunk[1])
        .map(|seed| hm.get("seed-to-soil").unwrap().convert(seed))
        .map(|soil| hm.get("soil-to-fertilizer").unwrap().convert(soil))
        .map(|fertilizer| hm.get("fertilizer-to-water").unwrap().convert(fertilizer))
        .map(|water| hm.get("water-to-light").unwrap().convert(water))
        .map(|light| hm.get("light-to-temperature").unwrap().convert(light))
        .map(|temperature| {
            hm.get("temperature-to-humidity")
                .unwrap()
                .convert(temperature)
        })
        .map(|humidity| hm.get("humidity-to-location").unwrap().convert(humidity))
        .min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
