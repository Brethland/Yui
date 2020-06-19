# Yui
Yui is a strongly-typed, interpreter-based language.

## Introduction
Yui is a language designed for shell script.

I write it to improve my Rust skill. So far I have not implemented the whole interpreter, while I still welcome advices and codes.

A sample of Yui Grammar is:

```yui
(import "ritsu")
# K-ON! 
# This is a comment
type Vec (S: Any) { # This is also a comment
    (nil)
    (cons S Vec)
}

type Prod (S: Any P: Any) {
    (prod S P)
}

(scope "yui")
    (open "mio")

    (let (incr: Int x: Int)
        (+ x 1))

    (generic (map: Vec xs: Vec fun: <Arrow Any Any>)
        (match xs  # This is also a comment
            (nil nil)
            ((cons x xs) (cons (fun x) (map xs fun)))))

    (say (map (cons 1 (cons 2 nil)) incr))

    (let prod: Prod@<Int String> (prod 1 "hello"))

(end "yui")
```

I am focusing on the type checker of Yui now and it is a bit difficult to me.
