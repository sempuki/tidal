# Q-0428: Does a call of a virtual member function instantiate the function when no object of the class exists?

**Status:** settled
**Affects:** INST-7, INST-11, `tests/templates/instantiation/virtual-called-through-pointer.cpp`
**Source:** G157 (`reports/clean-room/rust-frontend/report-23.md`) with the writer probe `reports/clean-room/roadmap-10/writer-probes/g157-virtual-instantiation-events.json`.

## Context

A class template has a virtual member function whose body is not
valid for `int`. A function that is never called takes a pointer to
the specialization for `int` and calls the virtual function through the
pointer. The program defines no object of the class. INST-7 says that
a call instantiates a member function. INST-11 covers only a virtual
function that no use names.

| Implementation | The call through a pointer |
|---|---|
| Clang 18 and 22 | rejects: the body is instantiated |
| GCC 14 and 16 | rejects: the body is instantiated |
| EDG 7.0 | accepts: the body is not instantiated |

The same probe shows that every compiler instantiates the function for
an object, a `new` expression, or a temporary of the class, and that
none does for a call of a nonvirtual member through the pointer.

## Evidence

The test in **Affects**. It expects a reject and records EDG as an
override.

## Options

1. The call names the function, so the call instantiates the function,
   as for any member function (INST-7; GCC, Clang).
2. The implementation makes a virtual function only when the program
   makes an object of the class, and a call through a pointer adds
   nothing (EDG).

## Answer

Yes; the call instantiates the function (option 1).
Two of three newest families agree, and EDG alone differs.

Settled on 2026-11-10 under decision 0054 (item RV6). The gap review of 2026-11-10 supports the answer with no doubt, and INST-P1 gives the same answer. The INST-P1 table says that a call instantiates a function, and a call through a pointer is a call. INST-P1 names INST-11 in `:exceptions:`, but INST-11 covers only a virtual function that no use names, and this call names the function. So the model rules out option 2. EDG 7.0 differs, and the test keeps its override.

## Opinions

### Gap review, 2026-11-10 (phase 13 text, expressions, overloading, and templates)

The source-reader gap review of the text phase 13 wrote on expressions, overloading, and templates (`reports/review/2026-11-10-gap-review-phase-13-expressions-overloading-templates.md`) reads the question: supports with no doubt (RV6-B-10). A call through the pointer names the virtual function and uses it, and that use needs the definition, so the member is instantiated. The unspecified case covers only a virtual function that no other use needs. A call inside a function that the program never calls is still such a use. EDG's result is outside the unspecified case.
