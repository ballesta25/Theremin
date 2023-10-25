use pest::Parser;
use std::{env, fs};
use theremin::sygus::{Rule, SygusParser};

fn main() {
    let args: Vec<String> = env::args().take(2).collect();
    if args.len() != 2 {
        println!("Usage: theremin <path>");
        return;
    }

    let file = fs::read_to_string(&args[1]).expect("cannot read file");
    let result = SygusParser::parse(Rule::sygus, &file);

    match result {
        Err(e) => println!("ERROR: {}", e),
        Ok(spec) => println!("{:#?}", spec),
    }
}
