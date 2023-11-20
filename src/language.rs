use regex::Regex;
use std::collections::HashMap;

pub type Term = Result<Expr, String>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Func {
    Append(Expr, Expr),
    StrLen(Expr),
    StrAt(Expr, Expr),
    SubStr(Expr, Expr, Expr),
    IsPre(Expr, Expr),
    IsPost(Expr, Expr),
    Contains(Expr, Expr),
    Index(Expr, Expr, Expr),
    Replace(Expr, Expr, Expr),
    ReplaceAll(Expr, Expr, Expr),

    Leq(Expr, Expr),
    Geq(Expr, Expr),
    Eql(Expr, Expr),
    Add(Expr, Expr),
    Min(Expr, Expr),
    Mult(Expr, Expr),
    Div(Expr, Expr),
    Abs(Expr),
    Mod(Expr, Expr),
    NegI(Expr),

    NegB(Expr),
    And(Expr, Expr),
    Or(Expr, Expr),
    LexEq(Expr, Expr),
    LexLeq(Expr, Expr),
    LexGeq(Expr, Expr),

    StrToInt(Expr),
    IntToStr(Expr),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RegLang {
    Nil,
    All,
    AllChar,
    RCall(Box<RegFun>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RegFun {
    Sequence(RegLang, RegLang),
    Union(RegLang, RegLang),
    Inter(RegLang, RegLang),
    Star(RegLang),
    One(RegLang),
    Opt(RegLang),
    Range(Expr, Expr),
    FromStr(Expr),
}

pub trait Translate {
    fn translate(&self) -> Regex;
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Expr {
    ConstStr(String),
    ConstInt(i64),
    ConstBool(bool),
    Var(String),
    Call(Box<Func>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
}

pub trait Eval {
    fn eval(self, env: &HashMap<String, Expr>) -> Term;
}

impl Expr {
    pub fn call(arg: Func) -> Expr {
        Expr::Call(Box::new(arg))
    }
}
