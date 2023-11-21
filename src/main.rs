use egg::Extractor;
use std::{collections::HashMap, env, fs, process::exit};
use theremin::{
    build_egraph, enumerate::bottom_up, language::Expr, sygus::parse_sygus_file, EvalCostFn, Spec,
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
                let components: HashMap<String, Vec<Expr>> = bank
                    .iter()
                    .map(|(sort, terms)| {
                        let exprs: Vec<Expr> =
                            terms.iter().map(|t| t.try_into().unwrap()).collect();
                        (sort.to_owned(), exprs)
                    })
                    .collect();
                let runner = build_egraph(Spec::Examples(vec![
                    (
                        Expr::ConstStr("Ducati100".into()),
                        Expr::ConstStr("Ducati".into()),
                    ),
                    (
                        Expr::ConstStr("Honda125".into()),
                        Expr::ConstStr("Honda".into()),
                    ),
                    (
                        Expr::ConstStr("Ducati250".into()),
                        Expr::ConstStr("Ducati".into()),
                    ),
                    (
                        Expr::ConstStr("Honda250".into()),
                        Expr::ConstStr("Honda".into()),
                    ),
                    (
                        Expr::ConstStr("Honda550".into()),
                        Expr::ConstStr("Honda".into()),
                    ),
                    (
                        Expr::ConstStr("Ducati125".into()),
                        Expr::ConstStr("Ducati".into()),
                    ),
                ]));
                let mut fills = HashMap::new();
                let cost_function = EvalCostFn::new(&runner.egraph, &components, &mut fills);
                let extractor = Extractor::new(&runner.egraph, cost_function);
                let ((cost_a, cost_b), best) = extractor.find_best(runner.roots[0]);
                println!(
                    "Result: {} with cost: {} holes, {} size",
                    best, cost_a, cost_b,
                );
                exit(0);
            }
        }
    }
}
