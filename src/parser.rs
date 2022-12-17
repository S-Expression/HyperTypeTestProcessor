use std::rc::Rc;
//일단 여기에 코드를 짜고 나중에 lib로 바꾸기
use pest::Parser;
use pest::error::Error;
use pest::iterators::Pair;

#[derive(Parser)]
#[grammar = "simple_grammer.pest"]
struct SimParser;

/*
Peg 문법으로 어휘, 구문 분석을 해 파스 트리를 만드는 라이브러리. pest라는 peg 파서를 사용했다. 
*/

pub enum SimLangToken<'a> {
    List(Vec<Rc<SimLangToken<'a>>>),
    Symbol(&'a str), //메타 심볼도 파싱 단계에서 얘로 바뀐다.
}

impl<'a> SimLangToken<'a> {
    pub(crate) fn as_str(&self) -> String {
        match self {
            SimLangToken::List(tokens) => {
                format!("({})", tokens.into_iter().map(|element| format!("{} ", (&element).as_str())).collect::<String>())
            }
            SimLangToken::Symbol(s) => String::from(*s),
        }
    }
}

impl Clone for SimLangToken<'_> {
    fn clone(&self) -> Self {
        match self {
            SimLangToken::List(vector) => {
                SimLangToken::List(vector.clone())
            }
            SimLangToken::Symbol(symbol) => {
                SimLangToken::Symbol(symbol.clone())
            }
        }
    }
}

pub fn parse_simlang(contents: &str) -> Result<Vec<Rc<SimLangToken>>, Error<Rule>> {
    let parsed = SimParser::parse(Rule::SimpleLang, &contents)?;

    fn parse_expression(pair: Pair<Rule>) -> Rc<SimLangToken> {
        Rc::new(match pair.as_rule() {
            Rule::list => SimLangToken::List(pair.into_inner().map(parse_expression).collect()),
            Rule::string => SimLangToken::Symbol(pair.as_str()),
            /*
            Rule::expressions=>{panic!("expressions 인식됨 ㅅㄱ");},
            Rule::EOI => {}
            Rule::SimpleLang => {}
            Rule::left_bracket => {}
            Rule::right_bracket => {}
            Rule::space => {}
            Rule::meaningless_space => {}
            Rule::space_symbol => {}
            */
            _ => {
                unreachable!("{}", pair.as_str());
            }
        })
    }

    let ast: Vec<Rc<SimLangToken>> = parsed.filter_map(|pair| {
        return if pair.as_rule() == Rule::EOI {
            None
        } else {
            Some(parse_expression(pair))
        };
    }).collect();

    return Ok(ast);
}
