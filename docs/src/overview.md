# Overview

Lisp is an abbrevation for **Lis**t **P**rocessor. Every Lisp program is recursive data structure made of lists as shown below

![List Recursion](images/list.png)

The goal of the interpreter is to parse the **text** of a Lisp program and create an in-memory List based recurive data structure. Once the data structure is created for the entire program, interpreter the program simply involves recurively evaluating these lists. 