mod atoms;
mod registry;
mod double_entry;

use double_entry::{Audit};
use atoms::{token_tree, tokens, Token};

fn main() {
    // let mut state = Audit::default();
    // state.tt = token_tree();

    let mut state = Audit {
        tt: token_tree(),
        ..Default::default()
    };
    let mut ts = tokens("
        if (1) { 
            while(-a+2 && x+1 || a+2) { 
                const abZc123 = -11 + 33 * 25 - 5
                var a = (((1+2)+(3+5))*2)+5
                bba = (1+2)+(3+5)*2+5;
                bba = 5 + a(8,5+3*2);
                bba = 5
                var fn = function() { x = 5 };
            }
        }
        const x = 5;
        const x = 5;
    ".bytes().peekable());
    ts.reverse();
    state.matcher = ts;
    state.double_entry(state.tt.get(&Token::Statement).unwrap().clone());
    state.audit();
    println!("done {:?}", state.registry);
}










