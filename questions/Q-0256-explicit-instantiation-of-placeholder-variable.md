# Q-0256: How does an explicit instantiation name a variable template declared with auto?

**Status:** provisional
**Affects:** INST-22, `tests/templates/instantiation/explicit-placeholder-variable.cpp`

## Context

`template <class T> auto zero = T(0);` is explicitly instantiated
with the deduced type or with `auto`.

| Compiler | `template int zero<int>;` | `template auto zero<int>;` | `template long zero<int>;` |
|---|---|---|---|
| Clang 18, 22, host | accepts | rejects | rejects |
| GCC 14, 16, host | accepts | rejects | accepts |
| EDG 7.0 | rejects | accepts | rejects |

## Evidence

The test above. Item 35 of the gap review `reports/review/2026-10-31-gap-review-dependent-names-and-instantiation.md` says
that the explicit instantiation writes the deduced type, which is the
opposite of the function case. The review gives no outcome for the
other forms.

## Options

1. The explicit instantiation writes the deduced type (Clang, GCC).
2. The explicit instantiation writes `auto` (EDG).

## Provisional answer

With the deduced type; Clang and GCC agree, EDG differs (option 1). Two of three implementations and the gap review agree. GCC also accepts a type other than the deduced one, and INST-22 does not allow it.

The question stays open under decision 0051 (2026-11-03). Clang 22 and GCC 16 accept the deduced type `int`, and EDG 7.0 rejects it, so two families disagree. Probes give the recorded results for the other forms: EDG 7.0 alone accepts `auto`, and GCC 16 alone accepts `long`. INST-P3 supports the provisional answer. An explicit instantiation names the type of the specialization (INST-22), and the specialization of a variable template has the deduced type. EDG's behavior is on the cluster 5 list of EDG differences that waits on the maintainer (`documentation/status.md`).

Kept on 2026-11-03. The source-reader gap review of question cluster 5 agrees with option 1.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 5 (`reports/review/2026-11-03-gap-review-question-cluster-5.md`) reads the reference as supporting option 1. A variable can use a placeholder only in a declaration with an initializer, and an explicit instantiation has no initializer, so the placeholder form is not valid. A variable can be redeclared with its deduced type. A third type differs from the type of the specialization and is not valid.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
