use std::collections::HashMap;
use std::sync::Arc;
use once_cell::sync::Lazy;
use crate::base_ops::{car, cdr};
use crate::parser::SimLangToken;
use crate::parser::SimLangToken::List;
use crate::semantic_analyzer::Variable::Const;

/*
타입 오류를 검사하는 라이브러리.
검사 과정에서 자연스럽게 최적화된 중간 트리를 만들 수 있다. 
물론 숫자 연산이나 파일 등 특수 연산 자체가 너무 많기 때문에 특수 연산 확장을 쉽게 할 수 있도록 설계하는것이 중요하다. 
Lambda나 재귀함수와 같이 기호테이블을 수정하는 등 내부 구현상으로 메모리 삭제가 필요한 중요 연산들 역시 모두 이 라이브러리에서 정의한다.

simbol_table은  

*/

static mut FUNCTIONS: Lazy<HashMap<String, Variable>> = Lazy::new(|| {
    HashMap::new()
});

pub fn put_symbol(name: String, value: Variable<'static>) -> Option<Variable> {
    unsafe {
        FUNCTIONS.insert(name, value)
    }
}

pub fn get_symbol<'a>(name: &'a str) -> Option<&Variable<'a>> {
    unsafe {
        FUNCTIONS.get(name)
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Variable<'a> {
    Const {
        /*상수. 모두 입력받아 하나의 값을 출력하는 상수 함수의 개념으로 생각하면 굳이 필요한 구현은 아니지만 속도 향상을 위해 정의했다.
        일반적인 프로그램에서 변수의 역할과 같다고 할 수 있다.
        */
        value: Arc<SimLangToken<'a>>
    },
    Lambda {
        /* SimplLangToken의 lambda 연산을 최적화? 한 상태이자 정의역 치역을 알려주는 함수

        커링을 직접 구현하는 것은 너무 시간, 공간 복잡도가 낲아지므로 다중 매개 변수가 가능하도록 설계했다. 동시에 All, Not 등 변수가 특수한 집합을 모두 지칭할 수 있다.
        변수 이름은 (1 2 3 4 5....) 형태로 SimpleLangToken 의 위치로 구분한다.(알파 동치에 의해 사용자가 정의한 이름을 사용하지 않아도 같은 동작을 한다)

        공간복잡도가 너무 커진다고 느낄 수 있으나 여기서 정의하는 Lambda는 순수함수 이고 변수의 이름 대신 SimLangToken으로 입출력 변수를 구분하기 때문에 오히려 공간 복잡도를 줄일 수 있다....?는 검토가 필요하다.
        */
        domain: Vec<Arc<SimLangToken<'a>>>,
        range: Vec<Arc<SimLangToken<'a>>>,
    },
}

impl<'a> Variable<'a> {
    pub fn optimize(token: Arc<SimLangToken>) -> Variable {
        let List(elements) = token.as_ref() else { return Const { value: token }; };

        let first = car(elements);
        let rest = cdr(elements);


    }

    pub fn compose(&'a self, right: &'a Variable<'a>) -> Variable<'a> {
        let Variable::Lambda { domain: left_domain, range: left_range } = self
            else { panic!("입력은 함수로 해야지 십년아") };

        let Variable::Lambda { domain: right_domain, range: right_range } = right
            else { panic!("이거는 optimize에서 해야한단다 병신아 ^^") };

        let new_domain = right_domain.clone();
        let new_range = left_domain.iter().enumerate().filter_map(|element| {
            let (index, token) = element;
            if right_range.contains(token) {
                Some(left_range[index].clone())
            } else {
                None
            }
        }).collect::<Vec<Arc<SimLangToken>>>();

        Variable::Lambda {
            domain: new_domain,
            range: new_range,
        }
    }
}

fn label(name: String, key: Variable) {// der 연산
    // let mut simbol_table = HashMap::new();
}
