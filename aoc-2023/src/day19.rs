use std::collections::HashMap;
use crate::day19::Op::*;

pub fn run(input: String) -> (usize, usize) {
    let split: Vec<&str> = input.split("\n\n").collect();

    let xmas_list: Vec<Xmas> = split[1].lines().map(|l| {
        let nums: Vec<usize> = l[1..l.len() - 1].split(",")
            .map(|s| {
                let (_, num) = s.split_once("=").unwrap();
                let num = num.parse::<usize>().unwrap();
                num
            }).collect();
        Xmas { x: nums[0], m: nums[1], a: nums[2], s: nums[3] }
    }).collect();

    let mut work_flows: HashMap<&str, WorkFlow> = HashMap::new();

    split[0].lines().map(|l| {
        let (name, s) = l[..l.len() - 1].split_once("{").unwrap();
        let instruction: Vec<Instruction> = s.split(",")
            .map(|s| {
                match s {
                    "A" => Instruction { letter: "x", op: Gt, val: 0, result: "A" },
                    "R" => Instruction { letter: "x", op: Gt, val: 0, result: "R" },
                    _ if s.contains("<") => { parse_instruction(s, "<") }
                    _ if s.contains(">") => { parse_instruction(s, ">") }
                    _ => Instruction { letter: "x", op: Gt, val: 0, result: s },
                }
            })
            .collect();
        (name, WorkFlow { instructions: instruction })
    }).for_each(|(name, instruction)| { work_flows.insert(name, instruction); });


    let mut ans_1 = 0;
    for xmas in xmas_list {
        let mut wf: &WorkFlow = work_flows.get("in").unwrap();
        let mut i = 0;
        let mut done = false;
        while !done {
            let inst: &Instruction = &wf.instructions[i];
            i += 1;
            let value = match inst.letter {
                "x" => { xmas.x }
                "m" => { xmas.m }
                "a" => { xmas.a }
                "s" => { xmas.s }
                _ => { panic!("Invalid instruction!") }
            };

            let passed = match inst.op {
                Gt => value > inst.val,
                Lt => value < inst.val,
            };

            if passed {
                match inst.result {
                    "A" => {
                        ans_1 += xmas.x + xmas.m + xmas.a + xmas.s;
                        done = true
                    }
                    "R" => { done = true }
                    _ => {
                        wf = work_flows.get(inst.result).unwrap();
                        i = 0;
                    }
                }
            }
        }
    }

    let mut ans_2 = 0;
    let mut work_queue: Vec<(&str, usize, ReXmas)> = Vec::new();
    work_queue.push(("in", 0, ReXmas { x: Range { start: 1, end: 4000 }, m: Range { start: 1, end: 4000 }, a: Range { start: 1, end: 4000 }, s: Range { start: 1, end: 4000 } }));
    while let Some((wf_name, wf_idx, rexmas)) = work_queue.pop() {
        let wf: &WorkFlow = work_flows.get(wf_name).unwrap();

        let inst: &Instruction = &wf.instructions[wf_idx];
        let range = match inst.letter {
            "x" => { rexmas.x }
            "m" => { rexmas.m }
            "a" => { rexmas.a }
            "s" => { rexmas.s }
            _ => { panic!("Invalid instruction!") }
        };

        let mut maybe_split_ranges: Vec<Range> = Vec::new();

        if range.start < inst.val && inst.val < range.end {
            if inst.op == Gt {
                maybe_split_ranges.push(Range { start: range.start, end: inst.val });
                maybe_split_ranges.push(Range { start: inst.val + 1, end: range.end });
            } else {
                maybe_split_ranges.push(Range { start: range.start, end: inst.val - 1 });
                maybe_split_ranges.push(Range { start: inst.val, end: range.end });
            }
        } else {
            maybe_split_ranges.push(range);
        }

        for range in maybe_split_ranges {

            let mut new_rexmas = rexmas.clone();
            match inst.letter {
                "x" => { new_rexmas.x = range }
                "m" => { new_rexmas.m = range }
                "a" => { new_rexmas.a = range }
                "s" => { new_rexmas.s = range }
                _ => { panic!("Invalid instruction!") }
            }

            let passed = match inst.op {
                Gt => range.start > inst.val,
                Lt => range.end < inst.val,
            };
            if passed {
                match inst.result {
                    "A" => {
                        let x_range = new_rexmas.x.end - new_rexmas.x.start + 1;
                        let m_range = new_rexmas.m.end - new_rexmas.m.start + 1;
                        let a_range = new_rexmas.a.end - new_rexmas.a.start + 1;
                        let s_range = new_rexmas.s.end - new_rexmas.s.start + 1;

                        ans_2 += x_range * m_range * a_range * s_range;
                    }
                    "R" => {  }
                    _ => {
                        let item = (inst.result, 0, new_rexmas);
                        work_queue.push(item);
                    }
                }
            } else {
                let item = (wf_name, wf_idx + 1, new_rexmas);
                work_queue.push(item);
            }
        }
    }
    (ans_1, ans_2)
}

#[derive(PartialEq)]
enum Op {
    Gt,
    Lt,
}

struct Instruction<'a> {
    letter: &'a str,
    op: Op,
    val: usize,
    result: &'a str,
}

struct WorkFlow<'a> {
    instructions: Vec<Instruction<'a>>,
}

struct Xmas {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

#[derive(Clone,Copy)]
struct Range {
    start: usize,
    end: usize,
}
#[derive(Clone,Copy)]
struct ReXmas {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

fn parse_instruction<'a>(s: &'a str, comparison: &'a str) -> Instruction<'a> {
    let (letter, x) = s.split_once(comparison).unwrap();
    let (val, result) = x.split_once(":").unwrap();
    let val: usize = val.parse::<usize>().unwrap();
    let op = match comparison {
        "<" => Lt,
        ">" => Gt,
        _ => panic!("Invalid instruction: {}", s),
    };
    Instruction { letter, op, val, result }
}