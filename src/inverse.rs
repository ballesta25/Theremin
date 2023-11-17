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

                let mut j = 0;

                let temp : Vec<Option<(Expr, Expr)>>  = a.iter().map(|(i , o)| {
                    
                    match (i,o) {
                        
                        (Expr::ConstStr(inn), Expr::ConstStr(out)) => {
                            
                            if let Some(k) = inn.find(out) {   
                                let ilen = &inn.len();
                                let olen = &out.len();

                                if k as i32 == 0 && (j == 0 || j == 1){

                                    j = 1;

                                    Some((i.clone(), Expr::ConstStr(
                                        
                                        out
                                        .chars()
                                        .skip(*ilen)
                                        .take(*olen)
                                        .collect::<String>())))

                                } else if k == olen - ilen && (j == 0 || j == 2) {
                                    
                                    j = 2;

                                    Some((i.clone(), Expr::ConstStr( out.chars().take(olen - ilen).collect::<String>())))

                                } else {
                                    
                                    j = 3;
                                    None

                                }


                            } else {

                                None // If 
                            }
                            
                        }, 
                        _ => None // If pattern doesn't match return none

                    }
                
                
                } ).collect();
                
                unwrap_vec(temp)


             },

            //   append0: => {

            //         this one will be interesting because it only makes sense to use append if our output is 
            //         either consistently appended to the end or appended to the beginning but not both, 
            //         That will be interesting. 
            //         I will also need to be checking to see if the output is ONLY an append, ie either the input string starts at 0
            //          or the input string starts at out.len() - in.len()
            //          

                 
                

                        




            //   },



            _ => Spec::Indeterminate,


        }

    },
    _ => spec.clone(),

   }


}
