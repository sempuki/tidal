# Q-0446: Which literal operator does a string literal call when a literal operator template with a class-type parameter exists?

**Status:** provisional
**Affects:** OPR-8, OPR-16, `tests/classes/operators/literal-operator-template-class-parameter-and-string-form.cpp`, `tests/classes/operators/literal-operator-template-class-parameter-not-valid-argument.cpp`
**Source:** point RV8-A-3 of the gap review of the phase 15 text on classes, declarations, foundations, and statements (`reports/review/2026-11-11-gap-review-phase-15-classes-declarations-foundations.md`), with the writer probe `reports/clean-room/roadmap-12/writer-probes/rv8-points.json`.
**Waits for:** a first review; newest families that agree

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

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
