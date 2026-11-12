# Q-0274: What does a name mean that finds an expanded using-declarator when the pack is empty?

**Status:** provisional
**Affects:** PACK-30, `tests/templates/packs/using-declaration-empty-pack-outer-name.cpp`
**Waits for:** the maintainer (pending.md row 4)

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

Used on 2026-11-04 by the source-reader gap review of the text phase 8 wrote on expressions, foundations, and templates (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`), Part B. The review applied PACK-P2 to the question's program, a using-declaration from an empty pack, and a call of a name that the namespace also declares (`using-declaration-empty-pack-outer-name.cpp`), without the rule text: the name was bound at the definition and means nothing in the empty instantiation, so the program is not valid with no diagnostic required, which is the provisional answer.

Kept open on 2026-11-04 under decision 0054, because the review finds the reference unclear, and it leans to option 2. GCC 16 and EDG 7.0 follow option 2, so two newest families differ from the answer.

Noted on 2026-11-05 under decision 0059, item QR. The answer needs no diagnostic, so both results conform, and only Clang overrides `using-declaration-empty-pack-outer-name.cpp`. The question no longer waits for newest families that agree. It waits for a review that reads the answer with no doubt.

Kept open on 2026-11-11 (decision 0066, item DQ). The second review reads the answer against (DQ-12) and reads option 2, the reading toward which the review of 2026-11-04 leaned. GCC 16 and EDG 7.0 call the namespace `f`, which both the answer and the review allow. Clang 22 rejects, which only the answer allows. The question waits with `pending.md` row 4, and the tests keep their expectations.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) agrees with option 1. A template whose meaning in a hypothetical instantiation right after its definition differs from its meaning in an actual instantiation is not valid, with no diagnostic required. Lookup at the definition finds the expanded using-declarator, and the instantiation with an empty pack finds nothing there. Both results are allowed.

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on expressions, foundations, and templates (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`) finds the reference unclear, leaning to option 2. A name that finds a using-declarator with a dependent final name refers to a member of the class being defined whose meaning waits for the arguments, so the call is dependent, and the rule on hypothetical instantiations does not reach it. A note says that a using-declarator is replaced during the lookup for a specialization. With an empty pack the replacement is nothing, and the lookup goes on to the namespace function, as GCC and EDG do. The reference does not say how that interacts with the rewrite of a member name into a member access.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Second review of the doubt-held questions, 2026-11-11

The source-reader second review of the questions held only by a doubt (`reports/review/2026-11-11-second-review-doubt-held-questions.md`) reads the question: against the answer (DQ-12). The using-declarator in the test has a final name that depends on the pack. A name that lookup finds through such a using-declarator counts as a dependent member of the class being defined. A member access whose name is dependent is type-dependent, so the call in `g` depends on the parameters. The rule on a hypothetical instantiation right after the definition covers only constructs that do not depend on the parameters, so the rule does not reach this call. A note says that a using-declarator is replaced during the lookup for a specialization. With an empty pack nothing replaces it, the search of the class finds nothing, and the lookup goes on to the namespace function. That is option 2, the result of GCC and EDG. The doubt is the rewrite of a class member name into a member access, which the definition's lookup triggers, when the specialization's lookup finds no member. The rule that makes a changed result of a second lookup an ambiguity names only qualified names.
