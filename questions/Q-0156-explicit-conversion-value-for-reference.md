# Q-0156: Can direct initialization of an rvalue reference use an explicit conversion function that returns a value?

**Status:** provisional
**Affects:** ICS-45, `tests/foundations/conversion-sequences/user-defined-explicit-rvalue-reference-value.cpp`

## Context

A class `Source` has only `explicit operator Item()`. A reference is
direct-initialized from a `Source` lvalue, for example
`const Item& r(s);` or `static_cast<Item&&>(s)`. The conversion function
returns a prvalue, so the reference would bind to that prvalue or to a
temporary.

| Compiler | `const Item&` | `Item&&` | `int&&` from `explicit operator int()` |
|---|---|---|---|
| Clang 18, Clang 22 | accepted | accepted | accepted |
| GCC 14, GCC 16 | rejected | rejected | rejected |
| EDG 7.0 | rejected | rejected | rejected |

Every compiler accepts `const Item&` from `explicit operator Item&()`,
where the reference binds directly to an lvalue. Every compiler also
accepts `Item&&` from `explicit operator Item&&()`. Every compiler
rejects `long&&` from `explicit operator int()` and `Base&&` from
`explicit operator Derived()`, where the result needs another
conversion. Clang accepts `const Item&&` from
`explicit operator Item()`, and GCC and EDG reject it.

The first column, an lvalue reference to `const`, is now
Q-0337. This question
covers the two rvalue reference columns.

## Evidence

`tests/foundations/conversion-sequences/user-defined-explicit-rvalue-reference-value.cpp`
and `user-defined-explicit-reference-xvalue.cpp`.

The table gives, for `static_cast` to each reference, whether the
`explicit` conversion function serves the cast, under ICS-45 and on
each compiler. `T` is a class type or `int`, and the results are the
same for both. `U` is a base of the class, or `long` for `int`. A cell
with one answer holds on every compiler and agrees with ICS-45.

| Target | `explicit operator T()` | `explicit operator T&()` | `explicit operator T&&()` |
|---|---|---|---|
| `T&&` | rule: yes; Clang: yes; GCC, EDG: no | no | yes |
| `const T&` | rule: no; Clang: yes; GCC, EDG: no | yes | rule: no; Clang, EDG: yes; GCC: no |
| `T&` | no | yes | no |
| `U&&` or `const U&` | no | no | no |

The tests are `user-defined-explicit-rvalue-reference-value.cpp`,
`user-defined-explicit-reference-value.cpp`,
`user-defined-explicit-reference-xvalue.cpp`, and
`user-defined-explicit-reference-other-type.cpp`. The `const T&` row is
Q-0337.

## Options

1. Only an explicit conversion function that returns an lvalue reference
   serves a reference (GCC, EDG).
2. An explicit conversion function that returns a value also serves a
   reference in direct initialization (Clang).

## Provisional answer

Yes, for an rvalue reference; the reference binds directly to the value (option 2). Clang agrees, and GCC and EDG reject.

Changed on 2026-11-03. The source-reader gap review of question cluster 2 (`reports/review/2026-11-03-gap-review-question-cluster-2.md`) reads the reference as supporting option 2 for the two rvalue reference columns, and option 1, weakly, for the lvalue reference to `const`. The review matches Clang 22 for the rvalue references, and it matches Tidal's own rules. OVL-15 gives an rvalue reference the conversion functions that return a value, and ICS-45 adds the `explicit` ones in direct initialization when only a qualification adjustment is left. An lvalue reference gets only the functions that return an lvalue, so the two kinds of reference differ. As the review suggests, the question is split. The lvalue reference to `const` moves to Q-0337. ICS-45 now gives option 2 for an rvalue reference, and the test `user-defined-explicit-rvalue-reference-value.cpp` expects it, with overrides for GCC and EDG.

The question stays open under decision 0051 (2026-11-03). Clang 22 accepts both rvalue reference forms, and GCC 16 and EDG 7.0 reject them, so two families disagree.

Kept open on 2026-11-03 under decision 0054, because the gap review does not support the answer with no doubt (it reads with doubt, weakly, unclearly, or for only part of the question).

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 2 (`reports/review/2026-11-03-gap-review-question-cluster-2.md`) reads the reference as supporting option 2 for the rvalue references. A cast to an rvalue reference, and its direct initialization, consider conversion functions that return a value, and the explicit ones are allowed when their result reaches the target by a qualification conversion. The reference binds directly to the materialized result. For the lvalue reference to `const`, the review reads the text as supporting option 1, with doubt, and suggests a split of the question.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
