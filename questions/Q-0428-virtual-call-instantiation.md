# Q-0428: Does a call of a virtual member function instantiate the function when no object of the class exists?

**Status:** provisional
**Affects:** INST-7, INST-11, `tests/templates/instantiation/virtual-called-through-pointer.cpp`
**Source:** G157 (`reports/clean-room/rust-frontend/report-23.md`) with the writer probe `reports/clean-room/roadmap-10/writer-probes/g157-virtual-instantiation-events.json`.
**Waits for:** a first review

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

## Provisional answer

Yes; the call instantiates the function (option 1).
Two of three newest families agree, and EDG alone differs.

## Opinions
