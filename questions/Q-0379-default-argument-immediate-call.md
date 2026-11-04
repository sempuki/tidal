# Q-0379: Is a `consteval` call in a default argument checked at the declaration?

**Status:** provisional
**Affects:** DEFARG-10, `tests/declarations/functions/default-arguments/default-immediate-call-checked-at-call.cpp`, `tests/declarations/functions/default-arguments/default-immediate-call-unused-not-checked.cpp`
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

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
