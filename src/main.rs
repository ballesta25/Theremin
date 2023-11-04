use std::{env, fs, process::exit};
use theremin::{enumerate::bottom_up, sygus::parse_sygus_file};

fn main() {
    let args: Vec<String> = env::args().take(3).collect();
    if args.len() != 3 {
        println!("Usage: theremin <path> <depth>");
        return;
    }

    let file = fs::read_to_string(&args[1]).expect("cannot read file");
    let conjecture = parse_sygus_file(&file);

    match conjecture {
        Err(err) => println!("{:#?}", err),
        Ok(conjecture) => {
            for function in conjecture.functions_to_synthesize {
                let bank = bottom_up(
                    function.grammar,
                    args[2].parse().expect("depth should be a number"),
                );
                println!("; {}", bank.len());
                for term in bank {
                    println!("{}", term);
                }
                exit(0);
            }
        }
    }
}
