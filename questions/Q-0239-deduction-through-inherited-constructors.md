# Q-0239: Does a class template that inherits constructors get the deduction guides of its base?

**Status:** settled
**Affects:** CTAD-16, CTAD-19, `tests/templates/class-template-deduction/inherited-constructors.cpp`, `inherited-user-guide.cpp`, `tie-own-constructor-types-differ.cpp`

## Context

`template <class T> struct Derived : Base<T> { using Base<T>::Base; };`
and then `Derived d(1);`, where `Base<T>` has the constructor
`Base(T)`.

| Compiler | Result |
|---|---|
| GCC 14, 16, host | accepts, `Derived<int>` |
| EDG 7.0 | accepts, `Derived<int>` |
| Clang 18, 22, host | rejects: no viable constructor or deduction guide |

GCC and EDG also take a user guide of the base, and order the guides
from the class's own constructors before the inherited ones
(`tie-own-constructor-template.cpp`, `tie-own-constructor-user-guide.cpp`).
With `Derived(const T&)` and an inherited `Base(T)`, GCC and EDG find
the two guides ambiguous, because their parameter types differ and the
tie-breaker does not apply. Clang has only the first guide and deduces
`Derived<int>` (`tie-own-constructor-types-differ.cpp`).

## Evidence

The tests above, the grid `forms.json` (column "inherited
constructor"), and the inventory items E1 to E4 of
`reports/review/2026-10-31-inventory-packs-forwarding-ctad.md`.

## Options

1. The class gets the guides of the base through its inherited
   constructors (GCC, EDG, and the inventory).
2. The class gets no guides from its base (Clang).

## Answer

Yes; GCC and EDG agree, Clang differs (option 1). Two of three implementations agree, and the source reader's
inventory lists the behavior as part of the language. The tie-breaker
tests of CTAD-19 give the same results on Clang, because Clang has no
inherited guide to compare.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) supports this answer with no doubt, and CTAD-P2 gives the same answer: a class template that inherits constructors gets the guides of its base. Clang 22 still differs. Tidal records Clang 22's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for Clang 22.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) agrees with option 1. A class template that inherits constructors from a direct base gets guides built from those of an alias template whose target is the base, with the results mapped back to the derived class. A user guide of the base takes part through the alias. The tie-breaker that prefers the class's own guides applies only when the parameter types match, so a pair whose types differ stays ambiguous, as GCC and EDG find. Clang does not implement this part of C++23.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Gap review, 2026-11-12 (phase 18 text)

The source-reader gap review of the phase 18 text (`reports/review/2026-11-12-gap-review-phase-18.md`) names the question in Part B, on RV8-B-6 at CTAD-16 (RV12-19). Clang makes no inherited guides, as this question records, and GCC and EDG agree on the test that separates the readings of CTAD-16. No test carries an override for the point, so the review finds the row 4 item "CTAD-16" enough. The review gives no new reading of this question's answer.
