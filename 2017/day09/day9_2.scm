(define le-input
  (lambda (ficheiro txt)
    (if (eof-object? (peek-char ficheiro))
        txt
        (le-input ficheiro (string-append txt (string (read-char ficheiro)))))))

(define conta-grupos
  (lambda (txt i counter lixo)
    (if (>= i (string-length txt))
        counter
        (cond
          ((and lixo (char=? (string-ref txt i) #\!)) (conta-grupos txt (+ i 2) counter lixo))
          ((and (not lixo) (char=? (string-ref txt i) #\<)) (conta-grupos txt (add1 i) counter #t))
          ((and lixo (char=? (string-ref txt i) #\>)) (conta-grupos txt (add1 i) counter #f))
          (lixo (conta-grupos txt (add1 i) (add1 counter) lixo))
          (else (conta-grupos txt (add1 i) counter lixo))))))
            
(define aux
  (lambda (ficheiro)
      (conta-grupos (le-input ficheiro (make-string 0)) 0 0 #f)))

(define dia9_2
  (lambda (txt_name)
    (call-with-input-file txt_name aux)))

(dia9_2 "input.txt")