(define aux_aux
  (lambda (vetor fich)
    (letrec((adiciona-registo!
             (lambda (reg)
               (set! vetor (list->vector (append (vector->list vetor) (list (cons reg 0)))))))        
            (procura-reg
             (lambda (reg i)
               (if (>= i (vector-length vetor))
                   (begin
                     (adiciona-registo! reg)
                     0)
                   (if (string=? reg (car (vector-ref vetor i)))
                       (cdr (vector-ref vetor i))
                       (procura-reg reg (add1 i))))))
            (verifica-cond?
             (lambda (reg condi val)
               (cond
                 ((string=? condi "==") (= (procura-reg reg 0) val))
                 ((string=? condi "!=") (not (= (procura-reg reg 0) val)))
                 ((string=? condi ">") (> (procura-reg reg 0) val))
                 ((string=? condi "<") (< (procura-reg reg 0) val))
                 ((string=? condi ">=") (>= (procura-reg reg 0) val))
                 ((string=? condi "<=") (<= (procura-reg reg 0) val)))))
            (ind-reg
             (lambda (reg i)
               (if (>= i (vector-length vetor))
                   (begin
                     (adiciona-registo! reg)
                     (sub1 (vector-length vetor)))
                   (if (string=? reg (car (vector-ref vetor i)))
                       i
                       (ind-reg reg (add1 i))))))
            (muda-val
             (lambda (reg i_d val ind)
               (cond
                 ((string=? i_d "inc") (vector-set! vetor ind (cons reg (+ (procura-reg reg 0) val))))
                 ((string=? i_d "dec") (vector-set! vetor ind (cons reg (- (procura-reg reg 0) val)))))))
            (processa-linha
             (lambda (r1 i_d v1 se r2 condi v2)
               (if (verifica-cond? r2 condi v2)
                   (muda-val r1 i_d v1 (ind-reg r1 0)))))
            (le-input
             (lambda (frase carater counter)
               (if (char=? carater #\return)
                    (begin
                      (read-char fich)
                      (processa-linha (vector-ref frase 0) (vector-ref frase 1) (vector-ref frase 2) (vector-ref frase 3) (vector-ref frase 4) (vector-ref frase 5) (vector-ref frase 6)))
                    (if (char=? carater #\space)
                        (cond
                          ((= counter 1) (begin (vector-set! frase 2 (read fich)) (le-input frase (read-char fich) 2)))
                          ((= counter 5) (begin (vector-set! frase 6 (read fich)) (le-input frase (read-char fich) 7)))
                          (else (le-input frase (read-char fich) (add1 counter))))
                        (begin
                          (vector-set! frase counter (string-append (vector-ref frase counter) (make-string 1 carater)))
                          (le-input frase (read-char fich) counter))))))
            (ve-maior
             (lambda (i par)
               (if (>= i (vector-length vetor))
                   (cdr par)
                   (if (> (cdr (vector-ref vetor i)) (cdr par))
                       (ve-maior (add1 i) (vector-ref vetor i))
                       (ve-maior (add1 i) par))))))
    (if (eof-object? (peek-char fich))
        (ve-maior 1 (vector-ref vetor 0))
        (begin
          (le-input (make-vector 7 (make-string 0)) (read-char fich) 0)
          (aux_aux vetor fich))))))

(define aux
  (lambda (ficheiro)
      (aux_aux (make-vector 0) ficheiro)))

(define dia8_1
  (lambda (txt_name)
    (call-with-input-file txt_name aux)))

(dia8_1 "input.txt")