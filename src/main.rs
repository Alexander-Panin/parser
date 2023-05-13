mod atoms;
mod registry;
mod double_entry;

use double_entry::{Audit};
use atoms::{token_tree, tokens};

fn main() {
    let mut state = Audit::default();
    state.tt = token_tree();
    // let mut ts = tokens("(1+2)+(3+5)*2+5");
    // let mut ts = tokens("x = (((1+2)+(3+5))*2)+5".bytes().peekable());
    // let mut ts = tokens("(((1+2)+(3+5))*2+5"); // invalid
    // let mut ts = tokens("const abZc123 = -11 + 33 * 25 - 5".bytes().peekable());
    let mut ts = tokens("
        if (1) { 
            while(-a+2) { 
                var a = 5
            }
        }
    ".bytes().peekable());
    ts.reverse();
    state.matcher = ts;
    state.double_entry_statement();
    state.audit();
    println!("done {:?}", state.registry);
}










