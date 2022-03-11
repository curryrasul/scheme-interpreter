
(define (adder x) (lambda (y) (+ x y)))

(display ((adder 5) 7))
(newline)

(define (fib n)
    (if (= n 0)
        1
        (if (= n 1)
            1
            (+ (fib (- n 1))
               (fib (- n 2))
))))

(display (fib 5))
(newline)
