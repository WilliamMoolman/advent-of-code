use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

advent_of_code::solution!(20);

#[derive(Debug)]
enum Gate {
    FlipFlop(Vec<String>, bool),
    Conjuction(Vec<String>, HashMap<String, bool>),
    Broadcast(Vec<String>),
}

impl Gate {
    fn outputs(&self) -> Vec<String> {
        match self {
            Gate::FlipFlop(outputs, _) => outputs.clone(),
            Gate::Conjuction(outputs, _) => outputs.clone(),
            Gate::Broadcast(outputs) => outputs.clone(),
        }
    }

    fn from_str(s: &str, gates: &mut HashMap<String, Gate>) {
        let mut parts = s.split(" -> ");
        let gate = parts.next().unwrap();
        let output = parts.next().unwrap();
        let outputs = output.split(", ").map(|s| s.to_string()).collect();
        let (gate_name, gate) = match gate.chars().nth(0).unwrap() {
            '%' => (gate[1..].to_string(), Gate::FlipFlop(outputs, false)),
            '&' => (
                gate[1..].to_string(),
                Gate::Conjuction(outputs, HashMap::new()),
            ),
            'b' => (gate[..].to_string(), Gate::Broadcast(outputs)),
            _ => panic!("Unknown gate type"),
        };
        gates.insert(gate_name, gate);
    }

    fn setup_conjunction(gates: &mut HashMap<String, Gate>) {
        let names = gates.keys().map(|s| s.clone()).collect::<Vec<_>>();
        for name in names {
            let gate = gates.get(&name).unwrap();
            for output in gate.outputs() {
                // println!("{} -> {}", name, output);
                if let Some(Gate::Conjuction(_, inputs)) = gates.get_mut(&output) {
                    inputs.insert(name.clone(), false);
                }
            }
        }
    }

    fn signal(&mut self, upstream: &str, pulse: bool) -> Vec<(String, bool)> {
        match self {
            Gate::FlipFlop(outputs, state) => {
                if pulse {
                    return Vec::new();
                }
                *state = !*state;
                let signals = outputs.iter().map(|s| (s.clone(), *state)).collect();
                return signals;
            }
            Gate::Conjuction(outputs, inputs) => {
                inputs.insert(upstream.to_string(), pulse);
                let mut signals = Vec::new();
                if inputs.iter().all(|(_, b)| *b) {
                    for output in outputs.iter() {
                        signals.push((output.clone(), false));
                    }
                } else {
                    for output in outputs.iter() {
                        signals.push((output.clone(), true));
                    }
                }
                return signals;
            }
            Gate::Broadcast(outputs) => outputs.iter().map(|s| (s.clone(), pulse)).collect(),
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut gates = HashMap::new();
    for line in input.lines() {
        Gate::from_str(line, &mut gates);
    }
    // for (name, gate) in gates.iter().sorted_by(|(a, _), (b, _)| a.cmp(b)) {
    //     println!("{}: {:?}", name, gate);
    // }
    Gate::setup_conjunction(&mut gates);
    // for (name, gate) in gates.iter_mut() {
    //     println!("{}: {:?}", name, gate);
    // }

    let mut queue = VecDeque::new();
    let (mut low_pulses, mut high_pulses) = (0, 0);

    for _ in 0..1000 {
        queue.push_back(("".to_owned(), "broadcaster".to_string(), false));
        low_pulses += 1;

        while let Some((from, name, pulse)) = queue.pop_front() {
            if let Some(gate) = gates.get_mut(&name) {
                let signals = gate.signal(&from, pulse);
                for (output, pulse) in signals {
                    if pulse {
                        high_pulses += 1;
                    } else {
                        low_pulses += 1;
                    }
                    // println!("{} -> {} {}", &name, output, pulse);
                    queue.push_back((name.clone(), output, pulse));
                }
            } else {
                // Hope this is rx
            }
        }
    }
    // println!("{} {}", low_pulses, high_pulses);
    Some(low_pulses * high_pulses)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut gates = HashMap::new();
    for line in input.lines() {
        Gate::from_str(line, &mut gates);
    }
    // for (name, gate) in gates.iter().sorted_by(|(a, _), (b, _)| a.cmp(b)) {
    //     println!("{}: {:?}", name, gate);
    // }
    Gate::setup_conjunction(&mut gates);
    // for (name, gate) in gates.iter_mut() {
    //     println!("{}: {:?}", name, gate);
    // }

    let mut queue = VecDeque::new();
    // let (mut low_pulses, mut high_pulses) = (0, 0);
    let mut i = 0;
    loop {
        queue.push_back(("".to_owned(), "broadcaster".to_string(), false));
        // low_pulses += 1;
        i += 1;

        while let Some((from, name, pulse)) = queue.pop_front() {
            if let Some(gate) = gates.get_mut(&name) {
                let signals = gate.signal(&from, pulse);
                for (output, pulse) in signals {
                    queue.push_back((name.clone(), output, pulse));
                }
            } else {
                assert_eq!(name, "rx");
                if !pulse {
                    return Some(i);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(32000000));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
