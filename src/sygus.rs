use core::fmt;
use pest::{error::Error, iterators::Pair, Parser};
use pest_derive::Parser;
use std::collections::HashMap;
use substring::Substring;

use crate::language::{Expr, Func};

#[derive(Parser)]
#[grammar = "sygus.pest"]
pub struct SygusParser;

// Translated from *The SyGuS Language Standard Version 2.1*,
// section 3 Semantics of Commands.
// A synthesis conjecture is represented by a closed formula:
//
//     ∃ f_1, ..., f_n . ∀ v_1, ..., v_m . (α_1 ∧ ... ∧ α_r) =⇒ (φ_1 ∧ ... ∧ φ_q)
//
#[derive(Debug)]
pub struct Conjecture {
    /// A list of functions f_1,...,f_n to synthesize
    pub functions_to_synthesize: Vec<Function>,
    /// A list of variables v_1,...,v_m, known as the universal variables
    pub universal_variables: Vec<String>,
    /// A list of formulas φ = φ_1, ..., φ_q, known as the current constraints
    pub constraints: Vec<i32>,
    /// A list of formulas α = α_1, ..., α_r, known as the current assumptions
    pub assumptions: Vec<i32>,
    /// The set of defined symbols in the current scope
    pub signature: HashMap<String, i32>,
    /// The SyGuS logic
    pub logic: Option<String>,
}

impl Conjecture {
    pub fn new() -> Self {
        Self {
            functions_to_synthesize: Vec::new(),
            universal_variables: Vec::new(),
            constraints: Vec::new(),
            assumptions: Vec::new(),
            signature: HashMap::new(),
            logic: None,
        }
    }
}

impl Default for Conjecture {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<(String, Sort)>,
    pub return_sort: Sort,
    pub grammar: Grammar,
}

#[derive(Debug)]
pub struct Grammar {
    pub declarations: Vec<(String, Sort)>,
    pub rules: Vec<(String, Sort, Vec<GTerm>)>,
}

#[derive(Debug)]
pub enum GTerm {
    // Constant(Sort),
    // Variable(Sort),
    BFTerm(BFTerm),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Sort {
    Identifier(String),
    Application(String, Vec<Sort>),
}

#[derive(Debug)]
pub enum BFTerm {
    Identifier(String),
    Literal(Literal),
    Application(String, Vec<BFTerm>),
    // Annotated(Box<BFTerm>, Vec<Attribute>), // Unimplemented
}

#[derive(Clone, Debug)]
pub enum Literal {
    Numeral(i64),
    Decimal(f64),
    Bool(bool),
    String(String),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Numeral(n) => write!(f, "{}", n),
            Literal::Decimal(n) => write!(f, "{}", n),
            Literal::Bool(true) => write!(f, "true"),
            Literal::Bool(false) => write!(f, "false"),
            Literal::String(s) => write!(f, "\"{}\"", s.replace('"', "\\\"")),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Term {
    Identifier(String),
    Literal(Literal),
    Application(String, Vec<Term>),
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Application(name, terms) => {
                write!(f, "({}", name)?;
                for term in terms {
                    write!(f, " {}", term)?;
                }
                write!(f, ")")
            }
            Term::Identifier(name) => write!(f, "{}", name),
            Term::Literal(lit) => write!(f, "{}", lit),
        }
    }
}

impl TryFrom<&Term> for Expr {
    type Error = ();

    fn try_from(value: &Term) -> Result<Self, Self::Error> {
        match value {
            Term::Identifier(name) => Ok(Expr::Var(name.to_owned())),
            Term::Literal(Literal::Bool(b)) => Ok(Expr::ConstBool(b.to_owned())),
            Term::Literal(Literal::Decimal(_)) => Err(()),
            Term::Literal(Literal::Numeral(n)) => Ok(Expr::ConstInt(n.to_owned())),
            Term::Literal(Literal::String(s)) => Ok(Expr::ConstStr(s.to_owned())),
            Term::Application(f, params) => Ok(Expr::Call(match f.as_str() {
                "str.++" if params.len() >= 2 => {
                    let a = params.get(0).unwrap().try_into()?;
                    let b = params.get(1).unwrap().try_into()?;
                    Box::new(Func::Append(a, b))
                }
                "str.len" if params.len() >= 1 => {
                    let a = params.get(0).unwrap().try_into()?;
                    Box::new(Func::StrLen(a))
                }
                "str.at" if params.len() >= 2 => {
                    let a = params.get(0).unwrap().try_into()?;
                    let b = params.get(1).unwrap().try_into()?;
                    Box::new(Func::StrAt(a, b))
                }
                "str.substr" if params.len() >= 3 => {
                    let a = params.get(0).unwrap().try_into()?;
                    let b = params.get(1).unwrap().try_into()?;
                    let c = params.get(2).unwrap().try_into()?;
                    Box::new(Func::SubStr(a, b, c))
                }
                "str.prefixof" if params.len() >= 2 => {
                    let a = params.get(0).unwrap().try_into()?;
                    let b = params.get(1).unwrap().try_into()?;
                    Box::new(Func::IsPre(a, b))
                }
                "str.suffixof" if params.len() >= 2 => {
                    let a = params.get(0).unwrap().try_into()?;
                    let b = params.get(1).unwrap().try_into()?;
                    Box::new(Func::IsPost(a, b))
                }
                "str.contains" if params.len() >= 2 => {
                    let a = params.get(0).unwrap().try_into()?;
                    let b = params.get(1).unwrap().try_into()?;
                    Box::new(Func::Contains(a, b))
                }
                "str.indexof" if params.len() >= 3 => {
                    let a = params.get(0).unwrap().try_into()?;
                    let b = params.get(1).unwrap().try_into()?;
                    let c = params.get(2).unwrap().try_into()?;
                    Box::new(Func::Index(a, b, c))
                }
                "str.replace" if params.len() >= 3 => {
                    let a = params.get(0).unwrap().try_into()?;
                    let b = params.get(1).unwrap().try_into()?;
                    let c = params.get(2).unwrap().try_into()?;
                    Box::new(Func::Replace(a, b, c))
                }
                "str.replace" if params.len() >= 3 => {
                    let a = params.get(0).unwrap().try_into()?;
                    let b = params.get(1).unwrap().try_into()?;
                    let c = params.get(2).unwrap().try_into()?;
                    Box::new(Func::Replace(a, b, c))
                }
                "str.replace_all" if params.len() >= 3 => {
                    let a = params.get(0).unwrap().try_into()?;
                    let b = params.get(1).unwrap().try_into()?;
                    let c = params.get(2).unwrap().try_into()?;
                    Box::new(Func::ReplaceAll(a, b, c))
                }
                "<=" if params.len() >= 2 => {
                    let a = params.get(0).unwrap().try_into()?;
                    let b = params.get(1).unwrap().try_into()?;
                    Box::new(Func::Leq(a, b))
                }
                "<=" if params.len() >= 2 => {
                    let a = params.get(0).unwrap().try_into()?;
                    let b = params.get(1).unwrap().try_into()?;
                    Box::new(Func::Leq(a, b))
                }
                ">=" if params.len() >= 2 => {
                    let a = params.get(0).unwrap().try_into()?;
                    let b = params.get(1).unwrap().try_into()?;
                    Box::new(Func::Geq(a, b))
                }
                "=" if params.len() >= 2 => {
                    let a = params.get(0).unwrap().try_into()?;
                    let b = params.get(1).unwrap().try_into()?;
                    Box::new(Func::Eql(a, b))
                }
                "+" if params.len() >= 2 => {
                    let a = params.get(0).unwrap().try_into()?;
                    let b = params.get(1).unwrap().try_into()?;
                    Box::new(Func::Add(a, b))
                }
                "-" if params.len() >= 2 => {
                    let a = params.get(0).unwrap().try_into()?;
                    let b = params.get(1).unwrap().try_into()?;
                    Box::new(Func::Min(a, b))
                }
                "-" if params.len() >= 1 => {
                    let a = params.get(0).unwrap().try_into()?;
                    Box::new(Func::NegI(a))
                }
                "*" if params.len() >= 2 => {
                    let a = params.get(0).unwrap().try_into()?;
                    let b = params.get(1).unwrap().try_into()?;
                    Box::new(Func::Mult(a, b))
                }
                "div" if params.len() >= 2 => {
                    let a = params.get(0).unwrap().try_into()?;
                    let b = params.get(1).unwrap().try_into()?;
                    Box::new(Func::Div(a, b))
                }
                "abs" if params.len() >= 1 => {
                    let a = params.get(0).unwrap().try_into()?;
                    Box::new(Func::Abs(a))
                }
                "mod" if params.len() >= 2 => {
                    let a = params.get(0).unwrap().try_into()?;
                    let b = params.get(1).unwrap().try_into()?;
                    Box::new(Func::Mod(a, b))
                }
                "and" if params.len() >= 2 => {
                    let a = params.get(0).unwrap().try_into()?;
                    let b = params.get(1).unwrap().try_into()?;
                    Box::new(Func::And(a, b))
                }
                "or" if params.len() >= 2 => {
                    let a = params.get(0).unwrap().try_into()?;
                    let b = params.get(1).unwrap().try_into()?;
                    Box::new(Func::Or(a, b))
                }
                "str.<=" if params.len() >= 2 => {
                    let a = params.get(0).unwrap().try_into()?;
                    let b = params.get(1).unwrap().try_into()?;
                    Box::new(Func::LexLeq(a, b))
                }
                "int.to.str" if params.len() >= 1 => {
                    let a = params.get(0).unwrap().try_into()?;
                    Box::new(Func::IntToStr(a))
                }
                "str.to.int" if params.len() >= 1 => {
                    let a = params.get(0).unwrap().try_into()?;
                    Box::new(Func::StrToInt(a))
                }
                _ => Err(())?,
            })),
        }
    }
}

pub fn parse_sygus_file(file: &str) -> Result<Conjecture, Box<Error<Rule>>> {
    let sygus = SygusParser::parse(Rule::sygus, file)?.next().unwrap();
    Ok(parse_conjecture(sygus))
}

fn parse_conjecture(pair: Pair<Rule>) -> Conjecture {
    let mut conjecture = Conjecture::new();

    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::set_logic => {
                conjecture.logic = Some(pair.into_inner().next().unwrap().as_str().to_string())
            }
            Rule::synth_fun => {
                let mut inner_rules = pair.into_inner();
                let name = inner_rules.next().unwrap().as_str().to_string();
                let parameters: Vec<(String, Sort)> = inner_rules
                    .next()
                    .unwrap()
                    .into_inner()
                    .map(|pair| {
                        let mut inner_rules = pair.into_inner();
                        let name = inner_rules.next().unwrap().as_str().to_string();
                        let sort = parse_sort(inner_rules.next().unwrap());
                        (name, sort)
                    })
                    .collect();
                let return_sort = parse_sort(inner_rules.next().unwrap());
                let grammar = parse_grammar(inner_rules.next().unwrap());
                conjecture.functions_to_synthesize.push(Function {
                    name,
                    parameters,
                    return_sort,
                    grammar,
                });
            }
            _ => {}
        }
    }

    conjecture
}

fn parse_sort(pair: Pair<Rule>) -> Sort {
    match pair.as_rule() {
        Rule::sort_identifier => {
            Sort::Identifier(pair.into_inner().next().unwrap().as_str().to_string())
        }
        Rule::sort_application => {
            let mut inner_rules = pair.into_inner();
            let identifier = inner_rules.next().unwrap().as_str().to_string();
            let sorts = inner_rules.map(parse_sort).collect();
            Sort::Application(identifier, sorts)
        }
        _ => unimplemented!("Unsupported sort: {:#?}", pair),
    }
}

fn parse_grammar(pair: Pair<Rule>) -> Grammar {
    let mut declarations = Vec::new();
    let mut rules: Vec<(String, Sort, Vec<GTerm>)> = Vec::new();

    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::sorted_var => {
                let mut inner_rules = pair.into_inner();
                let name = inner_rules.next().unwrap().as_str().to_string();
                let sort = parse_sort(inner_rules.next().unwrap());
                declarations.push((name, sort));
            }
            Rule::grouped_rule_list => {
                let mut inner_rules = pair.into_inner();
                let name = inner_rules.next().unwrap().as_str().to_string();
                let sort = parse_sort(inner_rules.next().unwrap());
                let terms: Vec<GTerm> = inner_rules.map(parse_g_term).collect();
                rules.push((name, sort, terms));
            }
            _ => unimplemented!("Unsupported rule in grammar: {:#?}", pair),
        }
    }

    Grammar {
        declarations,
        rules,
    }
}

fn parse_g_term(pair: Pair<Rule>) -> GTerm {
    let pair = pair.into_inner().next().unwrap();
    match pair.as_rule() {
        // Rule::constant => GTerm::Constant(parse_sort(pair.into_inner().next().unwrap())),
        // Rule::variable => GTerm::Variable(parse_sort(pair.into_inner().next().unwrap())),
        Rule::bf_term => GTerm::BFTerm(parse_bf_term(pair)),
        _ => unimplemented!("Unsupported g_term: {:#?}", pair),
    }
}

fn parse_bf_term(pair: Pair<Rule>) -> BFTerm {
    let pair = pair.into_inner().next().unwrap();
    match pair.as_rule() {
        // Rule::bf_attributes => unimplemented!("Unsupported bf_attributes"),
        Rule::bf_application => {
            let mut inner_rules = pair.into_inner();
            let identifier = inner_rules.next().unwrap().as_str().to_string();
            let terms: Vec<BFTerm> = inner_rules.map(parse_bf_term).collect();
            BFTerm::Application(identifier, terms)
        }
        Rule::literal => BFTerm::Literal(parse_literal(pair)),
        Rule::identifier => BFTerm::Identifier(pair.as_str().to_string()),
        _ => unimplemented!("Unsupported bf_term: {:#?}", pair),
    }
}

fn parse_literal(pair: Pair<Rule>) -> Literal {
    let pair = pair.into_inner().next().unwrap();
    match pair.as_rule() {
        Rule::numeral => Literal::Numeral(pair.as_str().to_string().parse::<i64>().unwrap()),
        Rule::decimal => Literal::Decimal(pair.as_str().to_string().parse::<f64>().unwrap()),
        Rule::bool_const => Literal::Bool(pair.as_str().to_string().parse::<bool>().unwrap()),
        Rule::string_const => {
            let literal = pair.as_str().to_string();
            Literal::String(
                literal
                    .substring(1, literal.len() - 1)
                    .replace("\"\"", "\"")
                    .to_string(),
            )
        }
        _ => unimplemented!("Unsupported literal: {:#?}", pair),
    }
}
