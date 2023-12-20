use std::collections::HashMap;

use itertools::Itertools;

use crate::simulation::{message::Message, FlipFlopState, Signal};

#[derive(Debug, Clone)]
pub enum Module<'a> {
    Broadcaster {
        name: &'a str,
        receivers: Vec<&'a str>,
    },
    FlipFlop {
        name: &'a str,
        state: FlipFlopState,
        receivers: Vec<&'a str>,
    },
    Conjunction {
        name: &'a str,
        record: HashMap<String, Signal>,
        receivers: Vec<&'a str>,
    },
    // Output
}

impl<'a> Module<'a> {
    pub fn name(&self) -> &'a str {
        match self {
            Module::Broadcaster { name, receivers } => name,
            Module::FlipFlop {
                name,
                state,
                receivers,
            } => name,
            Module::Conjunction {
                name,
                record,
                receivers,
            } => name,
        }
    }

    pub fn receivers(&self) -> &Vec<&'a str> {
        match self {
            Module::Broadcaster { name, receivers } => receivers,
            Module::FlipFlop {
                name,
                state,
                receivers,
            } => receivers,
            Module::Conjunction {
                name,
                record,
                receivers,
            } => receivers,
        }
    }

    pub fn process(&mut self, message: Message) -> Vec<Message> {
        let messages = match self {
            Module::Broadcaster {
                ref name,
                ref receivers,
            } => Some((name, message.signal, receivers)),
            Module::FlipFlop {
                ref name,
                state,
                ref receivers,
            } => match message.signal {
                Signal::HIGH => None,
                Signal::LOW => match *state {
                    FlipFlopState::OFF => {
                        *state = FlipFlopState::ON;
                        Some((name, Signal::HIGH, receivers))
                    }
                    FlipFlopState::ON => {
                        *state = FlipFlopState::OFF;
                        Some((name, Signal::LOW, receivers))
                    }
                },
            },
            Module::Conjunction {
                ref name,
                record,
                ref receivers,
            } => {
                record
                    .entry(message.sender)
                    .and_modify(|signal| *signal = message.signal);

                if record.iter().all(|(_, &signal)| signal == Signal::HIGH) {
                    Some((name, Signal::LOW, receivers))
                } else {
                    Some((name, Signal::HIGH, receivers))
                }
            }
        };

        match messages {
            Some((sender, signal, receivers)) => receivers
                .iter()
                .map(|receiver| Message {
                    sender: sender.to_string(),
                    signal,
                    receiver: receiver.to_string(),
                })
                .collect(),
            None => vec![],
        }
    }
}
