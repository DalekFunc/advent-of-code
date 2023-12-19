#[derive(Debug, Clone, Copy)]
pub enum Comparison{
    LessThan,
    GreaterThan,
}

#[derive(Debug, Clone, Copy)]
pub enum Category {
    X, M, A, S
}

impl From<char> for Category {
    fn from(value: char) -> Self {
        match value {
            'x' => Self::X,
            'm' => Self::M,
            'a' => Self::A,
            's' => Self::S,
            _ => panic!("invalid char"),
        }
    }
}

#[derive(Debug)]
pub struct Rule<'a> {
    pub cat: Category,
    pub cmp: Comparison,
    pub threshold: u32,
    pub target: &'a str,
}

impl<'a> Rule<'a> {
    pub fn satisfy(&self, part: Part) -> Option<&'a str>{
        match self.cmp {
            Comparison::LessThan => {
                match self.cat {
                    Category::X => (part.x<self.threshold).then(|| self.target),
                    Category::M => (part.m<self.threshold).then(|| self.target),
                    Category::A => (part.a<self.threshold).then(|| self.target),
                    Category::S => (part.s<self.threshold).then(|| self.target),
                }
            },
            Comparison::GreaterThan => {
                match self.cat {
                    Category::X => (part.x>self.threshold).then(|| self.target),
                    Category::M => (part.m>self.threshold).then(|| self.target),
                    Category::A => (part.a>self.threshold).then(|| self.target),
                    Category::S => (part.s>self.threshold).then(|| self.target),
                }
            },
        }
    }

}

#[derive(Debug)]
pub struct Workflow<'a> {
    pub name: &'a str,
    pub rules: Vec<Rule<'a>>,
    pub catch_all: &'a str,
}

#[derive(Debug, Clone, Copy)]
pub struct Part {
    pub x: u32,
    pub m: u32,
    pub a: u32,
    pub s: u32,
}

impl Part {
    pub fn rating(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}