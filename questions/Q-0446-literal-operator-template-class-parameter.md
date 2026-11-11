# Q-0446: Which literal operator does a string literal call when a literal operator template with a class-type parameter exists?

**Status:** provisional
**Affects:** OPR-8, OPR-16, `tests/classes/operators/literal-operator-template-class-parameter-and-string-form.cpp`, `tests/classes/operators/literal-operator-template-class-parameter-not-valid-argument.cpp`
**Source:** point RV8-A-3 of the gap review of the phase 15 text on classes, declarations, foundations, and statements (`reports/review/2026-11-11-gap-review-phase-15-classes-declarations-foundations.md`), with the writer probe `reports/clean-room/roadmap-12/writer-probes/rv8-points.json`.
**Waits for:** newest families that agree; a principle (OPR-P3)

## Context

A literal operator template can have one template parameter that is a
constant of class type, and an empty parameter list. A string literal
with the suffix passes itself as the template argument:

```cpp
struct Name { char data[4]; constexpr Name(const char (&t)[4]); };
template <Name N> constexpr int operator""_tag() { return 1; }
constexpr int operator""_tag(const char*, decltype(sizeof 0)) { return 2; }
```

Every compiler accepts the template, and `"abc"_tag` with only the
template calls it. The compilers split on two forms:

| Form | Clang 18 and 22 | GCC 14 and 16 | EDG 7.0 |
|---|---|---|---|
| the template and the string form, and `"abc"_tag`, which is a valid argument for `Name` | the template | the string form | the string form |
| the template and the string form, and `"abcd"_tag`, which is not a valid argument | the string form | the string form | the string form |
| only the template, and `"abcd"_tag` | rejects: no matching literal operator | rejects: no matching function | accepts |

## Evidence

The tests in **Affects**, `literal-operator-template-class-parameter.cpp`,
`literal-operator-template-class-parameter-string-form-fallback.cpp`, and
the probe in **Source**.

## Options

1. A string literal calls the template when the string literal is a
   valid template argument for the parameter, and the string form
   otherwise. With no string form, a string literal that is not a valid
   argument matches no literal operator (Clang for both forms, GCC for
   the second).
2. A string literal calls the string form when the string form exists,
   and the template only when the string form does not exist (GCC and
   EDG for the first form).

## Provisional answer

The template wins when the string literal is a valid argument for it
(option 1). Clang gives this result, and the review reads the template
as the first choice for a string literal, with the string form as the
fallback. GCC and EDG call the string form in the first form, and EDG
accepts the second form, so two newest families stand against the first
form.

Kept open on 2026-11-11 under decision 0054 (decision 0063, item RV8). The read of the RV8 repair diff supports the answer with no doubt. GCC 16 and EDG 7.0 differ in the first form, so two newest families stand against the answer. No principle gives the answer either. OPR-P3 says that a string literal can pass itself to a template whose one parameter is a constant of class type, and it does not say which form wins when the string form also exists. Each option fits OPR-P3, so OPR-P3 fails step 3 of the principle test. Item RV8 widened OPR-P3 in phase 15, and no reader or review has used it yet (decision 0055).

## Opinions

### Read of the RV8 repair diff, 2026-11-11

The source-reader read of the RV8 repair diff (`reports/review/2026-11-11-read-of-rv8-repair-diff.md`) gives the first reading of the question: supports (option 1). When the set of literal operators for the suffix has a template whose parameter can take the string literal, the literal calls the template. Otherwise the literal calls the form with a pointer and a length, and with no such form the program is not valid. GCC and EDG read against the first form, and EDG reads against the second. The read finds that the OPR-8, OPR-10, OPR-16, and OPR-P3 hunks and the Q-0446 line agree. It adds a small doubt on OPR-8 (RV9-R-5). The second template of `literal-operator-template-class-parameter.cpp` has a parameter whose type is a class template name with no arguments, deduced from the string literal. The reference asks for one constant parameter of class type and does not say whether a parameter with a deduced class type counts before the deduction. The deduced type is a class, and every compiler accepts the form. OPR-8 does not name the form, and no rule changes.
