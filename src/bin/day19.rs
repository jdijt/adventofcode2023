use aoc2023::{read_lines, run_timed};
use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Result {
    Ok,
    Reject,
    Goto(String),
}

impl Result {
    fn from(s: &str) -> Result {
        match s {
            "A" => Result::Ok,
            "R" => Result::Reject,
            other => Result::Goto(String::from(other)),
        }
    }
}

#[derive(Debug)]
enum Expression {
    Gt(String, u32),
    Lt(String, u32),
}

impl Expression {
    fn from(s: &str) -> Expression {
        if let Some((var, value)) = s.split_once('<') {
            Expression::Lt(String::from(var), value.parse().unwrap())
        } else if let Some((var, value)) = s.split_once('>') {
            Expression::Gt(String::from(var), value.parse().unwrap())
        } else {
            panic!("Invalid expression!")
        }
    }

    fn check(&self, context: &Part) -> bool {
        match self {
            Expression::Gt(r, v) => context.get(r).unwrap() > v,
            Expression::Lt(r, v) => context.get(r).unwrap() < v,
        }
    }
}

#[derive(Debug)]
struct Rule {
    condition: Option<Expression>,
    result: Result,
}

impl Rule {
    fn from(rs: &str) -> Rule {
        let (condition, result) = if let Some((expr_slice, res_slice)) = rs.split_once(':') {
            (Some(Expression::from(expr_slice)), Result::from(res_slice))
        } else {
            (None, Result::from(rs))
        };

        Rule { condition, result }
    }

    fn applies(&self, part: &Part) -> bool {
        if let Some(ref cond) = self.condition {
            cond.check(part)
        } else {
            true
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn from(line: &str) -> Workflow {
        let rules_start = line.find('{').unwrap();
        let name = String::from(&line[..rules_start]);
        let rules = line[rules_start + 1..line.len() - 1]
            .split(',')
            .map(|r| Rule::from(r))
            .collect();

        Workflow { name, rules }
    }

    fn run(&self, part: &Part) -> Result {
        self.rules
            .iter()
            .find(|r| r.applies(part))
            .unwrap()
            .result
            .clone()
    }
}

struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn new() -> Range {
        Range {
            start: 1,
            end: 4000,
        }
    }

    fn length(&self) -> u32 {
        self.end - self.start
    }
}

type Part = HashMap<String, u32>;

fn read_input(file_name: &str) -> (HashMap<String, Workflow>, Vec<Part>) {
    let mut lines = read_lines(file_name);
    let workflows = lines
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let w = Workflow::from(&l);
            (String::from(&w.name), w)
        })
        .collect();

    let parts = lines
        .map(|l| {
            l[1..l.len() - 1]
                .split(',')
                .map(|part_val| {
                    if let Some(idx) = part_val.find('=') {
                        (
                            String::from(&part_val[..idx]),
                            part_val[idx + 1..].parse().unwrap(),
                        )
                    } else {
                        panic!("Invalid part definion")
                    }
                })
                .collect()
        })
        .collect();

    (workflows, parts)
}

fn part1(workflows: &HashMap<String, Workflow>, parts: &Vec<Part>) -> u32 {
    let mut sum = 0;

    for part in parts {
        let mut step = Result::Goto(String::from("in"));
        while let Result::Goto(ref step_name) = step {
            step = workflows.get(step_name).unwrap().run(part)
        }
        if step == Result::Ok {
            sum += part.values().sum::<u32>();
        }
    }

    sum
}

fn main() {
    let (workflows, parts) = read_input("./inputs/day19");

    println!("Part 1:  {}", run_timed(|| part1(&workflows, &parts)))
}
