use crate::sygus::{BFTerm, GTerm, Grammar, Sort, Term};
use itertools::Itertools;
use std::collections::HashMap;

pub fn bottom_up(grammar: Grammar, depth: usize) -> HashMap<Sort, Vec<Term>> {
    let mut bank: HashMap<(String, usize), Vec<Term>> = HashMap::new();
    for d in 0..depth {
        for (name, _, rhs) in &grammar.rules[1..] {
            for term in new_terms(rhs, d, &bank) {
                bank.entry((name.to_owned(), d)).or_default().push(term);
            }
        }
    }
    let mut terms: HashMap<Sort, Vec<Term>> = HashMap::new();
    for d in 0..depth {
        for (name, sort, _) in &grammar.rules[1..] {
            if let Some(ts) = bank.get_mut(&(name.to_owned(), d)) {
                terms
                    .entry(sort.clone())
                    .and_modify(|e| e.append(ts))
                    .or_insert(ts.to_vec());
            }
        }
    }
    terms
}

fn new_terms(
    g_terms: &Vec<GTerm>,
    depth: usize,
    bank: &HashMap<(String, usize), Vec<Term>>,
) -> Vec<Term> {
    let mut terms: Vec<Term> = Vec::new();
    for g_term in g_terms {
        match g_term {
            GTerm::BFTerm(BFTerm::Application(name, holes)) if depth > 0 => {
                let fills = holes.iter().map(|hole| fill(hole, depth, bank).into_iter());
                for fill in fills.multi_cartesian_product() {
                    terms.push(Term::Application(name.to_owned(), fill));
                }
            }
            GTerm::BFTerm(BFTerm::Identifier(name)) if depth == 0 => {
                terms.push(Term::Identifier(name.to_owned()));
            }
            GTerm::BFTerm(BFTerm::Literal(lit)) if depth == 0 => {
                terms.push(Term::Literal(lit.to_owned()));
            }
            _ => {} // do nothing
        }
    }
    terms
}

fn fill(bf_term: &BFTerm, depth: usize, bank: &HashMap<(String, usize), Vec<Term>>) -> Vec<Term> {
    let mut terms: Vec<Term> = Vec::new();
    match bf_term {
        BFTerm::Identifier(name) => {
            for d in 0..depth {
                if let Some(filler) = bank.get(&(name.to_owned(), d)) {
                    for term in filler {
                        terms.push(term.to_owned());
                    }
                }
            }
        }
        _ => unimplemented!("Cannot fill a non-identifier"),
    }
    terms
}
