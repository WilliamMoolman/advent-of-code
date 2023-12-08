use num::integer::lcm;
use std::collections::HashMap;
advent_of_code::solution!(8);

struct Node(String, String);

impl Node {
    fn from_line<'a, 'b>(nodes: &'b mut HashMap<String, Node>, line: &str) {
        let (name, children) = line.split_once(" = ").unwrap();
        let children = children.replace('(', "").replace(')', "");
        let (lchild, rchild) = children.split_once(", ").unwrap();

        let node = Node(lchild.to_string(), rchild.to_string());
        nodes.insert(name.to_string(), node);
    }

    fn get<'g>(nodes: &'g HashMap<String, Node>, id: &str, cmd: char) -> &'g str {
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

fn travel_to_z(start_node: &String, nodes: &HashMap<String, Node>, cmds: &Vec<char>) -> u64 {
    let mut count = 0;
    let mut current_node = start_node.to_owned();
    let mut i = 0;
    loop {
        let cmd = cmds[i];
        i = (i + 1) % cmds.len();
        let next = Node::get(&nodes, &current_node, cmd).to_string();
        count += 1;

        if next.ends_with("Z") {
            break;
        }
        current_node = next;
    }

    count
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut nodes = HashMap::<String, Node>::new();
    let mut lines = input.lines();
    let cmds = lines.next().unwrap().chars().collect::<Vec<char>>();
    lines.next();
    for line in lines {
        Node::from_line(&mut nodes, line);
    }
    let count = travel_to_z(&"AAA".to_string(), &nodes, &cmds);

    Some(count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut nodes = HashMap::<String, Node>::new();
    let mut lines = input.lines();
    let cmds = lines.next().unwrap().chars().collect::<Vec<char>>();
    lines.next();
    for line in lines {
        Node::from_line(&mut nodes, line);
    }

    let current_nodes: Vec<(String, u64)> = nodes
        .keys()
        .filter(|x| x.ends_with('A'))
        .map(|x| (x.to_owned(), 0))
        .collect();

    let total = current_nodes
        .iter()
        .map(|(n, curr_count)| {
            let count = travel_to_z(n, &nodes, &cmds);
            curr_count + count
        })
        .fold(1, |acc, count| lcm(acc, count));

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
