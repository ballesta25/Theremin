use egg::Extractor;
use std::{collections::HashMap, env, fs, time::Instant};
use theremin::{build_runner, enumerate, language::Expr, sygus, EvalCostFn, Spec};

fn main() {
    let args: Vec<String> = env::args().take(3).collect();
    if args.len() != 3 {
        println!("Usage: theremin <path> <depth>");
        return;
    }

    let depth = args[2].parse().expect("depth should be a number");
    let file = fs::read_to_string(&args[1]).expect("cannot read file");

    let conjecture = sygus::parse_file(&file).expect("conjecture");

    // Only synthesizes for 1 function
    let function = conjecture
        .functions_to_synthesize
        .get(0)
        .expect("function to synthesize");

    let now = Instant::now();
    let bank = enumerate::bottom_up(&function.grammar, depth);
    println!("Bottom up enumeration took {}", now.elapsed().as_secs_f64());

    let now = Instant::now();
    let components: HashMap<String, Vec<Expr>> = bank
        .iter()
        .map(|(sort, terms)| {
            let exprs = terms.iter().map(|t| t.try_into().expect("expr")).collect();
            (sort.to_owned(), exprs)
        })
        .collect();
    println!("Component conversion took {}", now.elapsed().as_secs_f64());

    let specification = conjecture.specification().expect("specification");

    let now = Instant::now();
    let runner = build_runner(Spec::Examples(specification));

    let mut fills = HashMap::new();

    let cost_function = EvalCostFn::new(&runner.egraph, &components, &mut fills);

    let ((cost_a, cost_b, cost_c), best) =
        Extractor::new(&runner.egraph, cost_function).find_best(runner.roots[0]);
    println!("Egg took {}", now.elapsed().as_secs_f64());

    // println!("fills: {:#?}", fills);
    println!(
        "Result: {} with cost: {} unfillable, {} holes, {} size",
        best, cost_a, cost_b, cost_c,
    );
    // println!("{:?}", runner.egraph.lookup_expr_ids(&best));
}
