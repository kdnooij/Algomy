# Algomy

A very simple computer algebra system (CAS) inspired by Mathematica.

## Setup

To compile Algomy, the [Rust compiler](https://www.rust-lang.org/tools/install) (version `1.67.0-nightly` or higher) must be installed.
```shell
git clone https://github.com/kdnooij/Algomy.git
cd Algomy
cargo build
```

## Introduction

### Input modes

Currently, Algomy supports two input modes: REPL and file input. REPL-mode is the default when running the executable without command-line arguments. A file can be processed with the `-i <path>` command-line argument.
In both modes, lines are read and results are written to the standard output when applicable.

### Supported objects

Algomy currently supports the following mathematical objects and operations:
| Object | Example | Operations | 
| ------ | ------- | ---------- |
| Integer | `5`, `-3` | `+`, `-`, `*`, `/`, `^`, `!` |
| Rational | `4/3`, `-2/5` | See above |
| Gaussian rational | `I`, `3 + 2/3*I` | See above |
| Symbol | `x`, `S` |  |
| Predicates | `True`, `False` | `Not[predicate]` ( $\neg p$ ), `And[p1, p2, ...]` ( $p\land q$ ), `Or[p1, p2, ...]` ( $p\lor q$ )|
| Finite set | `{}`, `{1, 2, 3}`, `{x, y}` | `Union[set1, set2]` ( $A\cup B$ ), `Intersection[set1, set2]` ( $A\cap B$ ), `Difference[set1, set2]` ( $A\setminus B$ ), `Member[element, set]` ( $x \in S$ ) |

In addition, a number of operations on single-variable polynomials are supported.

### Assignments

An expression can be assigned to a symbol (`symbol := expr`), such that it is substituted in all subsequent expressions.
Example:
```nb
> x := 5
> x
5
> x := 3
> x + 10
13
```

### Functions
The following functions are supported:

| Function | Description |
| -------- | ----------- |
| `Numerator[expr]` | Computes the numerator of an expression |
| `Denominator[expr]` | Computes the denominator of an expression |
| `Re[expr]` | Computes the real part of an expression |
| `Im[expr]` | Computes the imaginary part of an expression |
| `Expand[expr]` | Expand expression algebraically, such that the top-level operation is a sum |
| `Coefficient[expr, var, exp]` | Computes the sum of the coefficients of all monomials with a variable part of the form `var^exp` |
| `PolynomialQuotient[expr1, expr2, var]` | Computes the quotient of the division of two single-variable polynomials in `var` |
| `PolynomialRemainder[expr1, expr2, var]` | Computes the remainder of the division of two single-variable polynomials in `var` |
| `Variables` | Returns a set containing the variables present in a multi-variable polynomial  |
| `FreeOf[expr1, expr2]` | Checks whether `expr2` is equal to a sub-expression of `expr1` |
| `Substitute[expr1, expr2, expr3]` | Substitutes every subexpression in `expr1` equaling `expr2` with `expr3` |

### REPL commands

In REPL-mode, Algomy accepts a number of commands. These are always prefixed with a double colon.

| Command | Description |
| ------- | ----------- |
| `:Exit` | Exit the session |
| `:ClearSession` | Clear all assignments in the session |
| `:Clear <var>` | Clear the assignment to a specific variable |

## References

During development, the following sources were consulted:

* Cohen, Joel S. (2002). *Computer Algebra and Symbolic Computation: Elementary Algorithms*
* Cohen, Joel S. (2003). *Computer Algebra and Symbolic Computation: Mathematical Methods*