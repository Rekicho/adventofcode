(define le-input
  (lambda (ficheiro lista numero carater)
    (if (eof-object? carater)
        lista
        (if (char=? carater #\:)
            (le-input ficheiro (append lista (list (cons numero (make-vector (read ficheiro) #f)))) 0 (begin (read-char ficheiro) (read-char ficheiro) (read-char ficheiro)))
            (le-input ficheiro lista (+ (* numero 10) (- (char->integer carater) 48)) (read-char ficheiro))))))

(define inicio
  (lambda (vetor i)
    (if (>= i (vector-length vetor))
        vetor
        (begin
          (vector-set! (cdr (vector-ref vetor i)) 0 #t)
          (inicio vetor (add1 i))))))

(define firewall
  (lambda (vetor i fim contador inc)
    (letrec ((verifica
              (lambda (j)
                (if (>= j (vector-length vetor))
                    0
                    (if (and (= i (car (vector-ref vetor j))) (vector-ref (cdr (vector-ref vetor j)) 0))
                        (* i (vector-length (cdr (vector-ref vetor j))))
                        (if (> i (car (vector-ref vetor j)))
                            (verifica (add1 j))
                            0)))))
             (move-seguranca
              (lambda (j k)
                (if (not (>= j (vector-length vetor)))
                    (if (>= k (vector-length (cdr (vector-ref vetor j))))
                        (move-seguranca (add1 j) 0)
                        (if (vector-ref (cdr (vector-ref vetor j)) k)
                            (begin
                              (vector-set! (cdr (vector-ref vetor j)) k #f)
                              (cond
                                ((= k 0) (vector-set! inc j 1))
                                ((= k (sub1 (vector-length (cdr (vector-ref vetor j))))) (vector-set! inc j -1)))
                              (vector-set! (cdr (vector-ref vetor j)) (+ k (vector-ref inc j)) #t)
                              (move-seguranca (add1 j) 0))
                            (move-seguranca j (add1 k))))))))                  
    (if (> i fim)
        contador
        (begin
          (set! contador (+ contador (verifica 0)))
          (move-seguranca 0 0)
          (firewall vetor (add1 i) fim contador inc))))))
          

(define firewall-aux
  (lambda (vetor)
    (firewall vetor 0 (car (vector-ref vetor (sub1 (vector-length vetor)))) 0 (make-vector (vector-length vetor) 1))))
           
(define aux
  (lambda (ficheiro)
      (firewall-aux (inicio (list->vector (le-input ficheiro (list) 0 (read-char ficheiro))) 0))))

(define dia13_1
  (lambda (txt_name)
    (call-with-input-file txt_name aux)))

(dia13_1 "input.txt")