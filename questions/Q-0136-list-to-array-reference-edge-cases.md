# Q-0136: Do empty braces or a designated list have a sequence to a reference to an array?

**Status:** provisional
**Affects:** ICS-17, ICS-18
**Parent:** Q-0294, the model of braced lists for references, arrays, and conversion sequences
**Waits for:** a review (doubt)

## Context

The lists grid in `tests/overloading/conversion-sequences/grids/tables.md` shows two cases where the compilers differ.

Empty braces `{}` and a parameter `int (&&)[]`, an array of unknown
bound:

| Compiler | Result |
|---|---|
| GCC 14, GCC 16 | a sequence exists, and it beats `int (&&)[2]` (fewer elements) and `Point` |
| Clang 18, Clang 22 | no sequence |
| EDG 7.0 | a sequence exists, and it ties with `int` and with `int (&&)[2]` |

A designated list `{.x = 1}` and a parameter `int (&&)[2]` or
`int (&&)[]`, against an aggregate `Point` with a member `x`:

| Compiler | Result |
|---|---|
| Clang 18, Clang 22 | no sequence to the array, so `Point` is chosen |
| GCC 14, GCC 16 | rejects ("name 'x' used in a GNU-style designated initializer for an array") |
| EDG 7.0 | rejects ("invalid designator kind") |

## Evidence

The rows "empty {}" and "designated {.x = 1}" of the lists grid in `tests/overloading/conversion-sequences/grids/tables.md` (`tests/overloading/conversion-sequences/grids/lists.json`).

## Options

1. No sequence in either case. An array has no element for empty braces
   to give a bound, and a member designator names no array element
   (Clang).
2. Empty braces form an array of no elements, and a designated list
   is checked against the array (GCC, EDG in part).

## Provisional answer

No; Clang agrees, GCC and EDG differ (option 1). ICS-17 needs the elements to give the bound, and ICS-18 needs
the named members. Clang is the only compiler with one consistent
answer for both cases.

Kept open on 2026-11-03 under decision 0054, because the review finds the empty braces unclear and leans to option 2 there.

Kept open on 2026-11-11 under decision 0054 (decision 0066, item DQ). The second review supports the answer with a narrower doubt (DQ-5). The designated list is settled for option 1. The doubt is only on empty braces for an array of unknown bound.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on overloading, references, and list initialization (`reports/review/2026-11-03-gap-review-questions-overloading.md`) agrees with option 1 for the designated list: a designator names a class member, and an array has none, so no sequence exists. For empty braces and a reference to an array of unknown bound, the review finds the reference unclear. Read literally, the array rule asks for a sequence from each of no elements, so a sequence exists and beats the array of two elements by the fewer-elements tie-breaker, which is GCC's result (option 2). The temporary that would follow is not valid, but a sequence can exist for a call that later fails.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Gap review, 2026-10-31

The source-reader gap review of conversion sequences (`reports/review/2026-10-31-gap-review-conversion-sequences.md`) reads a designated list as allowed only for a parameter of aggregate type. A reference to an aggregate is not an aggregate type, so the plain reading gives no sequence. The review puts any compiler split on this case in this question. The reading fits the provisional answer for the designated list, and the review says nothing on empty braces.

### Second review of the doubt-held questions, 2026-11-11

The source-reader second review of the questions held only by a doubt (`reports/review/2026-11-11-second-review-doubt-held-questions.md`) reads the question: supports with a doubt that remains (DQ-5). The designated list is settled. A designated list converts only to an aggregate that aggregate initialization accepts, and a designator cannot name an array element. For empty braces and an array of unknown bound, the list rule takes the worst of the element sequences, and there are no elements. The reference does not say what the worst of no sequences is. The reference also does not let empty braces initialize an array of unknown bound, so the temporary could never be made, which fits option 1. A sequence can still exist for a call whose initialization fails later, so the doubt now rests only on the empty set of element sequences.
