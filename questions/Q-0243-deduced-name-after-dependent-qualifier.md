# Q-0243: Can a deduced class template name follow a dependent qualifier?

**Status:** provisional
**Affects:** CTAD-1, `tests/templates/class-template-deduction/dependent-qualifier.cpp`, `dependent-qualifier-template-keyword.cpp`
**Waits for:** the maintainer (pending.md row 4)

## Context

In a function template, `typename Outer<T>::In x(1);`, where `In` is a
member class template of `Outer<T>`.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | accepts, `Outer<T>::In<int>` after instantiation |
| GCC 14, 16, host | accepts, the same type |
| EDG 7.0 | accepts, the same type |

The source reader's inventory (item G7) says that the qualifier before
a deduced name must not be dependent. Item 19 of the gap review
`reports/review/2026-10-31-gap-review-packs-and-ctad.md` reads the
reference the same way, and notes that the compilers accept the
dependent form once the template is instantiated.

The compilers differ when the name has the keyword `template` before
it: `typename Outer<T>::template In x(1);`.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | rejects: an argument list is expected after the name |
| GCC 14, 16, host | accepts, `Outer<T>::In<int>` |
| EDG 7.0 | accepts, `Outer<T>::In<int>` |

## Evidence

The tests above.

## Options

1. The qualifier can be dependent, as every compiler accepts.
2. The qualifier must not be dependent, as the inventory and the gap
   review read the reference.

## Provisional answer

Yes; every compiler accepts (option 1). Every implementation accepts the form without the keyword
`template`, and the specification follows the behavior that
implementations share. The form with `template` follows the same
answer, which GCC and EDG give. The question keeps both readings open
for contributors.

The question stays open under decision 0051 (2026-11-01). Clang 22 rejects the form with the keyword `template`, and GCC 16 and EDG 7.0 accept it, so two families disagree.

Kept on 2026-11-03. The source-reader gap review of question cluster 1 (`reports/review/2026-11-01-gap-review-question-cluster-1.md`) reads the reference as supporting option 2 for both forms. Clang 22, GCC 16, and EDG 7.0 all accept the form without the keyword `template`, so option 2 would make CTAD-1 reject a program that every implementation accepts. The specification follows the shared behavior of the matrix there, as for Q-0300, until the maintainer decides. The keyword `template` only marks the name as a template, so the form with the keyword keeps the same answer, which GCC 16 and EDG 7.0 give. Clang 22 rejects that form for a missing argument list, and the review reads both forms alike, so Clang's rejection does not separate the two forms.

Kept open on 2026-11-03 under decision 0054, because the question waits on the maintainer (`documentation/decisions/pending.md`).

Kept in `pending.md` row 4 on 2026-11-11 (decision 0063, item RV8). The gap review of the phase 15 text names the question and gives no new reading.

## Opinions

### Gap review, 2026-11-01

The source-reader gap review of question cluster 1 (`reports/review/2026-11-01-gap-review-question-cluster-1.md`) reads the reference as supporting option 2. A writer weighs the change to the provisional answer with its rules and tests.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Gap review, 2026-10-31

The source-reader gap review of packs and class template argument deduction (`reports/review/2026-10-31-gap-review-packs-and-ctad.md`) reads the qualifier before a deduced class template name as required to be non-dependent, so the reading rejects the program. Compilers accept the dependent form once the template is instantiated. The reading goes against the provisional answer, as the cluster 1 review does, and the review asks the question to keep both.

### Gap review, 2026-11-11 (phase 15 text, expressions, overloading, and templates)

The source-reader gap review of the phase 15 text on expressions, overloading, and templates (`reports/review/2026-11-11-gap-review-phase-15-expressions-overloading-templates.md`) names the question in its reading of CTAD-1. The older sentence of CTAD-1 on a dependent qualifier stays with this question in `pending.md` row 4. The review gives no new reading of the case.
