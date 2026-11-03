# Q-0309: Must constant evaluation reject a read of a union member after an explicit destructor call ends it?

**Status:** compiler defect
**Affects:** UNI-14, UNI-6, OBJ-2, `classes/unions/destroyed-member-read-constant.cpp`

## Context

UNI-14 makes a member of a union active when `std::construct_at` creates
an object of the member's type at the member's address. An explicit
destructor call on that member ends the member's lifetime (DTOR-6), and
the union then has no active member. A later read of the member reads an
object outside its lifetime (OBJ-2) and a member that is not active
(UNI-6). Constant evaluation must reject both (CE-9, CE-6).

| Probe | GCC 14, 16 | Clang 18, 22 | EDG 7.0 |
|---|---|---|---|
| construct a `Point` in a union member, call `~Point()`, read `x` | accepts, gives 0 | rejects, read outside the lifetime | rejects |
| the same for a `Point` with a user-provided `constexpr` destructor | accepts | rejects | rejects |
| a `Point` variable that is not in a union, call `~Point()`, read `x` | rejects, outside the lifetime | rejects, outside the lifetime | rejects |

## Evidence

`tests/classes/unions/destroyed-member-read-constant.cpp`. GCC rejects
the same read for an object that is not a union member
(`tests/classes/constructors/use-after-dtor-constant.cpp`), so GCC
models the end of the lifetime in general and misses the case of a
union member.

## Options

1. The read is undefined, and constant evaluation must reject it. GCC
   misses the case, which is a defect in GCC.
2. An explicit destructor call does not end a union member during
   constant evaluation. No rule gives such an exception, and the result
   of 0 that GCC gives is a value that no write stored.

## Provisional answer

Yes, undefined (UNI-14, OBJ-2); Clang and EDG reject, GCC accepts (option 1). UNI-14, DTOR-6, and OBJ-2 together make the read undefined,
and GCC rejects the same read outside a union. The test expects a
rejection and records GCC's acceptance with an override.

The question stays open under decision 0051 (2026-11-03). Clang 22 and EDG 7.0 reject the read, and GCC 16 accepts it, so two families disagree. CE-P3 supports the provisional answer, because constant evaluation rejects any operation that is undefined (CE-9, OBJ-2, UNI-6). The front end's interpreter rejects the read with UNI-6.

Kept on 2026-11-03. The source-reader gap review of question cluster 4 agrees with option 1.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 4 (`reports/review/2026-11-03-gap-review-question-cluster-4.md`) reads the reference as supporting option 1. A class object stops being alive when its destructor call begins, also for a trivial destructor. After the call the member is not active, and the union has no active member. A read of a member that is not active keeps an expression from being constant, apart from undefined behavior. GCC's acceptance has no support in the reference.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
