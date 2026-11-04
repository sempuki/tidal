# Q-0179: Which throws in a non-throwing function should get a warning?

**Status:** provisional
**Affects:** UNW-8, `tests/exceptions/unwinding/throw-unmatched-in-noexcept-warns.cpp`

## Context

UNW-8 recommends a warning for a `throw` in a non-throwing function when
no enclosing try block in the function can catch the exception. Clang
(`-Wexceptions`) and GCC (`-Wterminate`) warn under `-Wall`, and they
draw the line in different places.

| Function body | Clang 18, 22, host | GCC 14, 16, host |
|---|---|---|
| `throw 1;` | warns | warns |
| `try { throw 1; } catch (int) { throw; }` | warns | warns |
| `throw;` outside a handler | warns | warns |
| `try { throw 1; } catch (long) {}` | warns | no warning |
| `try { g(); } catch (...) { throw; }` | no warning | warns |
| `try { throw 1; } catch (int) {}` | no warning | no warning |
| `try { throw Derived{}; } catch (Base&) {}` | no warning | no warning |

## Evidence

Probes with `-Wall` on the six Clang and GCC compilers. Clang and GCC
give each warning in the table with no warning flag, and `-Wall` adds
none of them. EDG gives none of the warnings. The tests use
`-Wall -Werror`, and EDG skips them. The table gives UNW-8's answer
for each body and the test that checks it.

| Function body | UNW-8 | Test |
|---|---|---|
| `throw 1;` | warn | `throw-in-noexcept-warns.cpp` |
| `try { throw 1; } catch (int) { throw; }` | warn | `rethrow-in-noexcept-warns.cpp` |
| `throw;` outside a handler | warn | none |
| `try { throw 1; } catch (long) {}` | warn | `throw-unmatched-in-noexcept-warns.cpp` |
| `try { g(); } catch (...) { throw; }` | warn | none |
| `try { throw 1; } catch (int) {}` | no warning | `throw-caught-in-noexcept-no-warning.cpp` |
| `try { throw Derived{}; } catch (Base&) {}` | no warning | `throw-caught-in-noexcept-no-warning.cpp` |

The tests are under `tests/exceptions/unwinding/`.

## Options

1. A try block can catch a `throw` with an operand when a handler is
   `catch (...)` or matches the operand's static type. A try block can
   catch a rethrow only with `catch (...)`. Both compilers miss one case
   each.
2. Recommend only the cases where both compilers warn.

## Provisional answer

Those that no enclosing handler catches by static type; a rethrow needs `catch (...)` (option 1). The definition follows from TRY-2 and THR-4, and a
recommended rule allows an implementation that does not warn.

Kept open on 2026-11-03 under decision 0054, because the gap review of the exceptions chapter finds the reference silent, so the review does not support the answer.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Gap review, 2026-11-03

The source-reader gap review of the exceptions chapter (`reports/review/2026-11-03-gap-review-exceptions.md`) reads the reference as silent on this question. The reference recommends no warning for a throw in a non-throwing function. It says only that such a throw is valid and ends the program. Both options are Tidal's own choice.
