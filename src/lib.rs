pub mod enumerate;
pub mod sygus;

use egg::{rewrite as rw, *};
use once_cell::sync::Lazy;

pub type SLIALang = SymbolLang;
pub type Spec = ();

static grammar_rules: Lazy<[Rewrite<SLIALang, Spec>; 5]> = Lazy::new(|| {
    [
        // need to index 'int', not '='&c.
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
