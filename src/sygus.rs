use pest_derive::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "sygus.pest"]
pub struct SygusParser;

/// Translated from *The SyGuS Language Standard Version 2.1*,
/// section 3 Semantics of Commands.
/// A synthesis conjecture is represented by a closed formula:
///
///     ∃ f_1, ..., f_n . ∀ v_1, ..., v_m . (α_1 ∧ ... ∧ α_r) =⇒ (φ_1 ∧ ... ∧ φ_q)
///
#[derive(Debug)]
pub struct Conjecture {
    /// A list of functions f_1,...,f_n to synthesize
    functions_to_synthesize: Vec<String>,
    /// A list of variables v_1,...,v_m, known as the universal variables
    universal_variables: Vec<String>,
    /// A list of formulas φ = φ_1, ..., φ_q, known as the current constraints
    constraints: Vec<i32>,
    /// A list of formulas α = α_1, ..., α_r, known as the current assumptions
    assumptions: Vec<i32>,
    /// The set of defined symbols in the current scope
    signature: HashMap<String, i32>,
    /// The SyGuS logic
    logic: Option<String>,
}

#[derive(Debug)]
pub struct Function {
    name: String,
    parameters: Vec<(String, Sort)>,
    return_type: Sort,
    grammar: (),
}

#[derive(Debug)]
pub enum Sort {
    Base(String),
    Compound(String, Vec<Sort>),
}
