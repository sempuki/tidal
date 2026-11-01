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

Probes with `-Wall` on the six Clang and GCC compilers. Tests:
`tests/exceptions/unwinding/rethrow-in-noexcept-warns.cpp`,
`tests/exceptions/unwinding/throw-caught-in-noexcept-no-warning.cpp`,
`tests/exceptions/unwinding/throw-unmatched-in-noexcept-warns.cpp`.

## Options

1. A try block can catch a `throw` with an operand when a handler is
   `catch (...)` or matches the operand's static type. A try block can
   catch a rethrow only with `catch (...)`. Both compilers miss one case
   each.
2. Recommend only the cases where both compilers warn.

## Provisional answer

Option 1. The definition follows from TRY-2 and THR-4, and a
recommended rule allows an implementation that does not warn.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
