# Q-0165: Does one braced function name bind a reference to a function as `=` does?

**Status:** provisional
**Affects:** REF-6, `tests/declarations/initialization/references/braced-function-overload-set.cpp`, `tests/declarations/initialization/references/braced-function-noexcept.cpp`

## Context

A reference to a function type cannot bind to a temporary. REF-6 binds
such a reference to one braced expression as `=` does. Two cases split
the compilers.

| Declaration | Clang 18, 22, host | GCC 14, 16, host | EDG 7.0 |
|---|---|---|---|
| `void (&call)(int){show};`, with `show(int)` and `show(long)` | binds to `show(int)` | rejects | binds to `show(int)` |
| `void (&call)(){quiet};`, with `void quiet() noexcept` | binds to `quiet` | binds to `quiet` | rejects |
| `void (&call)(int){plain};`, with one `plain(int)` | binds | binds | binds |

After `=`, every compiler accepts all three declarations.

## Evidence

The two tests in **Affects**, and
`tests/declarations/initialization/references/braced-function-reference.cpp`
for the case where all agree. GCC's message is "invalid initialization
of non-const reference of type void (&)(int) from an rvalue of type
brace-enclosed initializer list". EDG's message says that a value of
type `void (*)() noexcept` cannot be used to initialize the reference.

## Options

1. Bind as `=` does in each case (Clang, and in each case two of three
   implementations).
2. Reject the overload set in braces (GCC).
3. Reject a function whose type differs from the referenced type
   (EDG).

## Provisional answer

Option 1. Clang accepts both cases, each case has two of three
implementations, and braces do not change the binding of a function
reference after `=`.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
