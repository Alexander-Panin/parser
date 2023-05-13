use std::rc::Rc;
use crate::atoms::{Node};

pub fn tree_length(op: Option<Rc<Node>>) -> usize {
    if op.is_none() { return 0; } 
    let n = op.unwrap();
    let a = tree_length(n.ok.as_ref().map(|x| x.clone()));
    let b = tree_length(n.err.as_ref().map(|x| x.clone()));
    return std::cmp::max(a,b) + 1;
}