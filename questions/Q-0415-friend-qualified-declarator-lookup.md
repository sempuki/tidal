# Q-0415: In a friend declaration with a qualified name, is a name after the declarator searched in the scope that the qualifier names?

**Status:** provisional
**Affects:** NAM-21, `tests/foundations/names/friend-qualified-declarator-lookup.cpp`
**Source:** the gap review `reports/review/2026-11-05-gap-review-phase-10-classes-declarations-foundations.md`, section NAM-21, from the probe that a clean-room writer ran for it.
**Waits for:** newest families that agree

## Context

Namespace `N` declares `using T = int;` and `void f(T);`. Class `S` declares `friend void N::f(T);`. The name `T` after the declarator names `N::T` only if the search starts in `N`.

| Implementation | Result |
|---|---|
| Clang 22 | rejects: `T` is not found |
| GCC 16 | accepts |
| EDG 7.0 | accepts |

## Evidence

The test in **Affects**.

## Options

1. The name is searched first in `N`, then by ordinary lookup (GCC, EDG, the gap review of 2026-11-05).
2. The name gets only ordinary lookup (Clang).

## Provisional answer

Searched first in `N` (option 1). Two of three families agree, and the gap review of the text phase 10 wrote reads it so.

Kept open on 2026-11-05 under decision 0054, because the review reads the answer with a doubt on NAM-21's wording (item RV5).

Kept open on 2026-11-11 under decision 0054 (item RV7). The review again reads NAM-21 with a doubt and says that the split stays.

## Opinions

### Gap review, 2026-11-05 (phase 10 text)

The source-reader gap review of the text phase 10 wrote (`reports/review/2026-11-05-gap-review-phase-10-classes-declarations-foundations.md`) reads the case as the question's option 1. See its section on NAM-21.

### Gap review, 2026-11-05 (phase 12 text, classes, declarations, foundations, statements, and exceptions)

The source-reader gap review of the text phase 12 wrote on classes, declarations, foundations, statements, and exceptions (`reports/review/2026-11-05-gap-review-phase-12-classes-declarations-foundations.md`) reads the question: supports with a doubt (RV5-A-10). For a friend whose declarator names a member through a qualifier, a plain name that follows the qualified name inside the declarator is searched in the named scope first and then by ordinary lookup. The test's parameter type is such a name. The doubt is in NAM-21's wording: it says "after the declarator", and the parameter list is part of the declarator.

### Gap review, 2026-11-11 (phase 14 text, classes, declarations, foundations, and statements)

The source-reader gap review of the text phase 14 wrote on classes, declarations, foundations, and statements (`reports/review/2026-11-11-gap-review-phase-14-classes-declarations-foundations.md`) names the question in its reading of NAM-21 (RV7-A-6). The trailing requires clause that NAM-21 now lists is part of the redeclaration after the qualified name, and the reference searches it in the named scope. The review keeps its doubt on NAM-21 and says that the split of this question stays. The Evidence note on a friend parameter type in an inline namespace states the RV6 reading correctly.

### Second review of the doubt-held questions, 2026-11-11

The source-reader second review of the questions held only by a doubt (`reports/review/2026-11-11-second-review-doubt-held-questions.md`) reads the question: supports, and the doubt on NAM-21's wording is resolved. The reference has a rule for exactly this case. In a friend declaration whose name is qualified, a plain name after the qualified name is searched first in the named scope and then by ordinary lookup. NAM-21 now states the parts after the qualified name in that way.
