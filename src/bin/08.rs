use num::integer::lcm;
use std::collections::HashMap;
advent_of_code::solution!(8);

struct Node(String, String);

impl Node {
    fn from_line<'a, 'b>(nodes: &'b mut HashMap<String, Node>, line: &str) {
        // Line: AAA = (BBB, CCC)
        let (name, children) = line.split_once(" = ").unwrap();
        let children = children.replace('(', "").replace(')', "");
        let (lchild, rchild) = children.split_once(", ").unwrap();

        let node = Node(lchild.to_string(), rchild.to_string());
        nodes.insert(name.to_string(), node);
    }

    fn get<'g>(
        nodes: &'g HashMap<String, Node>,
        // gates_compute: &mut HashMap<String, u16>,
        id: &str,
        cmd: char,
    ) -> &'g str {
        // if let Ok(num) = id.parse() {
        //     return num;
        // }
        if let Some(&Node(ref left, ref right)) = nodes.get(id) {
            return match cmd {
                'L' => &left,
                'R' => &right,
                _ => panic!("Wrong command"),
            };
        }
        panic!("No such node!")
    }
}

fn travel_to_z(
    start_node: &String,
    nodes: &HashMap<String, Node>,
    visited: &mut HashMap<(String, usize), (String, usize, u64)>,
    cmds: &Vec<char>,
    cmd_idx: usize,
) -> (String, usize, u64) {
    let mut count = 0;
    let mut current_node = start_node.to_owned();
    let mut i = cmd_idx;
    if let Some((new, idx, length)) = visited.get(&(current_node.clone(), i)) {
        // println!("Memoized: --> {new}");
        return (new.to_owned(), *idx, *length);
    }
    loop {
        let cmd = cmds[i];
        i = (i + 1) % cmds.len();
        let next = Node::get(&nodes, &current_node, cmd).to_string();
        count += 1;

        if next.ends_with("Z") {
            current_node = next;
            break;
        }
        current_node = next;
    }

    visited.insert(
        (start_node.to_owned(), cmd_idx),
        (current_node.to_owned(), i, count),
    );

    (current_node, i, count)
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut nodes = HashMap::<String, Node>::new();
    let mut lines = input.lines();
    let cmds = lines.next().unwrap().chars().collect::<Vec<char>>();
    lines.next();
    for line in lines {
        Node::from_line(&mut nodes, line);
    }
    let mut count = 0;
    let mut current_node = "AAA";
    'outer: loop {
        for cmd in cmds.iter() {
            let next = Node::get(&nodes, current_node, *cmd);
            count += 1;
            if next == "ZZZ" {
                break 'outer;
            }
            current_node = next;
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut nodes = HashMap::<String, Node>::new();
    let mut lines = input.lines();
    let cmds = lines.next().unwrap().chars().collect::<Vec<char>>();
    lines.next();
    for line in lines {
        Node::from_line(&mut nodes, line);
    }

    // let mut total_count;

    let mut current_nodes: Vec<(String, usize, u64)> = nodes
        .keys()
        .filter(|x| x.ends_with('A'))
        .map(|x| (x.to_owned(), 0, 0))
        .collect();

    let mut visited = HashMap::<(String, usize), (String, usize, u64)>::new();
    // loop {
    //     current_nodes.sort_by_key(|(_, _, x)| *x);
    //     current_nodes = current_nodes
    //         .iter()
    //         .map(|(n, curr_idx, curr_count)| {
    //             let (node, idx, count) = travel_to_z(n, &nodes, &mut visited, &cmds, *curr_idx);
    //             // println!("{n}-->{node} in {count} @ {idx}");
    //             (node, idx, curr_count + count)
    //         })
    //         .collect();
    //     total_count = current_nodes[0].2;
    //     if current_nodes.iter().all(|(_, _, c)| *c == total_count) {
    //         break;
    //     }
    // }
    let total = current_nodes
        .iter()
        .map(|(n, curr_idx, curr_count)| {
            let (node, idx, count) = travel_to_z(n, &nodes, &mut visited, &cmds, *curr_idx);
            // println!("{n}-->{node} in {count} @ {idx}");
            (node, idx, curr_count + count)
        })
        .fold(1, |acc, (_, _, count)| lcm(acc, count));

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
