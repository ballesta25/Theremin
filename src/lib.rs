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
        rw!("eq"; "(Bool ?s)" => "(Eql (Int (inv eq0 ?s)) (Int (inv eq1 ?s)))"),
        //rw!("gt"; "(Bool ?s)" => "(> (Int (inv gt0 ?s)) (Int (inv gt1 ?s)))"),
        rw!("ge"; "(Bool ?s)" => "(Geq (Int (inv ge0 ?s)) (Int (inv ge1 ?s)))"),
        //rw!("lt"; "(Bool ?s)" => "(< (Int (inv lt0 ?s)) (Int (inv lt1 ?s)))"),
        rw!("le"; "(Bool ?s)" => "(Leq (Int (inv le0 ?s)) (Int (inv le1 ?s)))"),
        rw!("substr"; "(String ?s)" => "(SubStr (String (inv substring0 ?s)) (Int (inv substring1 ?s)) (Int(inv substring2 ?s)))"),
        rw!("app"; "(String ?s)" => "(Append (String (inv append0  ?s)) (String (inv append1 ?s)))"),
        rw!("replace"; "(String ?s)" => "(Replace (String (inv replace0 ?s)) (String (inv replace1 ?s)) (String (inv replace2 ?s)))"),
        rw!("strlen"; "(Int ?s)" => "(StrLen (String (inv strlen0 ?s)))"),
        rw!("StrAt"; "(String ?s)" => "(StrAt (String (inv na ?s)) (Int (inv na ?s)) )"),
        rw!("IsPre"; "(Bool ?s)" => "(IsPre (String (inv na ?s)) (String (inv na ?s)) )"),
        rw!("IsPost"; "(Bool ?s)" => "(IsPost (String (inv na ?s)) (String (inv na ?s)) )"),
        rw!("Contains"; "(Bool ?s)" => "(Contains (String (inv na ?s)) (String (inv na ?s)) )"),
        rw!("Index"; "(Int ?s)" => "(Index (String (inv na ?s)) (String (inv na ?s)) (Int (inv na ?s)))"),
        rw!("replaceall"; "(String ?s)" => "(ReplaceAll (String (inv na ?s)) (String (inv na ?s)) (String (inv na ?s)))"),
        rw!("Add"; "(Int ?s)" => "(Add (Int (inv na ?s)) (Int (inv na ?s)) )"),
        rw!("Add"; "(Int ?s)" => "(Min (Int (inv na ?s)) (Int (inv na ?s)) )"),
        rw!("Mult"; "(Int ?s)" => "(Mult (Int (inv na ?s)) (Int (inv na ?s)) )"),
        rw!("Div"; "(Int ?s)" => "(Div (Int (inv na ?s)) (Int (inv na ?s)) )"),
        rw!("Abs"; "(Int ?s)" => "(Abs (Int (inv na ?s)) (Int (inv na ?s)) )"),
        rw!("Mod"; "(Int ?s)" => "(Mod (Int (inv na ?s)) (Int (inv na ?s)) )"),
        rw!("NegI"; "(Int ?s)" => "(NegI (Int (inv na ?s)) (Int (inv na ?s)) )"),
        rw!("NegB"; "(Bool ?s)" => "(NegB (Bool (inv na ?s)) )"),
        rw!("And"; "(Bool ?s)" => "(And (Bool (inv na ?s)) (Bool (inv na ?s)) )"),
        rw!("Or"; "(Bool ?s)" => "(Or (Bool (inv na ?s)) (Bool (inv na ?s)) )"),
        rw!("LexEq"; "(Bool ?s)" => "(LexEq (String (inv na ?s)) (String (inv na ?s)) )"),
        rw!("LexLeq"; "(Bool ?s)" => "(LexLeq (String (inv na ?s)) (String (inv na ?s)) )"),
        rw!("LexGeq"; "(Bool ?s)" => "(LexGeq (String (inv na ?s)) (String (inv na ?s)) )"),





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
