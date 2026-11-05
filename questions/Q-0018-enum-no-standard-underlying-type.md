# Q-0018: What happens when no standard integer type holds all of an unscoped enumeration's values?

**Status:** provisional
**Affects:** ENUM-5, `tests/declarations/enumerations/unscoped-value-too-large.cpp`, `tests/declarations/enumerations/unscoped-value-too-large-ilp32.cpp`

## Context

`enum Huge { A = 1ULL << 63, B = -1 };` needs a signed type with at least
65 bits. Clang and EDG reject it. GCC accepts it with a 16-byte underlying
type, even under `-pedantic-errors`. Clang's rejection is a pedantic
diagnostic (`-Wenum-too-large`), so Clang also treats such an enumeration
as an extension outside strict mode. With `-fshort-enums`, Clang and GCC
give a small enumeration a 1-byte underlying type. This behavior shows
that the choice of type is the implementation's.

## Evidence

`tests/RESULTS.md`, row `unscoped-value-too-large.cpp`. GCC 16 reports
`sizeof(Huge) == 16`.

## Options

1. `impl-defined`: an implementation may use a wider integer type of its
   own, or reject the enumeration.
2. `reject`: only standard integer types may be used.

## Provisional answer

Impl-defined (option 1). Both behaviors exist, and the choice of underlying type is
already the implementation's (ENUM-5).

Kept open on 2026-11-03 (replaced by the note of 2026-11-04) under decision 0054, because no principle on the page gives the answer. ENUM-P3 says that the implementation chooses a type wide enough, and it does not say what happens when no type is wide enough. The review also doubts that GCC's 128-bit type counts as an extended integer type under strict options.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). ENUM-P3 failed step 3, because the model did not say what happens when no type is wide enough. ENUM-P3 now says that the implementation can reject the enumeration when no integer type that it provides is wide enough, which fits the provisional answer. The review still doubts that GCC's 128-bit type counts as such a type under strict options, so the question stays open on the review as well. Under decision 0055, the question also does not settle on a principle that phase 8 wrote until a reader or a reviewer uses ENUM-P3.

Kept open on 2026-11-04 (replaced by the next note) under decision 0054, because the gap review reads with doubt, and under decision 0055 the question waits for a reader or a reviewer to use ENUM-P3.

Used on 2026-11-04 by the source-reader gap review of the text phase 8 wrote on the declarations and statements chapters (`reports/review/2026-11-04-gap-review-phase-8-declarations-statements.md`), under decision 0056. The review applied ENUM-P3 to the program of `unscoped-value-too-large.cpp` without the rule text and reached the provisional answer: the values need more bits than any standard type has, so an implementation with a wider type of its own can use it, and one without must reject the enumeration.

Kept open on 2026-11-04 under decisions 0054 and 0055, because ENUM-P3 changed in this commit: the implementation now rejects the enumeration when none of its types is wide enough. The review also still reads with doubt on GCC's 128-bit type. The question waits for the next use of the changed text.

Used on 2026-11-04 by the principle-use batch PU1 (`reports/clean-room/PU1/`), under decision 0056. A clean-room reader applied ENUM-P3, without the rule text, to the program of `unscoped-value-too-large.cpp` and gave no answer (`not-decided`). ENUM-P3 says that an implementation can provide wider integer types of its own, and it does not say whether the baseline compilers use one for these values (finding PU1-1). The use does not support the answer.

Kept open on 2026-11-04 under decisions 0054 and 0056, because the use in batch PU1 gave no answer (finding PU1-1), and the gap review still reads with doubt on GCC's 128-bit type.

Kept open on 2026-11-04 under decision 0055, because ENUM-P3 changed in this commit after finding PU1-1. ENUM-P3 now says what each baseline compiler does with an enumeration whose values need more than 64 bits. On x86-64, GCC accepts it with a 128-bit type, and Clang and EDG reject it. With `-m32`, every compiler rejects it (`tests/declarations/enumerations/unscoped-value-too-large-ilp32.cpp`). The question waits for the next use of ENUM-P3.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. The enumeration is not valid when no integral type holds every value. The integral types include any extended integer types that an implementation provides, and the implementation chooses whether such types exist. So an implementation can accept the enumeration only when it counts its wide type as an extended integer type. GCC does not document its 128-bit type as one, so the review finds GCC's acceptance under strict options doubtful.

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on the declarations and statements chapters (`reports/review/2026-11-04-gap-review-phase-8-declarations-statements.md`) agrees with option 1, with the doubt of its earlier reading. An implementation can choose any integral type that holds the values, its extended types included, and must reject the enumeration when it has none. ENUM-5 matches that reading. Whether GCC's 128-bit type counts as an extended type is GCC's own statement, and the reference says nothing about it. The review also found that ENUM-P3 said the implementation "can" reject where the reference requires the rejection.
