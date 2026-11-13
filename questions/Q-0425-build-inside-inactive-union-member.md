# Q-0425: Can a program build an object inside a union member that is not active, as an element of an array member or a member of a member union?

**Status:** provisional
**Affects:** UNI-14, UNI-6, CE-6, STG-10, `tests/classes/unions/construct-in-member-of-inactive-member-union-constant.cpp`, `tests/classes/unions/construct-in-member-of-inactive-member-union-run.cpp`, `tests/classes/unions/construct-at-array-member-element-constant.cpp`, `tests/classes/unions/array-member-element-address-no-active-member.cpp`, `tests/classes/unions/array-member-element-steps-no-active-member-constant.cpp`
**Source:** W20-2 (`reports/clean-room/W20/report.md`) with the writer probe `reports/clean-room/roadmap-10/writer-probes/w20-2-union-array-member.json`, and the round 10 programmer report (`reports/clean-room/roadmap-10/programmer.md`, sections 1.4, 1.5, and 6), reduction `construct_in_member_of_inactive_union_member.cpp`.
**Waits for:** a review (doubt); newest families that agree

## Context

UNI-14 makes a union member active when placement `new` or
`std::construct_at` creates an object of the member's type at the
member's address. No rule covers an object of another type inside the
member: an element of an array member, or a member of a member that is
itself a union. Two forms reach the case.

A union `Outer` has a member `inner` of union type, and no member of
`Outer` is active. In constant evaluation, `std::construct_at` builds a
`double` at the address of `inner.d`:

| Form | Clang 18 and 22 | GCC 14 and 16 | EDG 7.0 | Tidal front end |
|---|---|---|---|---|
| the constant | rejects | rejects | accepts | rejects (UNI-6) |
| the same at run time | runs, also with the sanitizers | runs, also with the sanitizers | accepts | stops (UNI-6) |

A union `Store` has a member `cells` of type `int[3]`, and no member is
active:

| Form | Clang 18 and 22 | GCC 14 | GCC 16 | EDG 7.0 | Tidal front end |
|---|---|---|---|---|---|
| `&s.cells[0]` and `&s.cells[1]` at run time, with no read | runs | runs | runs | accepts | stops ("unsupported") |
| `std::construct_at` of an element in constant evaluation | rejects | rejects | accepts | accepts | not probed |

Clang's note for the constant forms says that the construction of a
subobject of a member of a union with no active member is not allowed
in a constant expression.

## Evidence

The tests in **Affects**:

- The constant test with a member union expects GCC's and Clang's
  rejection and records EDG as an override.
- The run test with a member union expects a run on every compiler
  that runs programs, and in the front end's interpreter (UNI-14).
- The constant test of `std::construct_at` on an element of an array
  member expects the rejection of Clang and GCC 14, and records GCC 16
  and EDG as overrides.
- The run test forms pointers to two elements of an array member that
  is not active.
- The constant test steps between such pointers, subtracts them, and
  compares them. Every compiler accepts it, also with another member
  active (`reports/clean-room/roadmap-10/writer-probes/f3b-w20-2-step-in-inactive-array.json`).

The writer probe `reports/clean-room/roadmap-10/writer-probes/w20-2-union-array-member.json`
gives the forms of the array member.

## Options

1. Building an object inside a member that is not active does not make
   the member active. Constant evaluation rejects the construction. At
   run time, a pointer to an element or to a member of such a member
   can be formed (STG-10), and a program that only forms and compares
   such pointers runs. A construction there at run time is valid, and
   the program can use the new object through the pointer that the
   construction gives.
2. Building the object makes the enclosing member active, as UNI-14
   does for an object of the member's type. Constant evaluation accepts
   the construction (EDG, and GCC 16 for an array element).

## Provisional answer

No; the construction does not make the member active, and constant
evaluation rejects it (option 1). Clang and GCC reject the form with a
member union, and Clang and GCC 14 reject it with an array member. A
program that only forms pointers into the member runs on every
compiler.

Kept open on 2026-11-10 under decision 0054, because the review reads the answer with a doubt on the run-time form, and two newest families (GCC 16 and EDG) differ in the array form (item RV6).

Rule text on 2026-11-10 (decision 0061, item RV6): UNI-14 and UNI-6 limit the rejection to constant evaluation. At run time the construction is valid, and the program can use the new object through the pointer that the construction gives. The run test now expects a run on every compiler and in the front end's interpreter. Whether the enclosing member then becomes active is the review's remaining doubt, and no test tells the outcomes apart.

Kept open on 2026-11-11 under decision 0054 (item RV7). The review agrees with the run-time text of UNI-14 and keeps the RV6 doubt on the constant form, and two newest families still differ in the array form.

Rule text on 2026-11-11 (decision 0064, item F6, ledger row X25-4):
UNI-14 says that the enclosing member does not become active at run
time either, as the RV8-A-8 reading of the phase 15 review gives. A
read through the member's name is then a read of a member that is not
active (UNI-6), and a member that was active stays active unless the
new object takes its storage. On the six compilers that run programs,
the X25 probe `a4.cpp -DRUN` reads the new element through the
member's name and the old value of the member that was active, with
no sanitizer report
(`reports/clean-room/roadmap-13/writer-probes/run-probes.out`). The
read through the name is undefined, so no test holds it. The constant
form of the array case still splits. GCC 16 and EDG accept the
construction, and Clang 22 rejects it
(`reports/clean-room/roadmap-13/writer-probes/x25-constant.json`).

Named on 2026-11-12 (decision 0066, item RV12). Part B of the gap review of the phase 18 text keeps the RV9-7 point on `std::construct_at` of a whole union member that is not active without a question (RV12-17). The `pending.md` row 4 item "LSUP-10, UNI-14" holds that form, and this question holds the narrower forms that rest on the same allowance. The answer and the **Waits for:** line do not change.

## Opinions

### Gap review, 2026-11-10 (phase 13 text, classes, declarations, foundations, and statements)

The source-reader gap review of the text phase 13 wrote on classes, declarations, foundations, and statements (`reports/review/2026-11-10-gap-review-phase-13-classes-declarations-foundations.md`) reads the question: supports with a doubt (RV6-A-13, RV6-A-14). Constant evaluation rejects both constant forms, because the member that encloses the new object has not started its lifetime, which option 1 says. The doubt is at run time. The review reads the construction there as defined, against UNI-6's last sentence and the run test's expectation that the interpreter stops. The new object either reuses the union's storage, or, when a whole object of the enclosing member's type fits at the member's address, makes that member active. In the member-union form the inner union and the `double` have the same size, so the second outcome is possible. In the array form it is not.

### Gap review, 2026-11-11 (phase 14 text, classes, declarations, foundations, and statements)

The source-reader gap review of the text phase 14 wrote on classes, declarations, foundations, and statements (`reports/review/2026-11-11-gap-review-phase-14-classes-declarations-foundations.md`) names the question in its read of the RV6 repairs to `unions.md` (RV7-A-4). The Q-0425 divergence line and the run-time sentence of UNI-14 agree: at run time the construction is defined, and the program uses the new object through the pointer that the construction gives. The doubt of RV6-A-13 stays. The allowance that lets constant evaluation use the library's construction function names storage from the library allocator or an object whose lifetime began in the evaluation, and a member that is not active has not begun. The verdict stays supports with a doubt.

### Gap review, 2026-11-12 (phase 18 text)

The source-reader gap review of the phase 18 text (`reports/review/2026-11-12-gap-review-phase-18.md`) names the question in Part B, on RV9-7 at LSUP-10 and UNI-14 (RV12-17). The review finds no need for a new question there. Every compiler accepts `std::construct_at` of a whole union member that is not active, no test carries an override, and the row 4 item records the split. The review names this question as the natural place for a line on the point, since it holds the narrower forms that rest on the same allowance. The review gives no new reading of this question's answer.
