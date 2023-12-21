use aoc2023::{read_lines, run_timed};
use std::collections::{HashMap, HashSet, VecDeque};
use std::num::Wrapping;

type SendPulse = (Pulse, String, String);
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Pulse {
    High,
    Low,
}

trait Module {
    fn receive(&mut self, p: Pulse, source: &str) -> Vec<SendPulse>;

    fn add_incoming(&mut self, src: &str);

    fn add_outgoing(&mut self, dest: &str);

    fn high_count(&self) -> u64;
    fn low_count(&self) -> u64;
}

struct BroadCast {
    outgoing: HashSet<String>,
    h_count: Wrapping<u64>,
    l_count: Wrapping<u64>,
}

impl BroadCast {
    fn new() -> BroadCast {
        BroadCast {
            outgoing: HashSet::new(),
            h_count: Default::default(),
            l_count: Default::default(),
        }
    }
}

struct FlipFlop {
    id: String,
    on: bool,
    outgoing: HashSet<String>,
    h_count: Wrapping<u64>,
    l_count: Wrapping<u64>,
}

impl FlipFlop {
    fn new(id: String) -> FlipFlop {
        FlipFlop {
            id,
            on: false,
            outgoing: HashSet::new(),
            h_count: Default::default(),
            l_count: Default::default(),
        }
    }
}

struct Conjunction {
    id: String,
    incoming: HashMap<String, Pulse>,
    outgoing: HashSet<String>,
    h_count: Wrapping<u64>,
    l_count: Wrapping<u64>,
}

impl Conjunction {
    fn new(id: String) -> Conjunction {
        Conjunction {
            id,
            incoming: HashMap::new(),
            outgoing: HashSet::new(),
            h_count: Default::default(),
            l_count: Default::default(),
        }
    }
}

struct LowCounter {
    h_count: Wrapping<u64>,
    l_count: Wrapping<u64>,
}

impl LowCounter {
    fn new() -> LowCounter {
        LowCounter {
            h_count: Default::default(),
            l_count: Default::default(),
        }
    }
}

impl Module for LowCounter {
    fn receive(&mut self, p: Pulse, _: &str) -> Vec<SendPulse> {
        match p {
            Pulse::High => self.h_count += 1,
            Pulse::Low => self.l_count += 1,
        };
        vec![]
    }

    fn add_incoming(&mut self, _: &str) {}

    fn add_outgoing(&mut self, _: &str) {}

    fn high_count(&self) -> u64 {
        self.h_count.0
    }
    fn low_count(&self) -> u64 {
        self.l_count.0
    }
}

impl Module for BroadCast {
    fn receive(&mut self, p: Pulse, _: &str) -> Vec<SendPulse> {
        match p {
            Pulse::High => self.h_count += 1,
            Pulse::Low => self.l_count += 1,
        }
        self.outgoing
            .iter()
            .map(|m| (p, "broadcaster".to_string(), m.clone()))
            .collect()
    }

    fn add_incoming(&mut self, _: &str) {}

    fn add_outgoing(&mut self, dest: &str) {
        self.outgoing.insert(dest.to_string());
    }

    fn high_count(&self) -> u64 {
        self.h_count.0
    }

    fn low_count(&self) -> u64 {
        self.l_count.0
    }
}

impl Module for FlipFlop {
    fn receive(&mut self, p: Pulse, _source: &str) -> Vec<SendPulse> {
        match p {
            Pulse::High => {
                self.h_count += 1;
                vec![]
            }
            Pulse::Low => {
                self.l_count += 1;
                let to_send = if self.on { Pulse::Low } else { Pulse::High };
                self.on = !self.on;
                self.outgoing
                    .iter()
                    .map(|m| (to_send, self.id.clone(), m.clone()))
                    .collect()
            }
        }
    }

    fn add_incoming(&mut self, _: &str) {} //no-op, we dont count those here.

    fn add_outgoing(&mut self, dest: &str) {
        self.outgoing.insert(dest.to_string());
    }

    fn high_count(&self) -> u64 {
        self.h_count.0
    }

    fn low_count(&self) -> u64 {
        self.l_count.0
    }
}

impl Module for Conjunction {
    fn receive(&mut self, p: Pulse, source: &str) -> Vec<SendPulse> {
        match p {
            Pulse::High => self.h_count += 1,
            Pulse::Low => self.l_count += 1,
        }

        if let Some(last_p) = self.incoming.get_mut(source) {
            *last_p = p;
        } else {
            panic!("Got signal from unknown incoming connection {}", source);
        }

        let out_pulse = if self.incoming.values().all(|v| *v == Pulse::High) {
            Pulse::Low
        } else {
            Pulse::High
        };

        self.outgoing
            .iter()
            .map(|m| (out_pulse, self.id.clone(), m.clone()))
            .collect()
    }

    fn add_incoming(&mut self, src: &str) {
        self.incoming.insert(src.to_string(), Pulse::Low);
    }

    fn add_outgoing(&mut self, dest: &str) {
        self.outgoing.insert(dest.to_string());
    }

    fn high_count(&self) -> u64 {
        self.h_count.0
    }

    fn low_count(&self) -> u64 {
        self.l_count.0
    }
}

struct Circuit {
    modules: HashMap<String, Box<dyn Module>>,
}

impl Circuit {
    fn from_file(file_name: &str) -> Circuit {
        let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();
        modules.insert("rx".to_string(), Box::new(LowCounter::new()));

        let all_lines: Vec<String> = read_lines(file_name).collect();
        //first add all modules
        for line in all_lines.iter() {
            let (id_part, _) = line.split_once(" -> ").unwrap();
            if id_part == "broadcaster" {
                modules.insert("broadcaster".to_string(), Box::new(BroadCast::new()));
            } else {
                let id = id_part[1..].to_string();
                let module: Box<dyn Module> = match &id_part[0..1] {
                    "&" => Box::new(Conjunction::new(id.clone())),
                    "%" => Box::new(FlipFlop::new(id.clone())),
                    other => panic!("Invalid type identifier: {}", other),
                };
                modules.insert(id, module);
            }
        }

        for line in all_lines.iter() {
            let (id_parts, connect_to) = line.split_once(" -> ").unwrap();
            let id = match id_parts {
                "broadcaster" => "broadcaster",
                other => &other[1..],
            };

            for c in connect_to.split(", ") {
                if let Some(m) = modules.get_mut(c) {
                    m.add_incoming(id);
                }
                modules.get_mut(id).unwrap().add_outgoing(c);
            }
        }

        Circuit { modules }
    }

    fn process_press(&mut self) {
        let mut pulse_queue: VecDeque<SendPulse> = VecDeque::new();
        pulse_queue.push_back((Pulse::Low, "button".to_string(), "broadcaster".to_string()));

        while let Some((pulse, src, dest)) = pulse_queue.pop_front() {
            if let Some(dest_module) = self.modules.get_mut(&dest) {
                let new_actions = dest_module.receive(pulse, &src);
                pulse_queue.extend(new_actions)
            }
        }
    }
}

fn main() {
    println!(
        "Part 1: {}",
        run_timed(|| {
            let mut circuit = Circuit::from_file("./inputs/day20");

            for _ in 0..1000 {
                circuit.process_press();
            }

            let (low_total, high_total) = circuit
                .modules
                .values()
                .into_iter()
                .fold((0, 0), |(low, high), m| {
                    (low + m.low_count(), high + m.high_count())
                });

            low_total * high_total
        })
    );

    /* This runs for hours, needs a better way
    (i.e.: cycle length detection in subgraphs & then compute other cycles from there)
    */
    println!(
        "Part 2: {}",
        run_timed(|| {
            let mut circuit = Circuit::from_file("./inputs/day20");
            let mut press_count: u128 = 0;

            while circuit.modules.get("rx").unwrap().low_count() < 1 {
                circuit.process_press();
                press_count += 1;
                if press_count % 1_000_000 == 0 {
                    println!("{}", press_count)
                }
            }

            press_count
        })
    )
}
