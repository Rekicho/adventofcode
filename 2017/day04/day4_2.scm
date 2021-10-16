(define ve_linha
  (lambda (ficheiro)
    (letrec ((procura_min
              (lambda (palavra i temp_char i_temp)
                (if (= i_temp (string-length palavra))
                    i
                    (if (< (char->integer (string-ref palavra i_temp)) (char->integer temp_char))
                        (procura_min palavra i_temp (string-ref palavra i_temp) (add1 i_temp))
                        (procura_min palavra i temp_char (add1 i_temp))))))
             (alfabetica
              (lambda (palavra i temp_char i_temp)
                (if (= i (string-length palavra))
                    palavra
                    (begin
                      (set! i_temp (procura_min palavra i (string-ref palavra i) i))
                      (set! temp_char (string-ref palavra i_temp))
                      (string-set! palavra i_temp (string-ref palavra i))
                      (string-set! palavra i temp_char)
                      (alfabetica palavra (add1 i) #\0 0)))))
             (um_a_um
              (lambda (palavra lista)
                (if (null? lista)
                    0
                    (if (string=? (alfabetica palavra 0 #\0 0) (alfabetica (car lista) 0 #\0 0))
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

(define dia4_2
  (lambda (txt_name)
    (call-with-input-file txt_name aux)))

(dia4_2 "input.txt")