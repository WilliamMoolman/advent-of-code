use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

advent_of_code::solution!(8);

#[derive(PartialEq, Debug)]
struct Point(usize, usize, usize);

impl Point {
    fn dist(&self, other: &Point) -> f64 {
        (((self.0.abs_diff(other.0)).pow(2)
            + (self.1.abs_diff(other.1)).pow(2)
            + (self.2.abs_diff(other.2)).pow(2)) as f64)
            .sqrt()
        // (self.0.abs_diff(other.0) as f64)
        //     + (self.1.abs_diff(other.1) as f64)
        //     + (self.2.abs_diff(other.2) as f64)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let points = input
        .lines()
        .map(|line| {
            let mut nums = line.split(',');
            Point(
                nums.next().unwrap().parse().unwrap(),
                nums.next().unwrap().parse().unwrap(),
                nums.next().unwrap().parse().unwrap(),
            )
        })
        .collect_vec();

    println!("{} points", points.len());

    let mut dists = vec![];

    for (i, p1) in points.iter().enumerate() {
        for (j, p2) in points.iter().enumerate() {
            if i >= j {
                continue;
            }
            dists.push(((i, j), p1.dist(p2)));
        }
    }

    dists.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    println!("{} dists", dists.len());

    let mut nets: Vec<Vec<bool>> = vec![vec![false; points.len()]; points.len()];

    let max_connections = if points.len() > 20 { 1000 } else { 10 }; //example or real input
    println!("{} connections", max_connections);
    let mut connections = 0;

    for ((i, j), _dist) in dists {
        if nets[i][j] {
            // already connected, dont think incrememebnt?
            continue;
        }
        // println!("connect {:?} to {:?}", points[i], points[j]);
        nets[i][j] = true;
        nets[j][i] = true;
        connections += 1;
        if connections == max_connections {
            break;
        }
    }

    let mut networks = vec![];
    let mut visited: HashSet<usize> = HashSet::new(); // could replace with 2d array
    for root in 0..points.len() {
        if visited.contains(&root) {
            continue;
        }
        let mut network: HashSet<usize> = HashSet::new();
        let mut to_visit = vec![root];
        while let Some(node) = to_visit.pop() {
            network.insert(node);
            visited.insert(node);
            for (i, neigh) in nets[node].iter().enumerate() {
                if *neigh && nets[node][i] && !visited.contains(&i) {
                    to_visit.push(i);
                }
            }
        }
        networks.push(network)
    }

    networks.sort_by_key(|n| -(n.len() as i64));

    Some((networks[0].len() * networks[1].len() * networks[2].len()) as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let points = input
        .lines()
        .map(|line| {
            let mut nums = line.split(',');
            Point(
                nums.next().unwrap().parse().unwrap(),
                nums.next().unwrap().parse().unwrap(),
                nums.next().unwrap().parse().unwrap(),
            )
        })
        .collect_vec();

    println!("{} points", points.len());

    let mut dists = vec![];

    for (i, p1) in points.iter().enumerate() {
        for (j, p2) in points.iter().enumerate() {
            if i >= j {
                continue;
            }
            dists.push(((i, j), p1.dist(p2)));
        }
    }

    dists.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    println!("{} dists", dists.len());

    let mut nets: Vec<Vec<bool>> = vec![vec![false; points.len()]; points.len()];

    let max_connections = if points.len() > 20 { 1000 } else { 10 }; //example or real input
    println!("{} connections", max_connections);

    let mut last = (0, 0);
    for ((i, j), _dist) in dists {
        if nets[i][j] {
            // already connected, dont think incrememebnt?
            continue;
        }

        if dfs(&mut nets, i, j) {
            continue;
        }

        // println!("connect {:?} to {:?}", points[i], points[j]);
        nets[i][j] = true;
        nets[j][i] = true;
        last = (i, j);
    }
    println!("last is {:?} and {:?}", points[last.0], points[last.1]);

    Some((points[last.0].0 * points[last.1].0) as u64)

    // let mut networks = vec![];
    // let mut visited: HashSet<usize> = HashSet::new(); // could replace with 2d array
    // for root in 0..points.len() {
    //     if visited.contains(&root) {
    //         continue;
    //     }
    //     let mut network: HashSet<usize> = HashSet::new();
    //     let mut to_visit = vec![root];
    //     while let Some(node) = to_visit.pop() {
    //         network.insert(node);
    //         visited.insert(node);
    //         for (i, neigh) in nets[node].iter().enumerate() {
    //             if *neigh && nets[node][i] && !visited.contains(&i) {
    //                 to_visit.push(i);
    //             }
    //         }
    //     }
    //     networks.push(network)
    // }

    // networks.sort_by_key(|n| -(n.len() as i64));

    // Some((networks[0].len() * networks[1].len() * networks[2].len()) as u64)
}

fn dfs(
    nets: &mut Vec<Vec<bool>>,
    // networks: &mut Vec<HashSet<usize>>,
    from: usize,
    to: usize,
) -> bool {
    let mut visited = HashSet::new();
    // if visited.contains(&root) {
    //     return;
    // }
    // let mut network: HashSet<usize> = HashSet::new();
    let mut to_visit = VecDeque::new();
    to_visit.push_back(from);
    while let Some(node) = to_visit.pop_front() {
        // network.insert(node);
        visited.insert(node);
        for (i, neigh) in nets[node].iter().enumerate() {
            if *neigh && nets[node][i] {
                // nets[from][i] = true;
                if i == to {
                    return true;
                }

                if !visited.contains(&i) {
                    to_visit.push_back(i);
                }
            }
        }
    }
    return false;
    // networks.push(network)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
