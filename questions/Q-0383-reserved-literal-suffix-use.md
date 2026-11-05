# Q-0383: Does a literal call a literal operator whose suffix does not start with `_`?

**Status:** provisional
**Affects:** OPR-9, OPR-P3, KW-4, `tests/classes/operators/literal-reserved-suffix-used.cpp`, `tests/classes/operators/literal-reserved-suffix-u8.cpp`
**Source:** finding X19-1 of `reports/clean-room/X19/report.md`

## Context

A literal suffix that does not start with `_` is reserved for later
forms of literals. A program can still write a literal operator with
such a suffix, and compilers warn at the declaration. The question is
whether a literal with that suffix then calls the operator, and
whether the program is valid at all.

| Program | Clang 22 | GCC 16 | EDG 7.0 |
|---|---|---|---|
| `operator"" km` and the literal `1km` | rejects the literal | calls the operator | calls the operator |
| `operator"" u8` and the literal `1u8` | rejects the literal | calls the operator | rejects the literal |

Clang warns at the declaration that no literal will call the operator.
GCC and EDG warn that the suffix is reserved. EDG reads `1u8` as the
`u` suffix followed by extra text.

A suffix that is a form in the syntax of literals, such as `u` or `f`,
keeps its built-in meaning on every compiler
(`tests/classes/operators/literal-builtin-suffix-wins.cpp`).

## Evidence

The two tests in Affects. GCC accepts both and calls the operator.
Clang rejects both, and EDG rejects the `u8` test. Each test checks
only translation, because OPR-9 now needs no diagnostic.

## Options

1. The literal calls the literal operator (GCC, and EDG for `km`). The
   reservation only asks for a warning, as OPR-9 says.
2. The literal is not valid (Clang). A reserved suffix never matches a
   literal operator.
3. The declaration of the literal operator is not valid, and no
   diagnostic is required. Each result in the table conforms: a call, a
   rejected literal, or a rejected declaration.

## Provisional answer

The declaration is not valid, and no diagnostic is required (option 3).
OPR-9 is a `no-diagnostic` rule. A compiler can call the operator, as
GCC does, or reject the literal, as Clang does. The two gap reviews of
the text phase 8 wrote read the reference this way, and every newest
compiler gives a result that the class allows.

The answer was option 1 until 2026-11-04, when OPR-9 was a `recommended`
rule. Kept open on 2026-11-04, because OPR-9 changed class in the
commit that adopted option 3. The question waits for the next use of
OPR-9 or OPR-P3.

Used on 2026-11-04 by the principle-use batch PU1 (`reports/clean-room/PU1/`), under decision 0056. A clean-room reader applied OPR-P3, without the rule text, to the programs of `literal-reserved-suffix-used.cpp`, `literal-reserved-suffix-u8.cpp`, and `literal-builtin-suffix-wins.cpp`. Each answer names OPR-P3 and says the program runs, an outcome that the tests allow. The reader marked the three answers doubtful (finding PU1-5): the batch offers no outcome for "not valid, no diagnostic required", and OPR-P3 does not say whether `u8` is a suffix that the language uses.

Kept open on 2026-11-04 under decisions 0054 and 0055. Item RV2 changed the answer to option 3, a new option that no review read before the change, and the reader in batch PU1 answered each program with doubt (finding PU1-5). The question waits for a review or a reader batch that reads option 3 with no doubt.

Kept open on 2026-11-04 under decision 0055, because OPR-P3 changed in this commit after finding PU1-5. OPR-P3 now lists the suffixes that the language uses, says that `u8` is not one of them, says that no baseline compiler rejects the declaration under `-pedantic-errors`, and gives what each compiler does with a literal that uses such a suffix. The question waits for the next use of OPR-P3.

## Opinions

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on classes and overloading (`reports/review/2026-11-04-gap-review-phase-8-classes-overloading.md`) reads the reference against option 1. Suffixes that do not start with `_` are reserved for the language, and suffixes with two underscores in a row for the implementation. A declaration of a literal operator with a reserved suffix makes the program not valid with no diagnostic required, whether or not a literal uses the operator. So every result in the table conforms, OPR-9 is in the wrong class, and OPR-P3's advice to warn has the same problem. The review supports option 2 read as no diagnostic required. It reads `1u8` as one two-letter suffix, as GCC does, and finds EDG's reading of a `u` suffix with extra text after it doubtful. It notes that the form with a space between the quotes and the suffix is deprecated, and that in that form the usual reservation of identifiers also applies.

### Gap review, 2026-11-04 (expressions, foundations, and templates)

The source-reader gap review of the text phase 8 wrote on expressions, foundations, exceptions, and templates (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`) contradicts option 1 and supports neither option as written. The declaration makes the program not valid with no diagnostic required, so an implementation can call the operator or reject the literal. KW-4's class matches the reference, and OPR-9's class does not. The review reads the `u8` test as GCC does.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
