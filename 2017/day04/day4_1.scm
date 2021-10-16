(define ve_linha
  (lambda (ficheiro)
    (letrec ((um_a_um
              (lambda (palavra lista)
                (if (null? lista)
                    0
                    (if (string=? palavra (car lista))
                        1
                        (um_a_um palavra (cdr lista))))))
             (verifica
              (lambda (lista)
                (if (null? (cdr lista))
                    1
                    (if (= (um_a_um (car lista) (cdr lista)) 1)
                        0
                        (verifica (cdr lista))))))
             (linha_aux
              (lambda (lista palavra carater)
                (if (char=? carater #\return)
                    (begin
                      (read-char ficheiro)
                      (verifica (append lista (list palavra))))
                    (if (char=? carater #\space)
                        (linha_aux (append lista (list palavra)) (make-string 0 #\0) (read-char ficheiro))
                        (linha_aux lista (string-append palavra (make-string 1 carater)) (read-char ficheiro)))))))
      (linha_aux (list) (make-string 0 #\0) (read-char ficheiro)))))
                        

(define aux
  (lambda (ficheiro)
    (letrec ((aux_aux
              (lambda (counter)
                  (if (eof-object? (peek-char ficheiro))
                      counter
                      (aux_aux (+ counter (ve_linha ficheiro)))))))
      (aux_aux 0))))

(define dia4_1
  (lambda (txt_name)
    (call-with-input-file txt_name aux)))

(dia4_1 "input.txt")