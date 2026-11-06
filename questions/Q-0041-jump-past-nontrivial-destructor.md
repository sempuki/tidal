# Q-0041: May a jump skip a class object that has a non-trivial destructor and no initializer?

**Status:** provisional
**Affects:** JMP-6, SW-5, `tests/statements/jumps/goto-past-nontrivial-destructor.cpp`, `tests/statements/jumps/goto-skips-nontrivial-destructor.cpp`, `tests/statements/switch/case-skips-nontrivial-destructor.cpp`
**Waits for:** the maintainer (pending.md row 6); newest families that agree

## Context

A `goto` or a `case` label skips the declaration, without an
initializer, of a variable whose class has a trivial default
constructor and a user-provided destructor, such as `Tidy t;`.

| Jump | Clang 22, 18 | GCC 16, 14 | EDG 7.0 |
|---|---|---|---|
| `goto` past the declaration | rejects | accepts | rejects |
| `case` label past the declaration | rejects | accepts | rejects |

A jump past a class with a trivial constructor and a trivial destructor
is accepted everywhere.

SW-5 refers to JMP-6 for the types that a `case` label can skip. Q-0097
asked the same question for both rules and is a duplicate of this
question.

## Evidence

The three tests in **Affects**. Each expects acceptance and has
overrides for Clang and EDG.

The class-properties inventory (`reports/review/2026-11-03-inventory-class-properties.md`,
items OBS-jump-vacuous and GAP-JMP-6) describes the limit in terms of
the default constructor only. A jump can pass a declaration whose
default initialization does nothing: a scalar, or a class with a
trivial default constructor. The destructor is not part of the
condition. The skipped variable still exists after the jump (JMP-16),
and the end of the block destroys it, so a jump skips no code.

## Options

1. Rejected: only types with a trivial default constructor and a
   trivial destructor can be skipped. GCC's acceptance is a possible
   defect.
2. Accepted when the constructor is trivial.

## Provisional answer

Yes; JMP-6 and SW-5 accept the jump; Clang and EDG reject it (option 2). The
class-properties inventory names only the default constructor, GCC
agrees, and the jump skips no construction. The destructor runs at
the end of the block in either case.

Kept open on 2026-11-03 under decision 0054, because the question waits on the maintainer (`documentation/decisions/pending.md`).

Noted on 2026-11-05 under decision 0058, item PK2. A test whose notes name the question has two newest families that differ from the answer, so the question also waits for newest families that agree (`pending.md` row 1). The check of `tools/question_index.py` found the case.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of item O (`reports/review/2026-11-03-gap-review-item-o.md`) reads the reference as supporting option 2, the provisional answer. A jump can skip a variable whose default initialization does nothing. For a class, the default constructor must be trivial, and the destructor plays no part. The review describes the destructor condition as removed by a defect resolution in the C++20 cycle, which GCC follows and Clang and EDG do not.
