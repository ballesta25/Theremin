pub mod enumerate;
pub mod interpreter;
pub mod inverse;
pub mod language;
pub mod sygus;

use egg::{rewrite as rw, *};
use language::*;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::iter;

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
        match enode.op.as_str() {
            // inverse semantics here
            "inv" => {
                inverse::invert(
                    /*spec*/ &egraph[enode.children[1]].data,
                    egraph[enode.children[0]].nodes[0].op.as_str(), /*tag */
                )
            }

            _ => Indeterminate,
        }
    }
}

struct EvalCostFn<'a> {
    egraph: &'a EGraph<SLIALang, Spec>,
    component_fills: &'a mut HashMap<Id, Expr>,
}

impl<'a> EvalCostFn<'a> {
    fn new(egraph: &'a EGraph<SLIALang, Spec>, component_fills: &'a mut HashMap<Id, Expr>) -> Self {
        Self {
            egraph,
            component_fills,
        }
    }
}

impl<'a> CostFunction<SLIALang> for EvalCostFn<'a> {
    // (num_holes, size)
    type Cost = (usize, usize);
    fn cost<C>(&mut self, enode: &SLIALang, mut costs: C) -> Self::Cost
    where
        C: FnMut(Id) -> Self::Cost,
    {
        let (mut holes, mut size) = (0, 1);
        enode.fold((0, 1), |(a, b), id| {
            let (a1, b1) = costs(id);
            (a + a1, b + b1)
        });

        //check if enode *is* a hole
        let symbol = enode.op.as_str();
        let sorts: Vec<&str> = vec!["Int", "Bool", "String"];
        if sorts.iter().any(|&x| x == symbol) {
            // try to fill hole - if failure increment hole counter
            holes += 1;
        } else {
            // not a hole
        }
        (holes, size)
    }
}

pub fn grammar_rules() -> Vec<Rewrite<SLIALang, Spec>> {
    vec![
        rw!("eq"; "(Bool ?s)" => "(= (Int (inv eq0 ?s)) (Int (inv eq1 ?s)))"),
        rw!("gt"; "(Bool ?s)" => "(> (Int (inv gt0 ?s)) (Int (inv gt1 ?s)))"),
        rw!("ge"; "(Bool ?s)" => "(>= (Int (inv ge0 ?s)) (Int (inv ge1 ?s)))"),
        rw!("lt"; "(Bool ?s)" => "(< (Int (inv lt0 ?s)) (Int (inv lt1 ?s)))"),
        rw!("le"; "(Bool ?s)" => "(<= (Int (inv le0 ?s)) (Int (inv le1 ?s)))"),
    ]
}

fn build_egraph(examples: Spec) -> (EGraph<SLIALang, Spec>, Runner<SLIALang, Spec>) {
    let graph: EGraph<SLIALang, Spec> = Default::default();

    let start: RecExpr<SLIALang> = "(Bool root_spec)".parse().unwrap();

    let rules = grammar_rules();

    let runner = Runner::default().with_expr(&start).run(&rules);

    println!("{:#?}", runner);
    (graph, runner)
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_build_egraph() {
        let (egraph, runner) = build_egraph(Indeterminate);
        let mut fills = HashMap::new();
        let cost_function = EvalCostFn::new(&egraph, &mut fills);
        let extractor = Extractor::new(&runner.egraph, cost_function);
        let ((cost_a, cost_b), best) = extractor.find_best(runner.roots[0]);
        println!(
            "Result: {} with cost: {} holes, {} size",
            best, cost_a, cost_b,
        );
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
