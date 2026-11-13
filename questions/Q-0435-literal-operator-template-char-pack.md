# Q-0435: Is a literal operator template whose head is one pack of `char` valid, and which literals call it?

**Status:** provisional
**Affects:** OPR-8, OPR-10, OPR-16, `tests/classes/operators/literal-operator-template-char-pack.cpp`, `tests/classes/operators/literal-operator-template-long-digits.cpp`, `tests/classes/operators/literal-operator-template-head-not-char-pack.cpp`, `tests/classes/operators/literal-operator-template-numeric-form-wins.cpp`, `tests/classes/operators/literal-operator-template-and-raw-form.cpp`, `tests/classes/operators/literal-operator-template-string-literal.cpp`, `tests/classes/operators/literal-operator-template-and-raw-form-declared.cpp`, `tests/programs/roadmap/r12-bigint-second-try.cpp`
**Source:** the round 12 programmer report (`reports/clean-room/roadmap-12/programmer.md`, sections 1.5 and 6), reduction `literal_operator_template.cpp`, and the programmer's sketch `bigint_second_try.cpp`.
**Waits for:** a use of OPR-P3

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

Rule text on 2026-11-11 (decision 0063, item RV8): OPR-8 gives a second kind of literal operator template, with one constant template parameter of class type, which a string literal calls (RV8-A-3). OPR-10 accepts both kinds, so the last sentence of option 1 now leaves out the class-type kind, and Q-0446 holds the splits of that kind. OPR-8 says that the numeric form is the one of the literal's kind, and OPR-16 says that the `const char*` form and the template are valid declarations for one suffix (RV8-A-4). The test `literal-operator-template-and-raw-form-declared.cpp` has no literal and records GCC's rejection of the second declaration under this question. The question still waits for a review.

Kept open on 2026-11-11 under decision 0054 (decision 0063, item RV8). The read of the RV8 repair diff finds that the repairs of RV8-A-3 and RV8-A-4 agree with the reference, and it leaves no doubt on this question. GCC 16 alone differs, in `literal-operator-template-and-raw-form-declared.cpp`. OPR-P3 says that a numeric literal can pass its characters to a template, which rules out option 2. Item F5 wrote that sentence and item RV8 widened OPR-P3 in phase 15, so OPR-P3 gives no answer until a reader or a review uses it (decision 0055). The read names OPR-P3 and not the question's program, so the read is no use.

Kept open on 2026-11-12 under decision 0054 (decision 0066, item RV12). The gap review of the phase 18 text uses OPR-P3 on the question (RV12-15) and supports the answer with no doubt, so the wait for a use ends. OPR-P3 rules out option 2 and gives the first part of option 1, which literals call the template. The other parts of option 1 turn on details that only OPR-8, OPR-10, and OPR-16 state: the numeric form wins when it exists, the raw form and the template for one suffix make a call that is not valid, and other heads are not valid. So OPR-P3 fails step 3 of the principle test for those parts. GCC 16 alone differs, in `literal-operator-template-and-raw-form-declared.cpp`. The **Waits for:** line names a principle (OPR-P3), was a use of OPR-P3. Ledger row RV12-15 plans to widen OPR-P3 in part 2 of item RV12.

Principle text on 2026-11-12 (decision 0066, item RV12): OPR-P3 says that a template with any other head is not a literal operator, and gives a table of the forms that compete for one suffix (RV12-15). The numeric form of the literal's kind wins. The `const char*` form and the template with no numeric form make a numeric literal not valid. OPR-P3 now rules out option 2 and gives each part of option 1. The principle changed in this phase, so it gives no answer until a reader or a review uses it (decision 0055). The **Waits for:** line names a use of OPR-P3, was a principle (OPR-P3).

Kept open on 2026-11-12 under decision 0054 (decision 0066, item RV12). The read of the RV12 repair diff finds that OPR-P3 now covers the whole of option 1, with a small doubt on its wording (RV12-R-3). The read names OPR-P3 and the question's option 1, and it does not take the question's program through OPR-P3's model, so the read is no use of OPR-P3. The **Waits for:** line does not change.

## Opinions

### Gap review, 2026-11-11 (phase 15 text, classes, declarations, foundations, and statements)

The source-reader gap review of the phase 15 text on classes, declarations, foundations, statements, and the glossary (`reports/review/2026-11-11-gap-review-phase-15-classes-declarations-foundations.md`) reads the question: supports with a doubt (RV8-A-3, RV8-A-4). Option 1 matches for one pack of `char`. An integer or floating literal passes the source characters before its suffix, so a prefix, a point, an exponent, and digit separators all count, and the numeric form wins when it exists. The last sentence of option 1 is too wide. The reference also allows a template with one constant template parameter of class type, which a string literal calls when the string is a valid argument for that parameter. The second doubt is on the choice among the forms. A numeric form must match the kind of literal, and the conflict between the raw form and the template falls on a literal that uses the suffix, so the two declarations alone are valid. GCC rejects the second declaration, and the test uses a literal, so its expectation holds.

### Read of the RV8 repair diff, 2026-11-11

The source-reader read of the RV8 repair diff (`reports/review/2026-11-11-read-of-rv8-repair-diff.md`) reads the OPR-8 and OPR-16 hunks and the Q-0435 line on the two declarations: agrees. The repairs close RV8-A-3 and RV8-A-4, the two doubts of the phase 15 review on this question. The read's one doubt on OPR-8 (RV9-R-5) is on a template with a parameter of class type, which Q-0446 holds. So the read leaves no doubt on the template whose head is one pack of `char`.

### Gap review, 2026-11-12 (phase 18 text)

The source-reader gap review of the phase 18 text (`reports/review/2026-11-12-gap-review-phase-18.md`) uses OPR-P3 on the question: supports, and OPR-P3 answers the question in part (RV12-15). An integer or floating literal with a suffix can call a literal operator template whose head is one pack of `char`, and the source characters before the suffix become the arguments. A string literal calls only the template with one constant parameter of class type. OPR-P3 says this, so it rules out option 2 and answers which literals call the template. OPR-P3 does not give the rest of option 1: the choice of the numeric form when it exists, the call that is not valid when the raw form and the template both exist, and the rejection of other heads. The reference decides all three as option 1 says, and OPR-8, OPR-10, and OPR-16 state them. So the principle answers the first part of the question, and the rules answer the second.

### Read of the RV12 repair diff, 2026-11-12

The source-reader read of the RV12 repair diff (`reports/review/2026-11-12-read-of-rv12-repair-diff.md`) reads OPR-P3's new sentence and table: agrees with a small doubt (RV12-R-3). OPR-P3 now states the choice of the numeric form, the call that is not valid when the raw form and the template both exist, and the other heads, so the principle covers the whole of option 1. The doubt is on wording. "Is not a literal operator" also allows a reading where a declaration with another head is valid and lookup skips it, and OPR-10 makes the declaration not valid. The table has no row for a suffix where lookup finds only forms of the wrong kind, which OPR-16 states. The read gives no new reading of the question's answer.
