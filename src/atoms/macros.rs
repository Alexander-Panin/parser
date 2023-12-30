macro_rules! tree {
    ($(| $($x:ident),+ $(,)?)+) => {
        {
            let mut tt = TokenTree::new();
            $(
                let v = vec![$(Token::$x,)*];
                tt.add_right(v[0]);
                for x in v.into_iter().skip(1) {
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
