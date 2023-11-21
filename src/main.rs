use std::{collections::HashMap, env, fs, process::exit};
use theremin::{
    enumerate::bottom_up,
    language::{Eval, Expr},
    sygus::parse_sygus_file,
};

fn main() {
    let args: Vec<String> = env::args().take(3).collect();
    if args.len() != 3 {
        println!("Usage: theremin <path> <depth>");
        return;
    }

    let file = fs::read_to_string(&args[1]).expect("cannot read file");
    let conjecture = parse_sygus_file(&file);

    let mut env: HashMap<String, Expr> = HashMap::new();
    env.insert("name".into(), Expr::ConstStr("Input".into()));

    match conjecture {
        Err(err) => println!("{:#?}", err),
        Ok(conjecture) => {
            for function in conjecture.functions_to_synthesize {
                let bank = bottom_up(
                    function.grammar,
                    args[2].parse().expect("depth should be a number"),
                );
                println!("; {}", bank.len());
                for (sort, terms) in bank {
                    // let expr: Option<Expr> = (&term).try_into().ok();
                    // let res = expr.map(|e| e.eval(&env));
                    // println!("{}   ; ==> {:?}", term, res);
                    for term in terms {
                        println!("{}   ; {:?}", term, sort);
                    }
                }
                exit(0);
            }
        }
    }
}
