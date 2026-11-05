# Q-0025: Which declarations of a variable must carry `constinit`?

**Status:** provisional
**Affects:** CINIT-2, CINIT-3, `tests/declarations/constexpr/constinit-added-late.cpp`, `tests/declarations/constexpr/constinit-on-declaration-only.cpp`

## Context

| Program | Clang 22, 18 | GCC 16, 14 | EDG 7.0 |
|---|---|---|---|
| `int x = 1; extern constinit int x;` | reject ("added after initialization") | accept | accept |
| `extern constinit int y; int y = 1;` | reject (pedantic: "missing on initializing declaration") | accept | reject ("previously declared with constinit") |

## Options

1. If any declaration of a variable has `constinit`, the initializing
   declaration must have it. Both programs are rejected.
2. `constinit` on any declaration is enough. Both programs are accepted,
   as GCC does.
3. Follow the majority for each program separately: reject the second and
   accept the first. The result has no single rule behind it.

## Provisional answer

If any declaration of a variable has `constinit`, the initializing declaration must have it (option 1); a `constinit` that first appears after the initializing declaration needs no diagnostic (CINIT-3). Clang rejects both programs, EDG rejects the second, and GCC accepts both.

Changed on 2026-11-03 for the first program. The review reads it as not valid with no diagnostic required, GCC 16 and EDG 7.0 accept it, and CX-P4 gives the same answer: the check happens where the initializer is, and a `constinit` that first appears later comes too late for that check. The new rule CINIT-3 needs no diagnostic, and `constinit-added-late.cpp` now cites it, so all three results conform. CINIT-2 keeps the second program, which Clang and EDG reject. The question stays open, because GCC 16 still differs for the second program and the rule changed on the same day.

Kept open on 2026-11-04 under decision 0054, because GCC 16 differs from the answer for the second program, and the gap review of the rules that phase 7 added read only CINIT-3, the rule for the first program.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with the rule of option 1 and reads the first program as needing no diagnostic. If any declaration of a variable has `constinit`, the initializing declaration must have it. A breach needs no diagnostic when no declaration with `constinit` is reachable from the initializing declaration. In the first program the `constinit` comes after the initializer, so GCC's and EDG's acceptance conforms. In the second program the `constinit` comes first, a diagnostic is required, and GCC's acceptance is a defect.
