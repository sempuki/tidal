# Q-0234: Can an explicit instantiation declaration name a specialization with internal linkage?

**Status:** provisional
**Affects:** INST-29, `tests/templates/instantiation/explicit-declaration-internal.cpp`, `tests/templates/instantiation/explicit-declaration-internal-class.cpp`, `tests/templates/instantiation/explicit-declaration-internal-variable.cpp`
**Waits for:** newest families that agree

## Context

`template <class T> static int get(T)` and `extern template int
get(int);`. The other columns declare a function template or a class
template in an unnamed namespace, with the explicit instantiation
declaration in the same namespace, and a variable template declared
`static`.

| Compiler | `static` function | function, unnamed namespace | class, unnamed namespace | `static` variable |
|---|---|---|---|---|
| Clang 18, 22, host | rejects | rejects | rejects | rejects |
| EDG 7.0 | rejects | rejects | accepts | accepts |
| GCC 14, 16, host | accepts | accepts | accepts | accepts |

An explicit instantiation definition of the same specialization is
accepted by every compiler.

## Evidence

The tests above and `explicit-definition-internal.cpp`. Clang's
diagnostic list (clang-diag-sema) has "explicit instantiation
declaration of ... with internal linkage". Inventory item G14 agrees.

## Options

1. Not valid (Clang; EDG for a function).
2. Valid (GCC).

## Provisional answer

No; Clang agrees, EDG agrees for a function, GCC differs (option 1). A specialization with internal linkage has no definition in
another unit, so the declaration could never be satisfied. For a
class and a variable, the provisional answer goes against two of three
implementations, so a contributor can argue for option 2.

The question stays open under decision 0051 (2026-11-03). Clang 22 rejects every case, GCC 16 accepts every case, and EDG 7.0 rejects only the function, so two families disagree in each case. The class and variable columns come from probes of 2026-11-03, and `explicit-declaration-internal-class.cpp` and `explicit-declaration-internal-variable.cpp` record them. INST-P3 supports the provisional answer through INST-29. An explicit instantiation declaration says that another translation unit has the definition, and no other unit can name a specialization with internal linkage.

Kept on 2026-11-03. The source-reader gap review of question cluster 6 (`reports/review/2026-11-03-gap-review-question-cluster-6.md`) agrees with option 1 for each form. INST-29 stays a `reject` rule.

Kept open on 2026-11-03 under decision 0054, because two of the newest families differ from the answer in at least one form, and the rule records one differing family.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 6 (`reports/review/2026-11-03-gap-review-question-cluster-6.md`) reads the reference as supporting option 1. When a template has internal linkage, no explicit instantiation declaration can name one of its specializations. A `static` template and a template in an unnamed namespace both have internal linkage, and the rule covers a function, a class, and a variable alike. A diagnostic is required. An explicit instantiation definition has no such limit.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
