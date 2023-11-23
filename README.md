# Crafting Interpreters in Rust

This is an in progress project aiming to implement both versions of the Lox Language from the book [Crafting Interpreters](https://craftinginterpreters.com/) in Rust.

## Progress

- [x] Chapter 1: Introduction
- [x] Chapter 2: A Tree-Walk Interpreter
- [x] Chapter 3: The Lox Language
- [x] Chapter 4: Scanning
- [x] Chapter 5: Representing Code
- [x] Chapter 6: Parsing Expressions
- [ ] Chapter 7: Evaluating Expressions
- [ ] Chapter 8: Statements and State
- [ ] Chapter 9: Control Flow
- [ ] Chapter 10: Functions
- [ ] Chapter 11: Resolving and Binding
- [ ] Chapter 12: Classes
- [ ] Chapter 13: Inheritance

## Things I've learned so far

- How to implement an approximation of abstract classes and the visitor pattern in Rust using traits, generics and enums. [Here](https://github.com/JohnIrle/crafting-interpreters-rs/blob/6a1e7538b76919b9e0821ee5301ebd872ee3a3a9/src/expr.rs) and [Here](https://github.com/JohnIrle/crafting-interpreters-rs/blob/6a1e7538b76919b9e0821ee5301ebd872ee3a3a9/src/ast_printer.rs)
- How to generate a source file by mapping over a grammar configuration. [Here](https://github.com/JohnIrle/crafting-interpreters-rs/blob/6a1e7538b76919b9e0821ee5301ebd872ee3a3a9/src/generate_ast.rs)
- Scanning text to produce tokens. [Here](https://github.com/JohnIrle/crafting-interpreters-rs/blob/6a1e7538b76919b9e0821ee5301ebd872ee3a3a9/src/scanner.rs)
- Parsing tokens to produce an abstract syntax tree for expressions. [Here](https://github.com/JohnIrle/crafting-interpreters-rs/blob/6a1e7538b76919b9e0821ee5301ebd872ee3a3a9/src/parser.rs)
