macro_rules! tree {
    ($(| $($x:ident),+ $(,)?)+) => {
        {
            let v = [ $( _path_ok!([$($x)*]), )* ];
            v.into_iter().rev().fold(None, |acc, path| {
                match path {
                    Some(Word(val, ref ok, _)) =>
                        Some(Word(val, Arc::clone(ok), Arc::new(acc))),
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
            let mut word = None;
            $(
                let x = Some(Word(
                        Token::$x,
                        Arc::new(word),
                        Arc::new(
                            Some(Word(
                                Token::Never,
                                Arc::new(None),
                                Arc::new(None)
                            ))
                        )
                ));
                word = x;
            )*
            word
        }
    }
}
