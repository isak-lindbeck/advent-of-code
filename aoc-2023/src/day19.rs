use std::collections::HashMap;

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

    let mut work_flows: HashMap<&str, Vec<Box<dyn Fn(&Xmas) -> Option<String>>>> = HashMap::new();

    split[0].lines().map(|l| {
        let (name, s) = l[..l.len() - 1].split_once("{").unwrap();
        let instruction: Vec<Box<dyn Fn(&Xmas) -> Option<String>>> = s.split(",")
            .map(|s| {
                let instr: Box<dyn Fn(&Xmas) -> Option<String>> = match s {
                    "A" => Box::new(|_: &Xmas| -> Option<String> { Some("A".to_string()) }),
                    "R" => Box::new(|_: &Xmas| -> Option<String> { Some("R".to_string()) }),
                    _ if s.contains("<") => { parse_comparison(s, "<") }
                    _ if s.contains(">") => { parse_comparison(s, ">") }
                    _ => Box::new(|_: &Xmas| -> Option<String> { Some(s.to_string()) }),
                };
                instr
            })
            .collect();
        (name, instruction)
    }).for_each(|(name, instruction)| { work_flows.insert(name, instruction); });


    let mut ans_1 = 0;
    for xmas in xmas_list {
        let mut wf: &Vec<Box<dyn Fn(&Xmas) -> Option<String>>> = work_flows.get("in").unwrap();

        let mut option = None;
        let mut i = 0;
        while option.is_none() {
            option = wf[i](&xmas);
            i += 1;
            if let Some(ref x) = option {
                match x.as_str() {
                    "A" => { ans_1 += xmas.x + xmas.m + xmas.a + xmas.s; }
                    "R" => {  }
                    _ => {
                        wf = work_flows.get(x.as_str()).unwrap();
                        i = 0;
                        option = None;
                    }
                }
            }
        }
    }

    (ans_1, 0)
}

#[derive(Debug)]
struct Xmas {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

fn parse_comparison<'a>(s: &'a str, comparison: &'a str) -> Box<dyn Fn(&Xmas) -> Option<String> + 'a> {
    let (letter, x) = s.split_once(comparison).unwrap();
    let (num, result) = x.split_once(":").unwrap();
    let num: usize = num.parse::<usize>().unwrap();

    let resolve: Box<dyn Fn(usize) -> bool> = match comparison {
        "<" => Box::new(move |v: usize| -> bool { v < num }),
        ">" => Box::new(move |v: usize| -> bool { v > num }),
        _ => panic!("Invalid instruction: {}", s),
    };

    let get_value: Box<dyn Fn(&Xmas) -> usize> = match letter {
        "x" => Box::new(move |xmas: &Xmas| -> usize { xmas.x }),
        "m" => Box::new(move |xmas: &Xmas| -> usize { xmas.m }),
        "a" => Box::new(move |xmas: &Xmas| -> usize { xmas.a }),
        "s" => Box::new(move |xmas: &Xmas| -> usize { xmas.s }),
        _ => panic!("Invalid instruction: {}", s),
    };
    Box::new(move |xmas: &Xmas| -> Option<String> { if resolve(get_value(xmas)) { Some(result.to_string()) } else { None } })
}
