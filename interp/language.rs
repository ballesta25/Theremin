
use std::fmt;
use std::fmt::Display;
use std::rc::Rc;
use std::result::Result;
use std::collections::HashMap;

pub type Term = Result<Expr, String>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Func {
    Append(Expr, Expr),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Expr {
    ConstStr(String),
    ConstInt(i32),
    ConstBool(bool),
    Var(String),
    Call(Box<Func>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),

}

// impl Display for Expr {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Self::ConstStr(c) => write!(f, "{}", c),
//             // Self::Var(var) => write!(f, "{}", var),
//             // Self::Call(func) => write!(f, "{}", func),
//             // Self::If(cond, then, otherwise) => write!(f, "(if {} {} {})", cond, then, otherwise),
        
//         }
//     }
// }

// impl Display for Func {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Self::Append(arg1, arg2) => write!(f, "(append {} {})", arg1, arg2),
//         }
//     }
// }

pub trait Eval {

    fn eval(&self, env: &HashMap<String, Term>) -> Term;

}