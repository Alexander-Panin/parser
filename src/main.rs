use std::fs;
use std::path::PathBuf;

mod atoms;
mod registry;
mod double_entry;

use double_entry::{Audit};
use atoms::{token_tree, tokens, Token};

fn parse(file: PathBuf) {
    let mut state = Audit {
        tt: token_tree(),
        ..Default::default()
    };

    let str = fs::read_to_string(&file).expect("Unable to read file");
    let mut ts = tokens(str.bytes().peekable());
    ts.reverse();
    state.matcher = ts;
    state.double_entry(state.tt.get(&Token::Statement).unwrap().clone());
    state.audit();
    println!("{:?} done {:?}", file, state.registry);
}

fn main() {
    let files = fs::read_dir("./src/tests/").unwrap();
    for file in files {
        parse(file.unwrap().path());
    }
}










