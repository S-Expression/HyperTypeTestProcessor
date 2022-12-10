//일단 여기에 코드를 짜고 나중에 lib로 바꾸기
use pest::Parser;
use pest::error::Error;
use pest::iterators::Pair;

#[derive(Parser)]
#[grammar = "simple_grammer.pest"]
struct SimParser;

//지금은 수행평가 만드는 중이니까 나중에 메타 심볼 추가하기
pub enum SimLangToken<'a> {
    List(Vec<SimLangToken<'a>>),
    Symbol(&'a str), //메타 심볼도 파싱 단계에서 얘로 바뀐다.
}

impl<'a> SimLangToken<'a> {
    pub(crate) fn as_str(&self) -> String {
        match self {
            SimLangToken::List(tokens) => {
                tokens.into_iter().map(|element| format!("{} ", (&element).as_str())).collect::<String>()
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

pub fn parse_simlang(contents: &str) -> Result<Vec<SimLangToken>, Error<Rule>> {
    let parsed = SimParser::parse(Rule::SimpleLang, &contents)?;

    fn parse_expression(pair: Pair<Rule>) -> SimLangToken {
        match pair.as_rule() {
            Rule::expression => {
                // println!("expression 발견");
                panic!("dsfa")
                //  parse_value(pair.into_inner().next().unwrap())
            }
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
        }
    }

    let ast: Vec<SimLangToken> = parsed.filter_map(|pair| {
        return if pair.as_rule() == Rule::EOI {
            None
        } else {
            Some(parse_expression(pair))
        };
    }).collect();

    return Ok(ast);
}