use std::collections::HashMap;

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
    value: i64,
    verdict: Verdict,
}

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Compare(ComparisonInstruction),
    Judge(Verdict),
}

#[derive(Debug, PartialEq, Eq)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

impl Part {
    fn get_tagged(&self, tag: ValueTag) -> i64 {
        match tag {
            ValueTag::X => self.x,
            ValueTag::M => self.m,
            ValueTag::A => self.a,
            ValueTag::S => self.s,
        }
    }

    fn sum(&self) -> i64 {
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
                            Operator::Gt => val > c.value,
                            Operator::Lt => val < c.value,
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

    let sum: i64 = accepted_parts.map(|p| p.sum()).sum();
    println!("{}", sum);
}

pub fn part2() {
    let lines = file_lines("inp19_1.txt");
}
