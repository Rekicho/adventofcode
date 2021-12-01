(define le-input
  (lambda (ficheiro lista)
    (if (eof-object? (peek-char ficheiro))
        lista
        (le-input ficheiro (append lista (list (char->integer (read-char ficheiro))))))))
        
(define setup
  (lambda (vetor i)
    (if (>= i (vector-length vetor))
        vetor
        (begin
          (vector-set! vetor i i)
          (setup vetor (add1 i))))))

(define get-result
  (lambda (vetor nums j posicao salto tnt)
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
      (if (>= tnt 64)
          vetor
          (if (>= j (vector-length nums))
              (get-result vetor nums 0 posicao salto (add1 tnt))
              (begin
                (reverse! (get-vetor (make-vector (vector-ref nums j)) 0) (sub1 (vector-ref nums j)) 0)
                (get-result vetor nums (add1 j) (nexti (vector-ref nums j)) (add1 salto) tnt)))))))

(define sparse-hash
  (lambda (hash i vetor j)
    (if (>= i (vector-length hash))
        hash
        (begin
          (vector-set! hash i (bitwise-xor (vector-ref vetor j) (vector-ref vetor (add1 j)) (vector-ref vetor (+ j 2)) (vector-ref vetor (+ j 3)) (vector-ref vetor (+ j 4)) (vector-ref vetor (+ j 5)) (vector-ref vetor (+ j 6)) (vector-ref vetor (+ j 7)) (vector-ref vetor (+ j 8)) (vector-ref vetor (+ j 9)) (vector-ref vetor (+ j 10)) (vector-ref vetor (+ j 11)) (vector-ref vetor (+ j 12)) (vector-ref vetor (+ j 13)) (vector-ref vetor (+ j 14)) (vector-ref vetor (+ j 15))))
          (sparse-hash hash (add1 i) vetor (+ j 16))))))

(define formata
  (lambda (hash j vec i)
    (if (>= i (vector-length vec))
        hash
        (if (= (string-length (number->string (vector-ref vec i) 16)) 1)
            (begin
              (string-set! hash j #\0)
              (string-set! hash (add1 j) (string-ref (number->string (vector-ref vec i) 16) 0))
              (formata hash (+ j 2) vec (add1 i)))
            (begin
              (string-set! hash j (string-ref (number->string (vector-ref vec i) 16) 0))
              (string-set! hash (add1 j) (string-ref (number->string (vector-ref vec i) 16) 1))
              (formata hash (+ j 2) vec (add1 i)))))))
             
(define aux
  (lambda (ficheiro)
      (formata (make-string 32) 0 (sparse-hash (make-vector 16) 0 (get-result (setup (make-vector 256) 0) (list->vector (append (le-input ficheiro (list)) (list 17 31 73 47 23))) 0 0 0 0) 0) 0)))

(define dia10_2
  (lambda (txt_name)
    (call-with-input-file txt_name aux)))

(dia10_2 "input.txt")