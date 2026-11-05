# Q-0274: What does a name mean that finds an expanded using-declarator when the pack is empty?

**Status:** provisional
**Affects:** PACK-30, `tests/templates/packs/using-declaration-empty-pack-outer-name.cpp`

## Context

`template <class... B> struct D : B... { using B::f...; int g() { return f(0); } };`
with a function `f(int)` in the namespace. At the definition, `f` in
`g` finds the using-declarator. `D<>` has an empty pack.

| Compiler | `D<>().g()` |
|---|---|
| Clang 18, 22, host | rejects: the using-declaration expands to an empty pack |
| GCC 14, 16, host | accepts, and calls the namespace `f` |
| EDG 7.0 | accepts, and calls the namespace `f` |

With no namespace `f`, every compiler rejects the call.

## Evidence

The test above, and a probe without the namespace `f`. Item 15 of the
gap review `reports/review/2026-10-31-gap-review-packs-and-ctad.md` says that the program is not valid and that no
diagnostic is required.

## Options

1. The program is not valid, and no diagnostic is required. Both
   results are allowed (`no-diagnostic`).
2. The program is valid and calls the namespace `f` (GCC, EDG).
3. The implementation must reject the program (Clang).

## Provisional answer

Not valid, no diagnostic required; GCC and EDG accept, Clang rejects (option 1). The implementations differ, and the gap review reads the
rule as one that requires no diagnostic.

Kept open on 2026-11-03 (replaced by the note of 2026-11-04) under decision 0054, because no principle on the page gives the answer. PACK-30 is an exception of PACK-P2, and no principle on the packs page gives the missing diagnostic.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). PACK-P2 failed the test, because PACK-P2 named PACK-30 in its `:exceptions:` line. Item PQ widened PACK-P2. A name that finds an expanded using-declarator at the definition is bound to it ({principle}`DEP-P1`), and an empty pack leaves the name with nothing to mean. As for other changes of meaning between the definition and the instantiation ({principle}`DEP-P6`), the implementation need not diagnose the error. PACK-30 moved from the `:exceptions:` line to the `:rules:` line. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses PACK-P2.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) agrees with option 1. A template whose meaning in a hypothetical instantiation right after its definition differs from its meaning in an actual instantiation is not valid, with no diagnostic required. Lookup at the definition finds the expanded using-declarator, and the instantiation with an empty pack finds nothing there. Both results are allowed.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
