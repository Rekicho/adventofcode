(define le-input
  (lambda (ficheiro linhas par numero caracter bool)
    (if (eof-object? caracter)
        (begin
          (set-cdr! par (append (cdr par) (list numero)))
          (append linhas (list par)))
        (if bool
            (begin
              (read ficheiro)
              (read-char ficheiro)
              (le-input ficheiro linhas par 0 (read-char ficheiro) #f))
            (if (char=? caracter #\return)
                (begin
                  (read-char ficheiro)
                  (set-cdr! par (append (cdr par) (list numero)))
                  (le-input ficheiro (append linhas (list par)) (cons (read ficheiro) (list)) 0 (read-char ficheiro) #t))
                (if (char=? caracter #\,)
                    (begin
                      (read-char ficheiro)
                      (set-cdr! par (append (cdr par) (list numero)))
                      (le-input ficheiro linhas par 0 (read-char ficheiro) #f))
                    (le-input ficheiro linhas par (+ (* numero 10) (- (char->integer caracter) 48)) (read-char ficheiro) #f)))))))

(define conta-prog
  (lambda (ok lista bool again repete)
    (letrec((procura?
             (lambda (valor ll)
               (if (null? ll)
                   #f
                   (if (= valor (car ll))
                       #t
                       (procura? valor (cdr ll))))))
            (procura-l?
             (lambda (l1 l2)
               (if (and (null? l2) (= (length l1) 1))
                   #f
                   (if (null? l2)
                       (procura-l? (cdr l1) ok)
                       (if (= (car l1) (car l2))
                           #t
                           (procura-l? l1 (cdr l2))))))))
      (if (or (null? lista) (and (= (length lista) 1) (= (length (car lista)) 1)))
          (if again
              (conta-prog ok repete #f #f (reverse repete))
              ok)
          (if (= (length (car lista)) 1)
              (conta-prog ok (cdr lista) #f again repete)
              (if bool
                  (if (and (procura? (caar lista) ok) (boolean? (member (cadar lista) ok)))
                      (conta-prog (append ok (list (cadar lista))) (append (list (append (list (caar lista)) (cddar lista))) (cdr lista)) #t #t repete)
                      (conta-prog ok (cdr lista) #f again repete))
                  (if (and (procura-l? (cdar lista) ok) (boolean? (member (caar lista) ok)))
                      (conta-prog (append ok (list (caar lista))) lista #t #t repete)
                      (conta-prog ok lista #t again repete))))))))

(define already?
  (lambda (grupos i)
    (if (null? grupos)
        #f
        (if (boolean? (member i (car grupos)))
            (already? (cdr grupos) i)
            #t))))

(define repetido
  (lambda (input grupos i)
    (if (>= i (length input))
        (length grupos)
        (if (already? grupos i)
            (repetido input grupos (add1 i))
            (repetido input (append grupos (list (conta-prog (list i) input #f #f (reverse input)))) (add1 i))))))
       
(define aux
  (lambda (ficheiro)
      (repetido (le-input ficheiro (list) (cons (read ficheiro) (list)) 0 (read-char ficheiro) #t) (list) 0)))

(define dia12_2
  (lambda (txt_name)
    (call-with-input-file txt_name aux)))

(dia12_2 "input.txt")