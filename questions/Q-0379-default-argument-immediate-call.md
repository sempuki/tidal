# Q-0379: Is a `consteval` call in a default argument checked at the declaration?

**Status:** provisional
**Affects:** DEFARG-10, `tests/declarations/functions/default-arguments/default-immediate-call-checked-at-call.cpp`, `tests/declarations/functions/default-arguments/default-immediate-call-unused-not-checked.cpp`, `tests/declarations/functions/default-arguments/default-immediate-call-used-not-constant.cpp`
**Source:** reports/review/2026-11-04-gap-review-phase-7-additions.md, the DEFARG-10 section

## Context

DEFARG-10 checks a default argument at the declaration. The gap review
of the rules that phase 7 added reads the reference as making one
exception. A call of an immediate function in a default argument is
neither evaluated nor checked as a constant there. Each call that uses
the default argument checks it.

```cpp
consteval int width();
int pad(int w = width()) { return w; }    // width has no definition yet
consteval int width() { return 3; }
int main() { return pad() == 3 ? 0 : 1; }

int limit = 2;
consteval int clamp(int v) { return v; }
int fill(int w = clamp(limit)) { return w; }   // no call uses the default
```

| Program | Clang 22 | GCC 16 | EDG 7.0 |
|---|---|---|---|
| `width` defined after the declaration of `pad` | accepts, runs | accepts, runs | rejects at the declaration |
| `clamp(limit)` in a default that no call uses | accepts | accepts | rejects at the declaration |

## Evidence

The two tests in **Affects**. Each expects the result of Clang and GCC
and records EDG's rejection as an override.

## Options

1. The call is checked where a call of the function uses the default
   argument (Clang, GCC, and the review's reading).
2. The call is checked at the declaration, as the rest of the default
   argument is (EDG).

## Provisional answer

The call is checked where a call uses the default argument (option 1).
Clang 22 and GCC 16 agree, and Tidal follows the two when EDG alone
differs (`documentation/decisions/pending.md`, row 3). DEFARG-P1 makes a
default argument act as if written at the call.

Kept open on 2026-11-04 (replaced by the next note) under decision 0054, because no review has read the provisional answer, and EDG 7.0 differs from it. Item RV2 of decision 0056 reads the question.

Read on 2026-11-04 by the source-reader gap review of the text phase 8 wrote on the declarations and statements chapters (`reports/review/2026-11-04-gap-review-phase-8-declarations-statements.md`), which supports option 1 with no doubt. EDG 7.0 is the one newest family that differs. DEFARG-P1, which makes a default argument act as if written at the call, is not a principle that phase 8 widened, and its model gives the answer. Kept open on 2026-11-04 under decision 0055, because DEFARG-10, the rule the question affects, changed in this commit: a friend function defined in a class and a member function of a nested class now have their default arguments checked as if at the end of the outermost class. The question waits for the next review or reader batch that reads the changed DEFARG-10.

Used on 2026-11-04 by the principle-use batch PU1 (`reports/clean-room/PU1/`), under decision 0056. A clean-room reader applied the principles, without the rule text, to the programs of `default-immediate-call-checked-at-call.cpp` and `default-immediate-call-unused-not-checked.cpp`. Each answer gives the outcome that the tests expect. Neither answer names DEFARG-P2, the principle that the batch key lists. Both name DEFARG-P1 and CX-P3. The reader marked the answer for `default-immediate-call-unused-not-checked.cpp` doubtful (finding PU1-2), because neither principle says where a `consteval` call in a default argument is checked. The use does not support the answer.

Kept open on 2026-11-04 under decisions 0055 and 0056, because the use in batch PU1 does not support the answer: no answer names DEFARG-P2, and the reader read one form with doubt (finding PU1-2). EDG 7.0 is the one newest family that differs.

Kept open on 2026-11-04 under decision 0055, because DEFARG-P1 and CX-P3 changed in this commit after finding PU1-2. DEFARG-P1 now says that the declaration checks a default argument at once, except a call of an immediate function, which each call that uses the default argument evaluates and checks. DEFARG-P1 now lists DEFARG-10. A new test, `tests/declarations/functions/default-arguments/default-immediate-call-used-not-constant.cpp`, checks that every compiler rejects a call that uses such a default argument when the argument is not a constant. The question waits for the next use of DEFARG-P1. EDG 7.0 is the one newest family that differs.

## Opinions

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on the declarations and statements chapters (`reports/review/2026-11-04-gap-review-phase-8-declarations-statements.md`) agrees with option 1. The reference makes this exception in its rule on checking default arguments: a call of an immediate function there is neither evaluated nor checked as a constant at the declaration. The exception came with the C++23 change that lets immediate calls escalate, and was also applied to C++20 as a defect resolution. The review reads EDG 7.0 as not implementing it.

### Gap review of the expressions, foundations, and templates text, 2026-11-04

The source-reader gap review of the text phase 8 wrote on the expressions, foundations, exceptions, and templates chapters (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`) names this question only to leave it out, because the question affects a rule on another page. The review gives no reading.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
