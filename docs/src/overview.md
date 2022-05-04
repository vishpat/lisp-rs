# Overview

This project implements an interpreter, in Rust, for a small subset of [Scheme](https://en.wikipedia.org/wiki/Scheme_(programming_language)), a Lisp dialect.

The project was inspired by Peter Norvig's article [**(How to Write a (Lisp) Interpreter (in Python))**](http://www.norvig.com/lispy.html) and aims to make a user understand how an interpreter works by implementing one in Rust. Rust's rich programming constructs such as enum, pattern matching and error handling make it easy and a joy to implement a barebone interpreter. 

In order to make the most out of this project, it is expected that the user is aware of the following Computer Science concepts

- Lists
- Recursions 

Rust is a non-trivial language, however to implement the Lisp interpreter, the reader needs to be familiar of the following four concepts in Rust

- Enum
- Pattern Matching
- Smart Pointers
- Error handling 