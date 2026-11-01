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

| Probe | GCC 16.2 | Clang 22 |
|---|---|---|
| construct a `Point` in a union member, call `~Point()`, read `x` | accepts, gives 0 | rejects, read outside the lifetime |
| the same for a `Point` with a user-provided `constexpr` destructor | accepts | not probed |
| a `Point` variable that is not in a union, call `~Point()`, read `x` | rejects, outside the lifetime | rejects, outside the lifetime |

EDG has not run the probes yet.

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

Option 1. UNI-14, DTOR-6, and OBJ-2 together make the read undefined,
and GCC rejects the same read outside a union. The test expects a
rejection and records GCC's acceptance with an override.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
