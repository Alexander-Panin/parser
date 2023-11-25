use rayon::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

mod atoms;
mod double_entry;
mod registry;

use atoms::{token_tree, tokens, Token, TokenTree};
use double_entry::Audit;

fn audit(matcher: Vec<Token>, tt: &HashMap<Token, TokenTree>, filename: String) {
    // if format!("{:?}", filename) != "\"./src/js/basic.js\"" {
    //     return;
    // }
    let mut state = Audit::new(matcher, tt);
    let t = state.tt.get(&Token::Statement).unwrap();
    state.double_entry(t.cursor(), t.len);
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
        state.matcher.len(),
    );
    // println!("{:#?}", state.stats);
    if !state.matcher.is_empty() {
        println!(
            "{:?}",
            &state.matcher.iter().rev().collect::<Vec<_>>()[0..5]
        );
    }
}

fn par_run(input: Vec<PathBuf>) {
    let tt = token_tree();
    input
        .par_iter()
        .map(|path| {
            let filename = String::from(path.to_str().unwrap());
            let str = fs::read_to_string(path).expect("Unable to read file");
            let data = str.bytes().peekable();
            let v = tokens(data).into_iter().rev().collect();
            audit(v, &tt, filename);
        })
        .collect()
}

fn collect(path: PathBuf) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut files = vec![];
    let mut q = vec![path];
    while let Some(x) = q.pop() {
        if x.is_dir() {
            let files = fs::read_dir(x)?;
            q.extend(files.map(|f| f.unwrap().path()));
        } else {
            files.push(x);
        }
    }
    Ok(files)
}

fn main() -> Result<(), std::io::Error> {
    let files = collect(PathBuf::from("./src/js/"))?;
    par_run(files);
    Ok(())
}
