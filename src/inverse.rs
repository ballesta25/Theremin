use crate::Spec;
use crate::language::Expr;




pub fn invert(spec : Spec, symbol : &str) -> Spec {

   match spec {

    Spec::Examples(a) => {

        match symbol {


            substring0 => {

                a.iter().map( |(Expr::ConstStr(a), Expr::ConstStr(b))| (Expr::ConstStr(a), Expr::ConstStr(a))).collect()

            },
            _ => Spec::Indeterminate,


        }

    },
    _ => spec,

   }


}
