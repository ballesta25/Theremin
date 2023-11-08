use MyOwn::interpreter::{Eval};
use std::collections::HashMap;
use MyOwn::language::{Term, Expr, Func};

fn ezB(t: Expr) -> Box<Expr> {

    Box::new(t)
}

fn ezB2(t: Func) -> Box<Func> {

    Box::new(t)
}





fn main() {
    


    // let mut map = HashMap::<String, Term>::new();
    // map.entry(String::from("Arg0")).or_insert(Ok(Expr::ConstStr(String::from("World"))));
    // let expr : Term = Ok(Expr::Call(ezB(Func::Append(  Expr::ConstStr(String::from("Damn this ")) , Expr::Var(String::from("Arg0"))))));
    // let j = expr.eval(&map);
    // println!("{:?}", j);

    //let ex : Term = Ok(Expr::If(ezB(Expr::ConstBool(false)), ezB(Expr::ConstStr(String::from("Hello"))), ezB(Expr::ConstStr(String::from("Dummy")))));

    let ex : Term = Ok(Expr::Call(ezB2(Func::Geq(Expr::ConstInt(12), Expr::ConstInt(34)))));
    let mut map = HashMap::<String, Term>::new();
    let j = ex.eval(&map);
    println!("{:?}", j);
    



}   
