use std::{
    collections::{HashMap, VecDeque},
    io::{self, BufRead},
};

#[derive(Debug)]
enum ModuleType {
    Broadcaster,
    FlipFlop(bool),
    Conjunction(Vec<bool>, bool),
}

#[derive(Debug)]
struct Module {
    mod_type: ModuleType,
    name: String,
    output: Vec<String>,
}

impl Module {
    fn reset(&mut self) {
        match self.mod_type {
            ModuleType::FlipFlop(ref mut state) => *state = false,
            ModuleType::Conjunction(ref mut inputs, ref mut output) => {
                *inputs = vec![false; self.output.len()];
                *output = true;
            }
            ModuleType::Broadcaster => (),
        }
    }

    fn trigger(&mut self, value: bool, input_name: &String) {
        dbg!(&self, &value, &input_name);
        match self.mod_type {
            ModuleType::FlipFlop(ref mut state) => {
                if !value {
                    *state = !*state;
                }
            }
            ModuleType::Conjunction(ref mut input, ref mut output) => {
                let this_input = self
                    .input
                    .iter()
                    .position(|name| name == input_name)
                    .unwrap();
                input[this_input] = value;
                *output = !input.iter().fold(true, |acc, x| acc && *x);
            }
            ModuleType::Broadcaster => (),
        }
    }
}

fn parse_line(s: &str) -> Module {
    let mod_type = match s.chars().nth(0).unwrap() {
        '%' => ModuleType::FlipFlop(false),
        '&' => ModuleType::Conjunction(vec![], true),
        'b' => ModuleType::Broadcaster,
        _ => panic!("Invalid module type"),
    };

    let name = match mod_type {
        ModuleType::Broadcaster => "broadcaster".into(),
        _ => s
            .chars()
            .skip(1)
            .take_while(|c| c != &' ')
            .collect::<String>(),
    };

    let output = s.split("->").nth(1).unwrap();
    let names = output
        .chars()
        .filter(|c| c.is_alphabetic() || *c == ' ')
        .collect::<String>()
        .split(" ")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    return Module {
        name,
        mod_type,
        output: names[1..].to_vec(),
    };
}

fn main() {
    let lines: Vec<String> = io::stdin().lock().lines().map(|l| l.unwrap()).collect();
    let mut modules = lines.iter().map(|l| parse_line(l)).collect::<Vec<Module>>();

    let mut state: HashMap<String, bool> = HashMap::new();

    modules.iter_mut().for_each(|m| {
        m.reset();
        state.insert(m.name.clone(), false);
    });

    let mut q = VecDeque::new();
    let (mut low, mut high) = (0, 0);
    for _ in [0..1000] {
        q.push_back("broadcaster".to_string());
        while !q.is_empty() {
            let current = q.pop_front().unwrap();
            let mut modules = modules.iter_mut();

            let module = modules.find(|m| m.name == current).unwrap();
            match module.mod_type {
                ModuleType::Broadcaster => {
                    for out_mod in &module.output {
                        low += 1;
                        modules
                            .find(|m| m.name == *out_mod)
                            .unwrap()
                            .trigger(false, out_mod);
                        q.push_back(out_mod.clone());
                    }
                }
                ModuleType::FlipFlop(state) => {
                    for out_mod in &module.output {
                        match state {
                            false => low += 1,
                            true => high += 1,
                        }
                        modules
                            .find(|m| m.name == *out_mod)
                            .unwrap()
                            .trigger(state, out_mod);
                    }
                }
                ModuleType::Conjunction(_, state) => {
                    for (i, out_mod) in module.output.iter().enumerate() {
                        match state {
                            false => low += 1,
                            true => high += 1,
                        }
                        modules
                            .find(|m| m.name == *out_mod)
                            .unwrap()
                            .trigger(state, out_mod);
                    }
                }

                _ => (),
            }
        }
    }

    dbg!(low, high);
    println!("Part 1: {}", low * high);
}
