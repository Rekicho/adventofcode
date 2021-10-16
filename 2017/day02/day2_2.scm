(define tripaloski
  (lambda (lista)
    (letrec ((tripleaux
              (lambda (vetor i j)
                (if (and (> (vector-ref vetor i) (vector-ref vetor j))(zero? (remainder (vector-ref vetor i) (vector-ref vetor j))))
                    (/ (vector-ref vetor i) (vector-ref vetor j))
                    (if (= j 15) (tripleaux vetor (add1 i) 0)
                        (tripleaux vetor i (add1 j)))))))               
      (tripleaux (list->vector lista) 0 0))))

(define aux
  (lambda (ficheiro)
    (letrec ((aux_aux
              (lambda (counter fich)
                  (if (eof-object? (peek-char fich))
                      counter
                      (begin (set! counter (+ counter (tripaloski (list (read fich) (read fich) (read fich) (read fich) (read fich) (read fich) (read fich) (read fich) (read fich) (read fich) (read fich) (read fich) (read fich) (read fich) (read fich) (read fich))))) (aux_aux counter fich))))))
      (aux_aux 0 ficheiro))))

(define dia1_2
  (lambda (txt_name)
    (call-with-input-file txt_name aux)))

(dia1_2 "input.txt")