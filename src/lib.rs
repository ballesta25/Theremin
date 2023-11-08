pub mod enumerate;
pub mod sygus;

use egg::{rewrite as rw, *};
use once_cell::sync::Lazy;

pub type SLIALang = SymbolLang;
pub type Spec = ();

static grammar_rules: Lazy<[Rewrite<SLIALang, Spec>; 5]> = Lazy::new(|| {
    [
        rw!("eq"; "(Bool n)" => "(= (S(n)) int int)"),
        rw!("gt"; "(Bool n)" => "(> (S(n)) int int)"),
        rw!("ge"; "(Bool n)" => "(>= (S(n)) int int)"),
        rw!("lt"; "(Bool n)" => "(< (S(n)) int int)"),
        rw!("le"; "(Bool n)" => "(<= (S(n)) int int)"),
    ]
});

fn build_egraph(examples: Spec) -> EGraph<SLIALang, Spec> {
    let mut graph: EGraph<SLIALang, Spec> = Default::default();

    let start: RecExpr<SLIALang> = "(Bool 0)".parse().unwrap();

    let rules = grammar_rules.clone();

    let runner = Runner::default().with_expr(&start).run(&rules);

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
        println!("{:#?}", g);
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
