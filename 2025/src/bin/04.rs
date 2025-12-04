use advent_of_code::utils::Grid;

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
    let mut count = 0;
    loop {
        let coords = grid.coordinates();
        let removable = coords
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
            .collect::<Vec<_>>();
        if removable.len() == 0 {
            break;
        }
        count += removable.len();
        for (_, c) in removable {
            grid.set(c, '.');
        }
    }
    Some(count as u64)
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
