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

    graph
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
