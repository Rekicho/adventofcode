(define tripaloski
  (lambda (lista)
    (letrec ((tripleaux
              (lambda (list min max)
                (if (null? list)
                    (- max min)
                    (begin
                      (if (> (car list) max) (set! max (car list)))
                      (if (< (car list) min) (set! min (car list)))
                      (tripleaux (cdr list) min max))))))
      (tripleaux (cdr lista) (car lista) (car lista)))))

(define aux
  (lambda (ficheiro)
    (letrec ((aux_aux
              (lambda (counter fich)
                  (if (eof-object? (peek-char fich))
                      counter
                      (begin (set! counter (+ counter (tripaloski (list (read fich) (read fich) (read fich) (read fich) (read fich) (read fich) (read fich) (read fich) (read fich) (read fich) (read fich) (read fich) (read fich) (read fich) (read fich) (read fich))))) (aux_aux counter fich))))))
      (aux_aux 0 ficheiro))))

(define dia1_1
  (lambda (txt_name)
    (call-with-input-file txt_name aux)))

(dia1_1 "input.txt")