# Q-0027: May a member function with a ref-qualifier be overloaded with one without?

**Status:** provisional
**Affects:** CLS-6, `tests/classes/members/ref-qualified-mixed.cpp`

## Context

`struct S { void f() &; void f(); };` declares one overload of `f` with a
ref-qualifier and one without.

| Compiler | Result |
|---|---|
| Clang 22, 18 | rejects the declarations |
| GCC 14 | rejects: "cannot be overloaded with 'void S::f() &'" |
| GCC 16 | accepts the declarations; a call `s.f()` is ambiguous |
| EDG 7.0 | rejects the declarations |

## Options

1. The declarations are rejected.
2. The declarations are valid, and a call that can't choose between them
   is rejected, as GCC 16 does.

## Provisional answer

Option 1. Three implementations, including the older GCC, reject the
declarations. GCC 16's change is recorded here so contributors can say
whether it reflects a deliberate language change.

## Opinions
