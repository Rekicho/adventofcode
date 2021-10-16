(define ve_linha
  (lambda (ficheiro)
    (letrec ((linha_aux
              (lambda (lista palavra carater)
                (if (char=? carater #\return)
                    (begin
                      (read-char ficheiro)
                      (append lista (list palavra)))
                    (if (char=? carater #\space)
                        (linha_aux (append lista (list palavra)) (make-string 0 #\0) (read-char ficheiro))
                        (if (char=? carater #\,)
                            (linha_aux lista palavra (read-char ficheiro))
                            (linha_aux lista (string-append palavra (make-string 1 carater)) (read-char ficheiro))))))))
      (linha_aux (list) (make-string 0 #\0) (read-char ficheiro)))))


(define ve-prog
  (lambda (vetor i)
    (letrec ((e_este?
              (lambda (n lista)
                (if (= (add1 n) (vector-length vetor))
                    (if (null? lista)
                        #t
                        (if (string=? (car (vector-ref vetor i)) (car lista))
                            #f
                            (e_este? n (cdr lista))))
                    (if (null? lista)
                        (e_este? (add1 n) (cdr (vector-ref vetor (add1 n))))
                        (if (string=? (car (vector-ref vetor i)) (car lista))
                            #f
                            (e_este? n (cdr lista))))))))
      (if (e_este? 0 (cdr (vector-ref vetor 0))) 
          (car (vector-ref vetor i))
          (ve-prog vetor (add1 i))))))


(define aux
  (lambda (ficheiro)
    (letrec ((aux_aux
              (lambda (lista)
                (if (eof-object? (peek-char ficheiro))
                    (ve-prog (list->vector lista) 0)
                    (aux_aux (append lista (list (ve_linha ficheiro))))))))
      (aux_aux (list)))))



(define dia7_1
  (lambda (txt_name)
    (call-with-input-file txt_name aux)))

(dia7_1 "input.txt")