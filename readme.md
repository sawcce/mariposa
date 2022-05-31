- [🦋 Mariposa](#-mariposa)
- [Syntax](#syntax)
  - [Types](#types)
  - [Operators](#operators)
- [More to come!](#more-to-come)

# 🦋 Mariposa

Mariposa (from spanish: Butterfly), is a small interpreted language that aims to:
- Have a small interpreter
- Be type safe and memory safe
- Be fast

# Syntax
*The syntax definition uses [SEBNF](https://github.com/sawcce/sebnf)*

The syntax is comprised of simple operations with fixed argument numbers

For instance the `=` operator requires two arguments an id (int) and a value.

Example:
```
= 0 178
```

## Types

- Int, whole number, can be used as an id or value. Syntax: `int = digit+`
- Float, decimal number, can be used as a value. Syntax: `float = digit+ "." digit+`
- Value, copies the value of another variable, it counts as the type of the copied variable. Syntax: `"v" + int`

## Operators

For documentation purposes, an operator type definition is declared as follows: `symbol "(" (identifier ":" type,)* (identifier ":" type)? ")"`

- `=`, Assigns a variable with the given `id` and `value`. `= (id: int, value: value)`
- `+`, Adds two values to the target id, the two values must be able to be added together. `+ (a: value, b: value, target: int)`

# More to come! 