(define input (vector 5 1 10 0 1 7 13 14 3 12 8 10 7 12 0 6))

(define day6_1
  (lambda (input)
    (letrec ((ja_apareceu?
              (lambda (vetor anteriores)
                (if (null? (cdr anteriores))
                    #f
                    (if (equal? vetor (car anteriores))
                        #t
                        (ja_apareceu? vetor (cdr anteriores))))))
             (get_max
              (lambda (vetor i max i_temp)
                (if (>= i_temp (vector-length vetor))
                    (cons i max)
                    (if (> (vector-ref vetor i_temp) max)
                        (get_max vetor i_temp (vector-ref vetor i_temp) (add1 i_temp))
                        (get_max vetor i max (add1 i_temp))))))
             (aux_aux
              (lambda (vetor par)
                (if (zero? (cdr par))
                    vetor
                    (begin
                      (vector-set! vetor (if (= (car par) (sub1 (vector-length vetor))) 0 (add1 (car par))) (add1 (vector-ref vetor (if (= (car par) (sub1 (vector-length vetor))) 0 (add1 (car par))))))
                      (aux_aux vetor (cons (if (= (car par) (sub1 (vector-length vetor))) 0 (add1 (car par))) (sub1 (cdr par))))))))
             (aux_aux_aux
              (lambda (vetor par)
                (begin
                  (vector-set! vetor (car par) 0)
                  (aux_aux vetor par))))
             (distribui
              (lambda (vetor)
                (aux_aux_aux vetor (get_max vetor 0 (vector-ref vetor 0) 1))))
             (aux
              (lambda (vetor anteriores i)
                (if (ja_apareceu? (vector->list vetor) anteriores)
                    i
                    (aux (distribui vetor) (append anteriores (list (vector->list vetor))) (add1 i))))))       
    (aux input (list (vector->list input)) 0))))

(day6_1 input)