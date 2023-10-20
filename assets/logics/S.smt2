(set-logic S)

(synth-fun f ((xs String) (xr RegLan) (xi Int)) String

  ((y_str String) (y_rl RegLan) (y_int Int) (y_bool Bool))

  ((y_str String ((Constant String)
                  (Variable String)
                  (str.++ y_str y_str)
                  (str.at y_str y_str)
                  (str.substr y_str y_int y_int)
                  (str.indexof y_str y_str y_int)
                  (str.replace y_str y_str y_str)
                  (str.from_int y_int)
                  (str.from_code y_int)
                  (ite y_bool y_str y_str)))

   (y_rl RegLan ((Constant RegLan)
                 (Variable RegLan)
                 re.none
                 re.all
                 re.allchar
                 (str.to_re y_str)
                 (re.++ y_rl y_rl)
                 (re.union y_rl y_rl)
                 (re.inter y_rl y_rl)
                 (re.* y_rl)
                 (re.+ y_rl)
                 (re.opt y_rl)
                 (re.range y_str y_str)))

   (y_int Int ((Constant Int)
               (Variable Int)
               (str.len y_str)
               (str.to_int y_str)
               (str.to_code y_str)
               (ite y_bool y_int y_int)))
               
   (y_bool Bool ((Constant Bool)
                 (Variable Bool)
                 (str.in_re y_str y_rl)
                 (str.contains y_str y_str)
                 (str.prefixof y_str y_str)
                 (str.suffixof y_str y_str)
                 (str.< y_str y_str)
                 (str.<= y_str y_str)
                 (str.is_digit y_str)))))
