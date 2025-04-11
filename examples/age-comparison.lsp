(print ("How old is the first person?"))
(define x (readn))
(print ("How old is the second person?"))
(define y (readn))

(if (= x y) (print ("Both have the same age")) (
    if (> x y) (
        print("The first person is older")
    ) (
        print("The second person is older")
    )
))
