# Q-0389: Can a default argument read the value of a local constant of an enclosing function?

**Status:** provisional
**Affects:** DEFARG-4, DEFARG-P1, `tests/declarations/functions/default-arguments/local-constant-value-read.cpp`, `tests/declarations/functions/default-arguments/local-constexpr-value-read.cpp`
**Source:** reports/review/2026-11-04-gap-review-phase-8-declarations-statements.md, the DEFARG-P1 section
**Waits for:** the maintainer (pending.md row 4)

## Context

DEFARG-4 lets a default argument name a local variable of an enclosing
function only in an operand that is not evaluated. The gap review of
the text phase 8 wrote reads the C++23 text as banning only a use of
the local that needs the local's storage. A read of the value of a
local constant of integer type needs no storage, so the review reads it
as allowed.

```cpp
int main() {
  const int count = 3;
  int pick(int n = count);
  return 0;
}
```

| Program | Clang 22 | GCC 16 | EDG 7.0 |
|---|---|---|---|
| a `const int` local as the default | accepts | rejects | rejects |
| a `constexpr int` local in `limit + 1` | accepts | rejects | rejects |
| an `int` local that is not `const` | rejects | rejects | rejects |

## Evidence

The two tests in **Affects**. Each expects a rejection and records
Clang's acceptance as an override.

## Options

1. The default argument is not valid. A local of an enclosing function
   can appear only in an operand that is not evaluated (GCC, EDG).
2. The default argument is valid, because reading the value of a local
   constant does not use the local's storage (Clang, and the review's
   reading).

## Provisional answer

The default argument is not valid (option 1). GCC 16 and EDG 7.0
agree, and DEFARG-P1 says that the locals of an enclosing function
exist only inside a call. Tidal keeps the matrix's answer against the
review (`documentation/decisions/pending.md`, "Matrix against
review").

Kept open on 2026-11-04 under decision 0054, because the gap review reads against the answer, and Clang 22 differs from it.

## Opinions

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on the declarations and statements chapters (`reports/review/2026-11-04-gap-review-phase-8-declarations-statements.md`) supports option 2. For a parameter, the reference bans every use that could be evaluated. For a local of an enclosing function, it bans only a use that needs the object itself. Reading the value of a local constant of integral type is not such a use, so a default argument can read it. DEFARG-P1 and DEFARG-4 allow a local only in an operand that is not evaluated, which the review finds too narrow.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
