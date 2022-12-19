use std::sync::Arc;
use crate::base_ops::compile;

#[cfg(test)]

use crate::parser::parse_simlang;
use crate::parser::SimLangToken::Symbol;
use crate::semantic_analyzer::{get_symbol, put_symbol};
use crate::semantic_analyzer::Variable::Lambda;

#[test]
fn test_token_hash_and_equality_and_stuff_lol() {
    let f1 = String::from("f");
    let args1 = parse_simlang("(1 2 3 4)").unwrap().drain(1..=1).next().unwrap();
    let result1 = Arc::new(Symbol("0"));
    let v1 = Lambda {
        domain: vec![args1],
        range: vec![result1],
    };

    let f2 = "f";
    let args2 = parse_simlang("(1 2 3 4)").unwrap().drain(1..=1).next().unwrap();
    let result2 = Arc::new(Symbol("0"));
    let v2 = Lambda {
        domain: vec![args2],
        range: vec![result2],
    };

    put_symbol(f1, v1);
    assert_eq!(get_symbol(f2).unwrap(), &v2);
}

#[test]
fn test_symbol_table() {
    let f = String::from("f");
    let input = Symbol("1");
    let output = Symbol("a");
    let v = Lambda {
        domain: vec![Arc::new(input)],
        range: vec![Arc::new(output)],
    };

    put_symbol(f, v);

    let code = parse_simlang("(f 1)").unwrap()[0].clone();
    let compiled = compile(code);

    let to_test = Symbol("a");
    assert_eq!(*compiled, to_test)
}

#[test]
fn test_compose() {
    let f1_domain = vec![
        Arc::new(Symbol("1")),
        Arc::new(Symbol("2")),
        Arc::new(Symbol("3")),
        Arc::new(Symbol("4")),
        Arc::new(Symbol("5")),
        Arc::new(Symbol("6"))
    ];
    let f1_range = vec![
        Arc::new(Symbol("a")),
        Arc::new(Symbol("b")),
        Arc::new(Symbol("c")),
        Arc::new(Symbol("d")),
        Arc::new(Symbol("e")),
        Arc::new(Symbol("f"))
    ];

    let f2_domain = vec![
        Arc::new(Symbol("a")),
        Arc::new(Symbol("b")),
        Arc::new(Symbol("c")),
        Arc::new(Symbol("d")),
        Arc::new(Symbol("e")),
        Arc::new(Symbol("f")),
        Arc::new(Symbol("g")),
        Arc::new(Symbol("h")),
        Arc::new(Symbol("i")),
        Arc::new(Symbol("j")),
        Arc::new(Symbol("k")),
        Arc::new(Symbol("l")),
        Arc::new(Symbol("m")),
    ];

    let f2_range = vec![
        Arc::new(Symbol("A")),
        Arc::new(Symbol("B")),
        Arc::new(Symbol("C")),
        Arc::new(Symbol("D")),
        Arc::new(Symbol("E")),
        Arc::new(Symbol("F")),
        Arc::new(Symbol("G")),
        Arc::new(Symbol("H")),
        Arc::new(Symbol("I")),
        Arc::new(Symbol("J")),
        Arc::new(Symbol("K")),
        Arc::new(Symbol("L")),
        Arc::new(Symbol("M")),
    ];

    let lambda1 = Lambda {
        domain: f1_domain.clone(),
        range: f1_range,
    };

    let lambda2 = Lambda {
        domain: f2_domain,
        range: f2_range.clone(),
    };

    let Lambda { domain: composed_domain, range: composed_range} = (&lambda2).compose(&lambda1)
        else { unreachable!() };

    assert_eq!(composed_domain, f1_domain);
    assert_eq!(composed_range, f2_range[0..=5])
}