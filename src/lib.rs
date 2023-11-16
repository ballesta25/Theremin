pub mod enumerate;
pub mod interpreter;
pub mod inverse;
pub mod language;
pub mod sygus;

use egg::{rewrite as rw, *};
use language::*;
use once_cell::sync::Lazy;

pub type SLIALang = SymbolLang;
#[derive(Clone, Debug, Default)]
pub enum Spec {
    Examples(Vec<(Expr, Expr)>),
    Impossible,
    #[default]
    Indeterminate,
}
use Spec::*;

impl Analysis<SLIALang> for Spec {
    type Data = Spec;

    fn merge(&mut self, to: &mut Self::Data, from: Self::Data) -> DidMerge {
        match (to, from) {
            (Impossible, Impossible) => DidMerge(false, false),
            (_, _) => DidMerge(true, true), // fixme
        }
    }

    fn make(egraph: &EGraph<SLIALang, Self>, enode: &SLIALang) -> Self::Data {
        match enode {
            // inverse semantics here
            _ => Indeterminate,
        }
    }
}

struct EvalCostFn;
impl CostFunction<SLIALang> for EvalCostFn {
    // (wrong_count, num_holes, size)
    type Cost = (usize, usize, usize);
    fn cost<C>(&mut self, enode: &SLIALang, mut costs: C) -> Self::Cost
    where
        C: FnMut(Id) -> Self::Cost,
    {
        let (mut wrong, mut holes, size) = enode.fold((0, 0, 1), |(a, b, c), id| {
            let (a1, b1, c1) = costs(id);
            (a + a1, b + b1, c + c1)
        });

        //check if enode *is* a hole
        let mut dummy_vec = Vec::<Id>::new();
        dummy_vec.push(0.into());
        dummy_vec.push(0.into());
        let matches_hole = SymbolLang::new("hole", dummy_vec);
        if enode.matches(&matches_hole) {
            holes += 1;
        }
        // eval to check for wrong examples
        (wrong, holes, size)
    }
}

static grammar_rules: Lazy<[Rewrite<SLIALang, Spec>; 6]> = Lazy::new(|| {
    [
        rw!("eq"; "(Bool ?s)" => "(= (Int (inv (eq0 ?s))) (Int (inv (eq1 ?s))))"),
        rw!("gt"; "(Bool ?s)" => "(> (Int (inv (gt0 ?s))) (Int (inv (gt1 ?s))))"),
        rw!("ge"; "(Bool ?s)" => "(>= (Int (inv (ge0 ?s))) (Int (inv (ge1 ?s))))"),
        rw!("lt"; "(Bool ?s)" => "(< (Int (inv (lt0 ?s))) (Int (inv (lt1 ?s))))"),
        rw!("le"; "(Bool ?s)" => "(<= (Int (inv (le0 ?s))) (Int (inv (le1 ?s))))"),
        rw!("int_hole"; "(Int ?s)" => "(hole Int ?s)"),
    ]
});

fn build_egraph(examples: Spec) -> Runner<SLIALang, Spec> {
    let mut graph: EGraph<SLIALang, Spec> = Default::default();

    let start: RecExpr<SLIALang> = "(Bool 0)".parse().unwrap();

    let rules = grammar_rules.clone();

    let runner = Runner::default().with_expr(&start).run(&rules);

    println!("{:#?}", runner);
    runner
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_build_egraph() {
        let runner = build_egraph(Indeterminate);
        let extractor = Extractor::new(&runner.egraph, EvalCostFn);
        let ((cost_a, cost_b, cost_c), best) = extractor.find_best(runner.roots[0]);
        println!(
            "Result: {} with cost: {} wrong, {} holes, {} size",
            best, cost_a, cost_b, cost_c
        );
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
