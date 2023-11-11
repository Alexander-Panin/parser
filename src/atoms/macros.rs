macro_rules! tree {
    ($(| $($x:ident),+ $(,)?)+) => {
        {
            let tt = TokenTree::new();
            $(
                let v = _reverse_vec!([$($x)*]);
                tt.add_right(v.pop().unwrap());
                for x in v.into_iter().rev() { 
                    tt.add_left(x, Token::Never); 
                }
            )*
            tt
        }
    }
}

macro_rules! _reverse_vec {
    ([$first:ident $($rest:ident)*] $($acc:ident)*) => {
        _reverse_vec!([$($rest)*] $first $($acc)*)
    };
    ([] $($x:ident)*) => {
        vec![$(Token::$x,)*]
    }
}
