# Q-0351: How is the depth of nested instantiations counted, and must it reach a minimum?

**Status:** settled
**Affects:** TR-13, INST-15, `tests/foundations/translation/limit-depth-one-template-100.cpp`, `tests/foundations/translation/limit-depth-two-templates-100.cpp`, `tests/foundations/translation/limit-depth-two-templates-1000.cpp`

## Context

Each compiler of the matrix stops a chain of nested instantiations at a
limit. The compilers count the chain in two ways.

| Compiler | What the limit counts | Limit | Option |
|---|---|---|---|
| Clang 22, 18 | the nested instantiations of all templates | 1024 | `-ftemplate-depth=N` |
| GCC 16, 14 | the nested instantiations of all templates | 900 | `-ftemplate-depth=N` |
| EDG 7.0 | the nested instantiations of one template | 64 | `--pending_instantiations=N` |

| Chain | Clang | GCC | EDG |
|---|---|---|---|
| 100 instantiations of one template | accepts | accepts | rejects |
| 100 instantiations that alternate between two templates | accepts | accepts | accepts |
| 1000 instantiations that alternate between two templates | accepts | rejects | rejects |

The limits are the same with `-m32`. Each option changes the limit of
its compiler. The other compilers reject the option.

## Evidence

The `limit-depth-*` tests of `tests/foundations/translation/`, and
`recursion-*.cpp` in `tests/templates/instantiation/`. The messages
name the limit: "recursive template instantiation exceeded maximum
depth of 1024" (Clang), "template instantiation depth exceeds maximum
of 900" (GCC), "excessive recursion at instantiation" (EDG).

## Options

1. The limit and the way it is counted are both implementation-defined
   (TR-13), with no minimum. A program that needs a deep chain is
   portable only to the implementations whose limits allow it.
2. Set a minimum depth that every implementation must allow. EDG's
   default of 64 instantiations of one template would then decide the
   figure, or EDG would not conform.

## Answer

Both are implementation-defined, with no minimum (option 1). The
compilers differ in what they count and in the figure, and each
compiler has an option that changes the figure.

Settled on 2026-11-03 under decision 0054. The review supports this answer with no doubt, and TR-P6 gives the same answer: each implementation sets its limits, and Tidal sets no minimum. Each family's count and figure is an implementation-defined choice, so no family differs.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Gap review, 2026-11-03

The source-reader gap review of program structure (`reports/review/2026-11-03-gap-review-program-structure.md`) reads the reference as supporting option 1, the provisional answer, with no doubt. The reference names one limit on the total depth of nested instantiations, which can span several templates, and recommends a minimum of 1024 as a guideline only. EDG counts the depth of one template, and GCC's default is below the guideline. Neither choice breaks a requirement.
