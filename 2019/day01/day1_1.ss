(define day1.1
  (letrec ((aux
            (lambda (input counter)
              (let ((caracter (read input)))
                (if (eof-object? caracter)
                    counter
                    (aux input (+ counter (- (quotient caracter 3) 2))))))))
    (lambda ()
      (aux (open-input-file "input.txt") 0))))

(day1.1)