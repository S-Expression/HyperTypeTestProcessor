use std::collections::HashMap;
use std::sync::Arc;
use once_cell::sync::Lazy;
use crate::parser::SimLangToken;

/*
타입 오류를 검사하는 라이브러리.
검사 과정에서 자연스럽게 최적화된 중간 트리를 만들 수 있다. 
물론 숫자 연산이나 파일 등 특수 연산 자체가 너무 많기 때문에 특수 연산 확장을 쉽게 할 수 있도록 설계하는것이 중요하다. 
Lambda나 재귀함수와 같이 기호테이블을 수정하는 등 내부 구현상으로 메모리 삭제가 필요한 중요 연산들 역시 모두 이 라이브러리에서 정의한다.

simbol_table은  

*/
static mut FUNCTIONS: Lazy<HashMap<String, HashMap<Arc<SimLangToken>, Arc<SimLangToken>>>> = Lazy::new(|| {
    HashMap::new()
});

pub fn put_symbol<'a>(function_name: String, arguments: &Arc<SimLangToken<'static>>, result: &Arc<SimLangToken<'static>>) -> Option<Arc<SimLangToken<'a>>> {
    unsafe {
        match FUNCTIONS.get_mut(function_name.as_str()) {
            None => {
                let mut new_map: HashMap<Arc<SimLangToken>, Arc<SimLangToken>> = HashMap::new();
                new_map.insert(arguments.clone(), result.clone());
                FUNCTIONS.insert(function_name, new_map);
                None
            }
            Some(existing) => {
                existing.insert(arguments.clone(), result.clone())
            }
        }
    }
}

pub fn get_symbol<'a>(function_name: &'a str, arguments: &Arc<SimLangToken<'a>>) -> Option<Arc<SimLangToken<'a>>> {
    unsafe {
        match FUNCTIONS.get(function_name) {
            None => { None }
            Some(existing) => {
                println!("{}", existing.keys().map(|key| key.as_str()).collect::<String>());
                match existing.get(arguments.as_ref()) {
                    None => { None }
                    Some(result) => {
                        Some(result.clone())
                    }
                }
            }
        }
    }
}

struct Const<'a>{
    /*상수. 모두 입력받아 하나의 값을 출력하는 상수 함수의 개념으로 생각하면 굳이 필요한 구현은 아니지만 속도 향상을 위해 정의했다.
    일반적인 프로그램에서 변수의 역할과 같다고 할 수 있다.
    */
    value: Arc<SimLangToken<'a>>
}
struct Lambda<'a> {  
    /* SimplLangToken의 lambda 연산을 최적화? 한 상태이자 정의역 치역을 알려주는 함수
    
    커링을 직접 구현하는 것은 너무 시간, 공간 복잡도가 낲아지므로 다중 매개 변수가 가능하도록 설계했다. 동시에 All, Not 등 변수가 특수한 집합을 모두 지칭할 수 있다. 
    변수 이름은 (1 2 3 4 5....) 형태로 SimpleLangToken 의 위치로 구분한다.(알파 동치에 의해 사용자가 정의한 이름을 사용하지 않아도 같은 동작을 한다)

    공간복잡도가 너무 커진다고 느낄 수 있으나 여기서 정의하는 Lambda는 순수함수 이고 변수의 이름 대신 SimLangToken으로 입출력 변수를 구분하기 때문에 오히려 공간 복잡도를 줄일 수 있다....?는 검토가 필요하다.
    */
    domain: Vec<Arc<SimLangToken<'a>>>, //입력 가능한 i개의 정의역.
    range: Vec<Arc<SimLangToken<'a>>>, //i번째 입력에 대응하는 i개의 츨력(치역)
}

enum Variable{
    /*
    입력받은 SimplLangToken을 빠르고 안전하게 실행 가능하게 한 구조체이다. 
    나중에 Macro 타입도 추가하고 싶다.
    */
    Const,
    Lambda,
}
/*
fn compose
*/

fn label(name:String, key:Variable)  {// der 연산
   // let mut simbol_table = HashMap::new();
}
