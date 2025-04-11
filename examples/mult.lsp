(define mult (lambda (a b) (
    (if (< b 2) (
        a
    ) (
        (+ a ('mult (a (- b 1))))
    ))
)))

(print ("What's the a?"))
(define a (readn))

(print ("What's the b?"))
(define b (readn))

(print ('mult (a b)) )
