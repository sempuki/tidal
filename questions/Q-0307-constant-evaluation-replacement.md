# Q-0307: Must constant evaluation reject a use of an old pointer after a replacement that STG-9 does not allow?

**Status:** provisional
**Affects:** STG-19, STG-20, `tests/foundations/storage/replace-base-old-pointer-constant.cpp`, `tests/foundations/storage/reuse-const-automatic-object-constant.cpp`

## Context

STG-19 makes a use of an old pointer undefined when the new object does
not replace the old one under STG-9. STG-20 makes it undefined to create
an object in the storage of a `const` complete object that is not
dynamic. Constant evaluation must reject undefined behavior. Each probe
below uses a program-declared `std::construct_at`, because C++23
constant evaluation allows placement `new` only there.

| Probe | GCC 14 | GCC 16 | Clang 18 | Clang 22 | EDG 7.0 |
|---|---|---|---|---|---|
| create a `B` in a base subobject of type `B`, read through the old `B*` | accepts, new value | accepts, new value | accepts, new value | accepts, new value | accepts, new value |
| create an `int` in a `const int` variable, read the variable | accepts, new value | accepts, old value | accepts, new value | rejects: modification of a `const` object | accepts, old value |

## Evidence

The constant tests `replace-base-old-pointer-constant.cpp` and
`reuse-const-automatic-object-constant.cpp` record the table.

At run time, GCC at `-O2` calls the old virtual function through a
pointer to a replaced base subobject (`replace-base-old-pointer.cpp`),
and the `vptr` sanitizer reports a call through the enclosing object
after the replacement (`replace-base-ends-enclosing-sanitizer.cpp`).
Every compiler reads the old value of a replaced `const` automatic
variable (`reuse-const-automatic-object.cpp`), and a replaced `const`
static variable faults (`reuse-const-static-object-sanitizer.cpp`).

## Options

1. The uses are undefined (STG-19, STG-20). The constant evaluators of
   the matrix miss them, which is a defect in each.
2. Replacement of a base subobject reaches the old pointer, as the
   constant evaluators behave. STG-19 then has an exception that the
   run-time results at `-O2` contradict.

## Provisional answer

Yes, undefined (STG-19, STG-20); every constant evaluator accepts the base case, and only Clang 22 rejects the `const` case (option 1). The run-time results show that the implementations keep
facts about the old object, and the two `const` results differ between
compilers. The two constant tests expect a rejection and record each
acceptance with an override.

The question stays open under decision 0051 (2026-11-03). Clang 22, GCC 16, and EDG 7.0 accept the read through the old base pointer and give the new value. In the `const` case Clang 22 rejects, and GCC 16 and EDG 7.0 accept and give the old value. So no family rejects the base case, and two families disagree on the `const` case. CE-P3 supports the provisional answer, because constant evaluation rejects any operation that is undefined (CE-9). The front end's interpreter rejects both cases, with STG-19 and with STG-20.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
