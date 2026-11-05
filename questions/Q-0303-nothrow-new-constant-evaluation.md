# Q-0303: Can constant evaluation use a `new` expression with the nothrow tag?

**Status:** provisional
**Affects:** NEW-37, `tests/expressions/new-delete/constant-evaluation-nothrow.cpp`

## Context

Constant evaluation can use a `new` expression that calls a global
allocation function of the implementation. The nothrow forms are global
allocation functions that the library defines. A program that includes
no header declares them itself (LSUP-6). Clang 18 and 22 and GCC 16
evaluate `new (std::nothrow) int(4)` during constant evaluation. GCC 14
says the call is to a function that is not `constexpr`. EDG treats the
expression as a placement `new` and rejects it.

## Evidence

`tests/expressions/new-delete/constant-evaluation-nothrow.cpp`.

## Options

1. Constant evaluation accepts the nothrow forms. GCC 14 and EDG
   differ.
2. Constant evaluation rejects every `new` expression with placement
   arguments, the nothrow tag included. Clang and GCC 16 differ.

## Provisional answer

Yes (NEW-37); Clang and GCC 16 accept, GCC 14 and EDG reject (option 1). The newest Clang and GCC agree, and the nothrow forms are
global allocation functions of the implementation, as the forms
without placement arguments are.

The question stays open under decision 0051 (2026-11-03). Clang 22 and GCC 16 evaluate `new (std::nothrow) int(4)`, and EDG 7.0 rejects it, so two families disagree. GCC 14 still rejects it. CE-P1 supports the provisional answer, because constant evaluation can allocate, and the nothrow forms are allocation functions of the implementation (NEW-37). The front end's interpreter rejects the test, because the program declares the nothrow allocation function itself.

Kept on 2026-11-03. The source-reader gap review of question cluster 4 agrees with option 1.

Kept open on 2026-11-03 (replaced by the note of 2026-11-04) under decision 0054, because no principle on the page gives the answer by itself. NEW-P4 keeps placement forms out of constant evaluation and does not separate the nothrow forms.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). NEW-P4 failed step 3, because it kept every placement form out of constant evaluation and did not separate the nothrow forms. NEW-P4 now says that the nothrow forms are global allocation functions of the implementation and that an evaluation can use them, and it limits the excluded placement forms to those that the program defines. The existing test checks the case. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses NEW-P4.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 4 (`reports/review/2026-11-03-gap-review-question-cluster-4.md`) reads the reference as supporting option 1. A constant evaluation can use an allocating `new` expression that selects a replaceable global allocation function, if the storage is freed within the evaluation. The nothrow forms are among the replaceable global allocation functions, and placement arguments alone do not exclude a `new` expression. Whether a program can declare these functions itself without the header is a matter for LSUP-6. That point can explain the front end's rejection of the test, and it does not bear on the option.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
