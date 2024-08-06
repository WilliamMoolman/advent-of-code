use core::panic;
use std::collections::{HashMap, HashSet};

use indicatif::ProgressIterator;

advent_of_code::solution!(18);

#[derive(Debug)]
struct Bounds(i64, i64, i64, i64);

impl Bounds {
    fn extend(&mut self, pos: (i64, i64)) {
        if pos.0 < self.0 {
            self.0 = pos.0;
        }
        if pos.0 > self.1 {
            self.1 = pos.0;
        }
        if pos.1 < self.2 {
            self.2 = pos.1;
        }
        if pos.1 > self.3 {
            self.3 = pos.1;
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut trench = HashMap::new();
    let mut pos = (0, 0);
    trench.insert(pos, "");
    let mut bounds = Bounds(0, 0, 0, 0);
    for line in input.lines() {
        let (dir, line) = line.split_once(' ')?;
        let (dist, rgb) = line.split_once(' ')?;
        let dist: i64 = dist.parse().unwrap();
        let mut dig_trench = |dist: i64, dir: (i64, i64)| {
            for i in 0..dist {
                pos.0 += dir.0;
                pos.1 += dir.1;
                bounds.extend(pos);
                // println!("{pos:?}");
                trench.insert(pos, rgb);
            }
        };
        match dir {
            "U" => dig_trench(dist, (0, -1)), //pos.1 -= dist,
            "D" => dig_trench(dist, (0, 1)),
            "L" => dig_trench(dist, (-1, 0)),
            "R" => dig_trench(dist, (1, 0)),
            _ => panic!("Unknown instruction!"),
        }
    }

    // Odd line theorem?
    let mut volume = 0;
    println!("Bounds: {bounds:?}");
    for y in bounds.2 - 1..=bounds.3 + 1 {
        let mut inside = false;
        let mut in_wall = false;
        let mut start_wall_type = "";
        let mut wall_type = "";
        for x in bounds.0 - 1..=bounds.1 + 1 {
            if trench.contains_key(&(x, y)) {
                if !in_wall {
                    // Entering wall
                    start_wall_type = match (
                        trench.contains_key(&(x, y - 1)),
                        trench.contains_key(&(x, y + 1)),
                    ) {
                        (true, true) => "S",
                        (true, false) => "U",
                        (false, true) => "L",
                        _ => panic!("Non closed loop!"),
                    };
                    inside = !inside;
                    in_wall = true;
                }
                wall_type = match (
                    trench.contains_key(&(x, y - 1)),
                    trench.contains_key(&(x, y + 1)),
                ) {
                    (true, true) => "S",
                    (true, false) => "U",
                    (false, true) => "L",
                    _ => "-",
                };
            } else {
                if in_wall {
                    // Exiting wall
                    match (start_wall_type, wall_type) {
                        ("U", "U") | ("L", "L") => inside = !inside,
                        _ => (),
                    };
                }
                in_wall = false;
            }
            if in_wall || inside {
                volume += 1;
            }
        }
    }
    Some(volume)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut trench_rows = HashMap::new();
    let (mut row_idx, mut col_idx) = (0, 0);
    let mut bounds = Bounds(0, 0, 0, 0);
    for line in input.lines() {
        let (_, line) = line.split_once(' ')?;
        let (_, rgb) = line.split_once(' ')?;
        let mut rgb = rgb.chars();
        rgb.next();
        rgb.next();
        let hex = rgb.by_ref().take(5).collect::<String>();
        let dist = i64::from_str_radix(&hex, 16).unwrap();
        let dir = rgb.next().unwrap();

        match dir {
            '3' | '1' => {
                let dir_v = if dir == '3' { -1 } else { 1 };
                row_idx += dir_v;
                for _ in 1..dist - 1 {
                    if !trench_rows.contains_key(&row_idx) {
                        trench_rows.insert(row_idx, vec![]);
                    }
                    let row = trench_rows.get_mut(&row_idx).unwrap();
                    bounds.extend((row_idx, col_idx));

                    row.push((col_idx, 'V'));
                    row_idx += dir_v;
                }
                if !trench_rows.contains_key(&row_idx) {
                    trench_rows.insert(row_idx, vec![]);
                }
                let row = trench_rows.get_mut(&row_idx).unwrap();
                bounds.extend((row_idx, col_idx));
                if !row.contains(&(col_idx, 'B')) && !row.contains(&(col_idx, 'E')) {
                    row.push((col_idx, 'V'));
                }
                row_idx += dir_v;
            }
            '2' | '0' => {
                if !trench_rows.contains_key(&row_idx) {
                    trench_rows.insert(row_idx, vec![]);
                }

                let row = trench_rows.get_mut(&row_idx).unwrap();

                row.push((col_idx, if dir == '0' { 'B' } else { 'E' }));
                // print!("Ceiling col:{col_idx}->");
                bounds.extend((row_idx, col_idx));
                if dir == '0' {
                    col_idx += dist;
                } else {
                    col_idx -= dist;
                }
                bounds.extend((row_idx, col_idx));
                // println!("{col_idx} row:{row_idx}");
                row.push((col_idx, if dir == '0' { 'E' } else { 'B' }));
            } //R0
            _ => panic!("Unknown instruction!"),
        }
    }

    // Sort Rows
    for row in trench_rows.values_mut() {
        row.sort_by(|(a, _), (b, _)| a.cmp(b));
    }

    // Odd line theorem?
    let mut volume: u64 = 0;
    println!("Bounds: {bounds:?}");

    for row_y in bounds.0..bounds.1 + 1 {
        let mut prev_inside = false;
        let mut inside = false;
        let mut in_wall = false;
        let mut row_start = 0;
        let mut start_wall_type = "";
        let mut wall_type;
        let row = trench_rows.get(&row_y).unwrap();
        for (col_x, id) in row {
            if id == &'B' {
                if inside {
                    volume += (*col_x - row_start) as u64;
                } else {
                    volume += 1;
                }
                row_start = *col_x;
                let upper = if let Some(hm) = trench_rows.get(&(row_y - 1)) {
                    hm.contains(&(*col_x, 'V'))
                        || hm.contains(&(*col_x, 'B'))
                        || hm.contains(&(*col_x, 'E'))
                } else {
                    false
                };
                let lower = if let Some(hm) = trench_rows.get(&(row_y + 1)) {
                    hm.contains(&(*col_x, 'V'))
                        || hm.contains(&(*col_x, 'B'))
                        || hm.contains(&(*col_x, 'E'))
                } else {
                    false
                };
                // Entering wall
                start_wall_type = match (upper, lower) {
                    (true, true) => "S",
                    (true, false) => "U",
                    (false, true) => "L",
                    _ => {
                        println!("DEBUG: row:{row_y} col:{col_x}");
                        panic!("Non closed loop!")
                    }
                };
                prev_inside = inside;
                inside = !inside;
                in_wall = true;
            } else if id == &'E' {
                let upper = if let Some(hm) = trench_rows.get(&(row_y - 1)) {
                    hm.contains(&(*col_x, 'V'))
                        || hm.contains(&(*col_x, 'B'))
                        || hm.contains(&(*col_x, 'E'))
                } else {
                    false
                };
                let lower = if let Some(hm) = trench_rows.get(&(row_y + 1)) {
                    hm.contains(&(*col_x, 'V'))
                        || hm.contains(&(*col_x, 'B'))
                        || hm.contains(&(*col_x, 'E'))
                } else {
                    false
                };
                // Entering wall
                wall_type = match (upper, lower) {
                    (true, true) => "S",
                    (true, false) => "U",
                    (false, true) => "L",
                    _ => {
                        panic!("Non closed loop!")
                    }
                };
                in_wall = false;
                volume += (*col_x - row_start) as u64;

                row_start = *col_x;

                // Exiting wall
                match (start_wall_type, wall_type) {
                    ("U", "U") | ("L", "L") => {
                        prev_inside = !prev_inside;
                        inside = !inside;
                    }
                    _ => (),
                };
            } else if id == &'V' {
                if inside {
                    volume += (*col_x - row_start) as u64;
                } else {
                    volume += 1;
                }
                row_start = *col_x;
                inside = !inside;
            }
        }
        assert!(!in_wall);
        assert!(!inside);
    }
    Some(volume)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
