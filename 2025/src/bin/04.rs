use advent_of_code::utils::{Coord, Grid};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let grid = Grid::from_input(input, |c| c);
    let count = grid
        .coordinates()
        .iter()
        .filter(|(item, c)| {
            *item == '@'
                && grid
                    .neighbours8(c)
                    .iter()
                    .filter(|c| grid.at(c) == '@')
                    .count()
                    < 4
        })
        .count();
    Some(count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = Grid::from_input(input, |c| c);
    let mut papers: Vec<Coord> = grid
        .coordinates_iter()
        .filter(|(item, _)| *item == '@')
        .map(|(_, c)| c)
        .collect();
    let total_papers = papers.len();
    loop {
        let mut to_remove = vec![];
        papers.retain(|c| {
            let rem = grid.neighbours8_sat(c, |c| c == '@') < 4;
            if rem {
                to_remove.push(c.clone());
            }
            !rem
        });
        if to_remove.len() == 0 {
            break;
        }
        for c in to_remove {
            grid.set(&c, '.')
        }
    }
    Some(total_papers as u64 - papers.len() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
