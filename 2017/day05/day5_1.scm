(define operacao
  (lambda (vetor)
    (letrec ((op
              (lambda (i counter)
                (if (or (< i 0) (>= i (vector-length vetor)))
                    counter
                    (begin
                      (vector-set! vetor i (add1 (vector-ref vetor i)))
                      (op (+ i (sub1 (vector-ref vetor i))) (add1 counter)))))))
      (op 0 0))))
    

(define aux
  (lambda (ficheiro)
    (letrec ((aux_aux
              (lambda (lista)
                  (if (eof-object? (peek-char ficheiro))
                      (operacao (list->vector lista))
                      (aux_aux (append lista (list (read ficheiro))))))))
      (aux_aux (list)))))

(define dia5_1
  (lambda (txt_name)
    (call-with-input-file txt_name aux)))

(dia5_1 "input.txt")