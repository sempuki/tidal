# Q-0275: Can a template template parameter stand without arguments for class template argument deduction?

**Status:** provisional
**Affects:** CTAD-1, `tests/templates/class-template-deduction/template-template-parameter.cpp`, `template-template-parameter-alias.cpp`

## Context

`template <template <class> class W> int f() { W w(1); ... }` and
then `f<Box>()`, where `Box(T)` is a constructor of `Box`. A second
form uses a member alias `template <class T> using A = W<T>;` and
then `A a(1);`.

| Compiler | `W w(1);` | `A a(1);` |
|---|---|---|
| Clang 22, host | accepts, `Box<int>` | accepts, `Box<int>` |
| Clang 18 | accepts, `Box<int>` | rejects: no deduction through an alias (Q-0240) |
| GCC 14, 16, host | accepts, `Box<int>` | accepts, `Box<int>` |
| EDG 7.0 | accepts, `Box<int>` | accepts, `Box<int>` |

## Evidence

The tests above. Item 20 of the gap review `reports/review/2026-10-31-gap-review-packs-and-ctad.md` says that a template
template parameter is never usable for deduction, nor an alias whose
type is a specialization of one, and predicts a rejection.

## Options

1. Deduction uses the class template that the parameter names after
   substitution, as every compiler does.
2. The program is not valid, as the gap review predicts.

## Provisional answer

Yes; every compiler accepts (option 1). Every implementation deduces, and the specification follows
the behavior that implementations share.

The question stays open under decision 0051 (2026-11-01). Every newest version accepts both forms, but the gap review (item 20) objects to the answer and predicts a rejection.

Kept on 2026-11-03. The source-reader gap review of question cluster 1 (`reports/review/2026-11-01-gap-review-question-cluster-1.md`) reads the reference as supporting option 2 only weakly, on a strict reading, since the reference does not mention template template parameters. Clang 22, GCC 16, and EDG 7.0 deduce through both forms, and Clang 18 differs only on the alias form, for a reason that Q-0240 covers. Option 2 would make CTAD-1 reject a program that every implementation accepts. The specification follows the shared behavior until the maintainer decides, as for Q-0300.

Kept open on 2026-11-03 under decision 0054, because the question waits on the maintainer (`documentation/decisions/pending.md`).

## Opinions

### Gap review, 2026-11-01

The source-reader gap review of question cluster 1 (`reports/review/2026-11-01-gap-review-question-cluster-1.md`) reads the reference as weakly supporting option 2. A writer weighs the change with its rules and tests.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
