# Q-0400: Is a concept valid when its requirement is satisfied for no argument?

**Status:** provisional
**Affects:** CPT-35, `tests/templates/concepts/nested-requirement.cpp`, `tests/templates/concepts/nested-requirement-never-satisfied.cpp`
**Source:** finding R9B-9 of `reports/clean-room/R9/report-b.md`

## Context

A concept can hold a nested requirement that is valid and false for
every argument:

```cpp
template <class T> concept Never = requires { requires false; };
template <class T> concept Empty = requires { requires sizeof(T) == 0; };
```

Inventory item D1 of
`reports/review/2026-10-31-inventory-concepts-and-constraints.md` says
that a requirement that fails for every argument makes the program not
valid, with no diagnostic required. The item does not say whether
"fails" means a requirement that is not valid for any argument, or one
that is valid and never satisfied.

| Form | Clang 18, 22 | GCC 14, 16 | EDG 7.0 |
|---|---|---|---|
| `Never` and `Empty`, not used | accepts | accepts | accepts |
| `Never<int>` and `Empty<int>` in a `static_assert` with `!` | accepts, `false` | accepts, `false` | accepts, `false` |
| `Never<T>` on an overload that a call drops | accepts | accepts | accepts |
| an expression requirement that is not valid for any argument, the sum of two pointers to `T` | accepts, `false` | accepts, `false` | accepts, `false` |

## Evidence

The tests in **Affects**. Every compiler accepts both. The probes of
the last row are not tests.

## Options

1. A nested requirement that is valid and false for every argument is
   valid, and the requires expression is false for every argument
   (every compiler). Only a requirement that is not valid for any
   argument falls under inventory item D1.
2. Such a requirement makes the program not valid, with no diagnostic
   required. The tests then check only that every compiler accepts.

## Provisional answer

The concept is valid (option 1), as CPT-35 says. A nested requirement
holds when its constraint is satisfied, and `false` is a valid
constraint that is never satisfied. Every compiler agrees. Under option
2, the tests would keep their expectation, because a rule with no
diagnostic allows an implementation to accept.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
