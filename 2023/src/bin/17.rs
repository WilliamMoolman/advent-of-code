advent_of_code::solution!(17);

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

use itertools::Itertools;

// Maybe recursion + memoization?
// Or some sort of graph traversal? A*?
// Or maybe just brute force it and see if it works?

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Direction {
    Up(usize),
    Left(usize),
    Right(usize),
    Down(usize),
}

impl Direction {
    fn get_remaining(&self) -> usize {
        match &self {
            Direction::Up(r) => *r,
            Direction::Left(r) => *r,
            Direction::Right(r) => *r,
            Direction::Down(r) => *r,
        }
    }

    fn subtract(&self) -> Direction {
        match &self {
            Direction::Up(r) => Direction::Up(*r - 1),
            Direction::Left(r) => Direction::Left(*r - 1),
            Direction::Right(r) => Direction::Right(*r - 1),
            Direction::Down(r) => Direction::Down(*r - 1),
        }
    }

    // fn get_new_directions(&self) -> Vec<Direction> {
    //     match &self {
    //         Direction::New => 3,
    //         Direction::Up(r) => *r,
    //         Direction::Left(r) => *r,
    //         Direction::Right(r) => *r,
    //         Direction::Down(r) => *r,
    //     }
    // }
}

// Row, col, direction (ULDR), ttg
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
struct Node(usize, usize, Direction);

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Edge {
    cost: i64,
    node: Node,
}

fn reconstruct_path(came_from: HashMap<Node, Node>, current: Node) -> Vec<Node> {
    let mut total_path = Vec::new();
    let mut current = current;
    total_path.push(current);
    while came_from.keys().contains(&current) {
        current = *came_from.get(&current).unwrap();
        total_path.push(current);
    }
    total_path
}

// row, col, direction, ttg
fn a_star<F, G, H>(start: Node, goal: (usize, usize), neighbours: F, distance: G, h: H) -> Vec<Node>
where
    F: Fn(Node) -> Vec<Node>,
    G: Fn(Node) -> usize,
    H: Fn(Node) -> usize,
{
    let mut open_set = BinaryHeap::new();
    // let mut open_set_hash = HashSet::new();
    let mut closed_set = HashSet::new();

    let mut came_from = HashMap::new();

    let mut g_score = HashMap::new();
    g_score.insert(start, 0);

    let mut f_score = HashMap::new();
    f_score.insert(start, h(start));

    open_set.push(Edge {
        cost: h(start) as i64,
        node: start,
    });
    // open_set_hash.insert(start);

    while !open_set.is_empty() {
        let Edge { cost, node } = open_set.pop().unwrap();
        // println!("NODE: {node:?}");
        closed_set.insert(node);
        if node.0 == goal.0 && node.1 == goal.1 {
            // println!("CLOSED SET: {}", closed_set.len());
            return reconstruct_path(came_from, node);
        }

        for neighbour in neighbours(node) {
            let tentative_g_score = g_score.get(&node).unwrap() + distance(neighbour);
            if !g_score.contains_key(&neighbour)
                || tentative_g_score < *g_score.get(&neighbour).unwrap()
            {
                came_from.insert(neighbour, node);
                g_score.insert(neighbour, tentative_g_score);
                // println!("   gscore: {neighbour:?}{tentative_g_score}");
                f_score.insert(neighbour, tentative_g_score + h(neighbour));
                if !closed_set.contains(&neighbour) {
                    open_set.push(Edge {
                        cost: -(tentative_g_score as i64),
                        node: neighbour,
                    });
                }
            }
        }
    }

    panic!("No solution!") // Failure
}

pub fn part_one(input: &str) -> Option<usize> {
    let board: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    let start = Node(0, 0, Direction::Right(3));
    let goal = (board.len() - 1, board[0].len() - 1);

    let neighbours = |node: Node| {
        // 0,   1,   2,                3
        // Row, col, direction (ULDR), ttg
        // can go LRS(if below 3)
        // ULDR 0123
        let mut neighbour_nodes = Vec::new();

        // Straight
        for dir_types in [
            Direction::Left,
            Direction::Right,
            Direction::Up,
            Direction::Down,
        ] {
            let dir;
            if node.2 == dir_types(node.2.get_remaining()) {
                if node.2.get_remaining() == 0 {
                    continue;
                }
                dir = dir_types(node.2.get_remaining() - 1);
            } else {
                dir = dir_types(2);
            }

            match (node.2, dir) {
                (Direction::Down(_), Direction::Up(_)) | (Direction::Up(_), Direction::Down(_)) => {
                    continue
                }
                (Direction::Left(_), Direction::Right(_))
                | (Direction::Right(_), Direction::Left(_)) => continue,
                _ => (),
            }

            match dir {
                Direction::Up(_) => {
                    if node.0 != 0 {
                        neighbour_nodes.push(Node(node.0 - 1, node.1, dir));
                    }
                }
                Direction::Left(_) => {
                    if node.1 != 0 {
                        neighbour_nodes.push(Node(node.0, node.1 - 1, dir));
                    }
                }
                Direction::Down(_) => {
                    if node.0 != board.len() - 1 {
                        neighbour_nodes.push(Node(node.0 + 1, node.1, dir));
                    }
                }
                Direction::Right(_) => {
                    if node.1 != board[0].len() - 1 {
                        neighbour_nodes.push(Node(node.0, node.1 + 1, dir));
                    }
                }
                _ => (),
            }
        }
        // println!("NEIGHBOURS: {node:?} ==> {neighbour_nodes:?}");
        neighbour_nodes
    };

    let distance = |node: Node| board[node.0][node.1];

    let h = |node: Node| 0;

    let path = a_star(start, goal, neighbours, distance, h);

    let mut sum = 0;
    for p in path.iter().rev() {
        sum += board[p.0][p.1];
        // print!("({}, {})->", p.0, p.1);
    }
    sum -= board[0][0];
    // println!();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let board: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    let start = Node(0, 0, Direction::Right(11));
    let goal = (board.len() - 1, board[0].len() - 1);

    let neighbours = |node: Node| {
        // 0,   1,   2,                3
        // Row, col, direction (ULDR), ttg
        // can go LRS(if below 3)
        // ULDR 0123
        let mut neighbour_nodes = Vec::new();

        if node.0 == 0 && node.1 == 0 {
            return vec![
                Node(0, 1, Direction::Right(9)),
                Node(1, 0, Direction::Down(9)),
            ];
        }

        // Straight
        for dir_types in [
            Direction::Left,
            Direction::Right,
            Direction::Up,
            Direction::Down,
        ] {
            let dir;
            if node.2 == dir_types(node.2.get_remaining()) {
                if node.2.get_remaining() == 0 {
                    continue;
                }
                dir = dir_types(node.2.get_remaining() - 1);
            } else {
                dir = dir_types(9);
            }
            if node.2 != dir_types(node.2.get_remaining()) && node.2.get_remaining() > 6 {
                continue;
            }

            match (node.2, dir) {
                (Direction::Down(_), Direction::Up(_)) | (Direction::Up(_), Direction::Down(_)) => {
                    continue
                }
                (Direction::Left(_), Direction::Right(_))
                | (Direction::Right(_), Direction::Left(_)) => continue,
                _ => (),
            }

            match dir {
                Direction::Up(_) => {
                    if node.0 != 0 {
                        neighbour_nodes.push(Node(node.0 - 1, node.1, dir));
                    }
                }
                Direction::Left(_) => {
                    if node.1 != 0 {
                        neighbour_nodes.push(Node(node.0, node.1 - 1, dir));
                    }
                }
                Direction::Down(_) => {
                    if node.0 != board.len() - 1 {
                        neighbour_nodes.push(Node(node.0 + 1, node.1, dir));
                    }
                }
                Direction::Right(_) => {
                    if node.1 != board[0].len() - 1 {
                        neighbour_nodes.push(Node(node.0, node.1 + 1, dir));
                    }
                }
                _ => (),
            }
        }
        // println!("NEIGHBOURS: {node:?} ==> {neighbour_nodes:?}");
        neighbour_nodes
    };

    let distance = |node: Node| board[node.0][node.1];

    let h = |node: Node| 0;

    let path = a_star(start, goal, neighbours, distance, h);

    let mut sum = 0;
    for p in path.iter().rev() {
        sum += board[p.0][p.1];
        // print!("({}, {})->", p.0, p.1);
    }
    sum -= board[0][0];
    // println!();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }
}
