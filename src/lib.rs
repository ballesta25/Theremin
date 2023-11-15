pub mod enumerate;
pub mod sygus;

use egg::{rewrite as rw, *};
use once_cell::sync::Lazy;

pub type SLIALang = SymbolLang;
pub type Spec = ();

pub struct SLIASpec;
impl Analysis<SLIALang> for SLIASpec {
    type Data = Option<Spec>;

    fn merge(&mut self, to: &mut Self::Data, from: Self::Data) -> DidMerge {
        merge_option(to, from, |x, y| DidMerge(true, true))
    }

    fn make(egraph: &EGraph<SLIALang, Self>, enode: &SLIALang) -> Self::Data {
        match enode {
            // inverse semantics here
            _ => None,
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
        let (wrong, holes, size) = enode.fold((0, 0, 1), |(a, b, c), id| {
            let (a1, b1, c1) = costs(id);
            (a + a1, b + b1, c + c1)
        });
        // eval to check for wrong examples; check if enode *is* a hole
        (wrong, holes, size)
    }
}

static grammar_rules: Lazy<[Rewrite<SLIALang, Spec>; 5]> = Lazy::new(|| {
    [
        rw!("eq"; "(Bool ?s)" => "(= (int (eq0 ?s)) (int (eq1 ?s)))"),
        rw!("gt"; "(Bool ?s)" => "(> (int (gt0 ?s)) (int (gt1 ?s)))"),
        rw!("ge"; "(Bool ?s)" => "(>= (int (ge0 ?s)) (int (ge1 ?s)))"),
        rw!("lt"; "(Bool ?s)" => "(< (int (lt0 ?s)) (int (lt1 ?s)))"),
        rw!("le"; "(Bool ?s)" => "(<= (int (le0 ?s)) (int (le1 ?s)))"),
    ]
});

fn build_egraph(examples: Spec) -> EGraph<SLIALang, Spec> {
    let mut graph: EGraph<SLIALang, Spec> = Default::default();

    let start: RecExpr<SLIALang> = "(Bool 0)".parse().unwrap();

    let rules = grammar_rules.clone();

    let runner = Runner::default().with_expr(&start).run(&rules);

    println!("{:#?}", runner);
    graph
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_build_egraph() {
        let g = build_egraph(());
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
