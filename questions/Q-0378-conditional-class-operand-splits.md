# Q-0378: Which class operands of `?:` convert when the compilers split?

**Status:** provisional
**Affects:** COND-16, COND-3, COND-5, `tests/expressions/conditional/class-deleted-conversion-counts.cpp`, `tests/expressions/conditional/class-const-and-volatile-lvalues.cpp`
**Source:** reports/review/2026-11-04-gap-review-phase-7-additions.md and reports/review/2026-11-04-gap-review-phase-7-repairs.md, the COND-16 sections

## Context

COND-16 tries each operand of `?:` against the other when the types
differ and one is a class. The two gap reviews of the text phase 7
wrote read the reference as asking only whether a conversion sequence
can be formed, with access and deleted functions left out of that
test. For two operands of one class type, the conversion goes only
toward the operand whose type has every cv-qualifier of the other.

Probes on the newest compilers split in two forms:

```cpp
struct Tag;
struct Wrap { Wrap(); Wrap(const Tag&); operator Tag() const = delete; };
struct Tag { };
Wrap w; Tag t;
auto r = c ? w : t;            // form 1

struct Box { };
const Box fixed{}; volatile Box shared{};
void f() { (void)(c ? fixed : shared); }   // form 2
```

| Form | Clang 22 | GCC 16 | EDG 7.0 |
|---|---|---|---|
| 1: `Tag` converts to `Wrap`, and the conversion of `Wrap` to `Tag` is deleted | accepts | rejects | rejects |
| 2: a `const` and a `volatile` lvalue of one class type | rejects | rejects | accepts |

Every newest compiler counts a private conversion function in the same
place as the deleted one in form 1, and rejects the expression as
ambiguous.

## Evidence

The two tests in **Affects**. Each expects a rejection and records the
compiler that accepts as an override.

## Options

1. Form 1: the deleted conversion counts, and the expression is
   ambiguous (GCC, EDG, and the reviews' reading). Form 2: neither
   operand converts, and the expression is not valid (Clang, GCC).
2. Form 1: a deleted conversion does not count, so only `Tag` converts
   and the result is a `Wrap` (Clang). Form 2: the result is a prvalue,
   as for two such lvalues of a non-class type (EDG).

## Provisional answer

The deleted conversion counts, and a `const` and a `volatile` class
lvalue do not convert (option 1). In each form two of the three newest
families agree. Form 1 treats a deleted function as access is treated,
which every compiler leaves out of the test. Form 2 follows the cv
condition of the COND-16 table.

Kept open on 2026-11-04 under decision 0054, because no review has read the provisional answer, and in each form one newest family differs from it. Item RV2 of decision 0056 reads the question.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
