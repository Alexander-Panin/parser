use std::fs;
use std::path::PathBuf;
use rayon::prelude::*;

mod atoms;
mod double_entry;
mod registry;

use atoms::{token_tree, tokens, Token};
use double_entry::Audit;

fn audit(matcher: Vec<Token>, filename: String) {
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

fn par(input: Vec<PathBuf>) {
    input.par_iter().map(|path| {
        let filename = String::from(path.to_str().unwrap());
        let str = fs::read_to_string(path).expect("Unable to read file");
        let data = str.bytes().peekable();
        let v = tokens(data).into_iter().rev().collect();
        audit(v, filename);
    }).collect()
}

fn main() {
    let files = fs::read_dir("./src/js/").unwrap();
    par(files.map(|f| f.as_ref().unwrap().path()).collect());
}
