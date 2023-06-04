use std::fs;
use std::path::Display;

mod atoms;
mod registry;
mod double_entry;

use double_entry::{Audit};
use atoms::{token_tree, tokens, Token};

fn audit(matcher: Vec<Token>, filename: Display) {
    if format!("{:?}", filename) != "\"./src/tests/import.js\"" { return; }
    let mut state = Audit {
        matcher,
        tt: token_tree(),
        ..Default::default()
    };
    state.double_entry(state.tt.get(&Token::Statement).unwrap().clone());
    state.audit();
    println!("{:?} done {:?} [matcher size {}]", 
        filename, state.registry, state.matcher.len());
}

fn main() {
    let files = fs::read_dir("./src/tests/").unwrap();
    for file in files {
        let str = fs::read_to_string(file.as_ref().unwrap().path())
            .expect("Unable to read file");
        let data = str.bytes().peekable();
        let v = tokens(data).into_iter().rev().collect();
        audit(v, file.unwrap().path().display());
    }
}










