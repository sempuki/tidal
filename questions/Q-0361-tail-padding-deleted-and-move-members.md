# Q-0361: Does a base with a deleted constructor or a user-provided move assignment operator lend its tail padding?

**Status:** provisional
**Affects:** CLS-35, CLS-34, `tests/classes/members/layout-tail-padding-deleted-constructor.cpp`, `tests/classes/members/layout-tail-padding-deleted-constructor-ilp32.cpp`, `tests/classes/members/layout-tail-padding-move-assignment.cpp`, `tests/classes/members/layout-tail-padding-move-assignment-ilp32.cpp`

## Context

Each base below has an `int` and a `char` member, so it takes 8 bytes
and ends with 3 bytes of padding. A derived class adds one `char`. A
size of 12 means that the base keeps its end padding, and 8 means that
the derived member goes into it.

| Member that the base declares | GCC 16, Clang 22, Clang 18 | GCC 14 | EDG 7.0 |
|---|---|---|---|
| `B() = delete;` | 12 | 8 | 8 |
| `B(const B&) = delete;`, with a defaulted default constructor | 12 | 8 | 8 |
| `B(B&&) = delete;`, with a defaulted default constructor | 12 | 8 | 8 |
| `B& operator=(B&&) { return *this; }` | 12 | 12 | 8 |

The results are the same on x86-64 and with `-m32`. A
`[[no_unique_address]]` member of the last class type gives the same
sizes as the base. A user-provided copy assignment operator lends the
padding on every compiler. A defaulted move assignment operator and a
deleted destructor keep it on every compiler.

The use of the end padding of a base is unspecified (CLS-34), so each
answer is allowed. The question records which choice CLS-35 lists for
the matrix. Q-0342 records the GCC 14 split for a defaulted
constructor.

## Evidence

The tests in **Affects**, with overrides for GCC 14 and EDG.

## Options

1. The base keeps its padding, as GCC 16 and Clang do.
2. The base lends its padding, as EDG does, and as GCC 14 does for a
   deleted constructor.

## Provisional answer

The base keeps its padding (option 1). The newest versions of GCC and
Clang agree on this layout. CLS-35 lists the EDG and GCC 14 choices as
extra conditions of those compilers.

Kept open on 2026-11-03 under decision 0054, because the review finds the question outside the C++23 text.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) finds the question outside the C++23 text. The language does not fix whether a derived class reuses the end padding of a base. The choice belongs to the implementation and its ABI, so every compiler in the table conforms.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
