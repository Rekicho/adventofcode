(define day5.1
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
              (let ((opcode (remainder (vector-ref vetor i) 100))
                    (param1 (quotient (remainder (vector-ref vetor i) 1000) 100))
                    (param2 (quotient (remainder (vector-ref vetor i) 10000) 1000))
                    (param3 (quotient (remainder (vector-ref vetor i) 100000) 10000)))
                (if (not (= opcode 99))
                    (begin
                      (cond
                        ((= opcode 1) (begin (vector-set! vetor (if (zero? param3) (vector-ref vetor (+ i 3)) (+ i 3)) (+ (vector-ref vetor (if (zero? param1) (vector-ref vetor (+ i 1)) (+ i 1))) (vector-ref vetor (if (zero? param2) (vector-ref vetor (+ i 2)) (+ i 2))))) (set! i (+ i 4))))
                        ((= opcode 2) (begin (vector-set! vetor (if (zero? param3) (vector-ref vetor (+ i 3)) (+ i 3)) (* (vector-ref vetor (if (zero? param1) (vector-ref vetor (+ i 1)) (+ i 1))) (vector-ref vetor (if (zero? param2) (vector-ref vetor (+ i 2)) (+ i 2))))) (set! i (+ i 4))))
                        ((= opcode 3) (begin (vector-set! vetor (if (zero? param1) (vector-ref vetor (+ i 1)) (+ i 1)) (read)) (set! i (+ i 2))))
                        ((= opcode 4) (begin (display (vector-ref vetor (if (zero? param1) (vector-ref vetor (+ i 1)) (+ i 1)))) (newline) (set! i (+ i 2)))))
                      (percorre vetor i)))))))
    (lambda ()
      (percorre (aux (open-input-file "input.txt")) 0))))

(day5.1)