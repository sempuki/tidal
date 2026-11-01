# Q-0278: Is `T&&` of a class parameter a forwarding reference in a guide made through an alias template?

**Status:** provisional
**Affects:** CTAD-28, `tests/templates/class-template-deduction/alias-template-rvalue-reference.cpp`

## Context

`template <class T> struct Ref { Ref(T&&); };`,
`template <class T> using Alias = Ref<T>;`, and then `Alias a(i);`
with an `int` lvalue `i`. Without the alias, `Ref r(i);` is rejected
on every compiler (CTAD-8).

| Compiler | Result |
|---|---|
| Clang 22, host | rejects: no viable guide |
| Clang 18 | rejects: no deduction through an alias (Q-0240) |
| GCC 14, 16, host | rejects: deduction failed |
| EDG 7.0 | accepts |

## Evidence

The test above. Item 32 of the gap review `reports/review/2026-10-31-gap-review-packs-and-ctad.md` says that the alias
parameter stands for a class parameter, so the reference is not a
forwarding reference.

## Options

1. The parameter keeps its kind, and the lvalue does not bind (Clang,
   GCC).
2. The alias parameter is a parameter of the new guide, so `T&&` is a
   forwarding reference (EDG).

## Provisional answer

No; Clang and GCC agree, EDG differs (option 1). Two of three implementations agree, and the alias then gives
the same result as the class template.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
