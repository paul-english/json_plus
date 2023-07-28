use pest::Parser;
use pest_derive::Parser;
use pest::error::Error;

#[derive(Parser)]
#[grammar = "json_plus.pest"]
struct JSONPlusParser;

enum JSONValue<'a> {
    Object(Vec<(&'a str, JSONValue<'a>)>),
    Array(Vec<JSONValue<'a>>),
    String(&'a str),
    Number(f64),
    Boolean(bool),
    Null,
}

fn parse_json_file(file: &str) -> Result<JSONValue, Error<Rule>> {
    let json = JSONPlusParser::parse(Rule::json, file)?.next().unwrap();

    use pest::iterators::Pair;

    fn parse_value(pair: Pair<Rule>) -> JSONValue {
        //println!("parse_value: {:?}", pair.as_rule());
        //println!("pair: {:?}", pair);
        //println!("pair: {:?}", pair.into_inner());
        match pair.as_rule() {
            Rule::object => JSONValue::Object(
                pair.into_inner()
                    .map(|pair| {
                        //println!("pair: {:?}", pair);
                        let mut inner_rules = pair.into_inner();
                        //println!("inner_rules: {:?}", inner_rules);
                        let name = inner_rules
                            .next()
                            .unwrap()
                            .as_str();
                        let value = parse_value(inner_rules.next().unwrap());
                        (name, value)
                    })
                    .collect(),
            ),
            Rule::array => JSONValue::Array(pair.into_inner().map(parse_value).collect()),
            //Rule::string => JSONValue::String(pair.as_str()),
            //Rule::string => JSONValue::String(pair.into_inner().next().unwrap().as_str().parse().unwrap()),
            Rule::string => {
                let mut inner = pair.into_inner();
                let matched_string = inner.next().unwrap().as_str();
                JSONValue::String(&matched_string[2..matched_string.len() - 2])
            },
            Rule::number => JSONValue::Number(pair.as_str().parse().unwrap()),
            Rule::int => JSONValue::Number(pair.as_str().parse().unwrap()),
            Rule::exp => JSONValue::Number(pair.as_str().parse().unwrap()),
            Rule::bool => JSONValue::Boolean(pair.as_str().parse().unwrap()),
            Rule::null => JSONValue::Null,
            Rule::json
            | Rule::EOI
            | Rule::pair
            | Rule::value
            | Rule::double_inner
            | Rule::single_inner
            | Rule::unicode
            | Rule::escape
            | Rule::WHITESPACE => unreachable!(),
        }
    }

    Ok(parse_value(json))

}

fn serialize_jsonvalue(val: &JSONValue) -> String {
    use JSONValue::*;

    match val {
        Object(o) => {
            let contents: Vec<_> = o
                .iter()
                .map(|(name, value)|
                     format!("{}:{}", name, serialize_jsonvalue(value)))
                .collect();
            format!("{{{}}}", contents.join(","))
        }
        Array(a) => {
            let contents: Vec<_> = a.iter().map(serialize_jsonvalue).collect();
            format!("[{}]", contents.join(","))
        }
        String(s) => format!("\"{}\"", s),
        Number(n) => format!("{}", n),
        Boolean(b) => format!("{}", b),
        Null => format!("null"),
    }
}

pub fn main() {
    let unparsed_file = std::fs::read_to_string("data/data.jplus").expect("cannot read file");

    println!("unparsed: {}", unparsed_file);
    println!("------");

    // TODO preserve whitespace in parse & serialize...
    let json: JSONValue = parse_json_file(&unparsed_file).expect("unsuccessful parse");
    println!("{}", serialize_jsonvalue(&json));
}
