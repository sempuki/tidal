# Q-0297: Does a `const` variable template have one object for each specialization across translation units?

**Status:** provisional
**Affects:** ODR-12, LNK-3, `tests/foundations/odr/const-variable-template-one-object.cpp`

## Context

A header defines a `const` variable template and a `constexpr` variable
template at namespace scope, with no `inline` and no `extern`. Two
translation units include the header and take the address of the same
specialization of each template. A `const` variable that is not a
template has internal linkage, so each unit has its own object
(LNK-3).

| Compiler | Result |
|---|---|
| Clang 18, 22, host 22 | one object for each specialization, at `-O0` and `-O2` |
| GCC 16, host 16 | one object for each specialization, at `-O0` and `-O2` |
| GCC 14 | a separate object in each unit, for both templates |
| EDG 7.0 | accepts each unit; EDG does not link, so the test shows nothing |

## Evidence

The test above. The non-`const` variable template in
`header-variable-template-shared.cpp` has one object for each
specialization on every compiler that links.

## Options

1. A specialization of a `const` variable template is one object in the
   program, as for any other template (Clang, GCC 16).
2. Each unit has its own object, as for a `const` variable that is not
   a template (GCC 14).

## Provisional answer

Option 1. Every current compiler that links agrees, and the newer GCC
changed from option 2 to option 1.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
