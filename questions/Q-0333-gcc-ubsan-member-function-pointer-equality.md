# Q-0333: Does GCC rejecting member function pointer equality in constant evaluation under `-fsanitize=undefined` change MPTR-5?

**Status:** compiler defect
**Affects:** MPTR-5, CE-1, `tests/expressions/member-pointers/compare-equal-constant-ubsan.cpp`

## Context

MPTR-5 lets a program compare two pointers to members with `==` and
`!=`, and CE-1 lets constant evaluation do so. Every compiler in the
matrix evaluates such a comparison in a `static_assert` without a
sanitizer.

With `-fsanitize=undefined`, GCC 14 and 16 report that the comparison
of two pointers to member functions "is not a constant expression".
The same configuration compares two pointers to data members as a
constant. Clang evaluates both comparisons in every configuration. A
clean-room reader found the case in Write report W9.

## Evidence

- `tests/expressions/member-pointers/compare-equal-constant-ubsan.cpp`:
  GCC rejects; Clang accepts. EDG takes no sanitizer flag and accepts.
- `tests/expressions/member-pointers/compare-data-equal-constant-ubsan.cpp`:
  every compiler accepts.
- `tests/expressions/member-pointers/compare-equal-constant.cpp`: every
  compiler accepts without a sanitizer.

## Options

1. Keep MPTR-5 and CE-1, and treat the rejection as a GCC defect. A
   sanitizer adds checks at run time and does not change which
   expressions are constant.
2. Make the comparison of member function pointers a constant only
   outside some configurations.

## Provisional answer

No; GCC defect (option 1). The same GCC accepts the comparison without
the sanitizer, and accepts the comparison of data member pointers with
it.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
