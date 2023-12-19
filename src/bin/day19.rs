use aoc2023::{read_lines, run_timed};
use std::collections::{HashMap, VecDeque};

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

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn new() -> Range {
        Range {
            start: 1,
            end: 4001,
        }
    }

    fn len(&self) -> u32 {
        self.end - self.start
    }

    fn split(&self, at: &u32) -> (Range, Range) {
        if *at < self.start || *at > self.end {
            panic!("Split out of range")
        }
        (
            Range {
                start: self.start,
                end: *at,
            },
            Range {
                start: *at,
                end: self.end,
            },
        )
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
                .filter_map(|part_val| {
                    part_val
                        .split_once('=')
                        .map(|(var, value)| (String::from(var), value.parse().unwrap()))
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

fn part2(workflows: &HashMap<String, Workflow>) -> u64 {
    let mut todo = VecDeque::new();
    todo.push_back((
        workflows.get(&String::from("in")).unwrap(),
        HashMap::from([
            (String::from("x"), Range::new()),
            (String::from("m"), Range::new()),
            (String::from("a"), Range::new()),
            (String::from("s"), Range::new()),
        ]),
    ));

    let mut result_sum = 0u64;
    while let Some((wf, mut ranges)) = todo.pop_front() {
        for Rule { condition, result } in wf.rules.iter() {
            // Reject needs separate treatment as it requires just a _substraction_ from the available ranges.
            if *result == Result::Reject {
                //Shrink remaining ranges.
                match condition {
                    Some(Expression::Gt(var, val)) => {
                        if let Some(old) = ranges.get_mut(var) {
                            let (lt, _) = old.split(&(val + 1));
                            *old = lt;
                        }
                    }
                    Some(Expression::Lt(var, val)) => {
                        if let Some(old) = ranges.get_mut(var) {
                            let (_, gt) = old.split(&val);
                            *old = gt;
                        }
                    }
                    _ => {}
                }
                continue;
            }
            // for OK / Goto we need to first apply the rules in the same way.
            let mut new_ranges = ranges.clone();
            match condition {
                Some(Expression::Gt(var, val)) => {
                    if let (Some(current), Some(current_new)) =
                        (ranges.get_mut(var), new_ranges.get_mut(var))
                    {
                        let (rem, new) = current.split(&(val + 1));
                        *current = rem;
                        *current_new = new;
                    }
                }
                Some(Expression::Lt(var, val)) => {
                    if let (Some(current), Some(current_new)) =
                        (ranges.get_mut(var), new_ranges.get_mut(var))
                    {
                        let (new, rem) = current.split(&(val));
                        *current = rem;
                        *current_new = new;
                    }
                }
                _ => {}
            }
            // And then decide: add it to work queue, or add it to result vector.
            if *result == Result::Ok {
                result_sum += new_ranges.values().map(|r| r.len() as u64).product::<u64>()
            } else if let Result::Goto(wf_name) = result {
                todo.push_back((workflows.get(&wf_name as &str).unwrap(), new_ranges))
            }
        }
    }

    result_sum
}

fn main() {
    let (workflows, parts) = read_input("./inputs/day19");

    println!("Part 1:  {}", run_timed(|| part1(&workflows, &parts)));
    println!("Part 2:  {}", run_timed(|| part2(&workflows)));
}
