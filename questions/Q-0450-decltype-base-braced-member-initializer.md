# Q-0450: Can a member initializer that names its base with `decltype` take a braced list?

**Status:** provisional
**Affects:** DTY-3, `tests/expressions/decltype/member-initializer-base-braces.cpp`
**Source:** phase 17, item GR (decision 0065), a probe of the `mem_init_id` production.
**Waits for:** a first review

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

## Provisional answer

The braced form is valid (option 1). Clang and EDG accept it, the form
with parentheses is valid on every compiler, and DTY-3 lets the result
of `decltype` stand wherever a type name can stand (DTY-P2). The
grammar gives no separate form for a `decltype` base name. GCC's
rejection resembles its rejection of `decltype` after `~` in
{question}`Q-0089`.

## Opinions
