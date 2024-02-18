use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "csv.pest"]
pub struct CSVParser;

fn main() {
    let good_parse = CSVParser::parse(Rule::field, "-273.15");
    println!("{:?}", good_parse);

    let bad_parse = CSVParser::parse(Rule::field, "not a number");
    println!("{:?}", bad_parse);
}
