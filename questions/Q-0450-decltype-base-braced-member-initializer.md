# Q-0450: Can a member initializer that names its base with `decltype` take a braced list?

**Status:** settled
**Affects:** DTY-3, `tests/expressions/decltype/member-initializer-base-braces.cpp`
**Source:** phase 17, item GR (decision 0065), a probe of the `mem_init_id` production.

## Context

A constructor initializes its base in the member initializer list and
names the base with `decltype` of an object of the base type:

```cpp
struct Base { int v; Base(int x) : v(x) {} };
Base sample(0);
struct Derived : decltype(sample) {
  Derived() : decltype(sample){2} {}
};
```

| Form | Clang 18, 22, host | GCC 14, 16, host | EDG 7.0 |
|---|---|---|---|
| `decltype(sample)(2)` in the member initializer list | accepts | accepts | accepts |
| `decltype(sample){2}` in the member initializer list | accepts | rejects: expects an unqualified-id before `{` | accepts |

DTY-3 lets `decltype` stand where a type name can stand, and lists the
name of a base in a member initializer list. The `mem_init` production
takes parentheses or a braced list after any `mem_init_id`, and a
`mem_init_id` can be a `decltype_spec` (`classes/constructors.md`,
`classes/derived.md`).

## Evidence

The test in **Affects**, and `tests/expressions/decltype/member-initializer-base.cpp`
for the form with parentheses, which every compiler accepts.

## Options

1. The braced form is valid. A member initializer that names a base
   with `decltype` takes parentheses or a braced list, as a member
   initializer that names the base by its class name does.
2. Only parentheses can follow a `decltype` base name in a member
   initializer list.

## Answer

The braced form is valid (option 1). Clang and EDG accept it, the form
with parentheses is valid on every compiler, and DTY-3 lets the result
of `decltype` stand wherever a type name can stand (DTY-P2). The
grammar gives no separate form for a `decltype` base name. GCC's
rejection resembles its rejection of `decltype` after `~` in
{question}`Q-0089`.

Settled on 2026-11-11 under decision 0054 (decision 0065, item RV10). The gap review of the phase 17 text supports the answer with no doubt (RV10-1), and GCC 16 alone differs. DTY-P2 gives the answer: the result of `decltype` is a type, so it can stand wherever a type name can stand. The name of a base in a member initializer takes parentheses or a braced list, so the `decltype` form takes either one, and the model rules out option 2. DTY-P2 did not change in phase 17, so decision 0055 does not hold it back. Tidal records GCC 16's result as a deviation, and the test keeps its override for GCC.

## Opinions

### Gap review, 2026-11-11 (phase 17 text)

The source-reader gap review of the phase 17 text (`reports/review/2026-11-11-gap-review-phase-17.md`) gives the first reading of the question: supports with no doubt (option 1, RV10-1). The name of a base in a member initializer has one shared form in the reference, and the `decltype` form of an expression is one of its arms. Any arm of that form can be followed by parentheses or by a braced list. So DTY-3's item and `member-initializer-base-braces.cpp` agree, and GCC's rejection of the braced form reads against.
