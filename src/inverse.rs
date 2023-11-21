use crate::language::Expr;
use crate::Spec;
use std::cmp;

fn test_valid(example: &Option<(Expr, Expr)>) -> bool {
    match example {
        None => true,
        _ => false,
    }
}

fn unwrap_vec(vecs: Vec<Option<(Expr, Expr)>>) -> Spec {
    if vecs.iter().any(test_valid) {
        Spec::Impossible
    } else {
        Spec::Examples(
            vecs.into_iter()
                .map(|a: Option<(Expr, Expr)>| a.unwrap())
                .collect(),
        )
    }
}

pub fn invert(spec: &Spec, symbol: &str) -> Spec {
    match spec {
        Spec::Examples(a) => {
            match symbol {
                "substring0" => {
                    let temp: Vec<Option<(Expr, Expr)>> = a
                        .iter()
                        .map(|(i, o)| match (i, o) {
                            (Expr::ConstStr(inn), Expr::ConstStr(out)) => {
                                if let Some(_) = inn.find(out) {
                                    Some((Expr::ConstStr(inn.clone()), Expr::ConstStr(inn.clone())))
                                } else {
                                    None
                                }
                            }
                            _ => None,
                        })
                        .collect();

                    unwrap_vec(temp)
                }

                "substring1" => {
                    let temp: Vec<Option<(Expr, Expr)>> = a
                        .iter()
                        .map(|(i, o)| match (i, o) {
                            (Expr::ConstStr(inn), Expr::ConstStr(out)) => {
                                if let Some(k) = inn.find(out) {
                                    Some((i.clone(), Expr::ConstInt(k as i64)))
                                } else {
                                    None
                                }
                            }
                            _ => None,
                        })
                        .collect();

                    unwrap_vec(temp)
                }

                "substring2" => {
                    let temp: Vec<Option<(Expr, Expr)>> = a
                        .iter()
                        .map(|(i, o)| match (i, o) {
                            (Expr::ConstStr(inn), Expr::ConstStr(out)) => {
                                if let Some(k) = inn.find(out) {
                                    Some((i.clone(), Expr::ConstInt((k + out.len()) as i64)))
                                } else {
                                    None
                                }
                            }
                            _ => None,
                        })
                        .collect();

                    unwrap_vec(temp)
                }

                "append0" => {
                    let mut j = 0; // this is so that we can track if the append is being used at the beginning or end

                    let temp: Vec<Option<(Expr, Expr)>> = a
                        .iter()
                        .map(|(i, o)| {
                            match (i, o) {
                                (Expr::ConstStr(inn), Expr::ConstStr(out)) => {
                                    if let Some(k) = inn.find(out) {
                                        let ilen = &inn.len();
                                        let olen = &out.len();
                                        
                                        if ilen > olen {

                                            None
                                        }
                                        else if k as i64 == 0 && (j == 0 || j == 1) {
                                            j = 1; //this example uses prepend instead of append and no prior examples used append

                                            Some((i.clone(), i.clone()))
                                        } else if k == olen - ilen && (j == 0 || j == 2) {
                                            j = 2; // this example uses append instead of prepend and no prior examples use prepend

                                            Some((
                                                i.clone(),
                                                Expr::ConstStr(
                                                    out.chars()
                                                        .take(olen - ilen)
                                                        .collect::<String>(),
                                                ),
                                            ))
                                        } else {
                                            j = 3; // this example has used violated the prepend only/ append only, all future examples will be ignored
                                            None
                                        }
                                    } else {
                                        None // some example wasn't using append
                                    }
                                }
                                _ => None, // If pattern doesn't match return none
                            }
                        })
                        .collect();

                    unwrap_vec(temp)
                }

                "append1" => {
                    let mut j = 0; // this is so that we can track if the append is being used at the beginning or end

                    let temp: Vec<Option<(Expr, Expr)>> = a
                        .iter()
                        .map(|(i, o)| {
                            match (i, o) {
                                (Expr::ConstStr(inn), Expr::ConstStr(out)) => {
                                    if let Some(k) = inn.find(out) {
                                        let ilen = &inn.len();
                                        let olen = &out.len();
                                        
                                        if ilen > olen {
                                            None
                                        }
                                        else if k as i64 == 0 && (j == 0 || j == 1) {
                                            j = 1; //this example uses prepend instead of append and no prior examples used append

                                            Some((
                                                i.clone(),
                                                Expr::ConstStr(
                                                    out.chars()
                                                        .take(olen - ilen)
                                                        .collect::<String>(),
                                                ),
                                            ))
                                        } else if k == olen - ilen && (j == 0 || j == 2) {
                                            j = 2; // this example uses append instead of prepend and no prior examples use prepend

                                            Some((i.clone(), i.clone()))
                                        } else {
                                            j = 3; // this example has used violated the prepend only/ append only, all future examples will be ignored
                                            None
                                        }
                                    } else {
                                        None // some example wasn't using append
                                    }
                                }
                                _ => None, // If pattern doesn't match return none
                            }
                        })
                        .collect();

                    unwrap_vec(temp)
                }

                "strlen0" => {
                    let temp: Vec<Option<(Expr, Expr)>> = a
                        .iter()
                        .map(|(i, o)| {
                            match (i, o) {
                                (Expr::ConstStr(inn), Expr::ConstInt(out)) => {
                                    if *out == inn.len() as i64 {
                                        // just make sure the length actually matches
                                        Some((
                                            Expr::ConstStr(inn.clone()),
                                            Expr::ConstStr(inn.clone()),
                                        ))
                                    } else {
                                        None
                                    }
                                }
                                _ => None,
                            }
                        })
                        .collect();

                    unwrap_vec(temp)
                }

                "replace0" => {
                    // wait ok I'm gonna impose the rediculous idea that if the code could have been created by append in either direction,
                    // we will assume the incorrect function has been chosen.
                    // Also I would recommend we just state that if the previous string is a member of the new string it is also incorrect.

                    let temp: Vec<Option<(Expr, Expr)>> = a
                        .iter()
                        .map(|(i, o)| match (i, o) {
                            (Expr::ConstStr(inn), Expr::ConstStr(out)) => {
                                if let Some(_) = inn.find(out) {
                                    None
                                } else {
                                    Some((Expr::ConstStr(inn.clone()), Expr::ConstStr(inn.clone())))
                                }
                            }
                            _ => None,
                        })
                        .collect();

                    unwrap_vec(temp)
                }

                "replace1" => {
                    let temp: Vec<Option<(Expr, Expr)>> = a
                        .iter()
                        .map(|(i, o)| {
                            match (i, o) {
                                (Expr::ConstStr(inn), Expr::ConstStr(out)) => {
                                    if let Some(_) = inn.find(out) {
                                        // again we are assuming that if a replacement string has the original string in it,
                                        // replace is the wrong function and is therefore impossible
                                        None
                                    } else {
                                        let ilen = inn.len() as i64;
                                        let olen = out.len() as i64;
                                        let mut inn2 = inn.chars();
                                        let mut out2 = out.chars();
                                        let minimum = cmp::min(ilen, olen) - 1;
                                        let mut sindex = 0;
                                        let mut eindex = 0;

                                        for loc in 0..minimum {
                                            // finding start index of match

                                            if inn2.nth(loc as usize).unwrap().to_string()
                                                == out2.nth(loc as usize).unwrap().to_string()
                                            {
                                                sindex = loc;
                                            }

                                            if inn2.nth(loc as usize).unwrap().to_string()
                                                != out2.nth(loc as usize).unwrap().to_string()
                                            {
                                                break;
                                            }
                                        }

                                        for loc in minimum..0 {
                                            if inn2.nth(loc as usize).unwrap().to_string()
                                                == out2.nth(loc as usize).unwrap().to_string()
                                            {
                                                eindex = loc;
                                            }

                                            if inn2.nth(loc as usize).unwrap().to_string()
                                                != out2.nth(loc as usize).unwrap().to_string()
                                            {
                                                break;
                                            }
                                        }

                                        let tempstring = inn2
                                            .skip(sindex as usize)
                                            .take((eindex - sindex) as usize)
                                            .collect::<String>(); //we are assuming that if we are replacing the enitre
                                                                  // string we have the wrong function I should have done these cases in the opposite order

                                        if tempstring == *inn {
                                            None
                                        } else {
                                            Some((i.clone(), Expr::ConstStr(tempstring)))
                                        }
                                    }
                                }
                                _ => None,
                            }
                        })
                        .collect();

                    unwrap_vec(temp)
                }

                "replace2" => {
                    let temp: Vec<Option<(Expr, Expr)>> = a
                        .iter()
                        .map(|(i, o)| {
                            match (i, o) {
                                (Expr::ConstStr(inn), Expr::ConstStr(out)) => {
                                    if let Some(_) = inn.find(out) {
                                        // again we are assuming that if a replacement string has the original string in it,
                                        // replace is the wrong function and is therefore impossible
                                        None
                                    } else {
                                        let ilen = inn.len() as i64;
                                        let olen = out.len() as i64;
                                        let mut inn2 = inn.chars();
                                        let mut out2 = out.chars();
                                        let minimum = cmp::min(ilen, olen) - 1;
                                        let mut sindex = 0;
                                        let mut eindex = 0;

                                        for loc in 0..minimum {
                                            // finding start index of match

                                            if inn2.nth(loc as usize).unwrap().to_string()
                                                == out2.nth(loc as usize).unwrap().to_string()
                                            {
                                                sindex = loc;
                                            }

                                            if inn2.nth(loc as usize).unwrap().to_string()
                                                != out2.nth(loc as usize).unwrap().to_string()
                                            {
                                                break;
                                            }
                                        }

                                        for loc in minimum..0 {
                                            if inn2.nth(loc as usize).unwrap().to_string()
                                                == out2.nth(loc as usize).unwrap().to_string()
                                            {
                                                eindex = loc;
                                            }

                                            if inn2.nth(loc as usize).unwrap().to_string()
                                                != out2.nth(loc as usize).unwrap().to_string()
                                            {
                                                break;
                                            }
                                        }

                                        let tempstring = out2
                                            .skip(sindex as usize)
                                            .take((olen - sindex - 1) as usize)
                                            .collect::<String>(); //we are assuming that if we are replacing the enitre
                                                                  // string we have the wrong function I should have done these cases in the opposite order

                                        if tempstring == *inn {
                                            None
                                        } else {
                                            Some((i.clone(), Expr::ConstStr(tempstring)))
                                        }
                                    }
                                }
                                _ => None,
                            }
                        })
                        .collect();

                    unwrap_vec(temp)
                }

                "strat0" => {
                    let temp: Vec<Option<(Expr, Expr)>> = a
                        .iter()
                        .map(|(i, o)| {
                            match (i, o) {
                                (Expr::ConstStr(inn), Expr::ConstStr(out)) => {
                                    if let Some(_) = inn.find(out) {
                                        //if it even exists within the other string
                                        Some((
                                            Expr::ConstStr(inn.clone()),
                                            Expr::ConstStr(inn.clone()),
                                        ))
                                    } else {
                                        None
                                    }
                                }
                                _ => None,
                            }
                        })
                        .collect();

                    unwrap_vec(temp)
                }

                "strat1" => {
                    let temp: Vec<Option<(Expr, Expr)>> = a
                        .iter()
                        .map(|(i, o)| {
                            match (i, o) {
                                (Expr::ConstStr(inn), Expr::ConstStr(out)) => {
                                    if let Some(_) = inn.find(out) {
                                        //if it even exists within the other string
                                        Some((
                                            Expr::ConstStr(inn.clone()),
                                            Expr::ConstStr(inn.clone()),
                                        ))
                                    } else {
                                        None
                                    }
                                }
                                _ => None,
                            }
                        })
                        .collect();

                    let j = unwrap_vec(temp);
                    if j == Spec::Impossible {
                        Spec::Impossible
                    } else {
                        Spec::Indeterminate
                    }
                }

                "index0" => {
                    let temp: Vec<Option<(Expr, Expr)>> = a
                        .iter()
                        .map(|(i, o)| {
                            match (i, o) {
                                (Expr::ConstStr(inn), Expr::ConstInt(out)) => {
                                    if inn.len() as i64 > *out {
                                        //if it even exists within the other string
                                        Some((
                                            Expr::ConstStr(inn.clone()),
                                            Expr::ConstStr(inn.clone()),
                                        ))
                                    } else {
                                        None
                                    }
                                }
                                _ => None,
                            }
                        })
                        .collect();

                    unwrap_vec(temp)
                }

                "index1" => {
                    let temp: Vec<Option<(Expr, Expr)>> = a
                        .iter()
                        .map(|(i, o)| {
                            match (i, o) {
                                (Expr::ConstStr(inn), Expr::ConstInt(out)) => {
                                    if inn.len() as i64 > *out {
                                        //if it even exists within the other string
                                        Some((
                                            Expr::ConstStr(inn.clone()),
                                            Expr::ConstStr(inn.clone()),
                                        ))
                                    } else {
                                        None
                                    }
                                }
                                _ => None,
                            }
                        })
                        .collect();

                    let j = unwrap_vec(temp);
                    if j == Spec::Impossible {
                        Spec::Impossible
                    } else {
                        Spec::Indeterminate
                    }
                }

                "index2" => {
                    let temp: Vec<Option<(Expr, Expr)>> = a
                        .iter()
                        .map(|(i, o)| {
                            match (i, o) {
                                (Expr::ConstStr(inn), Expr::ConstInt(out)) => {
                                    if inn.len() as i64 > *out {
                                        //if it even exists within the other string
                                        Some((
                                            Expr::ConstStr(inn.clone()),
                                            Expr::ConstStr(inn.clone()),
                                        ))
                                    } else {
                                        None
                                    }
                                }
                                _ => None,
                            }
                        })
                        .collect();

                    let j = unwrap_vec(temp);
                    if j == Spec::Impossible {
                        Spec::Impossible
                    } else {
                        Spec::Indeterminate
                    }
                }

                _ => Spec::Indeterminate, // we don't have good inverse semantics for these
            }
        }

        // not great inverse for replaceall
        // none for any function that takes a string and outputs a bool
        // we can't say anything about a function that takes a string and
        _ => spec.clone(), // the original spec was bad
    }
}
