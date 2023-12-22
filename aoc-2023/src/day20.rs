use std::collections::HashMap;
use Module::{Broadcaster, FlipFlop};
use crate::day20::Module::Conjunction;
use crate::day20::Signal::{High, Low};
use crate::day20::State::{Off, On};

pub fn run(input: String) -> (usize, usize) {
    let mut flip_flop_states: HashMap<&ModuleId, State> = HashMap::new();
    let mut conjunction_states: HashMap<(&ModuleId, &ModuleId), Signal> = HashMap::new();

    let modules: HashMap<ModuleId, Module> = input.lines().map(|l| {
        let (source, target) = l.split_once(" -> ").unwrap();
        let target_ids: Vec<ModuleId> = target.split(", ")
            .map(|s| ModuleId(s.into()))
            .collect();
        let module_type = source.chars().nth(0).unwrap();
        let module_id = match module_type {
            'b' => ModuleId(source.parse().unwrap()),
            _ => ModuleId(source[1..].to_string())
        };

        let module: Module = match module_type {
            'b' => { Broadcaster(target_ids) }
            '%' => {
                FlipFlop(target_ids)
            }
            '&' => { Conjunction(target_ids) }
            _ => panic!("Invalid module type: {} in {}", module_type, source)
        };
        (module_id, module)
    }).collect();

    for m in modules.iter() {
        let (id, m) = m;
        let targets = match m {
            Broadcaster(t) => t,
            FlipFlop(t) => t,
            Conjunction(t) => t,
        };
        for target_id in targets {
            let target_module = modules.get(&target_id);
            match target_module {
                Some(Conjunction(_)) => {
                    conjunction_states.insert((id, target_id), Low);
                }
                _ => {}
            }
        }
    }

    // println!("Modules: {:?}", modules);

    let mut sum_low = 0;
    let mut sum_high = 0;
    let button_id = ModuleId("button".into());
    let broadcaster_id = ModuleId("broadcaster".into());

    // for i in 0..1000 {
    //     let mut work_queue: Vec<(Signal, &ModuleId, &ModuleId)> = Vec::new();
    //     work_queue.push((Low, &button_id, &broadcaster_id));
    //     while !work_queue.is_empty() {
    //         let (signal, source_id, target_id) = work_queue.remove(0);
    //         // println!("{:?} -{:?}-> {:?} : {} * {}", source_id.0, signal, target_id.0, sum_low, sum_high);
    //         match signal {
    //             High => { sum_high += 1 }
    //             Low => { sum_low += 1 }
    //         }
    //
    //         if target_id.0 == "rx" {
    //             println!("HEJ!")
    //         }
    //
    //         let target_module: Option<&Module> = modules.get(target_id);
    //         match target_module {
    //             Some(Broadcaster(t)) => { t.iter().for_each(|next_target_id| { work_queue.push((Low, target_id, next_target_id)) }) }
    //             Some(FlipFlop(t)) => {
    //                 let current_state: &State = flip_flop_states.get(target_id).unwrap_or(&Off);
    //                 if signal == Low {
    //                     let (next_state, next_signal) = match current_state {
    //                         On => (Off, Low),
    //                         Off => (On, High),
    //                     };
    //                     flip_flop_states.insert(target_id, next_state);
    //                     t.iter().for_each(|next_target_id| { work_queue.push((next_signal, target_id, next_target_id)) })
    //                 }
    //             }
    //             Some(Conjunction(t)) => {
    //                 conjunction_states.insert((source_id, target_id), signal);
    //                 let all_high_in_mem = conjunction_states.iter()
    //                     .filter(|((_, id), _)| *id == target_id)
    //                     .all(|((_, _), signal)| signal == &High);
    //                 let next_signal = if all_high_in_mem {
    //                     Low
    //                 } else {
    //                     High
    //                 };
    //                 t.iter().for_each(|next_target_id| { work_queue.push((next_signal, target_id, next_target_id)) })
    //             }
    //             _ => {}
    //         };
    //     }
    // }

    let ans_1 = sum_low * sum_high;

    let mut button_presses = 0;
    let mut rx_low_signals = 0;
    let mut _rx_high_signals = 0;
    while !(rx_low_signals >= 1) {
        rx_low_signals = 0;
        _rx_high_signals = 0;
        let mut work_queue: Vec<(Signal, &ModuleId, &ModuleId)> = Vec::new();
        work_queue.push((Low, &button_id, &broadcaster_id));
        button_presses += 1;
        if button_presses % 1_000_000 == 0 {
            println!("{:?}", button_presses);
        }
        while !work_queue.is_empty() {
            let (signal, source_id, target_id) = work_queue.remove(0);
            // println!("{:?} -{:?}-> {:?}", source_id.0, signal, target_id.0);
            match signal {
                High => {
                    if target_id.0 == "output" {
                        _rx_high_signals += 1;
                    }
                    sum_high += 1
                }
                Low => {
                    if target_id.0 == "output" {
                        rx_low_signals += 1;
                    }
                    sum_low += 1
                }
            }



            let target_module: Option<&Module> = modules.get(target_id);
            match target_module {
                Some(Broadcaster(t)) => { t.iter().for_each(|next_target_id| { work_queue.push((Low, target_id, next_target_id)) }) }
                Some(FlipFlop(t)) => {
                    let current_state: &State = flip_flop_states.get(target_id).unwrap_or(&Off);
                    if signal == Low {
                        let (next_state, next_signal) = match current_state {
                            On => (Off, Low),
                            Off => (On, High),
                        };
                        flip_flop_states.insert(target_id, next_state);
                        t.iter().for_each(|next_target_id| { work_queue.push((next_signal, target_id, next_target_id)) })
                    }
                }
                Some(Conjunction(t)) => {
                    conjunction_states.insert((source_id, target_id), signal);
                    let all_high_in_mem = conjunction_states.iter()
                        .filter(|((_, id), _)| *id == target_id)
                        .all(|((_, _), signal)| signal == &High);
                    let next_signal = if all_high_in_mem {
                        Low
                    } else {
                        High
                    };
                    t.iter().for_each(|next_target_id| { work_queue.push((next_signal, target_id, next_target_id)) })
                }
                _ => {}
            };
        }
    }
    let ans_2 = button_presses;

    (ans_1, ans_2)
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct ModuleId(String);

#[derive(Debug, Eq, PartialEq)]
enum State {
    On,
    Off,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Signal {
    High,
    Low,
}

#[derive(Debug)]
enum Module {
    Broadcaster(Vec<ModuleId>),
    FlipFlop(Vec<ModuleId>),
    Conjunction(Vec<ModuleId>),
}