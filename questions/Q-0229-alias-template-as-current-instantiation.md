# Q-0229: Does an alias template specialization name the current instantiation?

**Status:** provisional
**Affects:** DEP-15, `tests/templates/dependent-names/current-instantiation-alias.cpp`

## Context

`template <class U> using Same = Box<U>;` and, inside `Box<T>`, the
name `Same<T>`.

| Compiler | Member type without `typename` | Missing member, no dependent base, unused | Member of a dependent base |
|---|---|---|---|
| Clang 18, 22, host | accepts | rejects | accepts |
| GCC 16, host | accepts | rejects | rejects |
| GCC 14 | accepts | accepts | accepts |
| EDG 7.0 | rejects: needs `typename` | accepts | accepts |

## Evidence

The test above and the grid `current-instantiation` (row `Same<T>`).
Inventory item C1 says that an equivalent alias template
specialization names the current instantiation.

## Options

1. The alias names the current instantiation (Clang; GCC except with
   a dependent base).
2. The alias names a dependent specialization that is not the current
   instantiation (EDG).

## Provisional answer

Yes; Clang and GCC agree, EDG differs (option 1). Clang treats every case as the current instantiation, and
the inventory agrees. GCC's rejection of a dependent base member
through the alias is a separate defect under DEP-18.

The question stays open under decision 0051 (2026-11-03). Clang 22 and GCC 16 accept the member type through `Same<T>` without `typename`, and EDG 7.0 asks for `typename`, so two families disagree. Probes of the other two columns give the recorded results: Clang 22 and GCC 16 reject the missing member while EDG 7.0 and GCC 14 accept it, and GCC 16 alone rejects the member of a dependent base. DEP-P4 supports the provisional answer. Its table names the current instantiation by any name for `Box<T>`, and `Same<T>` is the same type as `Box<T>`. EDG's behavior is on the cluster 5 list of EDG differences that waits on the maintainer (`documentation/status.md`).

Kept on 2026-11-03. The source-reader gap review of question cluster 5 agrees with option 1.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 5 (`reports/review/2026-11-03-gap-review-question-cluster-5.md`) reads the reference as supporting option 1. The reference's definition of the current instantiation names an equivalent alias template specialization directly.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
