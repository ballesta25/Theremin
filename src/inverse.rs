use crate::Spec;
use crate::language::Expr;




fn test_valid (example : &Option<(Expr, Expr)>) -> bool {

    match example {

        None => true,
        _ => false,
    }

}


fn unwrap_vec (vecs : Vec<Option<(Expr, Expr)>>) -> Spec {

    if vecs.iter().any(test_valid) {

        Spec::Impossible

    }else{

        Spec::Examples( vecs.into_iter().map( |a: Option<(Expr,Expr)>| a.unwrap()).collect())

    } 

}




pub fn invert(spec : &Spec, symbol : &str) -> Spec {

   match spec {

    Spec::Examples(a) => {

        match symbol {


            "substring0" => {

                let temp : Vec<Option<(Expr, Expr)>>  = a.iter().map(|(i , o)| {
                    
                    match (i,o) {
                        
                        (Expr::ConstStr(inn), Expr::ConstStr(out)) => {
                            
                            if let Some(_) = inn.find(out) {
                                Some((Expr::ConstStr(inn.clone()), Expr::ConstStr(inn.clone())))
                            } else {
                                None
                            }
                            
                        }, 
                        _ => None

                    }
                
                
                } ).collect();
                
                unwrap_vec(temp)

            },

            "substring1" => {

                
                let temp : Vec<Option<(Expr, Expr)>>  = a.iter().map(|(i , o)| {
                    
                    match (i,o) {
                        
                        (Expr::ConstStr(inn), Expr::ConstStr(out)) => {
                            
                            if let Some(k) = inn.find(out) {
                                Some((i.clone(), Expr::ConstInt(k as i32)))
                            } else {
                                None
                            }
                            
                        }, 
                        _ => None

                    }
                
                
                } ).collect();
                
                unwrap_vec(temp)


            },

            "substring2" => {

                

                let temp : Vec<Option<(Expr, Expr)>>  = a.iter().map(|(i , o)| {
                    
                    match (i,o) {
                        
                        (Expr::ConstStr(inn), Expr::ConstStr(out)) => {
                            
                            if let Some(k) = inn.find(out) {
                                Some((i.clone(), Expr::ConstInt((k + out.len()) as i32)))
                            } else {
                                None
                            }
                            
                        }, 
                        _ => None

                    }
                
                
                } ).collect();
                
                unwrap_vec(temp)
                
               
 
             },



             "append0" => {

                let mut j = 0; // this is so that we can track if the append is being used at the beginning or end
                

                let temp : Vec<Option<(Expr, Expr)>>  = a.iter().map(|(i , o)| {
                    
                    match (i,o) {
                        
                        (Expr::ConstStr(inn), Expr::ConstStr(out)) => {
                            
                            if let Some(k) = inn.find(out) {   
                                let ilen = &inn.len();
                                let olen = &out.len();

                                if k as i32 == 0 && (j == 0 || j == 1){

                                    j = 1; //this example uses prepend instead of append and no prior examples used append

                                    Some((i.clone(), i.clone()))

                                } else if k == olen - ilen && (j == 0 || j == 2) {
                                    
                                    j = 2; // this example uses append instead of prepend and no prior examples use prepend

                                    Some((i.clone(), Expr::ConstStr( out.chars().take(olen - ilen).collect::<String>())))

                                } else {
                                    
                                    j = 3; // this example has used violated the prepend only/ append only, all future examples will be ignored
                                    None

                                }


                            } else {

                                None // some example wasn't using append
                            }
                            
                        }, 
                        _ => None // If pattern doesn't match return none

                    }
                
                
                } ).collect();
                
                unwrap_vec(temp)


             },

             "append1" => {

                let mut j = 0; // this is so that we can track if the append is being used at the beginning or end
                

                let temp : Vec<Option<(Expr, Expr)>>  = a.iter().map(|(i , o)| {
                    
                    match (i,o) {
                        
                        (Expr::ConstStr(inn), Expr::ConstStr(out)) => {
                            
                            if let Some(k) = inn.find(out) {   
                                let ilen = &inn.len();
                                let olen = &out.len();

                                if k as i32 == 0 && (j == 0 || j == 1){

                                    j = 1; //this example uses prepend instead of append and no prior examples used append

                                    Some((i.clone(), Expr::ConstStr( out.chars().take(olen - ilen).collect::<String>())))

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
                            
                        }, 
                        _ => None // If pattern doesn't match return none

                    }
                
                
                } ).collect();
                
                unwrap_vec(temp)


             },


             "StrLen0" => {

                let temp : Vec<Option<(Expr, Expr)>>  = a.iter().map(|(i , o)| {
                    
                    match (i,o) {
                        
                        (Expr::ConstStr(inn), Expr::ConstInt(out)) => {
                            
                            if *out == inn.len() as i32 { // just make sure the length actually matches
                                Some((Expr::ConstStr(inn.clone()), Expr::ConstStr(inn.clone())))
                            } else {
                                None
                            }
                            
                        }, 
                        _ => None

                    }
                
                
                } ).collect();
                
                unwrap_vec(temp)

            },
            
                 
             



            _ => Spec::Indeterminate,


        }

    },
    _ => spec.clone(),

   }


}
