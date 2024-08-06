use std::collections::HashMap;

advent_of_code::solution!(7);

enum Gate<'a> {
    AND(&'a str, &'a str),
    OR(&'a str, &'a str),
    LSHIFT(&'a str, &'a str),
    RSHIFT(&'a str, &'a str),
    NOT(&'a str),
    VALUE(&'a str),
}

use Gate::*;
// impl<'a> Gate<'a> {
impl<'g> Gate<'g> {
    // fn get(gates: &HashMap<String, Gate>, id: &str) -> Rc<Gate> {
    //     println!("Getting {id}");
    //     let
    //     Rc::clone(gates.get(&id.to_string()).unwrap())
    // }
    fn add_from_tokens<'a, 'b>(gates: &'b mut HashMap<String, Gate<'a>>, tokens: Vec<&'a str>) {
        let (id, gate) = match tokens.len() {
            3 => (tokens[2], VALUE(tokens[0])),
            4 => (tokens[3], NOT(tokens[1])),
            5 => (
                tokens[4],
                match tokens[1] {
                    "AND" => AND(tokens[0], tokens[2]),
                    "OR" => OR(tokens[0], tokens[2]),
                    "LSHIFT" => LSHIFT(tokens[0], tokens[2]),
                    "RSHIFT" => RSHIFT(tokens[0], tokens[2]),
                    _ => panic!("Unknown gate"),
                },
            ),
            _ => panic!("Unknown gate"),
        };
        gates.insert(id.to_string(), gate);
    }

    fn get(
        gates: &HashMap<String, Gate>,
        gates_compute: &mut HashMap<String, u16>,
        id: &str,
    ) -> u16 {
        if let Ok(num) = id.parse() {
            return num;
        }
        if let Some(&value) = gates_compute.get(id) {
            return value;
        }
        let gate = gates.get(id).unwrap();
        let value = gate.simulate(gates, gates_compute);
        gates_compute.insert(id.to_string(), value);
        value
    }

    fn simulate(
        &self,
        gates: &HashMap<String, Gate>,
        gates_compute: &mut HashMap<String, u16>,
    ) -> u16 {
        match self {
            AND(left, right) => {
                Gate::get(&gates, gates_compute, left) & Gate::get(&gates, gates_compute, right)
            }
            OR(left, right) => {
                Gate::get(&gates, gates_compute, left) | Gate::get(&gates, gates_compute, right)
            }
            LSHIFT(left, right) => {
                Gate::get(&gates, gates_compute, left) << Gate::get(&gates, gates_compute, right)
            }
            RSHIFT(left, right) => {
                Gate::get(&gates, gates_compute, left) >> Gate::get(&gates, gates_compute, right)
            }
            NOT(left) => !Gate::get(&gates, gates_compute, left),
            VALUE(left) => Gate::get(&gates, gates_compute, left),
        }
    }
}

pub fn part_one(input: &str) -> Option<u16> {
    let mut gates = HashMap::<String, Gate>::new();
    let cmds: Vec<&str> = input.lines().collect::<Vec<&str>>();
    cmds.iter().for_each(|line| {
        let tokens: Vec<&str> = line.split_ascii_whitespace().collect();
        Gate::add_from_tokens(&mut gates, tokens);
    });
    let mut gates_compute = HashMap::<String, u16>::new();
    Some(
        gates
            .get(&"a".to_string())
            .unwrap()
            .simulate(&gates, &mut gates_compute),
    )
}

pub fn part_two(input: &str) -> Option<u16> {
    let mut gates = HashMap::<String, Gate>::new();
    let cmds: Vec<&str> = input.lines().collect::<Vec<&str>>();
    cmds.iter().for_each(|line| {
        let tokens: Vec<&str> = line.split_ascii_whitespace().collect();
        Gate::add_from_tokens(&mut gates, tokens);
    });
    let mut gates_compute = HashMap::<String, u16>::new();
    let a_value = gates
        .get(&"a".to_string())
        .unwrap()
        .simulate(&gates, &mut gates_compute);
    let mut gates_compute = HashMap::<String, u16>::new();
    gates_compute.insert("b".to_string(), a_value);
    Some(
        gates
            .get(&"a".to_string())
            .unwrap()
            .simulate(&gates, &mut gates_compute),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
