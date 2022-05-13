# Overview

The [lisp-rs](https://github.com/vishpat/lisp-rs) project implements an interpreter, in Rust, for a small subset of [Scheme](https://en.wikipedia.org/wiki/Scheme_(programming_language)), a Lisp dialect. The main goal of this document is to make sure the reader understands the inner details of how the interpreter was implemented.

The project was inspired by Peter Norvig's article [**(How to Write a (Lisp) Interpreter (in Python))**](http://www.norvig.com/lispy.html) and the book [**Writing An Interpreter In Go**](https://interpreterbook.com). This document serves as a commentary on the [code](https://github.com/vishpat/lisp-rs/tree/0.0.1) that implements the interpreter. Rust's rich programming constructs such as enum, pattern matching, and error handling make it easy and a joy to implement this bare-bone interpreter. 

## Prerequisites
To make the most out of this project, it is expected that the user is aware of the following Computer Science concepts

- [Lists](https://en.wikipedia.org/wiki/List_(abstract_data_type))
- [Recursion](https://en.wikipedia.org/wiki/Recursion_(computer_science)) 

Rust is a non-trivial language, however, to implement the Lisp interpreter, the reader needs to have moderate experience with the language. Knowing the following four concepts should be enough for the user to understand the whole project 

- [Enums and Pattern Matching](https://doc.rust-lang.org/book/ch06-00-enums.html)
- [Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
- [Error handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)

## Lisp Dialect

In order to keep the interpreter simple and its implementation easy to understand, the number of features supported by it has been limited on purpose. Following are the data types and statements that will be supported by the interpreter.

### Data types
- integer
- boolean

### Statements
- variable definition and assignment
- if-else
- function definition using lambdas
- function calls

### Keywords

- define
- if-else
- lambda

### Examples

Following are some of the sample programs that you will be able to run using the interpreter

```lisp
    (
        (define factorial (lambda (n) (if (< n 1) 1 (* n (factorial (- n 1))))))
        (factorial 5)
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

The interpreter will be implemented from scratch and without the help of any tools such as [nom](https://docs.rs/nom/latest/nom/) or [pest](https://pest.rs/). The interpreter implementation is broken down into four parts

- [Lexer](./lexer.md) ~ 20 lines of code
- [Parser](./parser.md) ~ 60 lines of code
- [Evaluator](./evaluator.md) ~ 170 lines of code
- [REPL](./repl.md) ~ 30 lines of code

The best way to understand the implementation of the interpreter is to check out the code and walk through it while reading this document. 

```
git clone https://github.com/vishpat/lisp-rs
git checkout 0.0.1
```

Once you thoroughly understand the implementation, you will be equipped to add new features to it, such as support for new data types like strings, floating-point numbers, lists, or functional programming constructs such as map, filter, reduce functions, etc. 

### REPL

To run the interpreter and get its REPL (Read-Eval-Print-Loop)

```
cargo run
```


