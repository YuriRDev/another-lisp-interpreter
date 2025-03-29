# YALSDI - Yet Another Lisp Syntax-Directed Interpreter
Wowww, omggg, another Syntax-Directed Lisp Interpreter!! That's so original!!
I bet you learned a lot from doing this, didn't u?...


## Examples
```lisp
(
    (+ 1 2)         ; outputs 3
    (+ 2 2 2)       ; outputs 6
    (* 4 2 1) 
)
```

## What you can do
We have little keywords, but enought to be turing-complete.

### Defining Variables
```
(define x 3) | (define x "asd") | (define x true) | (define x false)
(define a x) | (define x (+ 1 2)) | ...
```

### Comparison
Lol, we only have three comparisons
```md
(< 1 2) ;; < number number
(> 2 1) ;; > number number
(= 1 2) ;; = number number | string string | boolean boolean 
(! true) ;; boolean
    (! (= 1 1)) ;; returns false
```

### Arithmetic
```md
(+ 1 2 3 4 5...)
(- 1 2 3 4 5...)
(* 1 2 3 4 5...)
(/ 1 2 3 4 5...) ;; Must have at least two numbers
```

### Conditionals
`(if CONDITION (THEN) (ELSE))`

```md
(if (> x 2) (
            define g 2
            ) (
                if (= x 4) (define g 4) (define g 1)
            ))
```

### Lambda
`(lambda (PARAMS) (EXPR))` or `lambda (PARAMS) (EXPR) args` 

```md
(lambda (a b) (+ a b))
(lambda (a b) (+ a b) 5 2)
(define foo lambda (a b) (+ a b))
(define duu lambda (a b) (+ a b) 1 2) ;; duu = 3
```

## Grammar
```md
s_expression := atomic_symbol 
               | "(" s_expression "." s_expression ")" 
               | list 
   
list := "(" s_expression <s_expression> ")"

atomic_symbol = letter atom_part

atom_part = empty | letter atom_part | number atom_part

letter = "a".."z"

number = "1".."9"
```
