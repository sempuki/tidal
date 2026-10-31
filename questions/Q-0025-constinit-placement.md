# Q-0025: Which declarations of a variable must carry `constinit`?

**Status:** provisional
**Affects:** CINIT-2, `tests/declarations/constexpr/constinit-added-late.cpp`, `tests/declarations/constexpr/constinit-on-declaration-only.cpp`

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

Option 1. This option is the one rule that explains both rejections, and
it matches Clang for both programs and EDG for the second.

## Opinions
