mod bug;
mod bug_stack;
mod color;

use crate::bug::{Bug, Kind};
use crate::bug_stack::BugStack;
use crate::color::Color;

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
