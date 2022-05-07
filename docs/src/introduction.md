# Introduction

Lisp is an abbreviation for **Lis**t **P**rocessor. Every Lisp program is simply a list. The elements of this list can either be atomic values such as an integer or a string or another list. Thus a Lisp program is a recursive list as shown below. 

![List Recursion](images/list.png)

The Lisp interpreter is a software program that parses the **text** of a Lisp program and creates an in-memory List-based recursive data structure. Once the Lisp program is represented as a data structure, interpreting the program involves recursively evaluating these lists. This is the core of what a Lisp interpreter does, there is nothing more to it. Simple and Beautiful.

The following chapters in this book with walk you through the code that implements this interpreter. The interpreting process can be broken down into three phases.

- Lexing: Converting the Lisp program text into a stream of tokens.
- Parsing: Converting the stream of tokens into an in-memory recursive list data structure.
- Evaluation: Walking the in-memory recursive list and producing the final result.

In addition, the final chapter will implement a simple REPL to evaluate the Lisp program. 






