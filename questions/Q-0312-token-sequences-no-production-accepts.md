# Q-0312: Should the specification list token sequences that no production accepts, such as `for constexpr`?

**Status:** open
**Affects:** `statements/index.md`, `statements/range-for.md`, the grammar of each statements page

## Context

The `statement` production and the `range_for_stmt` production accept
no keyword between `for` and `(`. So `for constexpr (...)` is not a
valid statement. Only the grammar shows this fact. No `reject` rule
names the form, and no test covers it.

A reader who looks for the forms that the language leaves unused reads
the rules and their tests. Such a reader does not find `for constexpr`,
because the form breaks no rule. The same holds for other keyword pairs
in a statement position that no production accepts.

| Probe | GCC 16.2 | Clang 22 |
|---|---|---|
| `for constexpr (int v : a) {}` in a function | rejects, expects `(` before `constexpr` | rejects, expects `(` after `for` |

## Evidence

The probe above. The grammar in `specification/statements/range-for.md`
and `specification/statements/loops.md`.

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

## Provisional answer

No; the grammar stays the only statement (option 1), and a list of such forms can live in the roadmap (option 3). The
grammar already rejects the form, and a rule for one unused keyword pair
would invite a rule for every pair. The maintainer decides whether the
roadmap keeps the list.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
