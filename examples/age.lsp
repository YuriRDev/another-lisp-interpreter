(define mult 
    (lambda (a b) 
    (if (= b 1) (a) (+ a ('mult (a (+ b (-1))))))
    )
)

(define fat 
    (lambda (x) 
        (if (< x 1) 
            (1) 
            ('mult (x ('fat ((+ x (-1))))))
    )
))

(print ('mult (2 600)))
