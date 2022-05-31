# lisp-rs

A simple Lisp interpreter/library in Rust. The interpreter/library was initially developed as a **teaching aid** to explain how Lisp interpreters work and how can they be implemented using the Rust programming language. However the interpreter seems to be taking life of it's own and has been ported to the [web](https://vishpat.github.io/lisp-rs-wasm) using [WASM](https://webassembly.org). The interpreter is available as a [crate](https://crates.io/crates/lisp-rs) and can be used to embed a Lisp interpreter in your Rust projects. The WASM implementation uses the lisp-rs as a library to implement the online interpreter.

## Dialect
The interpreter is based on a modified subset of [Scheme](https://en.wikipedia.org/wiki/Scheme_(programming_language)). Following are the features supported by the interpreter

- Variables and Constants
- Functions (lambdas)
- Functional constructs such as map, filter and reduce
- Closures
- Tail Call Optimization

More information about the dialect can be found at

- [Syntax](https://github.com/vishpat/lisp-rs/wiki/Lisp-Syntax)
- [Sample Programs](https://github.com/vishpat/lisp-rs/wiki/Sample-programs)

## Implementation

The interpreter has been implemented in three phases

- Phase I: Complete interpreter and REPL implementation with support for basic data types (integers and boolean), variables, lambdas, and lambda calls (branch 0.0.1)
- Phase II: Support for advanced data types (string, float, and lists) and functional constructs (branch 0.0.2)
- Phase III: Support for tail call optimization (branch 0.0.3) and closures (branch 0.0.4)


For a detailed code-walkthrough about Phase-I implementation refer to the [docs](https://vishpat.github.io/lisp-rs). For code-walkthrough of all the phases, get the [e-book](https://www.amazon.com/dp/B0B1Z48DMT/ref=cm_sw_r_apan_QD16PC5E2EZWMKB1EZMK).

[![asciicast](https://asciinema.org/a/VVQQfGpp15a4BaoNgnEKIqqrr.svg)](https://asciinema.org/a/VVQQfGpp15a4BaoNgnEKIqqrr)

## REPL
```
cargo run --features="build-binary"
```

## Test
```
cargo test
```

## WASM

The interpreter has also been compiled to WASM so that it can run in a browser and is hosted [here](https://vishpat.github.io/lisp-rs-wasm).
