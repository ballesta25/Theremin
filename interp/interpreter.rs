pub use crate::language::{Eval, Expr, Term, Func};
use std::collections::HashMap;
use std::result::Result;
use std::rc::Rc;


impl Eval for Term {
    fn eval(&self, env: &HashMap<String, Term>) -> Term {
        match self {
            Ok(s) => match s {
                Expr::ConstStr(j) => Ok(s.clone()),
                Expr::ConstInt(j) => Ok(s.clone()),
                Expr::ConstBool(j) => Ok(s.clone()),
                Expr::Var(x) => {
                    let j = env.get(x);
                    match j {
                        Some(i) => i.clone(),
                        None => Err(String::from("Uninstatiated Variable")),
                    }
                }
                Expr::Call(p) => (p.eval(env)).clone(),
                    
                //Self::Call(call) => call.eval(env),

                Expr::If(c, t, e) => {

                    let condition = Ok::<Expr, String>(*c.clone());
                    let then = Ok::<Expr, String>(*t.clone());
                    let els = Ok::<Expr, String>(*e.clone());

                    let cond = condition.eval(env);
                    let th = then.eval(env);
                    let el = els.eval(env);     
                    

                    match(cond, th, el){
                        
                         (Ok(Expr::ConstBool(j)), Ok(p), Ok(q)) => match j {
                            true => Ok(p),
                            _ => Ok(q),
                         }
                         _ => Err(String::from("Doesn't resolve to Boolean"))
                    }


                }

                //Self::If(cond, then, otherwise) => {
                    //let cond_evaled = cond.eval(env);
                    //let then_evaled = then.eval(env);
                    //let otherwise_evaled = otherwise.eval(env);
                    //match (cond_evaled, then_evaled, otherwise_evaled) {
                        //(Ok(c), Ok(t), Ok(o)) => Self::eval_if(c, t, o),
                        //_ => Err("if: invalid argument"),
                
                }

            
            Err(j) => panic!("Error:{}", j),
        }
        }
            //_ => unreachable!(),
        }
   // }
//}

impl Eval for Func {

    fn eval(&self, env: &HashMap<String, Term>) -> Term {
        match self {
            Self::Append(arg1, arg2) => {
                let arg1_evaled = Ok::<Expr, String>(arg1.clone()).eval(env);
                let arg2_evaled = Ok::<Expr, String>(arg2.clone()).eval(env);
                
                match (&arg1_evaled, &arg2_evaled) {
                    (Ok(Expr::ConstStr(a1)), Ok(Expr::ConstStr(a2))) => Ok(Expr::ConstStr(String::from(a1.clone() + a2))),
                    _ =>  {
                        let j = format!("append: invalid argument: arg1 = {:?}, arg2 = {:?}", arg1_evaled,   arg2_evaled );

                        Err(j)
                    }
                
                }


            }
        }
    }
}
    