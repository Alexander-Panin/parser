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
        class Foo {}
        class Foo extends Boo {
            foo() { this.a = 5; } 
            boo(c,d) {}
        }
        function foo() {
            if (5 == 3) { 
                while(true) {}
            }
            while(-a+2 && x == 1 || a+2 === 3) { 
                const abZc123 = -11 + 33 * 25 - 5
                var a = (((1+2)+(3+5))*2)+5
                bba = (a.x+2)+(c+5)*2+5;
                bba = 5 + foo(8,5+3*2);
                bb.f = (a+5)*2;
                const x = await get();
                var p = new Point(1,2);
                var fn = function() { x = 5 };
                const fn = (a,b) => a+b;
                const fn = (a,b) => { x = a+b; }
                const fn = () => { x = a+b; }
                const fn = () => (b,c) => { x = a+b; }
                foo.a.x(5,6);
                foo.a().x(5,6);
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










