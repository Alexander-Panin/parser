macro_rules! tree {
    ($(| $($x:ident),+ $(,)?)+) => {
        {
            let v = [ $( _path_ok!([$($x)*]), )* ];
            v.into_iter().rev().fold(Choice::Nil, |acc, path| {
                match path {
                    Choice::Word(val, ref ok, _) =>
                        Choice::Word(val, Arc::clone(ok), Arc::new(acc)),
                    _ => acc
                }
            })
        }
    }
}

macro_rules! _path_ok {
    ([$first:ident $($rest:ident)*] $($acc:ident)*) => {
        _path_ok!([$($rest)*] $first $($acc)*)
    };
    ([] $($x:ident)*) => {
        {
            let mut word = Choice::Nil;
            $(
                let x = Choice::Word(
                        Token::$x,
                        Arc::new(word),
                        Arc::new(
                            Choice::Word(
                                Token::Never,
                                Arc::new(Choice::Nil),
                                Arc::new(Choice::Nil)
                            )
                        )
                );
                word = x;
            )*
            word
        }
    }
}
