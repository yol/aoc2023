use std::collections::{BTreeMap, VecDeque};

use super::util::file_lines;
use itertools::Itertools;

trait ModBehaviorImpl {
    fn process_pulse(&mut self, origin: usize, p: bool) -> Option<bool>;
}

#[derive(Debug)]
struct FlipFlopModule {
    state: bool,
}

impl ModBehaviorImpl for FlipFlopModule {
    fn process_pulse(&mut self, _origin: usize, p: bool) -> Option<bool> {
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
    fn process_pulse(&mut self, _origin: usize, p: bool) -> Option<bool> {
        Some(p)
    }
}

#[derive(Debug)]
struct ConjunctionModule {
    state: BTreeMap<usize, bool>,
}

impl ConjunctionModule {
    fn new() -> ConjunctionModule {
        ConjunctionModule {
            state: BTreeMap::<usize, bool>::new(),
        }
    }

    fn add_pre(&mut self, pre: usize) {
        self.state.insert(pre, false);
    }
}

impl ModBehaviorImpl for ConjunctionModule {
    fn process_pulse(&mut self, origin: usize, p: bool) -> Option<bool> {
        let inp_state = self.state.get_mut(&origin);
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

impl ModBehaviorImpl for ModuleBehavior {
    fn process_pulse(&mut self, origin: usize, p: bool) -> Option<bool> {
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
    next_modules: Vec<usize>,
    // FIXME dynamic dispatch? enum dispatch?
    behavior: ModuleBehavior,
}

pub fn part1() {
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

    /*let mut n1 = 1_u64;
    let mut n2 = 1_u64;
    let mut n3 = 1_u64;
    loop {
        let r1 = n1 * 14401241;
        let r2 = 3928 + n2 * 3726;
        let mut r3 = 4048 + n3 * 4006;
        if r1 == r2 {
            println!("n1: {}, n2: {}  ==> {}", n1, n2, r1);
            while r3 < r1 {
                n3 += 1;
                r3 += 4006;
            }
            if r3 == r1 {
                println!("n1: {}, n2: {}, n3: {}  ==> {}", n1, n2, n3, r1);
                return;
            }
        }
        if r1 > r2 {
            n2 += 1;
        } else {
            n1 += 1;
        }
    }*/

    type ModuleMap = Vec<Module>;
    let mut modules = ModuleMap::new();
    let mut name_map = BTreeMap<String, usize>;
    for line in &lines {
        let parts = line.split_whitespace().collect_vec();
        let behavior = &parts[0][0..1];
        let targets = parts[2..]
            .iter()
            .map(|s| s.strip_suffix(',').unwrap_or(s).to_string())
            .collect_vec();
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

    struct QueueEntry {
        pulse: bool,
        origin_module: String,
        module_name: String,
    }

    // [dg] state: {"lk": false, "sp": false, "xt": false, "zv": false} }

    // 259356131034 too low
    // 28623445771888 too low#
    // 10028476985242

    // 1st: 3823 ...+3823 ...
    // 2nd: 3928 x2 + 3726 -> 7690 x2 +3726 -> 11452
    // 3rd: 3767 ...+3767 ...
    // 4th: 4048 x2 + 4006 -> 8054 x2 +4006 -> 12060

    //let pulse

    let mut last = 0;
    let idx = (1..)
        .find(|&i| {
            if (i % 1_000_000) == 0 {
                println!("!! {}", i);
            }
            let mut q = VecDeque::from([QueueEntry {
                pulse: false,
                origin_module: "".to_string(),
                module_name: "broadcaster".to_string(),
            }]);
            while let Some(entry) = q.pop_back() {
                if entry.module_name == "dg" {
                    let ModuleBehavior::Conjunction(cbv) = &modules[&entry.module_name].behavior
                    else {
                        panic!()
                    };
                    /*let diff = i - last;
                    if *cbv.state.iter().nth(2).unwrap().1 {
                        if diff != 3767 {
                            println!("diff: {}", i - last);
                        }
                        last = i;
                    } else {
                        if diff > 3767 {
                            println!(":(");
                        }
                    }*/

                    if cbv.state.iter().filter(|(_, &v)| v).count() > 1 {
                        let s = String::from_iter(
                            cbv.state.iter().map(|(_, &v)| if v { '!' } else { '.' }),
                        );
                        println!("[{:9}] {}", i, s);
                    }
                }
                if entry.module_name == "rx" && !entry.pulse {
                    return true;
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

            false
        })
        .unwrap();
    println!("{}", idx);
}
