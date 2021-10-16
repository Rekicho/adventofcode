(define le-input
  (lambda (ficheiro lista numero carater)
    (if (eof-object? carater)
        lista
        (if (char=? carater #\:)
            (le-input ficheiro (append lista (list (cons numero (read ficheiro)))) 0 (begin (read-char ficheiro) (read-char ficheiro) (read-char ficheiro)))
            (le-input ficheiro lista (+ (* numero 10) (- (char->integer carater) 48)) (read-char ficheiro))))))

(define inicio?
  (lambda (time size)
    (zero? (remainder time (- (* size 2) 2)))))
                  
(define solucao
  (lambda (firewall tempo)
    (letrec ((apanhado?
              (lambda (i)
                (if (= i (vector-length firewall))
                    #f
                    (if (inicio? (+ tempo (car (vector-ref firewall i))) (cdr (vector-ref firewall i)))
                        #t
                        (apanhado? (add1 i)))))))
      (if (apanhado? 0)
          (solucao firewall (add1 tempo))
          tempo))))

(define aux
  (lambda (ficheiro)
      (solucao (list->vector (le-input ficheiro (list) 0 (read-char ficheiro))) 0)))

(define dia13_2
  (lambda (txt_name)
    (call-with-input-file txt_name aux)))

(dia13_2 "input.txt")