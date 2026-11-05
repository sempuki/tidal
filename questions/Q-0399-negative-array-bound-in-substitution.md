# Q-0399: Is a negative array bound that substitution makes a substitution failure?

**Status:** provisional
**Affects:** DED-24, `tests/templates/deduction/substitution-failure-negative-bound-computed.cpp`, `tests/templates/deduction/substitution-failure-negative-bound-ellipsis.cpp`
**Source:** finding R9C-4 of `reports/clean-room/R9/report-c.md`

## Context

An array bound must be greater than zero (DCL-19). Substitution
can make a parameter type with a bound of zero or less. Every compiler
removes the template when the bound is zero, also when an expression
such as `sizeof(T) - sizeof(T)` gives the zero
(`substitution-failure-forms.cpp`). Every compiler also removes it when
an explicit argument of `-1` is the whole bound of a reference to an
array, or of a pointer to an array when the other overload takes `int`
(`substitution-failure-negative-bound.cpp`). The compilers differ in two
forms:

```cpp
template <int N> int shorter(char (&)[N - 2]);
template <int N> long shorter(...);
auto a = shorter<1>(0);        // form 1: a computed bound of -1

template <int N> int by_pointer(char (*)[N]);
template <int N> long by_pointer(...);
auto b = by_pointer<-1>(0);    // form 2: a bound of -1, other overload with an ellipsis
```

| Form | Clang 18, Clang 22 | GCC 14, GCC 16 | EDG 7.0 |
|---|---|---|---|
| 1: a computed negative bound | removes the template | rejects: the size of the array is not an integral constant expression | removes the template |
| 2: a negative bound, other overload with an ellipsis | removes the template | removes the template | rejects: the size of an array must be greater than zero |

EDG also rejects a pointer to an array of bound `N` in the result type
for `N = -1`.

## Evidence

The two tests in **Affects**. Each expects the template to drop out and
records the compiler that rejects as an override.

## Options

1. A bound of zero or less is a type that is not valid, so the template
   is removed in every form (Clang).
2. A computed negative bound is an error in the program (GCC, form 1).
3. A negative bound in a pointer type is an error in the program in
   some contexts (EDG, form 2).

## Provisional answer

Yes; the template is removed in every form (option 1). DED-24 makes every type that is
not valid a substitution failure, and DCL-19 makes such a bound not
valid. Clang gives this answer in every form, and in each form one
newest family differs.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
