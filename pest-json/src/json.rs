#![allow(unused)]
use pest::error::Error;

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

fn parse_json_file(file: &str) -> Result<JSONValue, Error<Rule>> {}
