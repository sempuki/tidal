# Q-0193: Can a constant refer to, and read, every temporary that a namespace reference extends?

**Status:** provisional
**Affects:** CE-14, CE-3, `tests/expressions/constant-expressions/extended-temporary-member.cpp`, `tests/expressions/constant-expressions/extended-temporary-nested.cpp`, `tests/expressions/constant-expressions/read-extended-temporary-class.cpp`, `tests/expressions/constant-expressions/read-extended-temporary-nested.cpp`

## Context

REF-4 extends a temporary in several ways: a direct binding, a binding
through parentheses, `?:`, a comma, or a cast, a binding to a member of
the temporary, and a reference member of a braced aggregate. A temporary
that a reference with static storage extends lives until the program
ends. CE-14 lets a constant refer to an object with static storage.

The compilers split on two points.

The first point is the declaration of the reference.

| `constexpr` reference at namespace scope | Clang | GCC | EDG |
|---|---|---|---|
| bound directly, or through a cast, `?:`, or a comma | accepts | accepts | accepts |
| bound to a member of a class temporary | accepts | accepts | rejects |
| bound to a braced aggregate whose reference member binds a temporary | accepts | accepts | rejects |

EDG says that the reference refers to a temporary with a limited
lifetime. EDG accepts a `constexpr` aggregate variable whose reference
member binds a temporary (CX-2).

The second point is a later read of the temporary in a constant
expression.

| Read through the `constexpr` reference | Clang | GCC | EDG |
|---|---|---|---|
| a `const` scalar temporary | accepts | accepts | accepts |
| a temporary that is not `const` | rejects | rejects | rejects |
| a `const` temporary of class type | accepts | rejects | accepts |
| the member reference of a braced aggregate temporary | accepts | rejects | rejects the declaration |

GCC says that the value of the temporary is not usable in a constant
expression. The results are the same for Clang 18 and 22 and for GCC 14
and 16.

## Evidence

The test files listed under **Affects**, and
`tests/expressions/constant-expressions/extended-temporary-through-cast.cpp`
and `read-extended-temporary-not-const.cpp`.

## Options

1. Every temporary that REF-4 extends for a reference with static
   storage counts as static, whatever the form of the binding. A
   constant can read such a temporary if its type is `const`. EDG
   differs on the declaration, and GCC differs on the read of a class.
2. Only a temporary bound directly, or through parentheses, `?:`, a
   comma, or a cast, counts as static. Clang and GCC differ.
3. As option 1, but a constant can read only a scalar temporary. Clang
   and EDG differ.

## Provisional answer

Option 1. REF-4 gives every form of extension the same lifetime, and
CE-14 follows the lifetime. Clang accepts every case, and on the read of
a class EDG agrees with Clang. A rule that depends on the form of the
binding would be harder to state and to check.

## Opinions
