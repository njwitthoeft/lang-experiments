use std::collections::HashMap;
use std::{env, fs};

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "ini.pest"]
pub struct INIParser;

fn parse_ini(filename: &str) -> HashMap<String, HashMap<String, String>> {
    let file_content = fs::read_to_string(filename).expect("Can't read file!");

    let parsed = INIParser::parse(Rule::file, &file_content)
        .expect("Can't parse file!")
        .next()
        .unwrap();

    let mut properties: HashMap<String, HashMap<String, String>> = HashMap::new();

    let mut current_section_name = "";

    for line in parsed.into_inner() {
        match line.as_rule() {
            Rule::section => {
                let mut inner_rules = line.into_inner();
                current_section_name = inner_rules.next().unwrap().as_str();
            }
            Rule::property => {
                let mut inner_rules = line.into_inner();
                let name: &str = inner_rules.next().unwrap().as_str();
                let value: &str = inner_rules.next().unwrap().as_str();
                let section = properties
                    .entry(current_section_name.to_string())
                    .or_default();
                section.insert(name.to_string(), value.to_string());
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
    properties
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = &args[1];
    let props: HashMap<String, HashMap<String, String>> = parse_ini(input_path);
    println!("{:#?}", props);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ini_works() {
        let testfile = "example.ini";
        let props = parse_ini(testfile);
        let anon_section = props.get("").unwrap();
        assert_eq!(
            anon_section.get("salt").unwrap().as_str(),
            "NaCl".to_string()
        );
    }
}
