use super::Node;
use std::rc::Rc;

pub fn tree_length(node: Option<Rc<Node>>) -> usize {
    if node.is_none() {
        return 0;
    }
    let node = node.unwrap();
    let a = tree_length(node.ok.as_ref().cloned());
    let b = tree_length(node.err.as_ref().cloned());
    std::cmp::max(a, b) + 1
}
