mod atoms;
mod registry;
mod double_entry;

use double_entry::{Audit};
use atoms::{token_tree, tokens};

fn main() {
    // let mut state = Audit::default();
    // state.tt = token_tree();

    let mut state = Audit {
        tt: token_tree(),
        ..Default::default()
    };


    // let mut ts = tokens("(1+2)+(3+5)*2+5");
    // let mut ts = tokens("x = (((1+2)+(3+5))*2)+5".bytes().peekable());
    // let mut ts = tokens("(((1+2)+(3+5))*2+5"); // invalid
    // let mut ts = tokens("const abZc123 = -11 + 33 * 25 - 5".bytes().peekable());
    let mut ts = tokens("
        if (1) { 
            while(-a+2) { 
                const abZc123 = -11 + 33 * 25 - 5
                var a = (((1+2)+(3+5))*2)+5
                bba = (1+2)+(3+5)*2+5;
                bba = 5;
                bba = 5
                var fn = function() { x = 5 };
            }
        }
        const x = 5;
        const x = 5;
    ".bytes().peekable());
    ts.reverse();
    state.matcher = ts;
    state.double_entry(state.tt.statement.clone());
    state.audit();
    println!("done {:?}", state.registry);
}










