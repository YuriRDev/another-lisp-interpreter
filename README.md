# YALI - Yet Another Lisp Interpreter
Wowww, omggg, another Lisp Interpreter!! That's so original!!
I bet you learned a lot from doing this, didn't u?...

## Technical Decisions
* **Heterogenous ASTs**: Because homogenous sucks. There are just a few nodes that really need children. Using a homogenous type would force us to waste memory _(a lot)_.
* **Parse don't just parse**: For now! For enhancing static types and ""optimizations"", we are moving this to another tree walk.
* **Monolithic scopes**: We only have one scope... And for the future `(lambda ...)` implementations we are going to change a few things.

## WIP
- [x] Lambda
- [ ] REPL _(Read-Eval-Print Loop)_.
- [ ] Read file instead of just hardcoding it... Obviously.
- [ ] Better error messages. (Current one it's terrible)
- [ ] Should use graphemes. `unicode-segmentation`
- [ ] Use static lifetimes instead of just `.clone()` everything. lol 
    That's gonna change the grammar into a CSG... We are going to use the
    symbol table at the parser...
- [ ] TokenType `FunCall` can be extinct.

## Examples
```lisp
(
    (+ 1 2)         ;  3
    (+ 2 2 2)       ;  6
    (* 4 2 1) 
)
```

### Defining Variables
```
(define <ID> (<expr>))
(define (x) ) | (define x ("asd")) | (define x (true)) | (define x (false))
(define a (x)) | (define x (+ 1 2)) | ...
```

### Comparison
Lol, we only have three comparisons
```md
(< 1 2) ;; < number number
(> 2 1) ;; > number number
(= 1 2) ;; = number number | string string | boolean boolean 
```

### Arithmetic
```md
(+ 1 2 3 4 5...)
(- 1 2 3 4 5...)
```

### Conditionals
`(if (<expr>) (<expr>) (<expr>))`

```md
(if (> x 2) (
            define g 2
            ) (
                if (= x 4) (define g 4) (define g 1)
            ))
```
