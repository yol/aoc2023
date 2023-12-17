use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use itertools::Itertools;

pub fn part1() {
    let file = File::open(Path::new("inp12_2.txt")).unwrap();
    let lines = io::BufReader::new(file).lines().peekable();

    const OK: i8 = 0;
    const DAMAGED: i8 = 1;
    const UNKNOWN: i8 = -1;

    let spring_combinations = lines.map(|l| {
        // FIXME why do I have to write this?
        let l_unwrap = l.unwrap();
        let mut line_parts = l_unwrap.split_whitespace();
        let spring = line_parts
            .next()
            .unwrap()
            .chars()
            .map(|c| match c {
                '.' => OK,
                '#' => DAMAGED,
                '?' => UNKNOWN,
                _ => panic!(),
            })
            .collect_vec();
        let grouped_record = line_parts
            .next()
            .unwrap()
            .split(',')
            .map(|r| r.parse().unwrap())
            .collect_vec();
        let open_positions = spring
            .iter()
            .enumerate()
            .filter(|(_, c)| **c == UNKNOWN)
            .map(|(pos, _)| pos)
            .collect_vec();

        (0..2_usize.pow(open_positions.len() as u32))
            .filter(|idx| {
                let mut this_spring = spring.clone();
                for (zero_idx, char_idx) in open_positions.iter().enumerate() {
                    let verdict = ((idx >> zero_idx) & 1) > 0;
                    this_spring[*char_idx] = match verdict {
                        false => OK,
                        true => DAMAGED,
                    }
                }
                let mut this_grouped_record: Vec<usize> = Vec::new();
                let mut this_group_size = 0_usize;
                // FIXME not nice
                for c in this_spring {
                    if c == DAMAGED {
                        this_group_size += 1;
                    } else {
                        if this_group_size > 0 {
                            this_grouped_record.push(this_group_size);
                        }
                        this_group_size = 0;
                    }
                }
                if this_group_size > 0 {
                    this_grouped_record.push(this_group_size);
                }
                this_grouped_record == grouped_record
            })
            .count()
    });
    let sum: usize = spring_combinations.sum();
    println!("{}", sum);
}

pub fn part2() {
    let file = File::open(Path::new("inp12_2.txt")).unwrap();
    let lines = io::BufReader::new(file).lines().collect_vec();

    const OK: i8 = 0;
    const DAMAGED: i8 = 1;
    const UNKNOWN: i8 = -1;

    fn print_entry(entry: &QueueEntry) {
        println!(
            "{} | {:?}",
            entry
                .spring
                .iter()
                .map(|&c| match c {
                    OK => '.',
                    DAMAGED => '#',
                    UNKNOWN => '?',
                    _ => panic!(),
                })
                .collect::<String>(),
            entry.groups_to_assign
        );
    }

    type Spring = Vec<i8>;
    type CacheKey = (Spring, Vec<usize>);

    //let mut cache: HashMap<CacheKey, usize> = HashMap::new();

    #[derive(Debug, PartialEq, Eq, Clone)]
    struct QueueEntry {
        spring: Spring,
        groups_to_assign: Vec<usize>,
        predecessors: Vec<CacheKey>,
        is_sentinel: bool,
    }

    let mut iteri = 0;

    let mut cache: HashMap<CacheKey, usize> = HashMap::new();

    let spring_combinations = lines
        .iter()
        //.skip(1)
        //.take(1)
        //.par_iter()
        //.progress()
        .map(|l| {
            println!("start {}", iteri);
            iteri += 1;
            // FIXME why do I have to write this? why as_ref()?
            let l_unwrap = l.as_ref().unwrap();
            let mut line_parts = l_unwrap.split_whitespace();
            let orig_spring = line_parts
                .next()
                .unwrap()
                .chars()
                .map(|c| match c {
                    '.' => OK,
                    '#' => DAMAGED,
                    '?' => UNKNOWN,
                    _ => panic!(),
                })
                .collect_vec();
            let mut spring = orig_spring.clone();
            for _ in 0..4 {
                let mut spring_copy = orig_spring.clone();
                spring.push(UNKNOWN);
                spring.append(&mut spring_copy);
            }
            // Let "DAMAGED" counting loop always finish with OK
            spring.push(OK);

            let orig_grouped_record = line_parts
                .next()
                .unwrap()
                .split(',')
                .map(|r| r.parse().unwrap())
                .collect_vec();
            let mut grouped_record = orig_grouped_record.clone();
            for _ in 0..4 {
                let mut record_copy = orig_grouped_record.clone();
                grouped_record.append(&mut record_copy);
            }

            // FIXME can be removed?
            let next_not_ok_char_idx = spring.iter().find_position(|&&c| c != OK).unwrap().0;
            let start_spring = &spring[next_not_ok_char_idx..];

            let mut q = VecDeque::from([QueueEntry {
                spring: start_spring.to_vec(),
                groups_to_assign: grouped_record.clone(), // FIXME remove clone
                predecessors: vec![],
                is_sentinel: false,
            }]);

            let mut ok_combinations: usize = 0;
            while let Some(entry) = q.pop_back() {
                if cfg!(debug_assertions) {
                    print_entry(&entry);
                }

                // Process sentinel
                if entry.is_sentinel {
                    cache
                        .entry((entry.spring.clone(), entry.groups_to_assign.clone()))
                        .or_insert(0);
                    continue;
                }

                // Skip OK times at the start, they are not interesting
                let next_not_ok_char_idx =
                    entry.spring.iter().find_position(|&&c| c != OK).unwrap().0;
                let spring = &entry.spring[next_not_ok_char_idx..];

                // Add sentinel
                q.push_back(QueueEntry {
                    spring: spring.to_vec(),
                    groups_to_assign: entry.groups_to_assign.clone(),
                    predecessors: vec![],
                    is_sentinel: true,
                });

                let mut add_queue = |next_spring: Spring, next_groups_to_assign: Vec<usize>| {
                    assert!(!next_groups_to_assign.is_empty());
                    if next_spring.is_empty() {
                        return;
                    }
                    // This could be a bit more intelligent
                    let total_not_ok_needed = next_groups_to_assign.iter().sum::<usize>();
                    let total_not_ok = next_spring.iter().filter(|&&c| c != OK).count();
                    if total_not_ok < total_not_ok_needed {
                        if cfg!(debug_assertions) {
                            //println!("-> exit4");
                        }
                        return;
                    }

                    let mut next_predecessors = entry.predecessors.clone();
                    next_predecessors.push((spring.into(), entry.groups_to_assign.clone()));
                    let next_entry = QueueEntry {
                        spring: next_spring,
                        groups_to_assign: next_groups_to_assign,
                        predecessors: next_predecessors,
                        is_sentinel: false,
                    };
                    q.push_back(next_entry);
                };

                // Check cache
                if let Some(&cache_entry) =
                    cache.get(&(spring.to_vec(), entry.groups_to_assign.clone()))
                {
                    if cfg!(debug_assertions) {
                        println!("!!OK!! cache: {}", cache_entry);
                    }
                    ok_combinations += cache_entry;
                    // Update cache
                    for predecessor in &entry.predecessors {
                        // FIXME clone()?
                        *cache.entry(predecessor.clone()).or_insert(0) += cache_entry;
                    }
                    continue;
                }

                let group_to_assign = entry.groups_to_assign[0];

                // Now the spring starts with UNKNOWN, try to place the next group somewhere
                let max_group_len_from_here = spring
                    .iter()
                    .find_position(|&&c| c != DAMAGED && c != UNKNOWN)
                    .unwrap()
                    .0;

                if group_to_assign > max_group_len_from_here {
                    // Not possible to place group here, try without the next ?/# group
                    let next_ok_char_idx = match spring.iter().find_position(|&&c| c == OK) {
                        None => {
                            if cfg!(debug_assertions) {
                                //println!("-> exit2");
                            }
                            continue;
                        }
                        Some(x) => x.0,
                    };
                    let next_spring = spring[next_ok_char_idx + 1..].into();
                    add_queue(next_spring, entry.groups_to_assign.clone());

                    continue;
                }

                // Try all placings
                let mut first_fixed_damage: Option<usize> = None;
                for start_i in 0..=(max_group_len_from_here - group_to_assign) {
                    // Check: we cannot overrun a contiguous group in the input
                    let this_ch = spring[start_i];
                    if this_ch == DAMAGED {
                        first_fixed_damage.get_or_insert(start_i);
                    }
                    // Check: it has to be possible to make an OK after the group
                    let ch_after_group = spring[start_i + group_to_assign];
                    if ch_after_group == DAMAGED {
                        // Not possible
                        continue;
                    }
                    if let Some(first_fixed_val) = first_fixed_damage {
                        if start_i + group_to_assign > group_to_assign + first_fixed_val {
                            break;
                        }
                    }
                    // FIXME additional check that we do not overrun the max. possible length
                    // Place group
                    /*let mut next_spring: Vec<i8> = spring[start_i..].into();
                    for col in 0..group_to_assign {
                        next_spring[col] = DAMAGED;
                    }
                    next_spring[group_to_assign] = OK;*/

                    let next_spring = spring[start_i + group_to_assign + 1..].to_vec();
                    let next_groups_to_assign: Vec<usize> = entry.groups_to_assign[1..].into();

                    if cfg!(debug_assertions) {
                        //println!("start_i {} -> entry:", start_i);
                        //print_entry(&next_entry);
                        //println!("+");
                    }

                    if next_groups_to_assign.is_empty() {
                        if next_spring.iter().find(|&&c| c == DAMAGED).is_some() {
                            // Not good...
                            continue;
                        }
                        if cfg!(debug_assertions) {
                            println!("OK: ///");
                            print_entry(&entry);
                            println!("    \\\\\\");
                        }
                        ok_combinations += 1;
                        // Update cache
                        for predecessor in &entry.predecessors {
                            // FIXME clone()?
                            *cache.entry(predecessor.clone()).or_insert(0) += 1;
                        }
                        *cache
                            .entry((spring.to_vec(), entry.groups_to_assign.clone()))
                            .or_insert(0) += 1;
                    } else {
                        add_queue(next_spring, next_groups_to_assign);
                    }
                }

                // Try skipping this placeholder altogether
                add_queue(
                    spring[max_group_len_from_here..].into(),
                    entry.groups_to_assign.clone(),
                );
                if cfg!(debug_assertions) {
                    //println!("--");
                }
            }

            println!(
                "====== {} / {}",
                ok_combinations,
                cache
                    .get(&(start_spring.to_vec(), grouped_record.clone()))
                    .unwrap()
            );

            assert_eq!(
                ok_combinations,
                *cache
                    .get(&(start_spring.to_vec(), grouped_record.clone()))
                    .unwrap()
            );

            // FIXME use cache
            ok_combinations
        });
    let sum: usize = spring_combinations.sum();
    println!("[4780439962549]");
    println!(" {}", sum);
}
