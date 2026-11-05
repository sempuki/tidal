# Q-0386: Must a program declare all three ordering classes before a use of `<=>` that yields only one of them?

**Status:** provisional
**Affects:** LSUP-2, `tests/foundations/language-support/ordering-strong-only-builtin.cpp`, `tests/foundations/language-support/ordering-strong-only-builtin-runtime.cpp`, `tests/foundations/language-support/ordering-strong-only-defaulted.cpp`, `tests/foundations/language-support/ordering-partial-not-declared.cpp`, `tests/overloading/overload-resolution/builtin-three-way.cpp`, `tests/overloading/overload-resolution/builtin-three-way-mixed.cpp`
**Source:** section 1.4, item 7, and section 6 of `reports/clean-room/roadmap-6/programmer.md`, from the probes that a clean-room writer ran for them.

## Context

LSUP-2 says that a program that uses `<=>` on built-in operands, or a
defaulted `<=>`, declares `std::strong_ordering`,
`std::weak_ordering`, and `std::partial_ordering` first. Each test
below declares only `std::strong_ordering`, and each use of `<=>`
yields that class.

| Use of `<=>` | Clang 22, GCC 16 | EDG 7.0 |
|---|---|---|
| a built-in `<=>` on two `int` values, evaluated as a constant | accept | reject |
| a built-in `<=>` on two `int` parameters, evaluated at run time | accept | accept |
| a defaulted `<=>` whose members are `int`, called at run time | accept | reject |

EDG's message names each class that the program does not declare. Clang
18 and GCC 14 accept the three programs, as Clang 22 and GCC 16 do.

`builtin-three-way.cpp` and `builtin-three-way-mixed.cpp` test the
result types of the built-in candidates (OVL-9) with `<=>` inside
`decltype`. Both declared two of the three classes, and every compiler
accepted them. Both now declare all three classes, so they meet LSUP-2
and still test OVL-9.

## Evidence

The three LSUP-2 tests in **Affects**. The first and the third carry
overrides for EDG. The run-time test carries none. Before the change, the two OVL-9 tests declared
`std::strong_ordering` and `std::partial_ordering` only, and EDG
accepted them.

## Options

1. Every use of `<=>` on built-in operands and every defaulted `<=>`
   needs all three classes (the rule as written). No compiler checks
   the run-time form.
2. Only the uses that EDG checks need all three classes: a built-in
   `<=>` evaluated as a constant and a defaulted `<=>`. Other uses need
   only the class that they yield.
3. Each use needs only the class that it yields (Clang, GCC).

## Provisional answer

No; each use needs only the ordering class that it yields (option 3).
Clang and GCC accept each program, and the gap review of the text
phase 8 wrote on foundations reads the reference the same way. EDG
alone asks for the other classes, and only in two of the three forms.
The earlier answer, option 1, asked a program for declarations that no
use needs, and two of the three newest families do not check them.

Changed on 2026-11-04 from option 1 to option 3, after the gap review of the text phase 8 wrote on expressions, foundations, and templates (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`). LSUP-2 now asks for the class that a use yields. The three LSUP-2 tests expect the programs to run, and the constant and defaulted forms record EDG's rejection as an override. A new test, `ordering-partial-not-declared.cpp`, checks that a use of a class that the program does not declare is rejected on every compiler.

Kept open on 2026-11-04 under decision 0055, because LSUP-2 changed in this commit. The question waits for a reader or a reviewer to use the changed rule. EDG 7.0 is the one newest family that differs.

Used on 2026-11-04 by the principle-use batch PU1 (`reports/clean-room/PU1/`), under decision 0056. A clean-room reader applied LSUP-P1 and the comparison principles, without the rule text, to the programs of `ordering-strong-only-defaulted.cpp`, `builtin-three-way-mixed.cpp`, `builtin-three-way.cpp`, `ordering-strong-only-builtin-runtime.cpp`, `ordering-partial-not-declared.cpp`, and `ordering-strong-only-builtin.cpp`. Each answer gives the outcome that the tests expect. The answer for `builtin-three-way.cpp` names OVL-P4 and CMP-P5 and not LSUP-P1. The reader marked the answers for `ordering-strong-only-builtin-runtime.cpp` and `ordering-strong-only-builtin.cpp` doubtful (finding PU1-3), because LSUP-P1 does not say which uses need the comparison and the conversions of an ordering class. The use does not support the answer.

Kept open on 2026-11-04 under decisions 0055 and 0056, because the use in batch PU1 does not support the answer: one of the six answers does not name LSUP-P1, and two read LSUP-P1 with doubt (finding PU1-3). EDG 7.0 is the one newest family that differs.

Kept open on 2026-11-04 under decision 0055, because LSUP-P1 changed in this commit after finding PU1-3. LSUP-P1 now gives, in a table, the parts of an ordering class that each use of `<=>` needs, and says that a use needs only the class that it yields. The question waits for the next use of LSUP-P1. EDG 7.0 is the one newest family that differs.

## Opinions

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on expressions, foundations, and templates (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`) supports option 3. The reference says that the three classes are not predefined, and it asks for a library declaration of a class before each use of that class, an implicit use by a built-in or defaulted `<=>` included. The condition names the class that the use needs, and nothing asks for the classes that the use does not yield. A defaulted `<=>` over integer members yields the strong class, and the choice of a common comparison type among the classes is not a use of the other two. A program that declares the classes itself is outside the reference, as the earlier review of LSUP-5 noted.

### Gap reviews of classes and of declarations, 2026-11-04

The source-reader gap reviews of the text phase 8 wrote on classes and overloading (`reports/review/2026-11-04-gap-review-phase-8-classes-overloading.md`) and on declarations and statements (`reports/review/2026-11-04-gap-review-phase-8-declarations-statements.md`) name the question only to say that it affects no rule on their pages. Neither gives a reading.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
