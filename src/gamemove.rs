pub static STACKS: (i32, i32) = (0, 110);
pub static UTILITIES: (i32, i32) = (0, 0);
pub static BUTTONS: (i32, i32) = (225, 0);
pub static ROSE: (i32, i32) = (300, 0);
pub static ORDERED: (i32, i32) = (375, 0);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StackPosition {
    pub stack: Stack,
    pub y: u32, 
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Ordered {
    Ordered0 = 0,
    Ordered1 = 1,
    Ordered2 = 2,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Utility {
    Utility0 = 0,
    Utility1 = 1,
    Utility2 = 2,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Attempt(pub Move);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Valid(pub Move);

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Move {
    pub src: Option<ClickTarget>,
    pub dst: Option<ClickTarget>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ClickTarget {
    Stack(StackPosition),
    Utility(Utility),
    Ordered(Ordered),
    GreenButton,
    RedButton,
    BlackButton,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Stack {
    Stack0 = 0,
    Stack1 = 1,
    Stack2 = 2,
    Stack3 = 3,
    Stack4 = 4,
    Stack5 = 5,
    Stack6 = 6,
    Stack7 = 7,
}

impl Stack {
    pub fn from_col(col: usize) -> Stack {
        use self::Stack::*;
        match col {
            0 => Stack0,
            1 => Stack1,
            2 => Stack2,
            3 => Stack3,
            4 => Stack4,
            5 => Stack5,
            6 => Stack6,
            7 => Stack7,
            _ => panic!("Attempted to convert a usize to a stack that doesn't exist."),
        }
    }
}

impl ClickTarget {
    pub fn from_coord(x: i32, y: i32) -> Option<ClickTarget> {
        use self::ClickTarget::*;
        use self::Utility::*;
        use self::Ordered::*;
        use self::{Stack, StackPosition};
        Some(match (x, y) {
            (0...70, 0...102) => Utility(Utility0),
            (75...145, 0...102) => Utility(Utility1),
            (150...220, 0...102) => Utility(Utility2),
            (375...445, 0...102) => Ordered(Ordered0),
            (450...520, 0...102) => Ordered(Ordered1),
            (525...595, 0...102) => Ordered(Ordered2),
            (245...275, 0...30) => RedButton,
            (245...275, 35...65) => GreenButton,
            (245...275, 70...100) => BlackButton,
            (0...600, y) => {
                Stack(StackPosition{
                    stack: Stack::from_col((x as usize) / 75),
                    y: y as u32,
                })
            },
            (_, _) => return None,
        })
    }
}

