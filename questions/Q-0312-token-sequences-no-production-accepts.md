# Q-0312: Should the specification list token sequences that no production accepts, such as `for constexpr`?

**Status:** provisional
**Affects:** `foundations/translation.md`, `statements/index.md`, `statements/range-for.md`, the grammar of each statements page
**Waits for:** the maintainer (pending.md row 9)

## Context

The `statement` production and the `range_for_stmt` production accept
no keyword between `for` and `(`. So `for constexpr (...)` is not a
valid statement. The grammar shows this fact. No rule names the form
itself.

A reader who looks for the forms that the language leaves unused reads
the rules and their tests. Such a reader finds `for constexpr` only in a
test of TR-12, the general rule for token sequences that the grammar
does not derive. The same holds for other keyword pairs in a statement
position that no production accepts.

| Probe | GCC 16.2 | Clang 22 |
|---|---|---|
| `for constexpr (int v : a) {}` in a function | rejects, expects `(` before `constexpr` | rejects, expects `(` after `for` |

## Evidence

The probe above. The grammar in `specification/statements/range-for.md`
and `specification/statements/loops.md`. The tests of TR-12 in
`tests/foundations/translation/`, which every compiler in the matrix
rejects.

## Options

1. Leave the grammar as the only statement. A program that no
   production accepts is not valid, and a list of such programs has no
   end.
2. Add a `reject` rule with a test for each form that a reader is
   likely to try, such as `for constexpr`, beside the grammar of its
   statement.
3. Keep a list of such forms outside the specification, for example in
   the roadmap's index of unused forms, and leave the specification
   unchanged.
4. Add one general `reject` rule that gives the grammar its effect, with
   tests for a few token sequences that no production derives.

## Provisional answer

The grammar is the statement, and TR-12 gives its effect (option
4). TR-12 makes a token sequence that the grammar does not derive a
program that is not valid, and the implementation rejects it. The
diagnostic need not name a rule other than TR-12. No rule lists such
sequences one by one, because the list has no end. So `for constexpr`
has no rule of its own, and its test cites TR-12. KW-1, JMP-17, TARG-4,
STMT-6, and DCL-13 cite TR-12. The maintainer decides whether the
roadmap also keeps a list of such forms (option 3).

Kept open on 2026-11-03 under decision 0054, because the question waits on the maintainer (`documentation/decisions/pending.md`, row 9).

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
