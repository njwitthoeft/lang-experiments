#![allow(unused)]
use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "json.pest"]
struct JSONParser;

enum JSONValue<'a> {
    Object(Vec<(&'a str, JSONValue<'a>)>),
    Array(Vec<JSONValue<'a>>),
    String(&'a str),
    Number(f64),
    Boolean(bool),
    Null,
}

fn serialize_jsonvalue(val: &JSONValue) -> String {
    use JSONValue as JV;

    match val {
        JV::Object(o) => {
            let contents: Vec<_> = o
                .iter()
                .map(|(name, value)| format!("\"{}\":{}", name, serialize_jsonvalue(value)))
                .collect();
            format!("{{{}}}", contents.join(","))
        }
        JV::Array(a) => {
            let contents: Vec<_> = a.iter().map(serialize_jsonvalue).collect();
            format!("[{}]", contents.join(","))
        }
        JV::String(s) => format!("\"{}\"", s),
        JV::Number(n) => format!("{}", n),
        JV::Boolean(b) => format!("{}", b),
        JV::Null => "null".to_string(),
    }
}

#[allow(clippy::result_large_err)]
fn parse_json_file(file: &str) -> Result<JSONValue, Error<Rule>> {
    let json = JSONParser::parse(Rule::json, file)?.next().unwrap();

    fn parse_value(pair: Pair<Rule>) -> JSONValue {
        match pair.as_rule() {
            Rule::object => JSONValue::Object(
                pair.into_inner()
                    .map(|pair| {
                        let mut inner_rules = pair.into_inner();
                        let name = inner_rules
                            .next()
                            .unwrap()
                            .into_inner()
                            .next()
                            .unwrap()
                            .as_str();
                        let value = parse_value(inner_rules.next().unwrap());
                        (name, value)
                    })
                    .collect(),
            ),
            Rule::array => JSONValue::Array(pair.into_inner().map(parse_value).collect()),
            Rule::string => JSONValue::String(pair.into_inner().next().unwrap().as_str()),
            Rule::number => JSONValue::Number(pair.as_str().parse().unwrap()),
            Rule::boolean => JSONValue::Boolean(pair.as_str().parse().unwrap()),
            Rule::null => JSONValue::Null,
            Rule::json
            | Rule::EOI
            | Rule::pair
            | Rule::value
            | Rule::inner
            | Rule::char
            | Rule::WHITESPACE => unreachable!(),
        }
    }
    Ok(parse_value(json))
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn roundtrip_json() {
        let unparsed_file = fs::read_to_string("example.json").unwrap();
        let json: JSONValue = parse_json_file(&unparsed_file).unwrap();

        let expected_serialization = "{\"nesting\":{\"inner object\":{}},\"an array\":[1.5,true,null,0.000001],\"string with escaped double quotes\":\"\\\"quick brown foxes\\\"\"}";
        assert_eq!(expected_serialization, serialize_jsonvalue(&json))
    }
}
