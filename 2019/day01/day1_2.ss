(define day1.2
  (letrec ((aux_aux
            (lambda (counter number)
              (let ((res (- (quotient number 3) 2)))
                (if (<= res 0)
                    counter
                    (aux_aux (+ counter res) res)))))
           (aux
            (lambda (input counter)
              (let ((caracter (read input)))
                (if (eof-object? caracter)
                    counter
                    (aux input (+ counter (aux_aux 0 caracter))))))))
    (lambda ()
      (aux (open-input-file "input.txt") 0))))

(day1.2)