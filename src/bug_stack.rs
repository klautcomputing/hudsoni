use crate::bug::Bug;
use crate::color::Color;

#[derive(Debug)]
pub struct BugStack {
    pub bugs: [Bug; 7],
    pub size: u8,
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
