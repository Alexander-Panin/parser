use std::fs;
use std::path::Display;

mod atoms;
mod double_entry;
mod registry;

use atoms::{token_tree, tokens, Token};
use double_entry::Audit;

fn audit(matcher: Vec<Token>, filename: Display) {
    // if format!("{:?}", filename) != "\"./src/js/basic.js\"" {
    //     return;
    // }
    let mut state = Audit {
        matcher,
        tt: token_tree(),
        ..Default::default()
    };
    let word = state.tt.get(&Token::Statement).unwrap();
    state.double_entry(word.clone());
    state.audit();
    println!(
        "{} {:?} {:?} [matcher size {}]",
        if state.matcher.is_empty() {
            "ok"
        } else {
            "NOT OK"
        },
        filename,
        state.registry,
        state.matcher.len()
    );
}

fn main() {
    let files = fs::read_dir("./src/js/").unwrap();
    for file in files {
        let str = fs::read_to_string(file.as_ref().unwrap().path()).expect("Unable to read file");
        let data = str.bytes().peekable();
        let v = tokens(data).into_iter().rev().collect();
        audit(v, file.unwrap().path().display());
    }
}
