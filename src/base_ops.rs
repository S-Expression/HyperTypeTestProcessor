use crate::SimLangToken::List;
use crate::SimLangToken::Symbol;
use crate::SimLangToken;

fn quote(args: Vec<SimLangToken>) -> SimLangToken {
    compile(args[0].clone()).clone()
}

fn atom(args: Vec<SimLangToken>) -> SimLangToken {
    Symbol(match compile(args[0].clone()) {
        List(_) => "f",
        Symbol(_) => "t"
    })
}


fn eq(args: Vec<SimLangToken>) -> SimLangToken {
    let left = compile(args[0].clone());
    let right = compile(args[1].clone());

    fn test(left: &SimLangToken, right: &SimLangToken) -> bool {
        match left {
            List(ltokens) => {
                match right {
                    List(rtokens) => {
                        if ltokens.len() != rtokens.len() {
                            return false;
                        }

                        for i in 0..ltokens.len() {
                            if !test(&ltokens[i], &rtokens[i]) {
                                return false;
                            }
                        }
                        true
                    }
                    Symbol(_) => false,
                }
            }
            Symbol(s1) => {
                match right {
                    List(_) => false,
                    Symbol(s2) => *s1 == *s2,
                }
            }
        }
    }

    Symbol(if test(&left, &right) { "t" } else { "f" })
}

fn car(args: Vec<SimLangToken>) -> SimLangToken {
    compile(args[0].clone())
}

fn cdr(args: Vec<SimLangToken>) -> SimLangToken {
    let List(elements) = compile(args[0].clone()) else { unreachable!() };
    List(elements[1..].to_vec())
}

fn cons(args: Vec<SimLangToken>) -> SimLangToken {
    let first = compile(args[0].clone());
    let second = compile(args[1].clone());

    let mut x = match first {
        List(vector) => vector,
        Symbol(_) => vec![first]
    };
    let List(mut y) = second else { unreachable!() };

    x.append(&mut y);

    List(x)
}

fn cond(args: Vec<SimLangToken>) -> SimLangToken {
    for arg in args {
        let compiled = compile(arg);
        let List(elements) = compiled else { unreachable!() };
        let Symbol(b) = compile(elements[0].clone()) else { unreachable!() };
        if b == "t" {
            return elements[1].clone();
        }
    }
    panic!("Unable to find matching case!");
}

pub fn compile(token: SimLangToken) -> SimLangToken {
    let List(elements) = &token else { return token };

    let first = &elements[0];
    let Symbol(operator) = first else { return token };

    let no_first = (elements[1..]).to_vec();

    match *operator {
        "quote" => quote(no_first),
        "atom" => atom(no_first),
        "eq" => eq(no_first),
        "car" => car(no_first),
        "cdr" => cdr(no_first),
        "cons" => cons(no_first),
        "cond" => cond(no_first),
        _ => {
            token
        }
    }
}
