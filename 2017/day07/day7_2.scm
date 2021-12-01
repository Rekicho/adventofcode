(define ve_linha
  (lambda (ficheiro)
    (letrec ((linha_aux
              (lambda (lista palavra carater)
                (if (char=? carater #\return)
                    (begin
                      (read-char ficheiro)
                      (list->vector (append lista (list palavra))))
                    (if (char=? carater #\space)
                        (linha_aux (append lista (list palavra)) (make-string 0 #\0) (read-char ficheiro))
                        (if (char=? carater #\,)
                            (linha_aux lista palavra (read-char ficheiro))
                            (linha_aux lista (string-append palavra (make-string 1 carater)) (read-char ficheiro))))))))
      (linha_aux (list) (make-string 0 #\0) (read-char ficheiro)))))
        
(define ve-prog
  (lambda (vetor i)
    (letrec ((acabou?
              (lambda (vetort j par)
                (if (>= j (vector-length vetort))
                    #f
                    (if (= j 3)
                        (acabou? vetort (add1 j) (vector-ref vetort j))
                        (if (not (= (+ (car par) (cdr par)) (+ (car (vector-ref vetort j)) (cdr (vector-ref vetort j)))))
                            #t
                            (acabou? vetort (add1 j) par))))))
             (final
              (lambda (vetort j par1 par2 par3)
                (if (= (+ (car par1) (cdr par1)) (+ (car par2) (cdr par2)))
                    (if (= (+ (car par1) (cdr par1)) (+ (car par3) (cdr par3)))
                        (final vetort (add1 j) par2 par3 (vector-ref vetort j))
                        (+ (car par3) (- (+ (car par1) (cdr par1)) (+ (car par3) (cdr par3)))))
                    (if (= (+ (car par1) (cdr par1)) (+ (car par3) (cdr par3)))
                        (+ (car par2) (- (+ (car par1) (cdr par1)) (+ (car par2) (cdr par2))))
                        (+ (car par1) (- (+ (car par2) (cdr par2)) (+ (car par1) (cdr par1))))))))
             (get-peso
              (lambda (vetort j counter)
                (if (>= j (vector-length vetort))
                    counter
                    (get-peso vetort (add1 j) (+ counter (car (vector-ref vetort j)) (cdr (vector-ref vetort j)))))))
             (substitui!
              (lambda (palavra par j k)
                (if (not (>= k (vector-length (vector-ref vetor j))))
                    (begin
                      (if (string? (vector-ref (vector-ref vetor j) k))
                          (if (string=? palavra (vector-ref (vector-ref vetor j) k))
                              (vector-set! (vector-ref vetor j) k par)))
                      (substitui! palavra par j (add1 k))))))                
             (remove!
              (lambda (palavra par j)
                (if (not (>= j (vector-length vetor)))
                    (if (string=? palavra (vector-ref (vector-ref vetor j) 0))
                        (remove! palavra par (add1 j))
                        (begin
                          (substitui! palavra par j 2)
                          (remove! palavra par (add1 j)))))))
             (vector-erase
              (lambda (palavra ni nvet nj)
                (if (>= ni (vector-length vetor))
                    nvet
                    (if (string=? palavra (vector-ref (vector-ref vetor ni) 0))
                        (vector-erase palavra (add1 ni) nvet nj)
                        (begin
                          (vector-set! nvet nj (vector-ref vetor ni))
                          (vector-erase palavra (add1 ni) nvet (add1 nj)))))))
             (atualiza!
              (lambda (vetort j)
                (if (>= j (vector-length vetor))
                    (set! vetor (vector-erase (vector-ref vetort 0) 0 (make-vector (- (vector-length vetor) 1)) 0))
                    (if (= (vector-length vetort) 2)
                        (begin
                          (remove! (vector-ref vetort 0) (cons (vector-ref vetort 1) 0) 0)
                          (atualiza! vetort (add1 j)))
                        (begin
                          (remove! (vector-ref vetort 0) (cons (vector-ref vetort 1) (get-peso vetort 3 0)) 0)
                          (atualiza! vetort (add1 j)))))))     
             (has_string?
              (lambda (vetort j)
                (if (>= j (vector-length vetort))
                    #f
                    (if (string? (vector-ref vetort j))
                        #t
                        (has_string? vetort (add1 j)))))))
      (if (has_string? (vector-ref vetor i) 3)
          (ve-prog vetor (add1 i))
          (if (acabou? (vector-ref vetor i) 3 (cons -1 -1))
              (final (vector-ref vetor i) 6 (vector-ref (vector-ref vetor i) 3) (vector-ref (vector-ref vetor i) 4) (vector-ref (vector-ref vetor i) 5))
              (begin
                (atualiza! (vector-ref vetor i) 0)
                (ve-prog vetor 0)))))))

(define str->num
  (lambda (palavra i counter)
    (if (>= i (string-length palavra))
        counter
        (if (or (char=? (string-ref palavra i) #\() (char=? (string-ref palavra i) #\)))
            (str->num palavra (add1 i) counter)
            (str->num palavra (add1 i) (+ (* 10 counter) (- (char->integer (string-ref palavra i)) 48)))))))

(define limpa_lixo
  (lambda (vetor i)
    (if (>= i (vector-length vetor))
        vetor
        (begin
          (vector-set! (vector-ref vetor i) 1 (str->num (vector-ref (vector-ref vetor i) 1) 0 0))
          (limpa_lixo vetor (add1 i))))))

(define aux
  (lambda (ficheiro)
    (letrec ((aux_aux
              (lambda (lista)
                (if (eof-object? (peek-char ficheiro))
                    (ve-prog (limpa_lixo (list->vector lista) 0) 0)
                    (aux_aux (append lista (list (ve_linha ficheiro))))))))
      (aux_aux (list)))))

(define dia7_2
  (lambda (txt_name)
    (call-with-input-file txt_name aux)))

(dia7_2 "input.txt")