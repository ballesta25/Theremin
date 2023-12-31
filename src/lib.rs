pub mod enumerate;
pub mod interpreter;
pub mod inverse;
pub mod language;
pub mod sygus;

use egg::{self, CostFunction, EGraph, Id, Language, RecExpr, Rewrite, Runner, SymbolLang};
use egg::{rewrite as rw, Analysis, DidMerge};
use language::{Eval, Expr, Func::*, Term};
use std::collections::HashMap;

pub type SLIALang = SymbolLang;
#[derive(Clone, Debug, Default, PartialEq)]
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

pub struct EvalCostFn<'a> {
    egraph: &'a EGraph<SLIALang, Spec>,
    components: &'a HashMap<String, Vec<Expr>>,
    component_fills: &'a mut HashMap<Id, Expr>,
}

impl<'a> EvalCostFn<'a> {
    pub fn new(
        egraph: &'a EGraph<SLIALang, Spec>,
        components: &'a HashMap<String, Vec<Expr>>,
        component_fills: &'a mut HashMap<Id, Expr>,
    ) -> Self {
        Self {
            egraph,
            components,
            component_fills,
        }
    }
}

impl<'a> CostFunction<SLIALang> for EvalCostFn<'a> {
    // (unfillable_holes, num_holes, size)
    type Cost = (usize, usize, usize);
    fn cost<C>(&mut self, enode: &SLIALang, mut costs: C) -> Self::Cost
    where
        C: FnMut(Id) -> Self::Cost,
    {
        let (mut unfillable, mut holes, size) = (0, 0, 1);

        let class = self.egraph.lookup(enode.clone()).unwrap();
        //check if enode *is* a hole
        let symbol = enode.op.as_str();
        match symbol {
            "Int" | "Bool" | "String" => {
                // try to fill hole - if failure increment hole counter

                let spec = &self.egraph[enode.children[0]].data;
                match spec {
                    Impossible => unfillable += 1,
                    Indeterminate => holes += 1,
                    Examples(ios) => {
                        // try to fill
                        let mut found_fill = false;
                        for e in self.components[symbol].iter() {
                            if ios.iter().all(|(i, o)| {
                                let mut env: HashMap<String, Expr> = HashMap::new();
                                env.insert(String::from("name"), i.clone());
                                e.clone().eval(&env) == Ok(o.clone())
                            }) {
                                found_fill = true;
                                self.component_fills.insert(class, e.clone());
                                break;
                            }
                        }
                        /* on failure */
                        if !found_fill {
                            unfillable += 1
                        };
                    }
                };
            }
            _ => (),
        };
        enode.fold((unfillable, holes, size), |(a, b, c), id| {
            let (a1, b1, c1) = costs(id);
            (a + a1, b + b1, c + c1)
        })
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
        rw!("StrAt"; "(String ?s)" => "(StrAt (String (inv strat0 ?s)) (Int (inv strat0 ?s)) )"),
        rw!("IsPre"; "(Bool ?s)" => "(IsPre (String (inv na ?s)) (String (inv na ?s)) )"),
        rw!("IsPost"; "(Bool ?s)" => "(IsPost (String (inv na ?s)) (String (inv na ?s)) )"),
        rw!("Contains"; "(Bool ?s)" => "(Contains (String (inv na ?s)) (String (inv na ?s)) )"),
        rw!("Index"; "(Int ?s)" => "(Index (String (inv index0 ?s)) (String (inv index1 ?s)) (Int (inv index2 ?s)))"),
        rw!("replaceall"; "(String ?s)" => "(ReplaceAll (String (inv repall0 ?s)) (String (inv repall1 ?s)) (String (inv repall2 ?s)))"),
        rw!("Add"; "(Int ?s)" => "(Add (Int (inv add0 ?s)) (Int (inv add1 ?s)) )"),
        rw!("Min"; "(Int ?s)" => "(Min (Int (inv min0 ?s)) (Int (inv min1 ?s)) )"),
        rw!("Mult"; "(Int ?s)" => "(Mult (Int (inv mult0 ?s)) (Int (inv mult1 ?s)) )"),
        rw!("Div"; "(Int ?s)" => "(Div (Int (inv div0 ?s)) (Int (inv div1 ?s)) )"),
        rw!("Abs"; "(Int ?s)" => "(Abs (Int (inv abs0 ?s)) (Int (inv abs1 ?s)) )"),
        rw!("Mod"; "(Int ?s)" => "(Mod (Int (inv mod0 ?s)) (Int (inv mod1 ?s)) )"),
        rw!("NegI"; "(Int ?s)" => "(NegI (Int (inv negi0 ?s)) )"),
        rw!("NegB"; "(Bool ?s)" => "(NegB (Bool (inv negb0 ?s)) )"),
        rw!("And"; "(Bool ?s)" => "(And (Bool (inv and0 ?s)) (Bool (inv and1 ?s)) )"),
        rw!("Or"; "(Bool ?s)" => "(Or (Bool (inv or0 ?s)) (Bool (inv or1 ?s)) )"),
        rw!("LexEq"; "(Bool ?s)" => "(LexEq (String (inv lexeq0 ?s)) (String (inv lexeq1 ?s)) )"),
        rw!("LexLeq"; "(Bool ?s)" => "(LexLeq (String (inv lexleq0 ?s)) (String (inv lexleq1 ?s)) )"),
        rw!("LexGeq"; "(Bool ?s)" => "(LexGeq (String (inv lexgeq0 ?s)) (String (inv lexgeq1 ?s)) )"),
    ]
}

pub fn build_runner(examples: Spec) -> Runner<SLIALang, Spec> {
    let start: RecExpr<SLIALang> = "(String root_spec)".parse().unwrap();
    let rules = grammar_rules();
    let mut runner = Runner::default().with_expr(&start);
    runner.egraph.set_analysis_data(0.into(), examples);
    runner.egraph.rebuild();

    runner.run(&rules)
}

pub fn get_term(
    egraph: &EGraph<SLIALang, Spec>,
    fills: &HashMap<Id, Expr>,
    prgm: &RecExpr<SLIALang>,
) -> Term {
    let ids = egraph.lookup_expr_ids(prgm).unwrap();
    get_term_rec(fills, prgm, &ids, ids.len() - 1)
}

enum ArgsVariant<X, Y> {
    One(fn(X) -> Y),
    Two(fn(X, X) -> Y),
    Three(fn(X, X, X) -> Y),
}
use crate::ArgsVariant::*;

fn get_term_rec(
    fills: &HashMap<Id, Expr>,
    prgm: &RecExpr<SLIALang>,
    ids: &Vec<Id>,
    i: usize,
) -> Term {
    if fills.contains_key(&ids[i]) {
        Ok(fills[&ids[i]].clone())
    } else {
        let constructor = match prgm[i.into()].op.as_str() {
            "Append" => Two(Append),
            "StrLen" => One(StrLen),
            "StrAt" => Two(StrAt),
            "SubStr" => Three(SubStr),
            "IsPre" => Two(IsPre),
            "IsPost" => Two(IsPost),
            "Contains" => Two(Contains),
            "Index" => Three(Index),
            "Replace" => Three(Replace),
            "ReplaceAll" => Three(ReplaceAll),
            "Leq" => Two(Leq),
            "Geq" => Two(Geq),
            "Eql" => Two(Eql),
            "Add" => Two(Add),
            "Min" => Two(Min),
            "Mult" => Two(Mult),
            "Div" => Two(Div),
            "Abs" => One(Abs),
            "Mod" => Two(Mod),
            "NegI" => One(NegI),
            "NegB" => One(NegB),
            "And" => Two(And),
            "Or" => Two(Or),
            "LexEq" => Two(LexEq),
            "LexLeq" => Two(LexLeq),
            "LexGeq" => Two(LexGeq),
            "StrToInt" => One(StrToInt),
            "IntToStr" => One(IntToStr),
            _ => {
                return Err(format!(
                    "not a complete program: unfilled hole with label: {}",
                    prgm[i.into()].op.as_str()
                )
                .into())
            }
        };
        let subterms: Vec<Expr> = prgm[i.into()]
            .children
            .iter()
            .map(|id| get_term_rec(fills, prgm, ids, usize::from(*id)))
            .collect::<Result<Vec<Expr>, String>>()?;
        match constructor {
            One(f) => Ok(Expr::Call(Box::new(f(subterms[0].clone())))),
            Two(f) => Ok(Expr::Call(Box::new(f(
                subterms[0].clone(),
                subterms[1].clone(),
            )))),
            Three(f) => Ok(Expr::Call(Box::new(f(
                subterms[0].clone(),
                subterms[1].clone(),
                subterms[2].clone(),
            )))),
        }
    }
}

#[cfg(test)]
mod tests {
    use egg::Extractor;

    use super::*;

    #[test]
    fn run_build_egraph() {
        let runner = build_runner(Examples(vec![
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
        let components = HashMap::new();
        let mut fills = HashMap::new();
        let cost_function = EvalCostFn::new(&runner.egraph, &components, &mut fills);
        let extractor = Extractor::new(&runner.egraph, cost_function);
        let ((cost_a, cost_b, cost_c), best) = extractor.find_best(runner.roots[0]);
        println!(
            "Result: {} with cost: {} unfillable, {} holes, {} size",
            best, cost_a, cost_b, cost_c,
        );
    }
}
