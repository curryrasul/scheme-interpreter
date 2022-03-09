(define (fact x)
  (if (< x 2)
      1
      (* (fact (- x 1)) x)))
