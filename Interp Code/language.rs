
use std::fmt;
use std::fmt::Display;
use std::rc::Rc;
use std::result::Result;
use std::collections::HashMap;
use regex::Regex;


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



}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RegLang{

    NIL, 
    ALL,
    ALLCHAR,
    RCALL(Box<RegFun>),
    

}


#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RegFun{

    SEQUENCE(RegLang, RegLang),
    UNION(RegLang, RegLang),
    INTER(RegLang, RegLang),
    STAR(RegLang),
    ONE(RegLang),
    OPT(RegLang),
    RANGE(Expr, Expr),
    FROMSTR(Expr),
    

    
    /*((Constant RegLan)
                 (Variable RegLan)
                 re.none
                 re.all
                 re.allchar
                
                 (re.++ y_rl y_rl)
                 (re.union y_rl y_rl)
                 (re.inter y_rl y_rl)
                 (re.* y_rl)
                 (re.+ y_rl)
                 (re.opt y_rl)
                 (re.range y_str y_str))) */


}

pub trait Translate {

    fn translate(&self) -> Regex; 

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