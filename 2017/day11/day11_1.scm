(define le-input
  (lambda (ficheiro lista palavra letra)
    (if (eof-object? letra)
        (append lista (list palavra))
        (if (char=? letra #\,)
            (le-input ficheiro (append lista (list palavra)) (make-string 0) (read-char ficheiro))
            (le-input ficheiro lista (string-append palavra (string letra)) (read-char ficheiro))))))

(define anda
  (lambda (tetal)
    (letrec ((desloca
              (lambda (tetal x y z)
                (if (null? tetal)
                    (/ (+ (abs x) (abs y) (abs z)) 2)
                    (begin
                      (cond
                        ((string=? (car tetal) "n") (begin (set! y (add1 y)) (set! z (sub1 z))))
                        ((string=? (car tetal) "nw") (begin (set! x (sub1 x)) (set! y (add1 y))))
                        ((string=? (car tetal) "ne") (begin (set! x (add1 x)) (set! z (sub1 z))))
                        ((string=? (car tetal) "sw") (begin (set! x (sub1 x)) (set! z (add1 z))))
                        ((string=? (car tetal) "se") (begin (set! x (add1 x)) (set! y (sub1 y))))
                        ((string=? (car tetal) "s") (begin (set! y (sub1 y)) (set! z (add1 z)))))
                      (desloca (cdr tetal) x y z))))))
      (desloca tetal 0 0 0))))
             
(define aux
  (lambda (ficheiro)
      (anda (le-input ficheiro (list) (make-string 0) (read-char ficheiro)))))

(define dia11_1
  (lambda (txt_name)
    (call-with-input-file txt_name aux)))

(dia11_1 "input.txt")