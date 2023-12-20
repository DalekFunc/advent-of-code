pub mod message;
pub mod module;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Signal(bool);

impl Signal {
    pub const HIGH: Self = Self(true);
    pub const LOW: Self = Self(false);

    pub fn opposite(&self) -> Self {
        Self(!self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FlipFlopState(bool);

impl FlipFlopState {
    pub const ON: Self = Self(true);
    pub const OFF: Self = Self(false);
}
