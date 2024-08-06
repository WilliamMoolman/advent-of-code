use num::abs;

advent_of_code::solution!(11);

/*
Read in universe
expand universe
find galaxies
perform dijkstra <-- actually just get abs diff in r,c idx
sum the lengths
*/
pub fn part_one(input: &str) -> Option<i64> {
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    let mut universe: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .flat_map(|line| {
            if line.iter().all(|&c| c == '.') {
                vec![line.clone(), line]
            } else {
                vec![line]
            }
        })
        .collect();
    let mut c = 0;
    let mut columns = universe[0].len();
    while c < columns {
        let mut empty_col = true;
        for row in universe.iter() {
            if row[c] == '#' {
                empty_col = false;
            }
        }
        if empty_col {
            for row in universe.iter_mut() {
                row.insert(c, '.');
            }
            c += 1;
            columns += 1;
        }
        c += 1;
    }

    for (r_idx, row) in universe.iter().enumerate() {
        for (c_idx, c) in row.iter().enumerate() {
            if *c == '#' {
                galaxies.push((r_idx, c_idx));
            }
        }
    }

    let mut cost: i64 = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let (r1, c1) = galaxies[i];
            let (r2, c2) = galaxies[j];
            cost += abs(r1 as i64 - r2 as i64) + abs(c1 as i64 - c2 as i64);
        }
    }
    Some(cost)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    let mut extra_rows: Vec<usize> = Vec::new();
    let mut extra_cols: Vec<usize> = Vec::new();
    let universe: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    for (r_idx, row) in universe.iter().enumerate() {
        if row.iter().all(|&c| c == '.') {
            extra_rows.push(r_idx);
        }
    }

    let mut c = 0;
    let columns = universe[0].len();
    while c < columns {
        let mut empty_col = true;
        for row in universe.iter() {
            if row[c] == '#' {
                empty_col = false;
            }
        }
        if empty_col {
            extra_cols.push(c);
        }
        c += 1;
    }

    let expansion = 1000000 - 1;
    for (r_idx, row) in universe.iter().enumerate() {
        for (c_idx, c) in row.iter().enumerate() {
            if *c == '#' {
                // Calculate effective idx
                let eff_r =
                    extra_rows.iter().take_while(|&&r| r < r_idx).count() * expansion + r_idx;
                let eff_c =
                    extra_cols.iter().take_while(|&&c| c < c_idx).count() * expansion + c_idx;
                galaxies.push((eff_r, eff_c));
            }
        }
    }

    let mut cost: i64 = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let (r1, c1) = galaxies[i];
            let (r2, c2) = galaxies[j];
            cost += abs(r1 as i64 - r2 as i64) + abs(c1 as i64 - c2 as i64);
        }
    }
    Some(cost)
}

#[allow(dead_code)]
fn print_board(board: &Vec<Vec<char>>) {
    for line in board.iter() {
        println!("{}", line.iter().collect::<String>());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
