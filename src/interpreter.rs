pub use crate::language::{Eval, Expr, Func, RegFun, RegLang, Term, Translate};
use regex::Regex;
use std::collections::HashMap;

impl Eval for Expr {
    fn eval(self, env: &HashMap<String, Expr>) -> Term {
        match self {
            Expr::ConstStr(_) | Expr::ConstInt(_) | Expr::ConstBool(_) => Ok(self),
            Expr::Var(x) => {
                let j = env.get(&x);
                match j {
                    Some(i) => Ok(i.clone()),
                    None => Err(String::from("Uninstantiated Variable")),
                }
            }
            Expr::Call(p) => p.eval(env),
            Expr::If(c, t, e) => match c.eval(env)? {
                Expr::ConstBool(true) => t.eval(env),
                Expr::ConstBool(false) => e.eval(env),
                _ => Err(String::from("If condition doesn't resolve to Boolean")),
            },
        }
    }
}

impl Eval for Func {
    fn eval(self, env: &HashMap<String, Expr>) -> Term {
        match self {
            Self::Append(arg1, arg2) => {
                let arg1_evaled = arg1.eval(env)?;
                let arg2_evaled = arg2.eval(env)?;

                match (&arg1_evaled, &arg2_evaled) {
                    (Expr::ConstStr(a1), Expr::ConstStr(a2)) => {
                        Ok(Expr::ConstStr(format!("{a1}{a2}")))
                    }
                    _ => Err(format!(
                        "append: invalid argument: arg1 = {:?}, arg2 = {:?}",
                        arg1_evaled, arg2_evaled
                    )),
                }
            }

            Self::Leq(arg1, arg2) => {
                let arg1_evaled = arg1.eval(env)?;
                let arg2_evaled = arg2.eval(env)?;

                match (&arg1_evaled, &arg2_evaled) {
                    (Expr::ConstInt(a1), Expr::ConstInt(a2)) => Ok(Expr::ConstBool(a1 <= a2)),
                    _ => Err(format!(
                        "Leq: invalid argument: arg1 = {:?}, arg2 = {:?}",
                        arg1_evaled, arg2_evaled
                    )),
                }
            }

            Self::Geq(arg1, arg2) => {
                let arg1_evaled = arg1.eval(env)?;
                let arg2_evaled = arg2.eval(env)?;

                match (&arg1_evaled, &arg2_evaled) {
                    (Expr::ConstInt(a1), Expr::ConstInt(a2)) => Ok(Expr::ConstBool(a1 >= a2)),
                    _ => Err(format!(
                        "Geq: invalid argument: arg1 = {:?}, arg2 = {:?}",
                        arg1_evaled, arg2_evaled
                    )),
                }
            }

            Self::Eql(arg1, arg2) => {
                let arg1_evaled = arg1.eval(env)?;
                let arg2_evaled = arg2.eval(env)?;

                match (&arg1_evaled, &arg2_evaled) {
                    (Expr::ConstInt(a1), Expr::ConstInt(a2)) => Ok(Expr::ConstBool(a1 == a2)),
                    _ => Err(format!(
                        "Eql: invalid argument: arg1 = {:?}, arg2 = {:?}",
                        arg1_evaled, arg2_evaled
                    )),
                }
            }

            Self::Add(arg1, arg2) => {
                let arg1_evaled = arg1.eval(env)?;
                let arg2_evaled = arg2.eval(env)?;

                match (&arg1_evaled, &arg2_evaled) {
                    (Expr::ConstInt(a1), Expr::ConstInt(a2)) => Ok(Expr::ConstInt(a1 + a2)),
                    _ => Err(format!(
                        "Add: invalid argument: arg1 = {:?}, arg2 = {:?}",
                        arg1_evaled, arg2_evaled
                    )),
                }
            }

            Self::Min(arg1, arg2) => {
                let arg1_evaled = arg1.eval(env)?;
                let arg2_evaled = arg2.eval(env)?;

                match (&arg1_evaled, &arg2_evaled) {
                    (Expr::ConstInt(a1), Expr::ConstInt(a2)) => Ok(Expr::ConstInt(a1 - a2)),
                    _ => Err(format!(
                        "Min: invalid argument: arg1 = {:?}, arg2 = {:?}",
                        arg1_evaled, arg2_evaled
                    )),
                }
            }

            Self::Mult(arg1, arg2) => {
                let arg1_evaled = arg1.eval(env)?;
                let arg2_evaled = arg2.eval(env)?;

                match (&arg1_evaled, &arg2_evaled) {
                    (Expr::ConstInt(a1), Expr::ConstInt(a2)) => Ok(Expr::ConstInt(a1 * a2)),
                    _ => Err(format!(
                        "Mult: invalid argument: arg1 = {:?}, arg2 = {:?}",
                        arg1_evaled, arg2_evaled
                    )),
                }
            }

            Self::Div(arg1, arg2) => {
                let arg1_evaled = arg1.eval(env)?;
                let arg2_evaled = arg2.eval(env)?;

                match (&arg1_evaled, &arg2_evaled) {
                    (Expr::ConstInt(a1), Expr::ConstInt(a2)) => Ok(Expr::ConstInt(a1 / a2)),
                    _ => Err(format!(
                        "Div: invalid argument: arg1 = {:?}, arg2 = {:?}",
                        arg1_evaled, arg2_evaled
                    )),
                }
            }

            Self::Abs(arg1) => {
                let arg1_evaled = arg1.eval(env)?;

                match &arg1_evaled {
                    Expr::ConstInt(a1) => Ok(Expr::ConstInt(a1.abs())),
                    _ => Err(format!("Abs: invalid argument: arg1 = {:?}", arg1_evaled)),
                }
            }

            Self::Mod(arg1, arg2) => {
                let arg1_evaled = arg1.eval(env)?;
                let arg2_evaled = arg2.eval(env)?;

                match (&arg1_evaled, &arg2_evaled) {
                    (Expr::ConstInt(a1), Expr::ConstInt(a2)) => Ok(Expr::ConstInt(a1 % a2)),
                    _ => Err(format!(
                        "Mod: invalid argument: arg1 = {:?}, arg2 = {:?}",
                        arg1_evaled, arg2_evaled
                    )),
                }
            }

            Self::NegI(arg1) => {
                let arg1_evaled = arg1.eval(env)?;

                match &arg1_evaled {
                    Expr::ConstInt(a1) => Ok(Expr::ConstInt(a1 * (-1))),
                    _ => Err(format!("NegI: invalid argument: arg1 = {:?}", arg1_evaled)),
                }
            }

            Self::NegB(arg1) => {
                let arg1_evaled = arg1.eval(env)?;

                match &arg1_evaled {
                    Expr::ConstBool(a1) => Ok(Expr::ConstBool(!a1)),
                    _ => Err(format!("NegI: invalid argument: arg1 = {:?}", arg1_evaled)),
                }
            }

            Self::And(arg1, arg2) => {
                let arg1_evaled = arg1.eval(env)?;
                let arg2_evaled = arg2.eval(env)?;

                match (&arg1_evaled, &arg2_evaled) {
                    (Expr::ConstBool(a1), Expr::ConstBool(a2)) => Ok(Expr::ConstBool(*a1 && *a2)),
                    _ => Err(format!(
                        "And: invalid argument: arg1 = {:?}, arg2 = {:?}",
                        arg1_evaled, arg2_evaled
                    )),
                }
            }

            Self::Or(arg1, arg2) => {
                let arg1_evaled = arg1.eval(env)?;
                let arg2_evaled = arg2.eval(env)?;

                match (&arg1_evaled, &arg2_evaled) {
                    (Expr::ConstBool(a1), Expr::ConstBool(a2)) => Ok(Expr::ConstBool(*a1 || *a2)),
                    _ => Err(format!(
                        "Or: invalid argument: arg1 = {:?}, arg2 = {:?}",
                        arg1_evaled, arg2_evaled
                    )),
                }
            }

            Self::StrLen(arg1) => {
                let arg1_evaled = arg1.eval(env)?;

                match &arg1_evaled {
                    Expr::ConstStr(a1) => Ok(Expr::ConstInt(a1.len() as i64)),
                    _ => Err(format!(
                        "StrLen: invalid argument: arg1 = {:?}",
                        arg1_evaled
                    )),
                }
            }

            Self::StrAt(arg1, arg2) => {
                let arg1_evaled = arg1.eval(env)?;
                let arg2_evaled = arg2.eval(env)?;

                match (&arg1_evaled, &arg2_evaled) {
                    (Expr::ConstStr(a1), Expr::ConstInt(a2)) => Ok(Expr::ConstStr(
                        a1.chars().nth(*a2 as usize).map_or("".into(), |c| c.into()),
                    )),
                    _ => Err(format!(
                        "StrLen: invalid argument: arg1 = {:?} arg2 = {:?}",
                        arg1_evaled, arg2_evaled
                    )),
                }
            }

            Self::SubStr(arg1, arg2, arg3) => {
                let arg1_evaled = arg1.eval(env)?;
                let arg2_evaled = arg2.eval(env)?;
                let arg3_evaled = arg3.eval(env)?;

                match (&arg1_evaled, &arg2_evaled, &arg3_evaled) {
                    (Expr::ConstStr(a1), Expr::ConstInt(a2), Expr::ConstInt(a3)) => {
                        if (a3 > &0) && (a2 < &(a1.len() as i64)) && (a2 >= &0) {
                            // if either the number to take is 0 or less or the index is out of bounds return mt string
                            if (a2 + a3 - 1) < (a1.len() as i64) {
                                // if index plus number to take is greater than length of string decide on number to take
                                Ok(Expr::ConstStr(
                                    a1.chars()
                                        .skip(*a2 as usize)
                                        .take((a3 - a2) as usize)
                                        .collect::<String>(),
                                ))
                            } else {
                                let j = (a1.len() as i64) - a2;
                                Ok(Expr::ConstStr(
                                    a1.chars()
                                        .skip(*a2 as usize)
                                        .take((j) as usize)
                                        .collect::<String>(),
                                ))
                            }
                        } else {
                            Ok(Expr::ConstStr("".to_string()))
                        }
                    }
                    _ => Err(format!(
                        "SubStr: invalid argument: arg1 = {:?} arg2 = {:?}, arg3 = {:?}",
                        arg1_evaled, arg2_evaled, arg3_evaled
                    )),
                }
            }

            Self::IsPre(arg1, arg2) => {
                let arg1_evaled = arg1.eval(env)?;
                let arg2_evaled = arg2.eval(env)?;

                match (&arg1_evaled, &arg2_evaled) {
                    (Expr::ConstStr(a1), Expr::ConstStr(a2)) => {
                        let l1 = a1.len();
                        let l2 = a2.len();

                        if l1 <= l2 {
                            let p: String = a2.chars().take(l1).collect();
                            Ok(Expr::ConstBool(p == *a1))
                        } else {
                            Ok(Expr::ConstBool(false))
                        }
                    }
                    _ => Err(format!(
                        "IsPre: invalid argument: arg1 = {:?} arg2 = {:?}",
                        arg1_evaled, arg2_evaled
                    )),
                }
            }

            Self::IsPost(arg1, arg2) => {
                let arg1_evaled = arg1.eval(env)?;
                let arg2_evaled = arg2.eval(env)?;

                match (&arg1_evaled, &arg2_evaled) {
                    (Expr::ConstStr(a1), Expr::ConstStr(a2)) => {
                        let l1 = a1.len() as i64;
                        let l2 = a2.len() as i64;

                        if l1 <= l2 {
                            let m = l2 - l1 - 1;
                            let p = a2
                                .chars()
                                .skip(m as usize)
                                .take(l1 as usize)
                                .collect::<String>();

                            Ok(Expr::ConstBool(p == *a1))
                        } else {
                            Ok(Expr::ConstBool(false))
                        }
                    }
                    _ => Err(format!(
                        "IsPost: invalid argument: arg1 = {:?} arg2 = {:?}",
                        arg1_evaled, arg2_evaled
                    )),
                }
            }

            Self::Contains(arg1, arg2) => {
                let arg1_evaled = arg1.eval(env)?;
                let arg2_evaled = arg2.eval(env)?;

                match (&arg1_evaled, &arg2_evaled) {
                    (Expr::ConstStr(a1), Expr::ConstStr(a2)) => {
                        Ok(Expr::ConstBool(a1.contains(a2)))
                    }
                    _ => Err(format!(
                        "Contains: invalid argument: arg1 = {:?} arg2 = {:?}",
                        arg1_evaled, arg2_evaled
                    )),
                }
            }

            Self::Index(arg1, arg2, arg3) => {
                let arg1_evaled = arg1.eval(env)?;
                let arg2_evaled = arg2.eval(env)?;
                let arg3_evaled = arg3.eval(env)?;

                match (&arg1_evaled, &arg2_evaled, &arg3_evaled) {
                    (Expr::ConstStr(a1), Expr::ConstStr(a2), Expr::ConstInt(a3)) => {
                        let l1 = a1.len() as i64;
                        let l2 = a2.len() as i64;

                        if a3 < &l1 {
                            if l2 == 0 {
                                Ok(Expr::ConstInt(*a3))
                            } else {
                                let b1 = a1
                                    .chars()
                                    .skip(*a3 as usize)
                                    .take((l1 - a3 - 1) as usize)
                                    .collect::<String>();

                                if let Some(k) = b1.find(a2) {
                                    Ok(Expr::ConstInt(k as i64))
                                } else {
                                    Ok(Expr::ConstInt(-1))
                                }
                            }
                        } else {
                            Ok(Expr::ConstInt(-1))
                        }
                    }
                    _ => Err(format!(
                        "Index: invalid argument: arg1 = {:?} arg2 = {:?}, arg3 = {:?}",
                        arg1_evaled, arg2_evaled, arg3_evaled
                    )),
                }
            }

            Self::Replace(arg1, arg2, arg3) => {
                let arg1_evaled = arg1.clone().eval(env)?;
                let arg2_evaled = arg2.clone().eval(env)?;
                let arg3_evaled = arg3.clone().eval(env)?;

                match (&arg1_evaled, &arg2_evaled, &arg3_evaled) {
                    (Expr::ConstStr(a1), Expr::ConstStr(a2), Expr::ConstStr(a3)) => {
                        let l1 = a2.len();
                        if l1 == 0 {
                            Func::Append(arg3.clone(), arg1.clone()).eval(env).clone()
                        } else {
                            Ok(Expr::ConstStr(a1.replacen(a2, a3, 1)))
                        }
                    }
                    _ => Err(format!(
                        "Replace: invalid argument: arg1 = {:?} arg2 = {:?}, arg3 = {:?}",
                        arg1_evaled, arg2_evaled, arg3_evaled
                    )),
                }
            }

            Self::ReplaceAll(arg1, arg2, arg3) => {
                let arg1_evaled = arg1.clone().eval(env)?;
                let arg2_evaled = arg2.clone().eval(env)?;
                let arg3_evaled = arg3.clone().eval(env)?;

                match (&arg1_evaled, &arg2_evaled, &arg3_evaled) {
                    (Expr::ConstStr(a1), Expr::ConstStr(a2), Expr::ConstStr(a3)) => {
                        let l1 = a2.len() as i64;
                        if l1 == 0 {
                            Func::Append(arg3.clone(), arg1.clone()).eval(env)
                        } else {
                            Ok(Expr::ConstStr(a1.replace(a2, a3)))
                        }
                    }
                    _ => Err(format!(
                        "ReplaceAll: invalid argument: arg1 = {:?} arg2 = {:?}, arg3 = {:?}",
                        arg1_evaled, arg2_evaled, arg3_evaled
                    )),
                }
            }

            Self::LexEq(arg1, arg2) => {
                let arg1_evaled = arg1.eval(env)?;
                let arg2_evaled = arg2.eval(env)?;

                match (&arg1_evaled, &arg2_evaled) {
                    (Expr::ConstStr(a1), Expr::ConstStr(a2)) => Ok(Expr::ConstBool(a1 == a2)),
                    _ => Err(format!(
                        "LexEq: invalid argument: arg1 = {:?} arg2 = {:?}",
                        arg1_evaled, arg2_evaled
                    )),
                }
            }

            Self::LexLeq(arg1, arg2) => {
                let arg1_evaled = arg1.eval(env)?;
                let arg2_evaled = arg2.eval(env)?;

                match (&arg1_evaled, &arg2_evaled) {
                    (Expr::ConstStr(a1), Expr::ConstStr(a2)) => Ok(Expr::ConstBool(a1 <= a2)),
                    _ => Err(format!(
                        "LexLeq: invalid argument: arg1 = {:?} arg2 = {:?}",
                        arg1_evaled, arg2_evaled
                    )),
                }
            }

            Self::LexGeq(arg1, arg2) => {
                let arg1_evaled = arg1.eval(env)?;
                let arg2_evaled = arg2.eval(env)?;

                match (&arg1_evaled, &arg2_evaled) {
                    (Expr::ConstStr(a1), Expr::ConstStr(a2)) => Ok(Expr::ConstBool(a1 >= a2)),
                    _ => Err(format!(
                        "LexGeq: invalid argument: arg1 = {:?} arg2 = {:?}",
                        arg1_evaled, arg2_evaled
                    )),
                }
            }

            Self::IntToStr(arg1) => {
                let arg1 = arg1.eval(env)?;
                match arg1 {
                    Expr::ConstInt(n) => Ok(Expr::ConstStr(n.to_string())),
                    _ => Err(format!("IntToStr: invalid argument: arg1 = {:?}", arg1)),
                }
            }

            Self::StrToInt(arg1) => {
                let arg1 = arg1.eval(env)?;
                match arg1 {
                    Expr::ConstStr(s) => {
                        let n = s.parse::<u32>();
                        Ok(Expr::ConstInt(n.map_or(-1, |n| n.into())))
                    }
                    _ => Err(format!("StrToInt: invalid argument: arg1 = {:?}", arg1)),
                }
            }
        }
    }
}

impl Translate for RegLang {
    fn translate(&self) -> Regex {
        match self {
            RegLang::Nil => Regex::new(r"[^$]").unwrap(),
            RegLang::All => Regex::new(r"^[ -~]+$").unwrap(),
            RegLang::AllChar => unimplemented!(), // Regex::new(r"(?!\A)").unwrap(), // look-around is not supported
            RegLang::RCall(j) => (*j).clone().translate(),
        }
    }
}

impl Translate for RegFun {
    fn translate(&self) -> Regex {
        match self {
            RegFun::Sequence(arg1, arg2) => {
                let eval1 = arg1.translate();
                let eval2 = arg2.translate();

                Regex::new(&format!("^ {} {} $", eval1, eval2)).unwrap()
            }

            RegFun::Union(arg1, arg2) => {
                let eval1 = arg1.translate();
                let eval2 = arg2.translate();

                Regex::new(&format!("^ {}|{} $", eval1, eval2)).unwrap()
            }

            RegFun::Inter(arg1, arg2) => {
                let eval1 = arg1.translate();
                let eval2 = arg2.translate();

                Regex::new(&format!("^ (?= {})(?= {}) $", eval1, eval2)).unwrap()
            }

            RegFun::Star(arg1) => {
                let eval1 = arg1.translate();

                Regex::new(&format!("({})*", eval1)).unwrap()
            }

            RegFun::One(arg1) => {
                let eval1 = arg1.translate();

                Regex::new(&format!("({})+", eval1)).unwrap()
            }

            RegFun::Opt(arg1) => {
                let eval1 = arg1.translate();

                Regex::new(&format!("{} | \"\"", eval1)).unwrap()
            }

            RegFun::FromStr(arg1) => {
                let j = HashMap::new();
                let eval1 = arg1.clone().eval(&j).clone();

                match eval1 {
                    Ok(Expr::ConstStr(a)) => Regex::new(&a.to_string()).unwrap(),
                    _ => RegLang::Nil.translate(),
                }
            }

            RegFun::Range(arg1, arg2) => {
                let j = HashMap::new();
                let eval1 = arg1.clone().eval(&j);
                let eval2 = arg2.clone().eval(&j);

                match (eval1, eval2) {
                    (Ok(Expr::ConstStr(a)), Ok(Expr::ConstStr(_))) => {
                        let l1 = a.len();
                        let l2 = a.len();

                        if l1 == l2 && l1 == 1 {
                            // I know this was supposed to take a range between
                            // any two same size strings but I couldn't figure that out just yet
                            Regex::new(&format!("[{}-{}]", l1, l2)).unwrap()
                        } else {
                            RegLang::Nil.translate()
                        }
                    }
                    _ => RegLang::Nil.translate(),
                }
            }
        }
    }
}
