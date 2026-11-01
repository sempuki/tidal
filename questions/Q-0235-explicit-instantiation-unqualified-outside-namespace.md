# Q-0235: Can an explicit instantiation outside the template's namespace use an unqualified name?

**Status:** provisional
**Affects:** INST-21, `tests/templates/instantiation/explicit-unqualified-outside-namespace.cpp`

## Context

`namespace lib { template <class T> struct Box {}; }`, then
`using namespace lib;` and `template struct Box<int>;` in the global
namespace.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | rejects: must occur in namespace `lib` |
| GCC 14, 16, host | rejects under `-pedantic-errors`: must use a nested-name-specifier |
| EDG 7.0 | accepts |

## Evidence

The test above and `explicit-enclosing-namespace.cpp`, which every
compiler accepts with a qualified name.

## Options

1. The name must be qualified (Clang, GCC).
2. An unqualified name that lookup finds is enough (EDG).

## Provisional answer

Option 1. Two of three implementations agree.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
