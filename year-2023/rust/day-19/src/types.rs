use crate::range::{PartRange, Range};

#[derive(Debug, Clone, Copy)]
pub enum Comparison {
    LessThan,
    GreaterThan,
}

#[derive(Debug, Clone, Copy)]
pub enum Category {
    X,
    M,
    A,
    S,
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
    pub fn satisfy(&self, part: Part) -> Option<&'a str> {
        match self.cmp {
            Comparison::LessThan => match self.cat {
                Category::X => (part.x < self.threshold).then(|| self.target),
                Category::M => (part.m < self.threshold).then(|| self.target),
                Category::A => (part.a < self.threshold).then(|| self.target),
                Category::S => (part.s < self.threshold).then(|| self.target),
            },
            Comparison::GreaterThan => match self.cat {
                Category::X => (part.x > self.threshold).then(|| self.target),
                Category::M => (part.m > self.threshold).then(|| self.target),
                Category::A => (part.a > self.threshold).then(|| self.target),
                Category::S => (part.s > self.threshold).then(|| self.target),
            },
        }
    }

    pub fn satisfy_range(&self, range: &PartRange) -> (Option<PartRange>, Option<PartRange>) {
        match self.cmp {
            Comparison::LessThan => match self.cat {
                Category::X => {
                    if range.x.end <= self.threshold {
                        return (Some(range.clone()), None);
                    } else if range.x.start >= self.threshold {
                        return (None, Some(range.clone()));
                    } else {
                        let satisfied = PartRange {
                            x: Range {
                                start: range.x.start,
                                end: self.threshold,
                            },
                            ..*range
                        };
                        let rest = PartRange {
                            x: Range {
                                start: self.threshold,
                                end: range.x.end,
                            },
                            ..*range
                        };

                        return (Some(satisfied), Some(rest));
                    }
                },
                Category::M => if range.m.end <= self.threshold {
                    return (Some(range.clone()), None);
                } else if range.m.start >= self.threshold {
                    return (None, Some(range.clone()));
                } else {
                    let satisfied = PartRange {
                        m: Range {
                            start: range.m.start,
                            end: self.threshold,
                        },
                        ..*range
                    };
                    let rest = PartRange {
                        m: Range {
                            start: self.threshold,
                            end: range.m.end,
                        },
                        ..*range
                    };

                    return (Some(satisfied), Some(rest));
                },
                Category::A => if range.a.end <= self.threshold {
                    return (Some(range.clone()), None);
                } else if range.a.start >= self.threshold {
                    return (None, Some(range.clone()));
                } else {
                    let satisfied = PartRange {
                        a: Range {
                            start: range.a.start,
                            end: self.threshold,
                        },
                        ..*range
                    };
                    let rest = PartRange {
                        a: Range {
                            start: self.threshold,
                            end: range.a.end,
                        },
                        ..*range
                    };

                    return (Some(satisfied), Some(rest));
                },
                Category::S => if range.s.end <= self.threshold {
                    return (Some(range.clone()), None);
                } else if range.s.start >= self.threshold {
                    return (None, Some(range.clone()));
                } else {
                    let satisfied = PartRange {
                        s: Range {
                            start: range.s.start,
                            end: self.threshold,
                        },
                        ..*range
                    };
                    let rest = PartRange {
                        s: Range {
                            start: self.threshold,
                            end: range.s.end,
                        },
                        ..*range
                    };

                    return (Some(satisfied), Some(rest));
                },
            },
            Comparison::GreaterThan => match self.cat {
                Category::X => {
                    if range.x.start >= self.threshold {
                        return (Some(range.clone()), None);
                    } else if range.x.end <= self.threshold {
                        return (None, Some(range.clone()));
                    } else {
                        let satisfied = PartRange {
                            x: Range {
                                start: self.threshold + 1,
                                end: range.x.end,
                            },
                            ..*range
                        };
                        let rest = PartRange {
                            x: Range {
                                start: range.x.start,
                                end: self.threshold + 1,
                            },
                            ..*range
                        };
                        return (Some(satisfied), Some(rest));
                    }
                },
                Category::M => {
                    if range.m.start >= self.threshold {
                        return (Some(range.clone()), None);
                    } else if range.m.end <= self.threshold {
                        return (None, Some(range.clone()));
                    } else {
                        let satisfied = PartRange {
                            m: Range {
                                start: self.threshold + 1,
                                end: range.m.end,
                            },
                            ..*range
                        };
                        let rest = PartRange {
                            m: Range {
                                start: range.m.start,
                                end: self.threshold + 1,
                            },
                            ..*range
                        };
                        return (Some(satisfied), Some(rest));
                    }
                },
                Category::A => {
                    if range.a.start >= self.threshold {
                        return (Some(range.clone()), None);
                    } else if range.a.end <= self.threshold {
                        return (None, Some(range.clone()));
                    } else {
                        let satisfied = PartRange {
                            a: Range {
                                start: self.threshold + 1,
                                end: range.a.end,
                            },
                            ..*range
                        };
                        let rest = PartRange {
                            a: Range {
                                start: range.a.start,
                                end: self.threshold + 1,
                            },
                            ..*range
                        };
                        return (Some(satisfied), Some(rest));
                    }
                },
                Category::S => {
                    if range.s.start >= self.threshold {
                        return (Some(range.clone()), None);
                    } else if range.s.end <= self.threshold {
                        return (None, Some(range.clone()));
                    } else {
                        let satisfied = PartRange {
                            s: Range {
                                start: self.threshold + 1,
                                end: range.s.end,
                            },
                            ..*range
                        };
                        let rest = PartRange {
                            s: Range {
                                start: range.s.start,
                                end: self.threshold + 1,
                            },
                            ..*range
                        };
                        return (Some(satisfied), Some(rest));
                    }
                },
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
