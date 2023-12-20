use std::{
    collections::{HashMap, VecDeque},
    iter,
};

use anyhow::{anyhow, Result};
use itertools::Itertools;
use parser::parse_file;
use simulation::{module::Module, Signal};

use crate::simulation::message::Message;

mod parser;
mod simulation;

pub fn part1(input: &str) -> Result<u64> {
    // parse file
    let (_, modules) = parse_file(input).expect("parse ok");

    let (boardcaster, mut modules) = {
        let mut broadcaster = None;
        let mut map = HashMap::new();

        for module in modules {
            match module {
                b @ Module::Broadcaster { .. } => broadcaster = Some(b),
                f @ Module::FlipFlop { .. } => {
                    map.insert(f.name().to_string(), f);
                }
                c @ Module::Conjunction { .. } => {
                    map.insert(c.name().to_string(), c);
                }
            }
        }
        let broadcaster = broadcaster.expect("broadcaster");

        // setup conjunction record
        let sender_receiver = map
            .iter()
            .flat_map(|(name, module)| {
                module
                    .receivers()
                    .iter()
                    .map(|r| (name.clone(), r.to_string()))
                    .collect_vec()
            })
            .chain(
                broadcaster
                    .receivers()
                    .iter()
                    .map(|r| ("broadcaster".to_string(), r.to_string()))
                    .collect_vec(),
            )
            .collect_vec();

        for (sender, receiver) in sender_receiver {
            match map.get_mut(&receiver) {
                Some(Module::Conjunction { record, .. }) => {
                    record.entry(sender.to_string()).or_insert(Signal::LOW);
                }
                _ => {}
            }
        }

        (broadcaster, map)
    };

    // dbg!(&boardcaster);
    // dbg!(&modules);

    // construct simulate
    // record initial states of the setup
    // OnOff State of FlipFlop and Conjunction record
    // save it as a (Vec<(name, ONoFF)>, Vec<name, record>)
    let mut low_count = 0;
    let mut high_count = 0;

    // for _ in 0..3 {
    for _ in 0..1_000 {
        // button low
        low_count += 1;
        dbg!("button low");

        let setup = {
            let mut states = Vec::new();
            let mut records = Vec::new();

            for (name, module) in &modules {
                match module {
                    Module::Broadcaster { name, receivers } => unreachable!(),
                    Module::FlipFlop {
                        name,
                        state,
                        receivers,
                    } => {
                        states.push((name, state));
                    }
                    Module::Conjunction {
                        name,
                        record,
                        receivers,
                    } => {
                        records.push((name, record));
                    }
                }
            }

            (states, records)
        };

        // if we found identical state in history, dont simulate

        // vecdeque init = Message { sender: "boardcast", signal: LOW, receiver: boardcast.receiver }
        let mut queue = VecDeque::new();
        for receiver in boardcaster.receivers() {
            queue.push_back(Message {
                sender: "broadcaster".to_string(),
                signal: Signal::LOW,
                receiver: receiver.to_string(),
            })
        }

        // while message_queue is not empty
        //   process message
        //   trace High and LOW total
        // end while
        while let Some(msg) = queue.pop_front() {
            dbg!(&msg);

            if msg.signal == Signal::HIGH {
                high_count += 1;
            } else {
                low_count += 1;
            }

            match modules.get_mut(&msg.receiver) {
                Some(module) => {
                    let new_messages = module.process(msg);

                    for msg in new_messages {
                        queue.push_back(msg);
                    }

                    // queue.extend(new_messages);
                }
                None => {}
            }
        }

        dbg!(low_count);
        dbg!(high_count);
    }

    // find cycle
    // simulate until we find a repeated end setup
    // CyclicSimulation
    // Order, State, Fn Simulate, Out
    // History: HashMap (State -> (Order, Out))

    // calculate total

    Ok( low_count * high_count)
}

pub fn part2(input: &str) -> Result<u64> {
    Err(anyhow!("Not Implemented."))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn quick_test() {}
}
