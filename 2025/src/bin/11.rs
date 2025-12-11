use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

advent_of_code::solution!(11);

fn dfs(
    graph: &HashMap<usize, Vec<usize>>,
    key: usize,
    visited: &mut HashSet<usize>,
    sorted_keys: &mut VecDeque<usize>,
) {
    if visited.contains(&key) {
        return;
    };
    for out in graph.get(&key).unwrap_or(&vec![]) {
        dfs(graph, *out, visited, sorted_keys);
    }
    visited.insert(key);
    sorted_keys.push_front(key);
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut key_lookup = HashMap::new();
    let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
    // Keymap
    let mut keys = input
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            let (key, _) = line.split_once(": ").unwrap();
            key_lookup.insert(String::from(key), idx);
            key
        })
        .collect_vec();
    // map
    input.lines().enumerate().for_each(|(idx, line)| {
        let (_key, outputs) = line.split_once(": ").unwrap();
        outputs.split(' ').for_each(|out| {
            if !key_lookup.contains_key(out) {
                println!("{out} not there");
                key_lookup.insert(String::from(out), key_lookup.len());
                keys.push(out);
            }
            let out_val = *key_lookup.get(out).unwrap();
            if map.contains_key(&idx) {
                map.get_mut(&idx).unwrap().push(out_val)
            } else {
                map.insert(idx, vec![out_val]);
            }
        });
    });

    let start = key_lookup.get("you").unwrap();
    let goal = key_lookup.get("out").unwrap();

    let mut sorted_keys = VecDeque::new();
    let mut visited = HashSet::new();

    // DFS to topologically sort
    dfs(&map, *start, &mut visited, &mut sorted_keys);
    let sorted_keys = sorted_keys
        .iter()
        .filter(|k| visited.contains(k))
        .cloned()
        .collect_vec();
    assert!(sorted_keys[0] == *start);
    assert!(sorted_keys[sorted_keys.len() - 1] == *goal);

    // Copy counts algorithm
    let mut counts = vec![0; keys.len()];
    counts[*start] = 1;
    for key in &sorted_keys {
        for out in map.get(&key).unwrap_or(&vec![]) {
            counts[*out] += counts[*key];
        }
    }

    Some(counts[*goal])
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut key_lookup = HashMap::new();
    let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
    // Keymap
    let mut keys = input
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            let (key, _) = line.split_once(": ").unwrap();
            key_lookup.insert(String::from(key), idx);
            key
        })
        .collect_vec();
    // map
    input.lines().enumerate().for_each(|(idx, line)| {
        let (_key, outputs) = line.split_once(": ").unwrap();
        outputs.split(' ').for_each(|out| {
            if !key_lookup.contains_key(out) {
                println!("{out} not there");
                key_lookup.insert(String::from(out), key_lookup.len());
                keys.push(out);
            }
            let out_val = *key_lookup.get(out).unwrap();
            if map.contains_key(&idx) {
                map.get_mut(&idx).unwrap().push(out_val)
            } else {
                map.insert(idx, vec![out_val]);
            }
        });
    });

    let svr = *key_lookup.get("svr").unwrap();
    let fft = *key_lookup.get("fft").unwrap();
    let dac = *key_lookup.get("dac").unwrap();
    let out = *key_lookup.get("out").unwrap();

    let n = keys.len();

    let paths = count_paths(&map, svr, fft, n)
        * count_paths(&map, fft, dac, n)
        * count_paths(&map, dac, out, n)
        + count_paths(&map, svr, dac, n)
            * count_paths(&map, dac, fft, n)
            * count_paths(&map, fft, out, n);
    Some(paths)
}

fn count_paths(map: &HashMap<usize, Vec<usize>>, start: usize, goal: usize, length: usize) -> u64 {
    let mut sorted_keys = VecDeque::new();
    let mut visited = HashSet::new();

    // DFS to topologically sort
    dfs(map, start, &mut visited, &mut sorted_keys);
    let sorted_keys = sorted_keys
        .iter()
        .filter(|k| visited.contains(k))
        .cloned()
        .collect_vec();

    // Copy counts algorithm
    let mut counts = vec![0; length];
    counts[start] = 1;
    for key in &sorted_keys {
        for out in map.get(&key).unwrap_or(&vec![]) {
            counts[*out] += counts[*key];
        }
    }

    counts[goal]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
