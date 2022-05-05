# Overview

## Crux
Lisp is an abbreviation for **Lis**t **P**rocessor. Every Lisp program is simply a list of lists. The elements of these lists can either be atomic values such as an integer or a string or another list. Thus a Lisp program is a recursive list as shown below. 

![List Recursion](images/list.png)


The Lisp interpreter is simply a software program that parses the **text** of a Lisp program and creates an in-memory List-based recursive data structure. Once the Lisp program is represented as a data structure, interpreting the program simply involves recursively evaluating these lists. This is the core of what a Lisp interpreter does, there is nothing more to it. Simple and Beautiful.




