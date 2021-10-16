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
                        1
                        (+ 1 (add-til-com vetor (cdr par) 0)))
                    (add-til-com vetor valor (+ i 1))))))
           (percorre
            (lambda (vetor i counter)
              (if (= i (vector-length vetor))
                  counter
                  (let ((par (vector-ref vetor i)))
                    (if (equal? (cdr par) "COM")
                        (percorre vetor (+ i 1) (+ 1 counter))
                        (percorre vetor (+ i 1) (+ 1 counter (add-til-com vetor (cdr par) 0)))))))))                      
    (lambda ()
      (percorre (list->vector (aux (open-input-file "input.txt") (list))) 0 0))))

(day6.1)