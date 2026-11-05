# Q-0394: Does a member function with no ref-qualifier override a base virtual function with one?

**Status:** provisional
**Affects:** VIRT-1, `tests/classes/virtual/override-ref-qualifier-missing.cpp`
**Source:** the VIRT-1 note of `reports/review/2026-11-04-gap-review-phase-8-classes-overloading.md`

## Context

A base class declares `virtual int f() &;`. A derived class declares
`int f();`, with the same name, parameters, and cv-qualifiers, and no
ref-qualifier. VIRT-1 asks for the same ref-qualifier, so the derived
function does not override.

| Compiler | `int f() override;` in the derived class | A call through `B&` |
|---|---|---|
| Clang 22 | rejects: a function without a ref-qualifier cannot overload one with a ref-qualifier | rejects the class |
| GCC 16 | rejects: `override` overrides nothing | calls `B::f` |
| EDG 7.0 | rejects: `override` overrides nothing, and warns that `D::f` hides `B::f` | runs no program |
| Clang 18 | accepts | calls `D::f` |

## Evidence

The test in **Affects**. It expects a rejection and records Clang 18's
acceptance.

## Options

1. The derived function does not override, because the ref-qualifiers
   differ (Clang 22, GCC 16, EDG 7.0).
2. The derived function overrides, because the test of when two member
   functions correspond sets references aside when one of them has no
   ref-qualifier (Clang 18, and the gap review's reading of the C++23
   text).

## Provisional answer

No; the ref-qualifiers must match (option 1). Every newest compiler
gives this answer. The gap review of the text phase 8 wrote reads the
C++23 text the other way and believes a later change restores the
condition. Tidal keeps the newest compilers' answer, and
`documentation/decisions/pending.md` row 4 lists the case.

Kept open on 2026-11-04 under decision 0054, because the gap review
reads against the answer, and the case waits on the maintainer
(`documentation/decisions/pending.md`, row 4).

## Opinions

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on classes and overloading (`reports/review/2026-11-04-gap-review-phase-8-classes-overloading.md`) reads option 2 in the C++23 text. A member function overrides a base virtual function when the two declarations correspond, trailing constraints aside. Correspondence compares the object parameters, and when one function has neither an explicit object parameter nor a ref-qualifier and the other has a ref-qualifier, it compares the object types with references dropped. The review believes that a later core issue on overriding and explicit object parameters restores the need for the same ref-qualifier, and could not confirm the resolution or its date. If the later text applies, VIRT-1 matches it.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
