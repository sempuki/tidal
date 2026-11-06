# Q-0193: Can a constant refer to, and read, every temporary that a namespace reference extends?

**Status:** provisional
**Affects:** CE-14, CE-3, `tests/foundations/constant-evaluation/extended-temporary-member.cpp`, `tests/foundations/constant-evaluation/extended-temporary-nested.cpp`, `tests/foundations/constant-evaluation/read-extended-temporary-class.cpp`, `tests/foundations/constant-evaluation/read-extended-temporary-nested.cpp`
**Waits for:** a review (doubt); newest families that agree

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
lifetime. Every compiler, EDG included, accepts a `constexpr` aggregate variable
whose reference member binds a temporary (CX-2).

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
`tests/foundations/constant-evaluation/extended-temporary-through-cast.cpp`
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

Yes for any REF-4 binding, and a read needs a `const` type; EDG and GCC differ (option 1). REF-4 gives every form of extension the same lifetime, and
CE-14 follows the lifetime. Clang accepts every case, and on the read of
a class EDG agrees with Clang. A rule that depends on the form of the
binding would be harder to state and to check.

The question stays open under decision 0051 (2026-11-03). The newest versions still split by part. EDG 7.0 rejects the two `constexpr` references whose temporary is extended through a member or through a braced aggregate, and Clang 22 and GCC 16 accept them. GCC 16 rejects the read of a `const` temporary of class type, and Clang 22 and EDG 7.0 accept it. CE-P2 supports the provisional answer, because a `const` temporary that a constant variable keeps alive is among the things that an evaluation can use, and REF-4 gives every form of binding the same lifetime. The front end's interpreter accepts the declarations and the read of a class temporary, and rejects the read of the nested temporary with OBJ-2.

Kept on 2026-11-03. The source-reader gap review of question cluster 4 agrees with option 1, with doubt. The review notes that the front end's interpreter rejects the read of the nested temporary with OBJ-2, and the provisional answer allows that read when both temporaries have a `const` type. This mismatch goes to item FE.

Kept open on 2026-11-03 under decision 0054, because the gap review does not support the answer with no doubt (it reads with doubt, weakly, unclearly, or for only part of the question).

Noted on 2026-11-05 under decision 0058, item PK2. A test whose notes name the question has two newest families that differ from the answer, so the question also waits for newest families that agree (`pending.md` row 1). The check of `tools/question_index.py` found the case.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 4 (`reports/review/2026-11-03-gap-review-question-cluster-4.md`) reads the reference as supporting option 1, with doubt. Every form of REF-4, including a member of a class temporary, extends the temporary to the lifetime of the reference, and a temporary bound to a reference member of an extended temporary is extended too. A constant reference must refer to an object with static storage, and no rule depends on the form of the binding, so EDG's rejection of the declarations has no basis. A constant evaluation can read an extended temporary of a `const`, non-volatile literal type, and the type need not be a scalar. So the read of a `const` class temporary is allowed, as Clang and EDG find. When both temporaries in the nested case have a `const` type, the read is allowed, as Clang finds. The front end's rejection of that read with OBJ-2 does not fit the provisional answer. The doubt is that the reference does not say plainly that an extended temporary has static storage.
