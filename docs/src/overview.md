# Overview

This project implements an interpreter, in Rust, for a small subset of [Scheme](https://en.wikipedia.org/wiki/Scheme_(programming_language)), a Lisp dialect.

The project was inspired by Peter Norvig's article [**(How to Write a (Lisp) Interpreter (in Python))**](http://www.norvig.com/lispy.html) and aims to make a user understand how a Lisp interpreter works by implementing one in Rust. Rust's rich programming constructs such as enum, pattern matching and error handling make it easy and a joy to implement this barebone interpreter. 

## Pre-requistes
In order to make the most out of this project, it is expected that the user is aware of the following Computer Science concepts

- Lists
- Recursions 

Rust is a non-trivial language, however to implement the Lisp interpreter, the reader needs have moderate experience with the language. Knowing the following four concepts with should be enough for the user to understand the whole project 

- [Enums and Pattern Matching](https://doc.rust-lang.org/book/ch06-00-enums.html)
- [Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
- [Error handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)

## Lisp Dialect

In order to simplify the understanding of the implementation of the interpreter, the number of features supported by it have been limited on purpose.

### Data types
- Integer
- Bool

### Statements
- variable definition and assignment
- if else
- function definition using lambdas
- function calls

### Examples

```lisp
    (
        (define fact (lambda (n) (if (< n 1) 1 (* n (fact (- n 1))))))
        (fact 5)
    )
```

```lisp
    (
        (define pix 314)
        (define r 10)
        (define sqr (lambda (r) (* r r)))
        (define area (lambda (r) (* pix (sqr r))))
        (area r)
    )
```

## Interpreter

The interpreter will be implemented from scratch and without the help of any tools such as [nom](https://docs.rs/nom/latest/nom/) or [pest](https://pest.rs/). The interperter is broken down into three phases

- [Lexer](./lexer.md) - 20 lines of code
- [Parser](./parser.md) - 60 lines of code
- [Evaluator](./evaluator.md) - 150 lines of code

## Features

Following are some of the sam