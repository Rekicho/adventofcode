(define input 265149)

(define day3_1
  (lambda (number)
    (letrec ((aux
              (lambda (i distance)
                (if (<= number (expt i 2))
                    (cons distance i)
                    (aux (+ i 2) (add1 distance)))))
             (auxaux
              (lambda (par)
                (if (= number (expt (cdr par) 2))
                    (car par)
                    (if (> number (- (expt (cdr par) 2) (cdr par)))
                        (+ (car par) (abs (- number (- (expt (cdr par) 2) (quotient (cdr par) 2)))))
                        (if (> number (- (expt (cdr par) 2) (* (cdr par) 2)))
                            (+ (car par) (abs (- number (- (expt (cdr par) 2) (cdr par) (quotient (cdr par) 2)))))
                            (if (> number (- (expt (cdr par) 2) (* (cdr par) 3)))
                                (+ (car par) (abs (- number (- (expt (cdr par) 2) (cdr par) (cdr par) (quotient (cdr par) 2))))))))))))
      (auxaux (aux 1 0)))))

(day3_1 input)
