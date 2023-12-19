use std::{
    cmp::{max, min},
    collections::{HashMap, VecDeque},
};

use super::util::file_lines;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ValueTag {
    X = 0,
    M = 1,
    A = 2,
    S = 3,
}

impl ValueTag {
    fn from_char(c: char) -> ValueTag {
        match c {
            'x' => ValueTag::X,
            'm' => ValueTag::M,
            'a' => ValueTag::A,
            's' => ValueTag::S,
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Operator {
    Lt,
    Gt,
}

impl Operator {
    fn from_char(c: char) -> Operator {
        match c {
            '<' => Operator::Lt,
            '>' => Operator::Gt,
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Verdict {
    Reject,
    Accept,
    GoTo(String),
}

impl Verdict {
    fn from_str(s: &str) -> Verdict {
        match s {
            "A" => Verdict::Accept,
            "R" => Verdict::Reject,
            x => Verdict::GoTo(x.to_string()),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ComparisonInstruction {
    tag: ValueTag,
    op: Operator,
    value: usize,
    verdict: Verdict,
}

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Compare(ComparisonInstruction),
    Judge(Verdict),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct PartGen<T> {
    x: T,
    m: T,
    a: T,
    s: T,
}

impl<T> PartGen<T> {
    fn get_tagged(&self, tag: ValueTag) -> &T {
        match tag {
            ValueTag::X => &self.x,
            ValueTag::M => &self.m,
            ValueTag::A => &self.a,
            ValueTag::S => &self.s,
        }
    }

    // FIXME necessary to implement twice?
    fn get_tagged_mut(&mut self, tag: ValueTag) -> &mut T {
        match tag {
            ValueTag::X => &mut self.x,
            ValueTag::M => &mut self.m,
            ValueTag::A => &mut self.a,
            ValueTag::S => &mut self.s,
        }
    }
}

type Part = PartGen<usize>;

impl Part {
    fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

fn parse(lines: &Vec<String>) -> (HashMap<String, Vec<Instruction>>, Vec<Part>) {
    let (instructions_s, parts_s) = lines.split_once(|l| l == "").unwrap();

    let re = Regex::new(r"^([xmas])([><])(\d+):(\w+)$").unwrap();

    let workflows: HashMap<String, Vec<Instruction>> = instructions_s
        .iter()
        .map(|l| {
            let (label, rem) = l.split_once('{').unwrap();
            let rem = &rem[0..rem.len() - 1];

            let instrs_s = rem.split(',').collect_vec();
            let instrs = instrs_s
                .iter()
                .map(|&i| {
                    let full_match = re.captures(i);
                    match full_match {
                        None => Instruction::Judge(Verdict::from_str(i)),
                        Some(full_match) => Instruction::Compare(ComparisonInstruction {
                            tag: ValueTag::from_char(
                                full_match.get(1).unwrap().as_str().chars().nth(0).unwrap(),
                            ),
                            op: Operator::from_char(
                                full_match.get(2).unwrap().as_str().chars().nth(0).unwrap(),
                            ),
                            value: full_match.get(3).unwrap().as_str().parse().unwrap(),
                            verdict: Verdict::from_str(full_match.get(4).unwrap().as_str()),
                        }),
                    }
                })
                .collect_vec();

            (label.to_string(), instrs)
        })
        .collect();

    let parts_re = Regex::new(r"^\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}$").unwrap();
    let parts = parts_s
        .iter()
        .map(|l| {
            let c = parts_re
                .captures(l)
                .unwrap()
                .iter()
                .skip(1)
                .map(|g| g.unwrap().as_str().parse().unwrap())
                .collect_vec();
            Part {
                x: c[0],
                m: c[1],
                a: c[2],
                s: c[3],
            }
        })
        .collect_vec();

    (workflows, parts)
}

pub fn part1() {
    let lines = file_lines("inp19_2.txt");
    let (workflows, parts) = parse(&lines);

    println!("{:?}", workflows);
    println!("{:?}", parts);

    let accepted_parts = parts.iter().filter(|part| {
        let mut workflow_name = "in".to_string();

        loop {
            let workflow = &workflows[&workflow_name];

            let mut jumped = false;
            for instr in workflow {
                match instr {
                    Instruction::Compare(c) => {
                        let val = part.get_tagged(c.tag);
                        let comp_match = match c.op {
                            Operator::Gt => *val > c.value,
                            Operator::Lt => *val < c.value,
                        };
                        if comp_match {
                            match &c.verdict {
                                Verdict::Accept => return true,
                                Verdict::Reject => return false,
                                Verdict::GoTo(w) => {
                                    jumped = true;
                                    workflow_name = w.clone();
                                }
                            }
                            break;
                        }
                    }
                    Instruction::Judge(v) => {
                        match v {
                            Verdict::Accept => return true,
                            Verdict::Reject => return false,
                            Verdict::GoTo(w) => {
                                jumped = true;
                                workflow_name = w.clone();
                            }
                        }
                        break;
                    }
                }
            }
            assert!(jumped);
        }
    });

    let sum: usize = accepted_parts.map(|p| p.sum()).sum();
    println!("{}", sum);
}

pub fn part2() {
    let lines = file_lines("inp19_2.txt");
    let (workflows, _) = parse(&lines);

    // Start and end, inclusive
    type TagRange = (usize, usize);

    type PartRange = PartGen<TagRange>;

    const FULL_RANGE: TagRange = (1, 4000);
    let start_part_range = PartRange {
        x: FULL_RANGE,
        m: FULL_RANGE,
        a: FULL_RANGE,
        s: FULL_RANGE,
    };

    #[derive(Debug)]
    struct QueueEntry {
        // FIXME how to include a reference here?
        workflow_name: String,
        workflow_index: usize,
        part_range: PartRange,
    }

    let mut q = VecDeque::from([QueueEntry {
        // FIXME how to get a literal as String?
        workflow_name: "in".to_string(),
        workflow_index: 0,
        part_range: start_part_range,
    }]);

    let mut accepted_part_ranges = vec![];

    fn range_overlap(a: TagRange, b: TagRange) -> Option<TagRange> {
        let start = max(a.0, b.0);
        let end = min(a.1, b.1);
        if end >= start {
            Some((start, end))
        } else {
            None
        }
    }

    while let Some(entry) = q.pop_back() {
        let workflow = &workflows[&entry.workflow_name];
        let instr = &workflow[entry.workflow_index];

        match instr {
            Instruction::Compare(c) => {
                let &prev_tag_range = entry.part_range.get_tagged(c.tag);
                let op_val = c.value;

                let new_tag_range_match = range_overlap(
                    prev_tag_range,
                    match c.op {
                        Operator::Gt => (op_val + 1, FULL_RANGE.1),
                        Operator::Lt => (FULL_RANGE.0, op_val - 1),
                    },
                );
                if let Some(new_tag_range_match) = new_tag_range_match {
                    let mut new_part_range = entry.part_range.clone();
                    *new_part_range.get_tagged_mut(c.tag) = new_tag_range_match;
                    match &c.verdict {
                        Verdict::Accept => accepted_part_ranges.push(new_part_range),
                        Verdict::Reject => {}
                        Verdict::GoTo(w) => {
                            q.push_back(QueueEntry {
                                workflow_name: w.clone(),
                                workflow_index: 0,
                                part_range: new_part_range,
                            });
                        }
                    }
                }

                let new_tag_range_mismatch = range_overlap(
                    prev_tag_range,
                    match c.op {
                        Operator::Gt => (FULL_RANGE.0, op_val),
                        Operator::Lt => (op_val, FULL_RANGE.1),
                    },
                );
                if let Some(new_tag_range_mismatch) = new_tag_range_mismatch {
                    let mut new_part_range = entry.part_range.clone();
                    *new_part_range.get_tagged_mut(c.tag) = new_tag_range_mismatch;
                    q.push_back(QueueEntry {
                        workflow_name: entry.workflow_name.clone(),
                        workflow_index: entry.workflow_index + 1,
                        part_range: new_part_range,
                    });
                }
            }
            Instruction::Judge(v) => match v {
                Verdict::Accept => {
                    // Finished, register and give up the branch
                    accepted_part_ranges.push(entry.part_range);
                }
                Verdict::Reject => {
                    // Give up this branch
                }
                Verdict::GoTo(w) => {
                    q.push_back(QueueEntry {
                        workflow_name: w.clone(),
                        workflow_index: 0,
                        part_range: entry.part_range,
                    });
                }
            },
        }
    }

    fn range_len(r: TagRange) -> usize {
        r.1 - r.0 + 1
    }

    let sum: usize = accepted_part_ranges
        .iter()
        .map(|pr| range_len(pr.x) * range_len(pr.m) * range_len(pr.a) * range_len(pr.s))
        .sum();

    println!("{}", sum);
}
