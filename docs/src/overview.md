# Crux

Lisp is an abbreviation for **Lis**t **P**rocessor. Every Lisp program is simply a list of lists. The elements of these lists can either be atomic values such as an integer or a string or another list. Thus a Lisp program is a recursive list as shown below. 

<p align="center">
![List Recursion](images/list.png)


The goal of the interpreter is to parse the **text** of a Lisp program and create an in-memory List-based recursive data structure. Once the data structure is created for the entire program, interpreting the program simply involves recursively evaluating these lists. This is the core of what a Lisp interpreter does, there is nothing more to it. Simple and Beautiful.


# Example 

```
(
	(define a 1)    # List 1   
	(define b 2)    # List 2
	(+ a b)         # List 3
)
```

The above Lisp program is a List of three Lists.



