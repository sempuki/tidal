# Q-0453: Is a program rejected when the constraint check of a hidden friend operator template needs its own result through a candidate whose other parameter cannot take the argument?

**Status:** provisional
**Affects:** CNS-35, CNS-11, `tests/templates/constraints/recursive-satisfaction-hidden-friend-reversed.cpp`
**Source:** the round 15 programmer report (`reports/clean-room/roadmap-15/programmer.md`, sections 1.5, 1.6, and 6), reduction `recursive_constraint_hidden_friend_mixin.cpp`, and the programmer's sketch `strong_first_try.cpp`.
**Waits for:** a first review

## Context

A class template `Scaled<D>` is a base for `D`. It declares two hidden
friend operator templates, `operator*(const D&, K)` and
`operator*(K, const D&)`. Each has the constraint that
`d.value() * k` is valid for a `const D&` and a `K`. A class `Meters`
derives from `Scaled<Meters>`, has an `explicit` constructor from
`long long`, and `value()` returns a `long long`.

For `Meters(4) * 2`, the candidate `operator*(K, const Meters&)`
deduces `K = Meters`. Its constraint needs `d.value() * k` for a
`long long` and a `Meters`. That expression considers
`operator*(const Meters&, K)` with `K = Meters`. The first parameter of
that candidate cannot take a `long long`, because the constructor is
`explicit`. If the constraint of that candidate is checked anyway, the
check needs `d.value() * k` for a `long long` and a `Meters` again, so
the check of the constraint with `K = Meters` needs its own result.

| Form | Clang 18 | Clang 22 | GCC 14 and 16 | EDG 7.0 |
|---|---|---|---|---|
| `Meters(4) * 2`, `explicit` constructor | accepts | rejects (the constraint depends on itself) | rejects (the same) | accepts |
| `3 * Meters(4)` and `2LL * Meters(4)`, `explicit` constructor | accepts | rejects | rejects | accepts |
| a concept that tests `a * b` for a `Meters` and an `int`, `explicit` constructor | accepts | rejects | rejects | accepts |
| `Meters(4) * 2`, constructor not `explicit` | rejects | rejects | rejects | accepts |
| `Meters(4) * 2`, only `operator*(const D&, K)` declared | accepts | accepts | accepts | accepts |

The old front end runs the program to 0, as EDG does.

## Evidence

The test in **Affects** holds the first form. It expects `reject` and
records EDG and Clang 18 as overrides. The writer probed each form of
the table on the matrix with `tools/probe_grid.py`.
`tests/templates/constraints/recursive-satisfaction-operator.cpp`
holds a concept form of CNS-35 that every compiler rejects.

## Options

1. CNS-11 checks the constraint of a candidate once deduction has given
   its template arguments, before the parameters that do not depend on
   them take their arguments. So the check needs its own result, and
   CNS-35 rejects the program (Clang 22, GCC).
2. A candidate leaves the set before its constraint is checked when a
   parameter with a fixed type, such as `const Meters&`, cannot take
   its argument. The recursion does not arise, and the program
   is valid (Clang 18 for the `explicit` forms).
3. The program is valid in every form of the table (EDG).

## Provisional answer

Yes; the program is rejected (option 1).
CNS-11 places the constraint check right after deduction and says
nothing of the parameters that do not depend on the template
parameters, so the check of `operator*(const Meters&, K)` with
`K = Meters` runs and needs its own result (CNS-35). Clang 22 and GCC
14 and 16 reject. EDG 7.0 accepts every form, and Clang 18 accepts the
forms where the first parameter cannot take the argument.

Opened on 2026-11-11 (decision 0066, item T7). Two newest families
agree, and EDG alone differs. No review has read the question.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
