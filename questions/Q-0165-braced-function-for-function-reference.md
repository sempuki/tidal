# Q-0165: Does one braced function name bind a reference to a function as `=` does?

**Status:** provisional
**Affects:** REF-6, REF-11, `tests/declarations/initialization/references/braced-function-overload-set.cpp`, `tests/declarations/initialization/references/braced-function-noexcept.cpp`
**Parent:** Q-0294, the model of braced lists for references, arrays, and conversion sequences
**Waits for:** a use of REF-P3; a review (doubt)

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

For an overload set, yes; GCC rejects it (option 1). For a `noexcept` function and a reference to a function type without `noexcept`, no; Clang and GCC accept it (option 3). A braced function binds directly only when it already has the referenced type or is an overload set that gives a function of that type (REF-P3), and a function type has no temporary.

Changed on 2026-11-03 for the `noexcept` function. The review reads option 3 there, EDG 7.0 rejects, and REF-P3 gives the same result: a braced element binds directly only if it is already what the reference names, and a function type has no temporary. REF-6 now binds a braced function only when its type is the referenced type or when it is an overload set, and REF-11 rejects the `noexcept` function. `braced-function-noexcept.cpp` expects a rejection, with overrides for Clang and GCC. The overload set keeps option 1, which Clang 22 and EDG 7.0 follow, because the review finds that case unclear.

Kept open on 2026-11-03 under decision 0054, because the review finds the overload set unclear. REF-6 and REF-11 also changed on the same day for the `noexcept` function, and a question settles only after a reader or a review has used the changed rule (decision 0055).

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on overloading, references, and list initialization (`reports/review/2026-11-03-gap-review-questions-overloading.md`) reads the `noexcept` function against the answer, with doubt, and finds the overload set unclear. The one braced element binds the reference as `=` does only when the referenced type is related to the element's type, and for function types that means the same type. A `noexcept` function has another type. After `=` the binding allows the function pointer conversion, but with braces a temporary of the function type would follow, which cannot exist. So the braced form is not valid, as EDG finds (option 3). For an overload set, the reference is a target that resolves the set, which supports Clang and EDG, but the list rule picks its branch from the element's type, which an overload set lacks, so GCC's rejection is a possible reading. The review has not checked whether a later change aligns braces with `=`.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
