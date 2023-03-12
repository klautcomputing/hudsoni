use bitfield_struct::bitfield;
use std::convert::From;

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
enum Color {
    White = 0,
    Black = 1,
}

impl From<u8> for Color {
    fn from(item: u8) -> Self {
        if item == 0 {
            return Color::White;
        }
        Color::Black
    }
}

impl From<Color> for u8 {
    fn from(item: Color) -> Self {
        if item == Color::White {
            return 0;
        }
        1
    }
}

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
enum Kind {
    Ant = 0,
    Beetle = 1,
    Grasshopper = 2,
    Ladybug = 3,
    Mosquito = 4,
    Pillbug = 5,
    Queen = 6,
    Spider = 7,
}

impl From<u8> for Kind {
    fn from(item: u8) -> Self {
        match item {
            0 => Kind::Ant,
            1 => Kind::Beetle,
            2 => Kind::Grasshopper,
            3 => Kind::Ladybug,
            4 => Kind::Mosquito,
            5 => Kind::Pillbug,
            6 => Kind::Queen,
            7 => Kind::Spider,
            _ => panic!(),
        }
    }
}

impl From<Kind> for u8 {
    fn from(item: Kind) -> Self {
        match item {
            Kind::Ant => 0,
            Kind::Beetle => 1,
            Kind::Grasshopper => 2,
            Kind::Ladybug => 3,
            Kind::Mosquito => 4,
            Kind::Pillbug => 5,
            Kind::Queen => 6,
            Kind::Spider => 7,
        }
    }
}

#[bitfield(u8)]
#[derive(PartialEq, Eq)]
struct Bug {
    /// The first field occupies the least significant bits
    #[bits(1)]
    color: Color,
    #[bits(3)]
    kind: Kind,
    #[bits(2)]
    number: usize,
    /// we need to fill the u8
    #[bits(2)]
    _padding: usize,
}

#[derive(Debug)]
struct BugStack {
    bugs: [Bug; 7],
    size: u8,
}

impl BugStack {
    pub fn new() -> Self {
        Self {
            bugs: [Bug::new(); 7],
            size: 0,
        }
    }

    pub fn empty(&self) -> bool {
        self.size == 0
    }

    pub fn top_bug_color(&self) -> Option<Color> {
        if self.empty() {
            return None;
        }
        Some(self.bugs[self.size as usize].color())
    }

    pub fn push_bug(&mut self, bug: Bug) {
        if self.size == 7 {
            panic!("Trying to add an 8th bug to a BugStack")
        }
        self.bugs[self.size as usize] = bug;
        self.size += 1;
    }

    pub fn pop_bug(&mut self) -> Bug {
        if self.size == 0 {
            panic!("Trying to remove a bug from an empty BugStack")
        }
        let bug = self.bugs[self.size as usize];
        self.bugs[self.size as usize] = Bug::new();
        self.size -= 1;
        bug
    }

    pub fn top_bug(&mut self) -> Option<Bug> {
        if self.size == 0 {
            return None;
        }
        Some(self.bugs[(self.size - 1) as usize])
    }
}

fn main() {
    static_assertions::assert_eq_size_val!(BugStack::new(), 0u64);

    let bug = Bug::new().with_color(Color::White).with_kind(Kind::Queen);
    let mut bug_stack = BugStack::new();
    println!("bug_stack: {:?}", bug_stack);
    println!("empty: {:?}", bug_stack.empty());
    bug_stack.push_bug(bug);
    assert_eq!(bug, bug_stack.top_bug().unwrap());
    println!("bug_stack: {:?}", bug_stack);
    println!("empty: {:?}", bug_stack.empty());
    println!("color: {:?}", bug_stack.top_bug_color().unwrap());
    bug_stack.pop_bug();
    println!("bug_stack: {:?}", bug_stack);
    println!("empty: {:?}", bug_stack.empty());
    println!("color: {:?}", bug_stack.top_bug_color());
}
