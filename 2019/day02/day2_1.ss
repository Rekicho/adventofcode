(define day2.1
  (letrec ((aux_aux
            (lambda (input lista)
              (let ((waste (read-char input))
                    (caracter (read input)))
                (if (eof-object? caracter)
                    (list->vector lista)
                    (aux_aux input (append lista (list caracter)))))))
           (aux
            (lambda (input)
              (aux_aux input (list (read input)))))
           (fix
            (lambda (vetor)
              (begin
                (vector-set! vetor 1 12)
                (vector-set! vetor 2 2)
                vetor)))
           (percorre
            (lambda (vetor i)
              (let ((opcode (vector-ref vetor i)))
                (if (= opcode 99)
                    (vector-ref vetor 0)
                    (begin
                      (if (= opcode 1)
                          (vector-set! vetor (vector-ref vetor (+ i 3)) (+ (vector-ref vetor (vector-ref vetor (+ i 1))) (vector-ref vetor (vector-ref vetor (+ i 2)))))
                          (vector-set! vetor (vector-ref vetor (+ i 3)) (* (vector-ref vetor (vector-ref vetor (+ i 1))) (vector-ref vetor (vector-ref vetor (+ i 2))))))
                      (percorre vetor (+ i 4))))))))                    
    (lambda ()
      (percorre (fix (aux (open-input-file "input.txt"))) 0))))

(day2.1)