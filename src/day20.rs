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

    /*let mut n1 = 1_u64;
    let mut n2 = 1_u64;
    let mut n3 = 1_u64;

    const LCMFAC1: u64 = 4051;
    const LCMFAC2: u64 = 3929;
    const LINOFF1: u64 = 3760;
    const LINSCALE1: u64 = 3438;
    const LINOFF2: u64 = 3820;
    const LINSCALE2: u64 = 3550;

    loop {
        let r1 = n1 * LCMFAC1 * LCMFAC2;
        let r2 = LINOFF1 + n2 * LINSCALE1;
        let mut r3 = LINOFF2 + n3 * LINSCALE2;
        if r1 == r2 {
            println!("n1: {}, n2: {}  ==> {}", n1, n2, r1);
            while r3 < r1 {
                n3 += 1;
                r3 += LINSCALE2;
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
    //println!("{:?}", modules);

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

    // println!("{:?}", modules);

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

    //return;

    struct QueueEntry {
        pulse: bool,
        origin_module: String,
        module_name: String,
    }

    // 259356131034 too low
    // 28623445771888 too low
    // 27162814304704 (too low)
    // 64377328301638 wrong
    // 229215609826339

    // rc -> dv: 3760 + n * 3438
    // nt -> xq:    0 + n * 3929
    // mg -> jc: 3820 + n * 3550
    // kx -> vv:    0 + n * 4051
    // lcm(4051,3929) -> 15916379
    // --> 47468690402020: wrong

    // rc -> dv:    0 + n * 3767
    // nt -> xq:    0 + n * 3929
    // mg -> jc:    0 + n * 3823
    // kx -> vv:    0 + n * 4051
    // lcm 229215609826339: correct

    // independent subunits:
    // rc -> dv, nt -> xq, mg -> jc, kx -> vv

    let start = "rc".to_string();
    let end = "dv".to_string();
    let investig = ["bl", "fd", "hx"];

    let mut done_in: Vec<usize> = Vec::new();

    let idx = (1..20000).find(|&i| {
        if (i % 1_000_000) == 0 {
            println!("!! {}", i);
        }
        let mut q = VecDeque::from([QueueEntry {
            pulse: false,
            origin_module: "".to_string(),
            module_name: start.clone(),
        }]);
        let mut was_good = false;
        let mut last_sent_by_end: Option<bool> = None;
        while let Some(entry) = q.pop_front() {
            if entry.module_name == end {
                let ModuleBehavior::Conjunction(cbv) = &modules[&end].behavior else {
                    panic!()
                };
                if cbv.state.values().all(|&v| v) {
                    let states = String::from_iter(investig.iter().map(|&i| {
                        if modules[i].behavior.ff_state() {
                            '!'
                        } else {
                            '.'
                        }
                    }));

                    /*println!("{} {}", i, states);
                    println!(
                        "{}",
                        String::from_iter(cbv.state.values().map(|&i| if i { '!' } else { '.' }))
                    );
                    println!(
                        "{}",
                        String::from_iter(modules.values().map(|m| {
                            match m.behavior {
                                ModuleBehavior::FlipFlop(_) => {
                                    if m.behavior.ff_state() {
                                        '!'
                                    } else {
                                        '.'
                                    }
                                }
                                _ => '_',
                            }
                        }))
                    );*/
                    was_good = true;
                    done_in.push(i);
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

            /*if entry.module_name == end && !p.unwrap() {
                //println!("{}", i);
                done_in.push(i);
                /*let ModuleBehavior::Conjunction(cbv) = &modules[&entry.module_name].behavior
                else {
                    panic!()
                };

                if cbv.state.iter().filter(|(_, &v)| v).count() > 1 {
                    let s = String::from_iter(
                        cbv.state.iter().map(|(_, &v)| if v { '!' } else { '.' }),
                    );
                    println!("[{:9}] {}", i, s);
                }*/
            }*/

            if let Some(p) = p {
                if entry.module_name == end {
                    last_sent_by_end = Some(p);
                }
                for next_module in &module.next_modules {
                    q.push_back(QueueEntry {
                        pulse: p,
                        origin_module: entry.module_name.clone(),
                        module_name: next_module.clone(),
                    });
                }
            }
        }

        let ModuleBehavior::Conjunction(cbv) = &modules[&end].behavior else {
            panic!()
        };
        /*if was_good {
            println!("==> {:?}", last_sent_by_end);
            println!(
                "{}",
                String::from_iter(cbv.state.values().map(|&i| if i { '!' } else { '.' }))
            );
            println!(
                "{}",
                String::from_iter(modules.values().map(|m| {
                    match m.behavior {
                        ModuleBehavior::FlipFlop(_) => {
                            if m.behavior.ff_state() {
                                '!'
                            } else {
                                '.'
                            }
                        }
                        _ => '_',
                    }
                }))
            );
        }*/
        if cbv.state.values().all(|&v| v) {
            println!("&&&&&&&&&& {}", i);
            done_in.push(i);
        }

        false
    });

    if idx.is_none() {
        for (a, b) in done_in.iter().tuple_windows() {
            println!("{:6} +{:6} -> {:6}", a, b - a, b);
        }
    }
}
