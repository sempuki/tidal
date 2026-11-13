# Q-0456: Can a braced list be a template argument?

**Status:** provisional
**Affects:** TARG-16, `tests/templates/arguments/class-arguments.cpp`, `tests/templates/arguments/same-specialization-class.cpp`, `tests/templates/arguments/braced-scalar-arguments.cpp`
**Source:** RV10-6 (`reports/review/2026-11-11-gap-review-phase-17.md`), and RV12-10 and RV12-16 (`reports/review/2026-11-12-gap-review-phase-18.md`, Part B), with the writer probes `reports/clean-room/roadmap-14/writer-probes/rv10-review.json` and `reports/clean-room/roadmap-15/writer-probes/rv11-rows.json`.
**Waits for:** the maintainer (pending.md row 4)

## Context

The `braced_init` arm of `template_arg` lets a braced list stand as
the argument for a constant parameter. TARG-16 says that the list
initializes an argument of class type by copy-initialization, and that
a list for a parameter of scalar type holds one value, or none, which
gives zero:

```cpp
struct Point { int x; int y; };
template <Point P> struct At { static constexpr int sum = P.x + P.y; };
static_assert(At<{3, 4}>::sum == 7);

template <int N> struct Count { static constexpr int value = N; };
static_assert(Count<{3}>::value == 3);
static_assert(Count<{}>::value == 0);
```

| Form | Clang 18 and 22 | GCC 14 and 16 | EDG 7.0 |
|---|---|---|---|
| `At<{3, 4}>` and `At<{}>` for a parameter of class type | accepts | accepts | accepts |
| `Count<{3}>` and `Count<{}>` for an `int` parameter | accepts | accepts | accepts |
| `{nullptr}` and `{}` for a pointer parameter | accepts | accepts | accepts |

The gap reviews of the phase 17 and phase 18 text read the C++23 text
as giving a template argument no braced form. There a class value needs
a written type, as in `At<Point{3, 4}>`.

## Evidence

The tests in **Affects** expect acceptance with no override, and the
probes in **Source** give each form. {question}`Q-0454` rests on the
same arm: a braced integer for a floating-point parameter. Its answer
rejects the program under either reading.

## Options

1. A braced list is a template argument for a constant parameter. It
   initializes a parameter of class type, and a list for a scalar
   parameter holds one value or none (every compiler).
2. A template argument has no braced form, and each form in the table
   is not valid (the gap reviews of the phase 17 and phase 18 text).

## Provisional answer

A braced list is a template argument (option 1). Every compiler
accepts each form, and TARG-16 and the `braced_init` arm of
`template_arg` follow the compilers. The `pending.md` row 4 item
"TARG-16" holds the split.

Opened on 2026-11-12 (decision 0066, item RV12). Part B of the gap
review of the phase 18 text asks for a question on RV10-6 (RV12-16).
A whole grammar arm and several tests rest on the point, and
{question}`Q-0454` can cite this question as its premise. No principle
gives the answer. TARG-P2 says that a constant parameter takes a
constant expression that converts to the parameter's type, and it does
not say whether a braced list is such an argument, so both options fit
TARG-P2's model.

Rule text on 2026-11-12 (decision 0066, item RV12): TARG-16, the TARG Notes, and the Divergences list name this question, and so do the notes of the three tests in **Affects** (RV12-16). The `pending.md` row 4 item is now "TARG-16, Q-0456". The answer and the **Waits for:** line do not change.

## Opinions

### Gap review, 2026-11-11 (phase 17 text)

The source-reader gap review of the phase 17 text (`reports/review/2026-11-11-gap-review-phase-17.md`) reads the `braced_init` arm of `template_arg` and TARG-16 against the C++23 text (option 2, RV10-6). The reference's template argument is a constant expression, a type, or a name, with no braced list. A class value needs a written type, as in a functional cast with braces, and that form is a constant expression. Every compiler accepts the braced forms, so the split is between the reference and every compiler.

### Gap review, 2026-11-12 (phase 18 text)

The source-reader gap review of the phase 18 text (`reports/review/2026-11-12-gap-review-phase-18.md`) reads TARG-16's sentence on a parameter of scalar type, the arguments paragraph, and `braced-scalar-arguments.cpp` against the C++23 text (option 2, RV12-10), as RV10-6 found for the class case. In Part B the review answers yes to a question for RV10-6. A whole grammar arm and several tests rest on the point, Q-0454 can then cite its premise, and a later change to the language may add the form, which `pending.md` row 7 keeps out of scope.

### Read of the RV12 repair diff, 2026-11-12

The source-reader read of the RV12 repair diff (`reports/review/2026-11-12-read-of-rv12-repair-diff.md`) reads the TARG lines and the three test notes that name the question (RV12-16): agrees as a record. TARG-16 and the braced arm of `template_arg` still read against the reference, which gives a template argument no braced form, and the lines state that correctly. The read gives no new reading of the question's answer.
