# Q-0319: When is an explicit object parameter initialized, relative to the other arguments?

**Status:** provisional
**Affects:** SEQ-1, SEQ-2, `tests/expressions/sequencing/explicit-object-order-unspecified.cpp`

## Context

A member function takes its object by value, as `this Gauge self`, and
two more parameters. The call `g.read(g, next())` copies `g` into
`self`, copies `g` into the second parameter, and calls `next()`. Each
copy constructor and `next()` record their turn.

| Compiler | First event |
|---|---|
| Clang 22, 18 | the copy into `self` |
| GCC 16, at `-O0` and `-O2` | the copy into `self` |
| GCC 14 | `next()` |

The object expression `g` is evaluated before the arguments on every
compiler (SEQ-2). The question concerns the copy into `self`.

## Evidence

The test above. A first version of the test expected the copy into
`self` first, and failed on GCC 14 only.

## Options

1. The copy into `self` takes an unspecified place among the argument
   evaluations (SEQ-1), as the initialization of any parameter does.
2. The copy into `self` comes first, as part of the object expression
   (SEQ-2).

## Provisional answer

In an unspecified place (SEQ-1); GCC 14 differs from GCC 16 and Clang (option 1). GCC 14 and GCC 16 differ, and option 1 allows both. The test
accepts every order.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
