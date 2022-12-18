use std::sync::Arc;
use crate::base_ops::compile;

#[cfg(test)]

use crate::parser::parse_simlang;
use crate::parser::SimLangToken;
use crate::semantic_analyzer::{get_symbol, put_symbol};

#[test]
fn test_token_hash_and_equality_and_stuff_lol() {
    let f1 = String::from("f");
    let args1 = &parse_simlang("(1 2 3 4)").unwrap()[0];
    let result1 = Arc::new(SimLangToken::Symbol("0"));

    let f2 = "f";
    let args2 = &parse_simlang("(1 2 3 4)").unwrap()[0];
    let result2 = SimLangToken::Symbol("0");

    put_symbol(f1, args1, &result1);
    assert_eq!(*get_symbol(f2, args2).unwrap(), result2);
}

#[test]
fn test_symbol_table() {
    let f = String::from("f");
    let args = Arc::new(SimLangToken::Symbol("1"));
    let result = Arc::new(SimLangToken::Symbol("a"));

    put_symbol(f, &args, &result);

    let code = parse_simlang("(f 1)").unwrap()[0].clone();
    let compiled = compile(code);
    assert_eq!(compiled, result)
}