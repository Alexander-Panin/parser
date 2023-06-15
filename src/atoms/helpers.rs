use super::Choice;
use std::rc::Rc;

pub fn tree_length(word: &Rc<Choice>) -> usize {
    let Choice::Word(_, ref ok, ref err) = *word.clone() else { return 0; };
    let a = tree_length(ok);
    let b = tree_length(err);
    std::cmp::max(a, b) + 1
}
