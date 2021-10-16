(define le-input
  (lambda (ficheiro txt)
    (if (eof-object? (peek-char ficheiro))
        txt
        (le-input ficheiro (string-append txt (string (read-char ficheiro)))))))

(define conta-grupos
  (lambda (txt i counter level lixo)
    (if (>= i (string-length txt))
        counter
        (cond
          ((and lixo (char=? (string-ref txt i) #\!)) (conta-grupos txt (+ i 2) counter level lixo))
          ((and (not lixo) (char=? (string-ref txt i) #\<)) (conta-grupos txt (add1 i) counter level #t))
          ((and lixo (char=? (string-ref txt i) #\>)) (conta-grupos txt (add1 i) counter level #f))
          ((and (not lixo) (char=? (string-ref txt i) #\{)) (conta-grupos txt (add1 i) (+ level counter) (add1 level) lixo))
          ((and (not lixo) (char=? (string-ref txt i) #\})) (conta-grupos txt (add1 i) counter (sub1 level) lixo))
          (else (conta-grupos txt (add1 i) counter level lixo))))))
            
(define aux
  (lambda (ficheiro)
      (conta-grupos (le-input ficheiro (make-string 0)) 0 0 1 #f)))

(define dia9_1
  (lambda (txt_name)
    (call-with-input-file txt_name aux)))

(dia9_1 "input.txt")