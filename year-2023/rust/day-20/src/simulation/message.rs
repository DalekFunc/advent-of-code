use crate::simulation::Signal;

#[derive(Debug, Clone)]
pub struct Message {
    pub sender: String,
    pub signal: Signal,
    pub receiver: String,
}
