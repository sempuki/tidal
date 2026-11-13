# Q-0240: Can a program deduce the arguments of a class template through an alias template?

**Status:** settled
**Affects:** CTAD-14, `tests/templates/class-template-deduction/alias-template.cpp`, `alias-template-fixed-argument.cpp`, `alias-template-fixed-argument-exact.cpp`, `alias-template-guides.cpp`, `alias-template-unreachable.cpp`, `template-template-parameter-alias.cpp`, `list-single-specialization-alias.cpp`

## Context

`template <class T> using Same = Pair<T, T>;` and then `Same s(1, 2);`.
A second form fixes an argument, as in
`template <class U> using IntPair = Pair<int, U>;`, and a third form
names an alias of an alias.

| Compiler | Alias that passes its parameters on | Alias that fixes an argument, alias of an alias, argument of the fixed type | The same, argument that converts to the fixed type |
|---|---|---|---|
| Clang 22, host | accepts | accepts | accepts |
| GCC 14, 16, host | accepts | accepts | accepts |
| EDG 7.0 | accepts | accepts | rejects: cannot deduce class template arguments |
| Clang 18 | rejects: argument deduction only allowed for class templates | rejects | rejects |

## Evidence

The tests above and the grid `forms.json` (column "alias template").

The tests give these aliases. "Converts" means that an argument for a
fixed position has another type than the fixed one. Clang 18 rejects
every row.

| Alias | Initializer | Clang 22, GCC | EDG 7.0 | Test |
|---|---|---|---|---|
| `Pair<T, T>` | `(1, 2)` | `Pair<int, int>` | `Pair<int, int>` | `alias-template.cpp` |
| `Box<T>` | `(&n)`, user guide `Box(T*)` | `Box<int>` | `Box<int>` | `alias-template-guides.cpp` |
| `Box<T*>` | `(&n)` | `Box<int*>` | `Box<int*>` | `alias-template-unreachable.cpp` |
| `Agg<int, U>`, an aggregate | `{1, 'c'}` | `Agg<int, char>` | `Agg<int, char>` | `alias-template-guides.cpp` |
| `Pair<int, U>` | `(1, 'c')` | `Pair<int, char>` | `Pair<int, char>` | `alias-template-fixed-argument-exact.cpp` |
| `Pair<int, U>` | `(1L, 'c')`, converts | `Pair<int, char>` | rejects | `alias-template-fixed-argument.cpp` |
| `Both<long, U>`, an alias of an alias | `(1L, 'c')` | `Pair<long, char>` | `Pair<long, char>` | `alias-template-fixed-argument-exact.cpp` |
| `Both<long, U>`, an alias of an alias | `(1, 'c')`, converts | `Pair<long, char>` | rejects | `alias-template-fixed-argument.cpp` |

## Options

1. Deduction works through an alias template that stands for a
   specialization of a class template, in each form (Clang 22, GCC).
2. Deduction through an alias template is not valid (Clang 18).
3. Deduction works through each alias, but an argument for a fixed
   position must have exactly the fixed type (EDG 7.0).

## Answer

Yes; Clang 22 and GCC agree, Clang 18 and EDG 7.0 differ (option 1). The newest compilers of two families agree, and the source
reader's inventory (items B2 to B8) describes the derived guides.
Clang 18 and EDG 7.0 are older or partial implementations.

The question stays open under decision 0051 (2026-11-03). Clang 22 and GCC 16 accept all three forms, and EDG 7.0 rejects an argument that converts to the fixed type, so two families disagree on that form. Clang 18 rejects every form. CTAD-P2 supports the provisional answer. An alias template gets the guides of its class template, matched to the alias (CTAD-14), and a guide's parameter of a fixed type takes an argument that converts to it, as any function parameter does. EDG's behavior is on the cluster 5 list of EDG differences that waits on the maintainer (`documentation/decisions/pending.md`).

Kept on 2026-11-03. The source-reader gap review of question cluster 5 agrees with option 1.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of question cluster 5 (`reports/review/2026-11-03-gap-review-question-cluster-5.md`) supports this answer with no doubt, and CTAD-P2 gives the same answer: an alias template gets the guides of its class template, and a parameter of a fixed type takes an argument that converts to it. EDG 7.0 still differs. Tidal records EDG's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for EDG. Clang 18 rejects every form.

Read again on 2026-11-12 (decision 0066, item RV12). The gap review of the phase 18 text supports the alias template case of a functional cast (RV12-5). The answer stands, and `functional-cast-alias-template-deduced.cpp` keeps its override for Clang 18.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 5 (`reports/review/2026-11-03-gap-review-question-cluster-5.md`) reads the reference as supporting option 1. The guides for the alias come from the guides of the class template. A fixed argument becomes a plain parameter type in the new guide, so an argument that converts to that type is accepted.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Gap review, 2026-11-12 (phase 18 text)

The source-reader gap review of the phase 18 text (`reports/review/2026-11-12-gap-review-phase-18.md`) reads the casts paragraph's alias template case and `functional-cast-alias-template-deduced.cpp`: supports (option 1, RV12-5). A template name with no argument list can stand for a deduced class type when the template is a class template, or an alias template whose defining type is a template-id of a simple shape. Such a deduced type can be the type of a functional cast. So the paragraph and the test agree, and Clang 18's rejection does not conform.
