# Q-0010: Is `decltype(auto) x{e};` valid, deducing from `e`?

**Status:** provisional
**Affects:** AUTO-11, `tests/declarations/auto/decltype-auto-braced.cpp`

## Context

`decltype(auto)` is rejected with `= { ... }` (AUTO-11), and `auto` with a
single-element braced initializer deduces from the element (AUTO-6). For
`decltype(auto)` with a single-element braced initializer and no `=`, the
writer predicted rejection. All seven compilers accept it and deduce from
the element, including its value category: `decltype(auto) b{(i)};` gives
`int&`.

## Evidence

`tests/RESULTS.md`, rows `decltype-auto-braced.cpp` and
`decltype-auto-braced-multiple.cpp`. Clang 22.1.8 and 18.1.3, GCC 16.2.1,
16.2.0, and 14.2.0, and EDG 7.0 all accept the single-element form and
reject the multiple-element form.

## Options

1. Valid, deducing from the single element as if the braces were absent.
   Matches every implementation.
2. Rejected. Programs using it today would break on a conforming
   implementation.

## Provisional answer

Option 1. Tidal describes what implementations do, and they agree.

## Opinions
