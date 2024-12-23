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

pub fn print_grid(grid: &Vec<Vec<char>>) {
    for line in grid {
        println!("{}", line.iter().collect::<String>());
    }
}
pub trait LinesExt<T> {
    fn numbers(&mut self) -> impl Iterator<Item = Vec<T>>;

    fn to_char_grid(self) -> Vec<Vec<char>>;

    fn coordinates(self) -> impl Iterator<Item = (char, (usize, usize))>;
}

impl LinesExt<u64> for Lines<'_> {
    fn numbers(&mut self) -> impl Iterator<Item = Vec<u64>> {
        self.map(|line| get_nums_from_line(line))
    }

    fn to_char_grid(self) -> Vec<Vec<char>> {
        self.map(|line| line.chars().collect()).collect()
    }

    fn coordinates(self) -> impl Iterator<Item = (char, (usize, usize))> {
        self.enumerate().flat_map(|(r, line)| {
            line.chars()
                .enumerate()
                .map(move |(c, item)| (item, (r + 1, c + 1)))
        })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Coord(pub usize, pub usize);

impl Coord {
    pub fn new(r: usize, c: usize) -> Coord {
        Coord(r, c)
    }
}

pub struct Grid<T> {
    grid: Vec<Vec<T>>,
}

impl<T: Copy> Grid<T> {
    pub fn from_input<F>(input: &str, transform: F) -> Grid<T>
    where
        F: Fn(char) -> T,
    {
        let grid = input
            .lines()
            .map(|line| line.chars().map(|c| transform(c)).collect())
            .collect();
        Grid { grid }
    }
    pub fn coordinates(&self) -> Vec<(T, Coord)> {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(r, line)| {
                line.iter()
                    .enumerate()
                    .map(move |(c, item)| (*item, Coord(r, c)))
            })
            .collect()
    }
    pub fn rlim(&self) -> usize {
        self.grid.len()
    }
    pub fn clim(&self) -> usize {
        self.grid[0].len()
    }
    pub fn n(&self, coord: &Coord) -> Option<Coord> {
        let Coord(r, c) = coord;
        if *r != 0 {
            Some(Coord(*r - 1, *c))
        } else {
            None
        }
    }
    pub fn s(&self, coord: &Coord) -> Option<Coord> {
        let Coord(r, c) = coord;
        if *r != self.rlim() - 1 {
            Some(Coord(*r + 1, *c))
        } else {
            None
        }
    }
    pub fn e(&self, coord: &Coord) -> Option<Coord> {
        let Coord(r, c) = coord;
        if *c != 0 {
            Some(Coord(*r, *c - 1))
        } else {
            None
        }
    }
    pub fn w(&self, coord: &Coord) -> Option<Coord> {
        let Coord(r, c) = coord;
        if *c != self.clim() - 1 {
            Some(Coord(*r, *c + 1))
        } else {
            None
        }
    }
    pub fn neighbours4(&self, coord: &Coord) -> Vec<Coord> {
        let Coord(r, c) = coord;
        let mut neighbours = vec![];
        if let Some(c) = self.n(coord) {
            neighbours.push(c)
        }
        if let Some(c) = self.e(coord) {
            neighbours.push(c)
        }
        if let Some(c) = self.s(coord) {
            neighbours.push(c)
        }
        if let Some(c) = self.w(coord) {
            neighbours.push(c)
        }
        neighbours
    }
    pub fn at(&self, coord: &Coord) -> T {
        self.grid[coord.0][coord.1]
    }
}
