use crate::Spec;
use crate::language::Expr;




pub fn invert(spec : Spec, symbol : &str) -> Spec {

   match spec {

    Spec::Examples(a) => {

        match symbol {


            substring0 => {

                a.iter().map( |Expr::ConstStr(i) , Expr::ConstStr(o)| (Expr::ConstStr(i), Expr::ConstStr(i))).collect()

            },

            substring1 => {

                
                
               let mut temp = a.iter().map( |(Expr::ConstStr(i), Expr::ConstStr(o))| (Expr::ConstStr(i), Expr::ConstInt(
                    if let Some(k) = i.find(o) {
                        k as i32
                    } else {
                        -1 as i32
                    }
                    ))).collect(); //we are relaxing inverse requirement by assuming first occurence is only occurence

               let c = temp.iter().any(|(Expr::ConstStr(i), Expr::ConstInt(o))| o < 0); // check that all specs are even possible 

               if c {

                Spec::Impossible

               } else {

                temp

               }

            },

            substring2 => {

                
                
                let mut temp = a.iter().map( |(Expr::ConstStr(i), Expr::ConstStr(o))| (Expr::ConstStr(i), Expr::ConstInt(
                    if let Some(k) = i.find(o) {
                        k + (o.len() as i32) as i32
                    } else {
                        -1 as i32
                    }
                    ))).collect(); //we are relaxing inverse requirement by assuming first occurence is only occurence
 
                let c = temp.iter().any(|(Expr::ConstStr(i), Expr::ConstInt(o))| o < 0);
 
                if c {
 
                 Spec::Impossible // if the substring isn't even in there 
 
                } else {
 
                 temp
 
                }
 
             },

             

            _ => Spec::Indeterminate,


        }

    },
    _ => spec,

   }


}
