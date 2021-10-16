(define input 265149)

(define aux
  (lambda (number matriz i j exp)
    (letrec ((adiciona-zero!
              (lambda (i)
                (if (not (>= i (vector-length matriz)))
                    (begin
                      (vector-set! matriz i (list->vector (append (list 0) (vector->list (vector-ref matriz i)) (list 0))))
                      (adiciona-zero! (add1 i))))))
             (expande
              (lambda ()
                (begin
                  (adiciona-zero! 0)
                  (set! matriz (list->vector (append (list (make-vector (+ (vector-length matriz) 2) 0)) (vector->list matriz) (list (make-vector (+ (vector-length matriz) 2) 0))))))))
             (se-possivel
              (lambda (k l)
                (if (or (< k 0) (> k (sub1 (vector-length matriz))) (< l 0) (> l (sub1 (vector-length matriz))))
                    0
                    (vector-ref (vector-ref matriz k) l))))
             (calcula-val
              (lambda ()
                (begin
                  (vector-set! (vector-ref matriz i) j (+ (se-possivel (sub1 i) (sub1 j)) (se-possivel (sub1 i) j) (se-possivel (sub1 i) (add1 j)) (se-possivel i (sub1 j)) (se-possivel i (add1 j)) (se-possivel (add1 i) (sub1 j)) (se-possivel (add1 i) j) (se-possivel (add1 i) (add1 j))))
                  (vector-ref (vector-ref matriz i) j)))))
      (if exp
          (begin
            (expande)
            (aux number matriz (add1 i) (+ j 2) #f))
          (if (zero? (vector-ref (vector-ref matriz i) j))
              (if (> (calcula-val) number)
                  (vector-ref (vector-ref matriz i) j)
                  (cond
                    ((and (= i (sub1 (vector-length matriz))) (= i j)) (aux number matriz i j #t))
                    ((and (= j (sub1 (vector-length matriz))) (not (= i 0))) (aux number matriz (sub1 i) j #f))
                    ((and (= i 0) (not (= j 0))) (aux number matriz i (sub1 j) #f))
                    ((and (= j 0) (not (= i (sub1 (vector-length matriz))))) (aux number matriz (add1 i) j #f))
                    ((and (= i (sub1 (vector-length matriz))) (not (= i j))) (aux number matriz i (add1 j) #f)))))))))
            
(define day3_2
  (lambda (number)
    (aux number (make-vector 1 (make-vector 1 1)) 0 0 #t)))

(day3_2 input)