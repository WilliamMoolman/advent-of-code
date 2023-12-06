advent_of_code::solution!(6);

/*
Distance - d, Time - t, Charge up Time - n, Record - R
d(n) = (t-n)*n
d(n) = t*n - n^2
R = d(n) = t*n - n^2; n^2 - t*n + d(n) = 0; n = (t +- sqrt(t^2 - 4*d(n)))/2
Rlower = (t - sqrt(t^2 - 4*R))/2
Rupper = (t + sqrt(t^2 - 4*R))/2
Range of Inputs = ceil(Rlower) to floor(Rupper); Range = floor(Rupper) - ceil(Rlower) + 1
!!! If Rlower is a whole number, add 1 to it, if Rupper is a whole number, subtract 1 from it
*/

fn get_margin(time: u64, record: u64) -> u64 {
    let time = time as f64;
    let record = record as f64;
    let mut lower = (time - (time.powf(2.0) - 4.0 * record).sqrt()) / 2.0;
    let mut upper = (time + (time.powf(2.0) - 4.0 * record).sqrt()) / 2.0;

    if lower.ceil() == lower {
        lower += 1.0;
    }
    if upper.floor() == upper {
        upper -= 1.0;
    }

    upper.floor() as u64 - lower.ceil() as u64 + 1
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let total = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .zip(lines.next().unwrap().split_ascii_whitespace())
        .map(|(time, record)| {
            if time.chars().nth(0).unwrap().is_alphabetic() {
                return 1;
            };
            get_margin(time.parse().unwrap(), record.parse().unwrap())
        })
        .product();

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let mut time = lines.next().unwrap().to_string();
    time.retain(|c| c.is_numeric());
    let mut record = lines.next().unwrap().to_string();
    record.retain(|c| c.is_numeric());
    let time = time.parse().unwrap();
    let record = record.parse().unwrap();
    let margin = get_margin(time, record);

    Some(margin)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
