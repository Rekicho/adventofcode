(define operacao
  (lambda (vetor)
    (letrec ((op
              (lambda (i counter temp)
                (if (or (< i 0) (>= i (vector-length vetor)))
                    counter
                    (begin
                      (set! temp (vector-ref vetor i))
                      (if (< (vector-ref vetor i) 3)
                          (vector-set! vetor i (add1 (vector-ref vetor i)))
                          (vector-set! vetor i (sub1 (vector-ref vetor i))))
                      (op (+ i temp) (add1 counter) 0))))))
      (op 0 0 0))))
    

(define aux
  (lambda (ficheiro)
    (letrec ((aux_aux
              (lambda (lista)
                  (if (eof-object? (peek-char ficheiro))
                      (operacao (list->vector lista))
                      (aux_aux (append lista (list (read ficheiro))))))))
      (aux_aux (list)))))

(define dia5_2
  (lambda (txt_name)
    (call-with-input-file txt_name aux)))

(dia5_2 "input.txt")