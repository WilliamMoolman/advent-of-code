use core::panic;
use std::collections::HashMap;

advent_of_code::solution!(19);

enum Attribute {
    X,
    M,
    A,
    S,
}

enum Comparison {
    LT(Attribute, u64),
    GT(Attribute, u64),
    TRUE,
}

impl Comparison {
    fn compare(&self, part: &Part) -> bool {
        match self {
            Comparison::LT(attribute, value) => {
                if part.get_attribute(attribute) < *value {
                    return true;
                }
            }
            Comparison::GT(attribute, value) => {
                if part.get_attribute(attribute) > *value {
                    return true;
                }
            }
            Comparison::TRUE => return true,
        }
        false
    }
    fn compare_range(&self, part: &PartRange) -> (Option<PartRange>, Option<PartRange>) {
        match self {
            Comparison::LT(attribute, value) => {
                let (lower, upper) = part.get_attribute(attribute);
                if lower < *value {
                    let mut good_part = part.clone();
                    let mut invalid_part = part.clone();
                    good_part.set_attribute(attribute, (lower, *value - 1));
                    invalid_part.set_attribute(attribute, (*value, upper));
                    return (Some(good_part), Some(invalid_part));
                }
                return (None, Some(part.clone()));
            }
            Comparison::GT(attribute, value) => {
                let (lower, upper) = part.get_attribute(attribute);
                if upper > *value {
                    let mut good_part = part.clone();
                    let mut invalid_part = part.clone();
                    good_part.set_attribute(attribute, (*value + 1, upper));
                    invalid_part.set_attribute(attribute, (lower, *value));
                    return (Some(good_part), Some(invalid_part));
                }
                return (None, Some(part.clone()));
            }
            Comparison::TRUE => return (Some(part.clone()), None),
        }
    }
}

struct Rule(Comparison, String);

struct Workflow {
    rules: Vec<Rule>,
}

#[derive(Clone)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

#[derive(Clone, Debug)]
struct PartRange {
    x: (u64, u64),
    m: (u64, u64),
    a: (u64, u64),
    s: (u64, u64),
}

impl PartRange {
    fn get_attribute(&self, attribute: &Attribute) -> (u64, u64) {
        match attribute {
            Attribute::X => self.x,
            Attribute::M => self.m,
            Attribute::A => self.a,
            Attribute::S => self.s,
        }
    }

    fn set_attribute(&mut self, attribute: &Attribute, value: (u64, u64)) {
        match attribute {
            Attribute::X => self.x = value,
            Attribute::M => self.m = value,
            Attribute::A => self.a = value,
            Attribute::S => self.s = value,
        }
    }

    fn process(&self, workflow: &Workflow) -> Vec<(String, PartRange)> {
        let mut results = Vec::new();
        let mut part = self.clone();
        for rule in workflow.rules.iter() {
            let (good_part, invalid_part) = rule.0.compare_range(&part);

            if let Some(new_part) = good_part {
                results.push((rule.1.clone(), new_part));
            }
            if let Some(new_part) = invalid_part {
                part = new_part;
            } else {
                break;
            }
        }
        results
    }

    fn get_accepted(workflows: &HashMap<String, Workflow>) -> Vec<PartRange> {
        let part = PartRange {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        };
        let mut processing_parts = vec![("in".to_string(), part)];
        let mut completed_parts = Vec::new();
        while !processing_parts.is_empty() {
            let (workflow_name, part) = processing_parts.pop().unwrap();
            let workflow = workflows.get(&workflow_name).unwrap();
            // println!("Processing part: {:?}", part);
            for (new_workflow, part) in part.process(workflow) {
                // println!("  next part {:?} with workflow: {}", part, new_workflow);
                match new_workflow.as_str() {
                    "A" => completed_parts.push(part),
                    "R" => continue,
                    _ => processing_parts.push((new_workflow.clone(), part)),
                }
            }
        }
        completed_parts
    }

    fn value(&self) -> u64 {
        (1 + self.x.1 - self.x.0)
            * (1 + self.m.1 - self.m.0)
            * (1 + self.a.1 - self.a.0)
            * (1 + self.s.1 - self.s.0)
    }
}

impl Part {
    fn get_attribute(&self, attribute: &Attribute) -> u64 {
        match attribute {
            Attribute::X => self.x,
            Attribute::M => self.m,
            Attribute::A => self.a,
            Attribute::S => self.s,
        }
    }

    fn from_line(line: &str) -> Part {
        let mut attributes = line.split(',').map(|s| {
            s.chars()
                .filter(|c| c.is_digit(10))
                .collect::<String>()
                .parse()
                .unwrap()
        });
        let x = attributes.next().unwrap();
        let m = attributes.next().unwrap();
        let a = attributes.next().unwrap();
        let s = attributes.next().unwrap();

        Part { x, m, a, s }
    }

    fn process(&self, workflow: &Workflow) -> String {
        for rule in workflow.rules.iter() {
            if rule.0.compare(self) {
                return rule.1.clone();
            }
        }
        panic!("No rule matched");
    }

    fn is_accepted(&self, workflows: &HashMap<String, Workflow>) -> bool {
        let mut workflow = workflows.get("in").unwrap();
        // println!("Processing part: x={}", self.x);
        loop {
            let next_workflow = self.process(workflow);
            // println!("  next workflow: {}", next_workflow);
            if next_workflow == "A" {
                return true;
            }
            if next_workflow == "R" {
                return false;
            }
            workflow = workflows.get(&next_workflow).unwrap();
        }
    }

    fn value(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

impl Workflow {
    fn from_line(line: &str) -> Workflow {
        // Line has structure attribute<value:workflow,attribute>value:workflow,workflow
        let mut rules = Vec::new();
        let rule_strings: Vec<&str> = line.split(',').collect();
        for rule in rule_strings[..rule_strings.len() - 1].iter() {
            let mut rule = rule.split(':');
            let mut comparison_chars = rule.next().unwrap().chars();
            let workflow = rule.next().unwrap();
            let attribute = match comparison_chars.next().unwrap() {
                'x' => Attribute::X,
                'm' => Attribute::M,
                'a' => Attribute::A,
                's' => Attribute::S,
                _ => panic!("Invalid attribute"),
            };
            let comparison = match (
                comparison_chars.next().unwrap(),
                comparison_chars.collect::<String>().parse().unwrap(),
            ) {
                ('>', value) => Comparison::GT(attribute, value),
                ('<', value) => Comparison::LT(attribute, value),
                _ => panic!("Invalid comparison"),
            };
            rules.push(Rule(comparison, workflow.to_string()));
        }
        let rule = rule_strings.last().unwrap();
        rules.push(Rule(Comparison::TRUE, rule.to_string()));

        Workflow { rules }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut workflows = HashMap::new();
    let mut parts = Vec::new();
    let mut processing_workflows = true;
    for line in input.lines() {
        if line.is_empty() {
            processing_workflows = false;
            continue;
        }
        if processing_workflows {
            let (workflow_name, line) = line.split_once('{').unwrap();
            let workflow_line = line.split_once('}').unwrap().0;
            let workflow = Workflow::from_line(workflow_line);
            workflows.insert(workflow_name.to_string(), workflow);
        } else {
            parts.push(Part::from_line(line));
        }
    }

    Some(
        parts
            .iter()
            .filter(|part| part.is_accepted(&workflows))
            .map(|part| part.value())
            .sum::<u64>(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut workflows = HashMap::new();
    for line in input.lines() {
        if line.is_empty() {
            break;
        }
        let (workflow_name, line) = line.split_once('{').unwrap();
        let workflow_line = line.split_once('}').unwrap().0;
        let workflow = Workflow::from_line(workflow_line);
        workflows.insert(workflow_name.to_string(), workflow);
    }

    Some(
        PartRange::get_accepted(&workflows)
            .iter()
            // .inspect(|part| // println!("Part: {:?}", part))
            .map(|part| part.value())
            .sum::<u64>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}
