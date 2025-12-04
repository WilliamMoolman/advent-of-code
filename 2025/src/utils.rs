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

#[derive(Clone)]
pub struct Grid<T> {
    grid: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T: Copy> Grid<T> {
    pub fn from_input<F>(input: &str, transform: F) -> Grid<T>
    where
        F: Fn(char) -> T,
    {
        let lines = input.lines();
        let mut rows = 0;
        let grid: Vec<T> = lines
            .flat_map(|line| {
                rows += 1;
                line.chars().map(|c| transform(c))
            })
            .collect();
        let cols = grid.len() / rows;
        Grid { grid, rows, cols }
    }
    pub fn coordinates(&self) -> Vec<(T, Coord)> {
        self.grid
            .iter()
            .enumerate()
            .map(|(idx, item)| (*item, Coord(idx / self.cols, idx % self.cols)))
            .collect()
    }
    pub fn coordinates_iter(&self) -> impl Iterator<Item = (T, Coord)> {
        self.grid
            .iter()
            .enumerate()
            .map(|(idx, item)| (*item, Coord(idx / self.cols, idx % self.cols)))
    }
    pub fn rlim(&self) -> usize {
        self.rows
    }
    pub fn clim(&self) -> usize {
        self.cols
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
    pub fn neighbours8(&self, coord: &Coord) -> Vec<Coord> {
        let mut neighbours = vec![];
        if let Some(c) = self.n(coord) {
            neighbours.push(c);
            if let Some(c2) = self.e(&c) {
                neighbours.push(c2);
            }
            if let Some(c2) = self.w(&c) {
                neighbours.push(c2);
            }
        }
        if let Some(c) = self.e(coord) {
            neighbours.push(c)
        }
        if let Some(c) = self.s(coord) {
            neighbours.push(c);
            if let Some(c2) = self.e(&c) {
                neighbours.push(c2);
            }
            if let Some(c2) = self.w(&c) {
                neighbours.push(c2);
            }
        }
        if let Some(c) = self.w(coord) {
            neighbours.push(c)
        }
        neighbours
    }
    pub fn neighbours8_sat<F>(&self, coord: &Coord, predicate: F) -> usize
    where
        F: Fn(T) -> bool,
    {
        let mut count = 0;
        if let Some(c) = self.n(coord) {
            count += if predicate(self.at(&c)) { 1 } else { 0 };
            if let Some(c2) = self.e(&c) {
                count += if predicate(self.at(&c2)) { 1 } else { 0 };
            }
            if let Some(c2) = self.w(&c) {
                count += if predicate(self.at(&c2)) { 1 } else { 0 };
            }
        }
        if let Some(c) = self.e(coord) {
            count += if predicate(self.at(&c)) { 1 } else { 0 };
        }
        if let Some(c) = self.s(coord) {
            count += if predicate(self.at(&c)) { 1 } else { 0 };
            if let Some(c2) = self.e(&c) {
                count += if predicate(self.at(&c2)) { 1 } else { 0 };
            }
            if let Some(c2) = self.w(&c) {
                count += if predicate(self.at(&c2)) { 1 } else { 0 };
            }
        }
        if let Some(c) = self.w(coord) {
            count += if predicate(self.at(&c)) { 1 } else { 0 };
        }
        count
    }
    pub fn at(&self, coord: &Coord) -> T {
        self.grid[coord.0 * self.cols + coord.1]
    }
    pub fn set(&mut self, coord: &Coord, item: T) {
        self.grid[coord.0 * self.cols + coord.1] = item;
    }
}
