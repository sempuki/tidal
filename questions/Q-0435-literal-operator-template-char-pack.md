# Q-0435: Is a literal operator template whose head is one pack of `char` valid, and which literals call it?

**Status:** provisional
**Affects:** OPR-8, OPR-10, OPR-16, `tests/classes/operators/literal-operator-template-char-pack.cpp`, `tests/classes/operators/literal-operator-template-long-digits.cpp`, `tests/classes/operators/literal-operator-template-head-not-char-pack.cpp`, `tests/classes/operators/literal-operator-template-numeric-form-wins.cpp`, `tests/classes/operators/literal-operator-template-and-raw-form.cpp`, `tests/classes/operators/literal-operator-template-string-literal.cpp`, `tests/programs/roadmap/r12-bigint-second-try.cpp`
**Source:** the round 12 programmer report (`reports/clean-room/roadmap-12/programmer.md`, sections 1.5 and 6), reduction `literal_operator_template.cpp`, and the programmer's sketch `bigint_second_try.cpp`.
**Waits for:** a review (doubt)

## Context

A program declares a literal operator as a function template. The
template head is one pack of `char`, and the parameter list is empty:

```cpp
template <char... Cs> constexpr int operator""_width() { return sizeof...(Cs); }
static_assert(4096_width == 4);
```

OPR-8 lists the parameter lists of a literal operator, one row for each
form of literal. OPR-10 says that a literal operator must have one of
those lists. An empty list with a template head is not in the table, so
the text read alone rejects the declaration.

Every compiler accepts the declaration. A call passes each character of
the literal as one template argument, in order. The writer's probes on
host GCC 16 and Clang 22 under `-std=c++23 -pedantic-errors` give these
results:

| Form | GCC | Clang |
|---|---|---|
| an integer literal, with a prefix such as `0x` or digit separators: the characters as written | accepts | accepts |
| a floating literal, with a point or an exponent | accepts | accepts |
| a string literal with only the template for its suffix | rejects | rejects |
| a character literal with only the template for its suffix | rejects | rejects |
| the template and the `unsigned long long` form for one suffix, with an integer literal | accepts, the `unsigned long long` form | accepts, the `unsigned long long` form |
| the template and the `long double` form for one suffix, with a floating literal | accepts, the `long double` form | accepts, the `long double` form |
| the template and the `const char*` form for one suffix | rejects the second declaration | rejects the call as ambiguous |
| a template head other than one pack of `char`, such as `char` then a pack of `char` | rejects | rejects |

A literal whose digits no integer type holds also calls the template
(`literal-operator-template-long-digits.cpp`).
{question}`Q-0436` holds that case
for PRIM-2.

## Evidence

The tests in **Affects**. The matrix runs the two `run` tests and the
reject test on every baseline compiler, and all pass. EDG accepts the
two `run` tests and rejects the head with a `char` before the pack.
The round 12 programmer report gives the front end's result: unsupported,
"literal operator template".

## Options

1. A literal operator template whose head is one pack of `char` and
   whose parameter list is empty is valid. An integer or floating
   literal with its suffix calls it, with the characters of the literal
   as the template arguments. The numeric form wins over the template
   when both exist, as it wins over the `const char*` form. The template
   and the `const char*` form for one suffix make a call that is not
   valid. Other template heads are not valid.
2. The template is not valid, as OPR-10 reads now. Every compiler
   accepts it.

## Provisional answer

Valid, for integer and floating literals (option 1). Every compiler
accepts the form and gives the same results, and a library type for big
numbers or units uses it. The tests follow option 1. Item F5 of
decision 0063 writes the text for OPR-8 and OPR-10.

Rule text on 2026-11-11 (decision 0063, item F5): OPR-8 defines the literal operator template, the characters that an integer or floating literal passes, and the choice of the numeric form when both exist. OPR-10 accepts the form, and OPR-16 rejects a numeric literal when only the `const char*` form and the template exist. Three tests join: `literal-operator-template-numeric-form-wins.cpp`, `literal-operator-template-and-raw-form.cpp`, and `literal-operator-template-string-literal.cpp`, and every baseline compiler passes them. The question now waits for a first review.

Kept open on 2026-11-11 under decision 0054 (decision 0063, item RV8). The review supports the answer with a doubt. Every compiler agrees. The last sentence of option 1 is too wide for a template with one constant parameter of class type, which a string literal calls.

## Opinions

### Gap review, 2026-11-11 (phase 15 text, classes, declarations, foundations, and statements)

The source-reader gap review of the phase 15 text on classes, declarations, foundations, statements, and the glossary (`reports/review/2026-11-11-gap-review-phase-15-classes-declarations-foundations.md`) reads the question: supports with a doubt (RV8-A-3, RV8-A-4). Option 1 matches for one pack of `char`. An integer or floating literal passes the source characters before its suffix, so a prefix, a point, an exponent, and digit separators all count, and the numeric form wins when it exists. The last sentence of option 1 is too wide. The reference also allows a template with one constant template parameter of class type, which a string literal calls when the string is a valid argument for that parameter. The second doubt is on the choice among the forms. A numeric form must match the kind of literal, and the conflict between the raw form and the template falls on a literal that uses the suffix, so the two declarations alone are valid. GCC rejects the second declaration, and the test uses a literal, so its expectation holds.
