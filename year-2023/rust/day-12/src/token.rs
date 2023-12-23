#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Token {
    Empty,
    Block,
    Uncertain,
}

impl Token {
    pub fn is_empty(&self) -> bool {
        *self == Self::Empty
    }

    pub fn is_block(&self) -> bool {
        *self == Self::Block
    }

    pub fn is_uncertain(&self) -> bool {
        *self == Self::Uncertain
    }
}

pub fn print_tokens(tokens: &[Token]) -> String {
    tokens
        .iter()
        .map(|t| match t {
            Token::Empty => '.',
            Token::Block => '#',
            Token::Uncertain => '?',
        })
        .collect()
}
