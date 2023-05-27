macro_rules! tree {
    ($(| $($x:ident),+ $(,)?)+) => {
        {
            let v = [ $( _path_ok!([$($x)*]), )* ];
            v.into_iter().rev().fold(None, |acc, path| {
                let val = path.as_ref().unwrap().val; 
                let ok = path.unwrap().ok.as_ref().cloned(); 
                Some(Rc::new(
                    Node{ val, ok, err: acc }
                ))
            }).unwrap()
        }
    }
}

macro_rules! _path_ok {
    ([$first:ident $($rest:ident)*] $($acc:ident)*) => {
        _path_ok!([$($rest)*] $first $($acc)*)
    };
    ([] $($x:ident)*) => {
        {
            let mut node = None;
            $(
                let x = Some(Rc::new(
                    Node{
                        val: Token::$x,
                        ok: node,
                        err: Some(Rc::new(
                            Node{
                                val: Token::Never,
                                ok: None,
                                err: None,
                            }
                        )), 
                    }
                ));
                node = x;
            )*
            node
        }    
    }    
}
