advent_of_code::solution!(4);

fn count_xmas(input: &Vec<Vec<char>>) -> u32 {
    input
        .iter()
        .map(|row| {
            let mut count = 0;
            for i in 0..row.len() - 3 {
                let word = &row[i..i + 4].iter().collect::<String>();
                if word == "XMAS" || word == "SAMX" {
                    count += 1;
                }
            }
            count
        })
        .sum()
}

fn transpose(mat: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    (0..mat[0].len())
        .map(|i| mat.iter().map(|row| row[i]).collect())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let hor: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let ver = transpose(&hor);

    let diag_one: Vec<Vec<char>> = transpose(
        &hor.iter()
            .enumerate()
            .map(|(i, row)| {
                let mut new_row = vec![' ' as char; i];
                new_row.extend(row);
                new_row.extend(vec![' ' as char; hor.len() - i]);
                new_row
            })
            .collect(),
    );
    let diag_two: Vec<Vec<char>> = transpose(
        &hor.iter()
            .enumerate()
            .map(|(i, row)| {
                let mut new_row = vec![' ' as char; hor.len() - i];
                new_row.extend(row);
                new_row.extend(vec![' ' as char; i]);
                new_row
            })
            .collect(),
    );

    Some(count_xmas(&hor) + count_xmas(&ver) + count_xmas(&diag_one) + count_xmas(&diag_two))
}

fn is_mas(a: char, b: char, c: char) -> bool {
    (a == 'M' && b == 'A' && c == 'S') || (a == 'S' && b == 'A' && c == 'M')
}
fn is_3x3_xmas(input: [[char; 3]; 3]) -> bool {
    is_mas(input[0][0], input[1][1], input[2][2]) && is_mas(input[2][0], input[1][1], input[0][2])
}

pub fn part_two(input: &str) -> Option<u32> {
    let hor: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let total = (0..hor.len() - 2)
        .map(|i| {
            (0..hor[0].len() - 2)
                .map(|j| {
                    if is_3x3_xmas([
                        [hor[i][j], hor[i][j + 1], hor[i][j + 2]],
                        [hor[i + 1][j], hor[i + 1][j + 1], hor[i + 1][j + 2]],
                        [hor[i + 2][j], hor[i + 2][j + 1], hor[i + 2][j + 2]],
                    ]) {
                        1
                    } else {
                        0
                    }
                })
                .sum::<u32>()
        })
        .sum();
    Some(total)
}

// fn print_mat(hor: &Vec<Vec<char>>) {
//     hor.iter().for_each(|row| {
//         println!(
//             "{}",
//             row.iter()
//                 .map(|c| format!("{c} "))
//                 .collect::<Vec<String>>()
//                 .concat()
//         )
//     });
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
