use std::str::Lines;

use nom::Err;

fn get_next_num(input: &str) -> nom::IResult<&str, u64, (&str, nom::error::ErrorKind)> {
    let (input, _) = nom::bytes::complete::take_while(|c: char| !c.is_numeric())(input)?;
    let (input, num) = nom::bytes::complete::take_while(|c: char| c.is_numeric())(input)?;
    if num.is_empty() {
        return Err(Err::Error((input, nom::error::ErrorKind::Digit)));
    }
    Ok((input, num.parse().unwrap()))
}

pub fn get_nums_from_line(input: &str) -> Vec<u64> {
    let mut nums = Vec::new();
    let mut input = input;
    loop {
        let (next_input, num) = get_next_num(input).unwrap();
        nums.push(num);
        if next_input.is_empty() {
            break;
        }
        input = next_input;
    }
    nums
}

pub trait LinesExt<T> {
    fn numbers(&mut self) -> impl Iterator<Item = Vec<T>>;
}

impl LinesExt<u64> for Lines<'_> {
    fn numbers(&mut self) -> impl Iterator<Item = Vec<u64>> {
        self.map(|line| get_nums_from_line(line))
    }
}
