use std::collections::{BTreeMap, VecDeque};

use super::util::file_lines;
use itertools::Itertools;

trait ModBehaviorImpl {
    fn process_pulse(&mut self, origin: &str, p: bool) -> Option<bool>;
}

#[derive(Debug)]
struct FlipFlopModule {
    state: bool,
}

// TODO use dynamic dispatch?
impl ModBehaviorImpl for FlipFlopModule {
    fn process_pulse(&mut self, _origin: &str, p: bool) -> Option<bool> {
        if !p {
            self.state = !self.state;
            Some(self.state)
        } else {
            None
        }
    }
}

impl FlipFlopModule {
    fn new() -> FlipFlopModule {
        FlipFlopModule { state: false }
    }
}

#[derive(Debug)]
struct BroadcasterModule {}

impl BroadcasterModule {
    fn new() -> BroadcasterModule {
        BroadcasterModule {}
    }
}

impl ModBehaviorImpl for BroadcasterModule {
    fn process_pulse(&mut self, _origin: &str, p: bool) -> Option<bool> {
        Some(p)
    }
}

#[derive(Debug)]
struct ConjunctionModule {
    state: BTreeMap<String, bool>,
}

impl ConjunctionModule {
    fn new() -> ConjunctionModule {
        ConjunctionModule {
            state: BTreeMap::<String, bool>::new(),
        }
    }

    fn add_pre(&mut self, pre: &str) {
        self.state.insert(pre.to_string(), false);
    }
}

impl ModBehaviorImpl for ConjunctionModule {
    fn process_pulse(&mut self, origin: &str, p: bool) -> Option<bool> {
        let inp_state = self.state.get_mut(origin);
        *inp_state.unwrap() = p;
        Some(!self.state.iter().all(|(_, &v)| v))
    }
}

#[derive(Debug)]
enum ModuleBehavior {
    Broadcaster(BroadcasterModule),
    FlipFlop(FlipFlopModule),
    Conjunction(ConjunctionModule),
}

impl ModuleBehavior {
    fn ff_state(&self) -> bool {
        let ModuleBehavior::FlipFlop(fv) = self else {
            panic!()
        };
        fv.state
    }
}

impl ModBehaviorImpl for ModuleBehavior {
    fn process_pulse(&mut self, origin: &str, p: bool) -> Option<bool> {
        // FIXME not so nice
        match self {
            Self::Broadcaster(x) => x.process_pulse(origin, p),
            Self::FlipFlop(x) => x.process_pulse(origin, p),
            Self::Conjunction(x) => x.process_pulse(origin, p),
        }
    }
}

#[derive(Debug)]
struct Module {
    next_modules: Vec<String>,
    // FIXME dynamic dispatch? enum dispatch?
    behavior: ModuleBehavior,
}

use rand::seq::SliceRandom;

pub fn part1() {
    let lines = file_lines("inp20_2.txt");

    type ModuleMap = BTreeMap<String, Module>;
    let mut modules = ModuleMap::new();
    for line in &lines {
        let parts = line.split_whitespace().collect_vec();
        let behavior = &parts[0][0..1];
        let mut targets = parts[2..]
            .iter()
            .map(|s| s.strip_suffix(',').unwrap_or(s).to_string())
            .collect_vec();
        targets.shuffle(&mut rand::thread_rng());
        match behavior {
            "b" => modules.insert(
                parts[0].to_string(),
                Module {
                    next_modules: targets,
                    behavior: ModuleBehavior::Broadcaster(BroadcasterModule::new()),
                },
            ),
            "%" => modules.insert(
                parts[0][1..].to_string(),
                Module {
                    next_modules: targets,
                    behavior: ModuleBehavior::FlipFlop(FlipFlopModule::new()),
                },
            ),
            "&" => modules.insert(
                parts[0][1..].to_string(),
                Module {
                    next_modules: targets,
                    behavior: ModuleBehavior::Conjunction(ConjunctionModule::new()),
                },
            ),
            _ => panic!(),
        };
    }
    println!("{:?}", modules);

    let module_names = modules.keys().cloned().collect_vec();
    for module_name in module_names {
        let next_modules = modules[&module_name].next_modules.clone();
        for next_module in next_modules {
            // FIXME this could be nicer
            if !modules.contains_key(&next_module) {
                continue;
            }
            if let ModuleBehavior::Conjunction(cm) =
                &mut modules.get_mut(&next_module).unwrap().behavior
            {
                println!("add_pre {} to {}", module_name, next_module);
                cm.add_pre(&module_name);
            }
        }
    }

    println!("{:?}", modules);

    let mut low_pulses = 0_u64;
    let mut high_pulses = 0_u64;
    struct QueueEntry {
        pulse: bool,
        origin_module: String,
        module_name: String,
    }

    for _ in 0..1000 {
        let mut q = VecDeque::from([QueueEntry {
            pulse: false,
            origin_module: "".to_string(),
            module_name: "broadcaster".to_string(),
        }]);
        low_pulses += 1;
        while let Some(entry) = q.pop_back() {
            let module = modules.get_mut(&entry.module_name);
            if module.is_none() {
                continue;
            }
            let module = module.unwrap();
            let p = module
                .behavior
                .process_pulse(&entry.origin_module, entry.pulse);

            if let Some(p) = p {
                for next_module in &module.next_modules {
                    q.push_back(QueueEntry {
                        pulse: p,
                        origin_module: entry.module_name.clone(),
                        module_name: next_module.clone(),
                    });
                    if p {
                        high_pulses += 1;
                    } else {
                        low_pulses += 1;
                    }
                }
            }
        }
    }

    let pulse_mult = high_pulses * low_pulses;
    println!("{}", pulse_mult);
}

pub fn part2() {
    let lines = file_lines("inp20_2.txt");

    type ModuleMap = BTreeMap<String, Module>;
    let mut modules = ModuleMap::new();
    for line in &lines {
        let parts = line.split_whitespace().collect_vec();
        let behavior = &parts[0][0..1];
        let targets = parts[2..]
            .iter()
            .map(|s| s.strip_suffix(',').unwrap_or(s).to_string())
            .collect_vec();
        //targets.reverse();
        match behavior {
            "b" => modules.insert(
                parts[0].to_string(),
                Module {
                    next_modules: targets,
                    behavior: ModuleBehavior::Broadcaster(BroadcasterModule::new()),
                },
            ),
            "%" => modules.insert(
                parts[0][1..].to_string(),
                Module {
                    next_modules: targets,
                    behavior: ModuleBehavior::FlipFlop(FlipFlopModule::new()),
                },
            ),
            "&" => modules.insert(
                parts[0][1..].to_string(),
                Module {
                    next_modules: targets,
                    behavior: ModuleBehavior::Conjunction(ConjunctionModule::new()),
                },
            ),
            _ => panic!(),
        };
    }

    let module_names = modules.keys().cloned().collect_vec();
    for module_name in module_names {
        let next_modules = modules[&module_name].next_modules.clone();
        for next_module in next_modules {
            // FIXME this could be nicer
            if !modules.contains_key(&next_module) {
                continue;
            }
            if let ModuleBehavior::Conjunction(cm) =
                &mut modules.get_mut(&next_module).unwrap().behavior
            {
                //println!("add_pre {} to {}", module_name, next_module);
                cm.add_pre(&module_name);
            }
        }
    }

    // Output for graphviz to show the graph
    println!("digraph {{");

    for module in &modules {
        let color = match module.1.behavior {
            ModuleBehavior::Broadcaster(_) => "red",
            ModuleBehavior::Conjunction(_) => "blue",
            ModuleBehavior::FlipFlop(_) => "green",
        };
        println!("{} [color = {}]", module.0, color);
        for (i, next_module) in module.1.next_modules.iter().enumerate() {
            println!("{} -> {} [label={}]", module.0, next_module, i);
        }
    }
    println!("}}");

    struct QueueEntry {
        pulse: bool,
        origin_module: String,
        module_name: String,
    }

    let end_module = modules
        .iter()
        .find(|m| m.1.next_modules.contains(&"rx".to_string()))
        .unwrap()
        .0
        .clone();
    let ModuleBehavior::Conjunction(cbv) = &modules[&end_module].behavior else {
        panic!()
    };
    let mut done_in: Vec<Option<usize>> = vec![None; cbv.state.len()];

    'outer: for i in 1.. {
        let mut q = VecDeque::from([QueueEntry {
            pulse: false,
            origin_module: "".to_string(),
            module_name: "broadcaster".to_string(),
        }]);
        while let Some(entry) = q.pop_front() {
            if entry.module_name == end_module {
                let ModuleBehavior::Conjunction(cbv) = &modules[&entry.module_name].behavior else {
                    panic!()
                };
                for (state_idx, &state) in cbv.state.values().enumerate() {
                    if state {
                        done_in[state_idx].get_or_insert(i);
                    }
                }
                if done_in.iter().all(|v| v.is_some()) {
                    break 'outer;
                }
            }
            let module = modules.get_mut(&entry.module_name);
            if module.is_none() {
                continue;
            }
            let module = module.unwrap();
            let p = module
                .behavior
                .process_pulse(&entry.origin_module, entry.pulse);

            if let Some(p) = p {
                for next_module in &module.next_modules {
                    q.push_back(QueueEntry {
                        pulse: p,
                        origin_module: entry.module_name.clone(),
                        module_name: next_module.clone(),
                    });
                }
            }
        }
    }

    println!("{:?}", done_in);

    let lcm = done_in
        .iter()
        .fold(1_usize, |acc, n| num::Integer::lcm(&acc, &n.unwrap()));
    println!("{}", lcm);
}
