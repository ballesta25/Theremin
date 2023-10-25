(set-logic LIA)

(synth-fun f ((x Int)) Int

  ((y_int Int) (y_const_int Int) (y_bool Bool))

  ((y_int Int (y_const_int
               (Variable Int)
               (- y_int)
               (+ y_int y_int)
               (- y_int y_int)
               (* y_const_int y_int)
               (* y_int y_const_int)
               (div y_int y_const_int)
               (mod y_int y_const_int)
               (abs y_int)
               (ite y_bool y_int y_int)))

   (y_const_int Int ((Constant Int)))

   (y_bool Bool ((= y_int y_int)
                 (> y_int y_int)
                 (>= y_int y_int)
                 (< y_int y_int)
                 (<= y_int y_int)))))
