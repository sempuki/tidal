# Q-0396: Does partial ordering remove the references inside the function types of ORD-9?

**Status:** provisional
**Affects:** ORD-9, ORD-2, `tests/templates/ordering/address-reference-against-value.cpp`
**Source:** finding I2-5 of `reports/clean-room/I2/report.md`, from the probes that a clean-room writer ran for it.

## Context

For the address of an overload set, a declaration of a specialization,
and a placement deallocation function, the whole function types take
part in partial ordering (ORD-9). The question is whether the function
types form one pair, compared as written, or one pair for each
parameter, on which ORD-2 removes references and qualifiers.

The test takes the address of `read` with the target type `int(int&)`.
Two templates give that type: `read(T)` with `T = int&`, and
`read(T&)` with `T = int`.

| Implementation | Result |
|---|---|
| Clang 22 | rejects: the address is ambiguous |
| GCC 16 | chooses `read(T&)` |
| EDG 7.0 | chooses `read(T&)` |

With one pair, `read(T)` is deduced from the invented `void(X&)` with
`T = X&`, and `read(T&)` is not deduced from `void(Y)`. So `read(T&)`
is more specialized. With one pair for each parameter, ORD-2 removes
the reference, both directions succeed, and neither ORD-10 nor ORD-11
applies, because only one of the two types was a reference.

`address-reference-kinds.cpp` compares `read(T&)` and `read(T&&)` for
the target `int(int&)`. Every compiler chooses `read(T&)`, so that pair
does not separate the options.

## Evidence

The test in **Affects**, with an override for Clang.

## Options

1. The function types form one pair, and ORD-2, ORD-10, and ORD-11 do
   not act inside it (GCC, EDG).
2. Each parameter and the result form a pair, as in a call, and ORD-2
   removes references from each (Clang).

## Provisional answer

One pair of whole function types (option 1). Two of three families
agree, and ORD-9 says that the whole function types take part.

The question stays open under decision 0051, because Clang 22 differs
from GCC 16 and EDG 7.0.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
