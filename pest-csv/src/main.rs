use std::env;
use std::fs;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "csv.pest"]
pub struct CSVParser;

struct ParseSumCount {
    sum: f64,
    count: u64,
}

fn parse_then_sum_and_count(file: &str) -> ParseSumCount {
    let file_content = fs::read_to_string(file).expect("Can't read file!");
    let parsed = CSVParser::parse(Rule::file, &file_content)
        .expect("Can't parse file!")
        .next()
        .unwrap();

    let mut field_sum: f64 = 0.0;
    let mut record_count: u64 = 0;

    for record in parsed.into_inner() {
        match record.as_rule() {
            Rule::record => {
                record_count += 1;

                for field in record.into_inner() {
                    field_sum += field.as_str().parse::<f64>().unwrap()
                }
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    ParseSumCount {
        sum: field_sum,
        count: record_count,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = &args[1];

    let parse = parse_then_sum_and_count(input_path);

    println!("Sum of fields: {}", parse.sum);
    println!("Number of records: {}", parse.count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn good_parse() {
        let good_parse = CSVParser::parse(Rule::field, "-273.15");
        assert!(good_parse.is_ok())
    }
    #[test]
    fn bad_parse() {
        let bad_parse = CSVParser::parse(Rule::field, "not a number");
        assert!(bad_parse.is_err())
    }
    #[test]
    fn parse_numbers_sum_count() {
        let parse = parse_then_sum_and_count("numbers.csv");
        assert_eq!(parse.sum, 2643429302.327908);
        assert_eq!(parse.count, 5_u64);
    }
}
