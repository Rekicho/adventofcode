(define day4.1
  (letrec ((number-repeat
            (lambda (lista)
              (if (eq? lista (list))
                  0
                  (+ (if (memq (car lista) (cdr lista)) 1 0) (number-repeat (cdr lista))))))
           (verifica
            (lambda (number)
              (let ((a (quotient number 100000))
                    (b (remainder (quotient number 10000) 10))
                    (c (remainder (quotient number 1000) 10))
                    (d (remainder (quotient number 100) 10))
                    (e (remainder (quotient number 10) 10))
                    (f (remainder number 10)))
                (if (and
                     (<= a b)
                     (<= b c)
                     (<= c d)
                     (<= d e)
                     (<= e f)
                     (>= (number-repeat (list a b c d e f)) 1))
                    1
                    0))))
           (aux
            (lambda (start end)
              (if (> start end)
                  0
                  (+ (verifica start) (aux (+ start 1) end))))))
    (lambda ()
      (aux 248345 746315))))

(day4.1)