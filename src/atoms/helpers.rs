use std::rc::Rc;
use crate::atoms::{Node};

pub fn tree_length(op: Option<Rc<Node>>) -> usize {
    if op.is_none() { return 0; } 
    let node = op.unwrap();
    let a = tree_length(node.ok.as_ref().cloned()); 
    let b = tree_length(node.err.as_ref().cloned()); 
    std::cmp::max(a,b) + 1
}