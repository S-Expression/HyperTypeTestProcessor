use std::rc::Rc;
use crate::parser::SimLangToken;
use crate::parser::SimLangToken::{List, Symbol};

fn quote(arg: Rc<SimLangToken>) -> Rc<SimLangToken> {
    arg
}

fn atom(args: Vec<Rc<SimLangToken>>) -> Rc<SimLangToken> {
    Rc::new(Symbol(match *compile(args[0].clone()) {
        List(_) => "f",
        Symbol(_) => "t"
    }))
}


fn eq(args: Vec<Rc<SimLangToken>>) -> Rc<SimLangToken> {
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

    Rc::new(Symbol(if test(&left, &right) { "t" } else { "f" }))
}

fn car(args: Vec<Rc<SimLangToken>>) -> Rc<SimLangToken> {
    compile(args[0].clone())
}

fn cdr(args: Vec<Rc<SimLangToken>>) -> Rc<SimLangToken> {
    let List(ref elements) = *(compile(args[0].clone())) else { unreachable!() };
    let inner = elements[1..].to_vec();
    return if inner.len() == 1 {
        inner[0].clone()
    } else {
        Rc::new(List(inner))
    }
}

fn cons(args: Vec<Rc<SimLangToken>>) -> Rc<SimLangToken> {
    let first = compile(args[0].clone());
    let second = compile(args[1].clone());

    let mut x = match *(first) {
        List(ref vector) => vector.clone(),
        Symbol(_) => vec![first]
    };
    let List(ref y) = *(second) else { unreachable!() };

    x.append(&mut y.clone());

    Rc::new(List(x))
}

fn cond(args: Vec<Rc<SimLangToken>>) -> Rc<SimLangToken> {
    for arg in args {
        let List(ref elements) = *compile(arg) else { unreachable!() };
        let Symbol(b) = *compile(elements[0].clone()) else { unreachable!() };
        if b == "t" {
            return elements[1].clone();
        }
    }
    panic!("Unable to find matching case!");
}

pub fn compile(token: Rc<SimLangToken>) -> Rc<SimLangToken> {
    let List(ref elements) = *token else { return token };

    let first = elements[0].clone();
    let Symbol(operator) = *first else { return token };

    let no_first = (elements[1..]).to_vec();

    match operator {
        "quote" => quote(no_first[0].clone()),
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
