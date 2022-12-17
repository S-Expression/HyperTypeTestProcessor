use core::panic;
use std::collections::HashMap;
use std::rc::Rc;
use crate::parser::SimLangToken;
use crate::parser::SimLangToken::{List, Symbol};

/*
개발 계획: 
1. side_effect_ops.rs가 어느정도 완성되면 compile 함수와 6개 연산에 symbol_tabel:HashMap<Variable> 매개변수를 추가한다.
2. 나중에는 7연산, 특수연산들을 Enum으로 바꿔서 더 빠른 실행이 가능하도록 만든다. 
side_effect_ops.rs에서 모든 타입 오류를 검증하고 사이드 이펙트가 있는 명령어를 제외한 기본 7연산과 Symbol 함성으로 이루어진 부분을 연산하는 역할을 한다. 
*/

//let mut scores = HashMap::new();
struct Const<'a>{
    value: SimLangToken<'a>
}

struct Lambda<'a> {  
    /* 타입 검사 과정에서 가장 핵심인 구조체이다. 커링을 직접 구현하는 것은 너무 시간, 공간 복잡도가 낲아지므로 다중 매개 변수가 가능하도록 설계하고, 동시에 All, Not 등 변수가 특수한 집합을 모두 지칭할 수 있다. 
    변수 이름은 (1 2 3 4 5....) 형태로 SimpleLangToken 의 위치로 구분한다.(알파 동치에 의해 사용자가 정의한 이름을 사용하지 않아도 같은 동작을 한다)
    */
    domain: Vec<SimLangToken<'a>>, //입력 가능한 i개의 값.
    range: Vec<SimLangToken<'a>>, //i번째 입력에 대응하는 i개의 출력 
}

/*impl typecheck for Lambda{

}*/

enum Variable{
    Const,
    Lambda,
}


fn quote(arg: Vec<Rc<SimLangToken>>) -> Rc<SimLangToken> {
    Rc::new(SimLangToken::List(arg))
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
    let List(ref elements) = *compile(args[0].clone()) else { unreachable!() };
    compile(elements[0].clone())
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


fn lambda(arg: Vec<Rc<SimLangToken>>) -> Rc<SimLangToken> {
    panic!(
        "이거 '해줘'"
    );
    //Rc::new(SimLangToken::List(arg))
}
pub fn test()-> i8{
    
    return  0;
}

pub fn meaning_analysis(token: Rc<SimLangToken>) -> Rc<SimLangToken>{ //의미 분석 단계: 인터프린터 함수 실행 전에 
    let List(ref elements) = *token else { return token };

    let first = elements[0].clone();
    let rest = (elements[1..]).to_vec();

    let Symbol(operator) = *first else { return token };
    panic!();
}

pub fn compile(token: Rc<SimLangToken>) -> Rc<SimLangToken> { /*
    인자로 추가된 symbol_table은 compile 함수가 실행되는 지점에서의 정의되어있는 기호 테이블이다. 연산마다 기호 테이블이 조금씩 다르기 때문에 인자로 받았다.  def 연산은 사이드 이펙트이브로 compile 함수가 실행하는 SimLangToken은 def 가 없다.  
    symbol_table에는 반드시 사이드이펙트가 없는 순수 함수만이 있다.
    */
    
    
    let List(ref elements) = *token else { return token };

    let first = elements[0].clone();
    let no_first = (elements[1..]).to_vec();

    let Symbol(operator) = *first else { return token };
    /* */

    match operator {
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
