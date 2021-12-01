(define le-input
  (lambda (ficheiro lista num letra)
    (if (eof-object? letra)
        (append lista (list num))
        (if (char=? letra #\,)
            (le-input ficheiro (append lista (list num)) 0 (read-char ficheiro))
            (le-input ficheiro lista (+ (* num 10) (- (char->integer letra) 48)) (read-char ficheiro))))))

(define setup
  (lambda (vetor i)
    (if (>= i (vector-length vetor))
        vetor
        (begin
          (vector-set! vetor i i)
          (setup vetor (add1 i))))))

(define get-result
  (lambda (vetor lista posicao salto)
    (letrec ((nexti
              (lambda (tamanho)
                (remainder (+ posicao tamanho salto) (vector-length vetor))))
             (get-vetor
              (lambda (vetort i)
                (if (>= i (vector-length vetort))
                    vetort
                    (begin
                      (vector-set! vetort i (vector-ref vetor (remainder (+ posicao i) (vector-length vetor))))
                      (get-vetor vetort (add1 i))))))
             (reverse!
              (lambda (vetort ri i)
                (if (not (< ri 0))
                    (begin
                      (vector-set! vetor (remainder (+ posicao i) (vector-length vetor)) (vector-ref vetort ri))
                      (reverse! vetort (sub1 ri) (add1 i)))))))               
      (if (null? lista)
          (* (vector-ref vetor 0) (vector-ref vetor 1))
          (begin
            (reverse! (get-vetor (make-vector (car lista)) 0) (sub1 (car lista)) 0)
            (get-result vetor (cdr lista) (nexti (car lista)) (add1 salto)))))))
             
(define aux
  (lambda (ficheiro)
      (get-result (setup (make-vector 256) 0) (le-input ficheiro (list) 0 (read-char ficheiro)) 0 0)))

(define dia10_1
  (lambda (txt_name)
    (call-with-input-file txt_name aux)))

(dia10_1 "input.txt")