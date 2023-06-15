macro_rules! tree {
    ($(| $($x:ident),+ $(,)?)+) => {
        {
            let v = [ $( _path_ok!([$($x)*]), )* ];
            v.into_iter().rev().fold(Rc::new(Choice::Nil), |acc, path| {
                match *path {
                    Choice::Word(val, ref ok, _) =>
                        Rc::new(Choice::Word(val, ok.clone(), acc)),
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
            let mut word = Rc::new(Choice::Nil);
            $(
                let x = Rc::new(
                    Choice::Word(
                        Token::$x,
                        word,
                        Rc::new(
                            Choice::Word(
                                Token::Never,
                                Rc::new(Choice::Nil),
                                Rc::new(Choice::Nil)
                            )
                        )
                    ),
                );
                word = x;
            )*
            word
        }
    }
}
