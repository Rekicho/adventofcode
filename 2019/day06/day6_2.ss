(define day6.1
  (letrec ((update-lista
            (lambda (lista o1 o2)
              (append lista (list (cons o2 o1)))))
           (aux
            (lambda (input lista)
              (let ((caracter (read-char input)))
                (if (eof-object? caracter)
                    lista
                    (let ((o1 (string caracter (read-char input) (read-char input))) ;Assumes three char object name
                          (waste (read-char input))
                          (o2 (string (read-char input) (read-char input) (read-char input)))
                          (other-waste (read-char input))
                          (another-waste (read-char input)))
                      (aux input (update-lista lista o1 o2)))))))
           (add-til-com
            (lambda (vetor valor i)
              (let ((par (vector-ref vetor i)))
                (if (equal? (car par) valor)
                    (if (equal? (cdr par) "COM")
                        (append (list valor) (list "COM"))
                        (append (list valor) (add-til-com vetor (cdr par) 0)))
                    (add-til-com vetor valor (+ i 1))))))
           (find
            (lambda (valor lista counter)
              (if (eq? lista (list))
                  #f
                  (if (eq? valor (car lista))
                      counter
                      (find valor (cdr lista) (+ counter 1))))))                 
           (find-con
            (lambda (you san counter)
              (let ((res (find (car you) san 0)))
                (if res
                    (- (+ counter res) 4)
                    (find-con (cdr you) san (+ counter 1)))))))
    (lambda ()
      (let ((mapa (list->vector (aux (open-input-file "input.txt") (list)))))
        (find-con (add-til-com mapa "YOU" 0) (add-til-com mapa "SAN" 0) 0)))))

(day6.1)