pub use crate::language::{Eval, Expr, Term, Func, RegLang, RegFun, Translate};
use std::collections::HashMap;
use std::result::Result;
use std::rc::Rc;
use regex::Regex;



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
                         _ => Err(String::from("If condition doesn't resolve to Boolean"))
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

            Self::Leq(arg1, arg2) => {

                let arg1_evaled  = Ok::<Expr, String>(arg1.clone()).eval(env);
                let arg2_evaled  = Ok::<Expr, String>(arg2.clone()).eval(env);

                match(&arg1_evaled, &arg2_evaled) {
                    
                    (Ok(Expr::ConstInt(a1)), Ok(Expr::ConstInt(a2))) => Ok(Expr::ConstBool(a1 <= a2)),
                    _ => {

                        let j = format!("Leq: invalid argument: arg1 = {:?}, arg2 = {:?}", arg1_evaled,   arg2_evaled );

                        Err(j)
                    }

                }

            }


            Self::Geq(arg1, arg2) => {

                let arg1_evaled  = Ok::<Expr, String>(arg1.clone()).eval(env);
                let arg2_evaled  = Ok::<Expr, String>(arg2.clone()).eval(env);

                match(&arg1_evaled, &arg2_evaled) {
                    
                    (Ok(Expr::ConstInt(a1)), Ok(Expr::ConstInt(a2))) => Ok(Expr::ConstBool(a1 >= a2)),
                    _ => {

                        let j = format!("Geq: invalid argument: arg1 = {:?}, arg2 = {:?}", arg1_evaled,   arg2_evaled );

                        Err(j)
                    }

                }

            }

            Self::Eql(arg1, arg2) => {

                let arg1_evaled  = Ok::<Expr, String>(arg1.clone()).eval(env);
                let arg2_evaled  = Ok::<Expr, String>(arg2.clone()).eval(env);

                match(&arg1_evaled, &arg2_evaled) {
                    
                    (Ok(Expr::ConstInt(a1)), Ok(Expr::ConstInt(a2))) => Ok(Expr::ConstBool(a1 == a2)),
                    _ => {

                        let j = format!("Eql: invalid argument: arg1 = {:?}, arg2 = {:?}", arg1_evaled,   arg2_evaled );

                        Err(j)
                    }

                }

            }

            Self::Add(arg1, arg2) => {

                let arg1_evaled  = Ok::<Expr, String>(arg1.clone()).eval(env);
                let arg2_evaled  = Ok::<Expr, String>(arg2.clone()).eval(env);

                match(&arg1_evaled, &arg2_evaled) {
                    
                    (Ok(Expr::ConstInt(a1)), Ok(Expr::ConstInt(a2))) => Ok(Expr::ConstInt(a1 + a2)),
                    _ => {

                        let j = format!("Add: invalid argument: arg1 = {:?}, arg2 = {:?}", arg1_evaled,   arg2_evaled );

                        Err(j)
                    }

                }

            }

            Self::Min(arg1, arg2) => {

                let arg1_evaled  = Ok::<Expr, String>(arg1.clone()).eval(env);
                let arg2_evaled  = Ok::<Expr, String>(arg2.clone()).eval(env);

                match(&arg1_evaled, &arg2_evaled) {
                    
                    (Ok(Expr::ConstInt(a1)), Ok(Expr::ConstInt(a2))) => Ok(Expr::ConstInt(a1 - a2)),
                    _ => {

                        let j = format!("Min: invalid argument: arg1 = {:?}, arg2 = {:?}", arg1_evaled,   arg2_evaled );

                        Err(j)
                    }

                }

            }

            Self::Mult(arg1, arg2) => {

                let arg1_evaled  = Ok::<Expr, String>(arg1.clone()).eval(env);
                let arg2_evaled  = Ok::<Expr, String>(arg2.clone()).eval(env);

                match(&arg1_evaled, &arg2_evaled) {
                    
                    (Ok(Expr::ConstInt(a1)), Ok(Expr::ConstInt(a2))) => Ok(Expr::ConstInt(a1 * a2)),
                    _ => {

                        let j = format!("Mult: invalid argument: arg1 = {:?}, arg2 = {:?}", arg1_evaled,   arg2_evaled );

                        Err(j)
                    }

                }

            }

            Self::Mult(arg1, arg2) => {

                let arg1_evaled  = Ok::<Expr, String>(arg1.clone()).eval(env);
                let arg2_evaled  = Ok::<Expr, String>(arg2.clone()).eval(env);

                match(&arg1_evaled, &arg2_evaled) {
                    
                    (Ok(Expr::ConstInt(a1)), Ok(Expr::ConstInt(a2))) => Ok(Expr::ConstInt(a1 * a2)),
                    _ => {

                        let j = format!("Mult: invalid argument: arg1 = {:?}, arg2 = {:?}", arg1_evaled,   arg2_evaled );

                        Err(j)
                    }

                }

            }


            Self::Div(arg1, arg2) => {

                let arg1_evaled  = Ok::<Expr, String>(arg1.clone()).eval(env);
                let arg2_evaled  = Ok::<Expr, String>(arg2.clone()).eval(env);

                match(&arg1_evaled, &arg2_evaled) {
                    
                    (Ok(Expr::ConstInt(a1)), Ok(Expr::ConstInt(a2))) => Ok(Expr::ConstInt(a1 / a2)),
                    _ => {

                        let j = format!("Div: invalid argument: arg1 = {:?}, arg2 = {:?}", arg1_evaled,   arg2_evaled );

                        Err(j)
                    }

                }

            }
            
            Self::Div(arg1, arg2) => {

                let arg1_evaled  = Ok::<Expr, String>(arg1.clone()).eval(env);
                let arg2_evaled  = Ok::<Expr, String>(arg2.clone()).eval(env);

                match(&arg1_evaled, &arg2_evaled) {
                    
                    (Ok(Expr::ConstInt(a1)), Ok(Expr::ConstInt(a2))) => Ok(Expr::ConstInt(a1 / a2)),
                    _ => {

                        let j = format!("Div: invalid argument: arg1 = {:?}, arg2 = {:?}", arg1_evaled,   arg2_evaled );

                        Err(j)
                    }

                }

            }

            Self::Abs(arg1) => {

                let arg1_evaled  = Ok::<Expr, String>(arg1.clone()).eval(env);
                

                match(&arg1_evaled) {
                    
                    Ok(Expr::ConstInt(a1)) => Ok(Expr::ConstInt(a1.abs())),
                    _ => {

                        let j = format!("Abs: invalid argument: arg1 = {:?}", arg1_evaled );

                        Err(j)
                    }

                }

            }


            Self::Mod(arg1, arg2) => {

                let arg1_evaled  = Ok::<Expr, String>(arg1.clone()).eval(env);
                let arg2_evaled  = Ok::<Expr, String>(arg2.clone()).eval(env);

                match(&arg1_evaled, &arg2_evaled) {
                    
                    (Ok(Expr::ConstInt(a1)), Ok(Expr::ConstInt(a2))) => Ok(Expr::ConstInt(a1 % a2)),
                    _ => {

                        let j = format!("Mod: invalid argument: arg1 = {:?}, arg2 = {:?}", arg1_evaled,   arg2_evaled );

                        Err(j)
                    }

                }

            }

            Self::NegI(arg1) => {

                let arg1_evaled  = Ok::<Expr, String>(arg1.clone()).eval(env);
                

                match(&arg1_evaled) {
                    
                    Ok(Expr::ConstInt(a1)) => Ok(Expr::ConstInt(a1*(-1)))   ,
                    _ => {

                        let j = format!("NegI: invalid argument: arg1 = {:?}", arg1_evaled );

                        Err(j)
                    }

                }

            }

            Self::NegB(arg1) => {

                let arg1_evaled  = Ok::<Expr, String>(arg1.clone()).eval(env);
                

                match(&arg1_evaled) {
                    
                    Ok(Expr::ConstBool(a1)) => Ok(Expr::ConstBool(!a1))   ,
                    _ => {

                        let j = format!("NegI: invalid argument: arg1 = {:?}", arg1_evaled );

                        Err(j)
                    }

                }

            }



            Self::And(arg1, arg2) => {

                let arg1_evaled  = Ok::<Expr, String>(arg1.clone()).eval(env);
                let arg2_evaled  = Ok::<Expr, String>(arg2.clone()).eval(env);

                match(&arg1_evaled, &arg2_evaled) {
                    
                    (Ok(Expr::ConstBool(a1)), Ok(Expr::ConstBool(a2))) => Ok(Expr::ConstBool(*a1 && *a2)),
                    _ => {

                        let j = format!("And: invalid argument: arg1 = {:?}, arg2 = {:?}", arg1_evaled,   arg2_evaled );

                        Err(j)
                    }

                }

            }

            Self::Or(arg1, arg2) => {

                let arg1_evaled  = Ok::<Expr, String>(arg1.clone()).eval(env);
                let arg2_evaled  = Ok::<Expr, String>(arg2.clone()).eval(env);

                match(&arg1_evaled, &arg2_evaled) {
                    
                    (Ok(Expr::ConstBool(a1)), Ok(Expr::ConstBool(a2))) => Ok(Expr::ConstBool(*a1 || *a2)),
                    _ => {

                        let j = format!("Or: invalid argument: arg1 = {:?}, arg2 = {:?}", arg1_evaled,   arg2_evaled );

                        Err(j)
                    }

                }

            }


            Self::StrLen(arg1) => {

                let arg1_evaled  = Ok::<Expr, String>(arg1.clone()).eval(env);
                

                match &arg1_evaled {
                    
                    Ok(Expr::ConstStr(a1)) => Ok(Expr::ConstInt(a1.len() as i32)),
                    _ => {

                        let j = format!("StrLen: invalid argument: arg1 = {:?}", arg1_evaled );

                        Err(j)
                    }

                }

            }

            
            Self::StrAt(arg1, arg2) => {

                let arg1_evaled  = Ok::<Expr, String>(arg1.clone()).eval(env);
                let arg2_evaled  = Ok::<Expr, String>(arg2.clone()).eval(env);
            
                
                match(&arg1_evaled, &arg2_evaled) {
                    
                    (Ok(Expr::ConstStr(a1)), Ok(Expr::ConstInt(a2))) => {

                        if(a2 < &(a1.len() as i32)) {

                            Ok(Expr::ConstStr(String::from(a1.chars().nth(*a2 as usize).unwrap())))   
                            
                        }else{
                            
                            Ok(Expr::ConstStr(String::from("")))

                        } 
                    },
                    _ => {

                        let j = format!("StrLen: invalid argument: arg1 = {:?} arg2 = {:?}", arg1_evaled, arg2_evaled );

                        Err(j)
                    }

                }

            }


            Self::SubStr(arg1, arg2, arg3) => {

                let arg1_evaled  = Ok::<Expr, String>(arg1.clone()).eval(env);
                let arg2_evaled  = Ok::<Expr, String>(arg2.clone()).eval(env);
                let arg3_evaled  = Ok::<Expr, String>(arg3.clone()).eval(env);
                
                match(&arg1_evaled, &arg2_evaled, &arg3_evaled) {
                    
                    (Ok(Expr::ConstStr(a1)), Ok(Expr::ConstInt(a2)), Ok(Expr::ConstInt(a3))) => {

                        if((a3 > &0) && (a2 < &(a1.len() as i32)) && (a2 >= &0)) { //if either the number to take is 0 or less or the index is out of bounds return mt string

                            if((a2 + a3 -1) < (a1.len() as i32)) { // if index plus number to take is greater than length of string decide on number to take
                            
                            Ok(Expr::ConstStr(a1.chars().skip(*a2 as usize).take((a3 - a2) as usize).collect::<String>()))   

                            } else {

                                let j = &(a1.len() as i32) - a2;
                                Ok(Expr::ConstStr(a1.chars().skip(*a2 as usize).take((j) as usize).collect::<String>()))

                            }
                            
                        }else{
                            
                            Ok(Expr::ConstStr(String::from("")))

                        } 
                    },
                    _ => {

                        let j = format!("SubStr: invalid argument: arg1 = {:?} arg2 = {:?}, arg3 = {:?}", arg1_evaled, arg2_evaled, arg3_evaled );

                        Err(j)
                    }

                }

            }


            Self::IsPre(arg1, arg2) => {

                let arg1_evaled  = Ok::<Expr, String>(arg1.clone()).eval(env);
                let arg2_evaled  = Ok::<Expr, String>(arg2.clone()).eval(env);
            
                
                match(&arg1_evaled, &arg2_evaled) {
                    
                    (Ok(Expr::ConstStr(a1)), Ok(Expr::ConstStr(a2))) => {

                        let l1  = a1.len();
                        let l2  = a2.len();

                        if(l1 <= l2) {

                            let p = a2.chars().take(l1).collect::<String>();
                            
                            Ok(Expr::ConstBool(p == *a1))
                            
                        }else{
                            
                            Ok(Expr::ConstBool(false))

                        } 
                    },
                    _ => {

                        let j = format!("IsPre: invalid argument: arg1 = {:?} arg2 = {:?}", arg1_evaled, arg2_evaled );

                        Err(j)
                    }

                }

            }

            Self::IsPost(arg1, arg2) => {

                let arg1_evaled  = Ok::<Expr, String>(arg1.clone()).eval(env);
                let arg2_evaled  = Ok::<Expr, String>(arg2.clone()).eval(env);
            
                
                match(&arg1_evaled, &arg2_evaled) {
                    
                    (Ok(Expr::ConstStr(a1)), Ok(Expr::ConstStr(a2))) => {

                        let l1  = (a1.len() as i32);
                        let l2  = (a2.len() as i32);

                        if(l1 <= l2) {

                            let m = (l2 - l1 -1);
                            let p = a2.chars().skip(m as usize).take(l1 as usize).collect::<String>();
                            
                            Ok(Expr::ConstBool(p == *a1))
                            
                        }else{
                            
                            Ok(Expr::ConstBool(false))

                        } 
                    },
                    _ => {

                        let j = format!("IsPost: invalid argument: arg1 = {:?} arg2 = {:?}", arg1_evaled, arg2_evaled );

                        Err(j)
                    }

                }

            }

            Self::Contains(arg1, arg2) => {

                let arg1_evaled  = Ok::<Expr, String>(arg1.clone()).eval(env);
                let arg2_evaled  = Ok::<Expr, String>(arg2.clone()).eval(env);
            
                
                match(&arg1_evaled, &arg2_evaled) {
                    
                    (Ok(Expr::ConstStr(a1)), Ok(Expr::ConstStr(a2))) => {

                        Ok(Expr::ConstBool(a1.contains(a2)))
                    },
                    _ => {

                        let j = format!("Contains: invalid argument: arg1 = {:?} arg2 = {:?}", arg1_evaled, arg2_evaled );

                        Err(j)
                    }

                }

            }


            Self::Index(arg1, arg2, arg3) => {

                let arg1_evaled  = Ok::<Expr, String>(arg1.clone()).eval(env);
                let arg2_evaled  = Ok::<Expr, String>(arg2.clone()).eval(env);
                let arg3_evaled  = Ok::<Expr, String>(arg3.clone()).eval(env);
                
                match(&arg1_evaled, &arg2_evaled, &arg3_evaled) {
                    
                    (Ok(Expr::ConstStr(a1)), Ok(Expr::ConstStr(a2)), Ok(Expr::ConstInt(a3))) => {
                        
                        let l1 = a1.len() as i32;
                        let l2 = a2.len() as i32;

                        if(a3 < &l1){

                            if(l2 == 0) {

                                Ok(Expr::ConstInt(*a3))

                            }else{


                                let b1 = a1.chars().skip(*a3 as usize).take((l1 - a3 - 1) as usize).collect::<String>();
                                
                                if let Some(k) = b1.find(a2) {

                                    Ok(Expr::ConstInt(k as i32))     

                                } else {

                                    Ok(Expr::ConstInt(-1))

                                }

                        }


                        }else{

                            let j = format!("Index: arg3 out of bounds: arg1 = {:?}, arg3 = {:?}", arg1_evaled, arg3_evaled );
                            Err(j)
                        }

                        
                    },
                    _ => {

                        let j = format!("Index: invalid argument: arg1 = {:?} arg2 = {:?}, arg3 = {:?}", arg1_evaled, arg2_evaled, arg3_evaled );

                        Err(j)
                    }

                }

            }

            Self::Replace(arg1, arg2, arg3) => {

                let arg1_evaled  = Ok::<Expr, String>(arg1.clone()).eval(env);
                let arg2_evaled  = Ok::<Expr, String>(arg2.clone()).eval(env);
                let arg3_evaled  = Ok::<Expr, String>(arg3.clone()).eval(env);
                
                
                match(&arg1_evaled, &arg2_evaled, &arg3_evaled) {
                    
                    (Ok(Expr::ConstStr(a1)), Ok(Expr::ConstStr(a2)), Ok(Expr::ConstStr(a3))) => {

                        let l1 = (a2.len() as i32);
                        if(l1 == 0){

                            Func::Append(arg3.clone(), arg1.clone()).eval(env).clone()

                        } else {

                            Ok(Expr::ConstStr(a1.replacen(a2, a3, 1)))

                        }
                        
                    },
                    _ => {

                        let j = format!("Replace: invalid argument: arg1 = {:?} arg2 = {:?}, arg3 = {:?}", arg1_evaled, arg2_evaled, arg3_evaled );

                        Err(j)
                    }

                }

            }

            Self::ReplaceAll(arg1, arg2, arg3) => {

                let arg1_evaled  = Ok::<Expr, String>(arg1.clone()).eval(env);
                let arg2_evaled  = Ok::<Expr, String>(arg2.clone()).eval(env);
                let arg3_evaled  = Ok::<Expr, String>(arg3.clone()).eval(env);
                
                
                match(&arg1_evaled, &arg2_evaled, &arg3_evaled) {
                    
                    (Ok(Expr::ConstStr(a1)), Ok(Expr::ConstStr(a2)), Ok(Expr::ConstStr(a3))) => {

                        let l1 = (a2.len() as i32);
                        if(l1 == 0){
                                
                            Func::Append(arg3.clone(), arg1.clone()).eval(env).clone()

                        } else {

                            Ok(Expr::ConstStr(a1.replace(a2, a3)))

                        }
                        
                    },
                    _ => {

                        let j = format!("ReplaceAll: invalid argument: arg1 = {:?} arg2 = {:?}, arg3 = {:?}", arg1_evaled, arg2_evaled, arg3_evaled );

                        Err(j)
                    }

                }

            }


            Self::LexEq(arg1, arg2) => {

                let arg1_evaled  = Ok::<Expr, String>(arg1.clone()).eval(env);
                let arg2_evaled  = Ok::<Expr, String>(arg2.clone()).eval(env);
            
                
                match(&arg1_evaled, &arg2_evaled) {
                    
                    (Ok(Expr::ConstStr(a1)), Ok(Expr::ConstStr(a2))) => {

                        Ok(Expr::ConstBool(a1 == a2))
                    },
                    _ => {

                        let j = format!("LexEq: invalid argument: arg1 = {:?} arg2 = {:?}", arg1_evaled, arg2_evaled );

                        Err(j)
                    }

                }

            }

            Self::LexLeq(arg1, arg2) => {

                let arg1_evaled  = Ok::<Expr, String>(arg1.clone()).eval(env);
                let arg2_evaled  = Ok::<Expr, String>(arg2.clone()).eval(env);
            
                
                match(&arg1_evaled, &arg2_evaled) {
                    
                    (Ok(Expr::ConstStr(a1)), Ok(Expr::ConstStr(a2))) => {

                        Ok(Expr::ConstBool(a1 <= a2))
                    },
                    _ => {

                        let j = format!("LexLeq: invalid argument: arg1 = {:?} arg2 = {:?}", arg1_evaled, arg2_evaled );

                        Err(j)
                    }

                }

            }

            Self::LexGeq(arg1, arg2) => {

                let arg1_evaled  = Ok::<Expr, String>(arg1.clone()).eval(env);
                let arg2_evaled  = Ok::<Expr, String>(arg2.clone()).eval(env);
            
                
                match(&arg1_evaled, &arg2_evaled) {
                    
                    (Ok(Expr::ConstStr(a1)), Ok(Expr::ConstStr(a2))) => {

                        Ok(Expr::ConstBool(a1 >= a2))
                    },
                    _ => {

                        let j = format!("LexGeq: invalid argument: arg1 = {:?} arg2 = {:?}", arg1_evaled, arg2_evaled );

                        Err(j)
                    }

                }

            }

        




            



        }
    }
}

impl Translate for RegLang {

    fn translate(&self) -> Regex {

        match self {

            RegLang::NIL => Regex::new(r"[^$]").unwrap(),
            RegLang::ALL => Regex::new(r"^[ -~]+$").unwrap(),
            RegLang::ALLCHAR => Regex::new(r"(?!\A)").unwrap(),   
            RegLang::RCALL(j) => (*j).clone().translate(),  
            


        }


    }
}

impl Translate for RegFun {

    fn translate(&self) -> Regex {

        match self {
        
            RegFun::SEQUENCE(arg1, arg2) => {

                let eval1 = arg1.translate();
                let eval2 = arg2.translate();

                Regex::new(&format!("^ {} {} $", eval1, eval2)).unwrap()
            }

            RegFun::UNION(arg1, arg2) => {

                let eval1 = arg1.translate();
                let eval2 = arg2.translate();

                Regex::new(&format!("^ {}|{} $", eval1, eval2)).unwrap()

            }

            RegFun::INTER(arg1, arg2) => {

                let eval1 = arg1.translate();
                let eval2 = arg2.translate();

                Regex::new(&format!("^ (?= {})(?= {}) $", eval1, eval2)).unwrap()

            }

            RegFun::STAR(arg1) => {

                let eval1 = arg1.translate();
                

                Regex::new(&format!("({})*", eval1)).unwrap()

            }

            RegFun::ONE(arg1) => {

                let eval1 = arg1.translate();
                

                Regex::new(&format!("({})+", eval1)).unwrap()

            }

            RegFun::OPT(arg1) => {

                let eval1 = arg1.translate();
                

                Regex::new(&format!("{} | \"\"", eval1)).unwrap()

            }

            RegFun::FROMSTR(arg1) => {

                let j = HashMap::<String, Term>::new();
                let eval1 = Ok::<Expr, String>(arg1.clone()).eval(&j).clone();

                match eval1 {

                    Ok(Expr::ConstStr(a)) => Regex::new(&format!("{}", a)).unwrap(),
                    _ => RegLang::NIL.translate(),

                }
                
                


            }

            

            RegFun::RANGE(arg1, arg2) => {
                
                let j = HashMap::<String, Term>::new();
                let eval1 = Ok::<Expr, String>(arg1.clone()).eval(&j).clone();
                let eval2 = Ok::<Expr, String>(arg2.clone()).eval(&j).clone();

                match(eval1, eval2)  {

                    (Ok(Expr::ConstStr(a)), Ok(Expr::ConstStr(b))) => {

                        let l1 = a.len() as i32;
                        let l2 = a.len() as i32;

                        if(l1 == l2 && l1 == 1){

                            Regex::new(&format!("[{}-{}]", l1, l2)).unwrap() // I know this was supposed to take a range between 
                            //any two same size strings but I couldn't figure that out just yet


                        }else{

                            RegLang::NIL.translate()
                        }

                    }
                    _ => RegLang::NIL.translate()


                }


            }




        }

        

    }

}


    