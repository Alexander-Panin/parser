use super::Choice;

pub fn tree_length(word: &Choice) -> usize {
    let Choice::Word(_, ref ok, ref err) = word else { return 0; };
    let a = tree_length(ok);
    let b = tree_length(err);
    std::cmp::max(a, b) + 1
}
