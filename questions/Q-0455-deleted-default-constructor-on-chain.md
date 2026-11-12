# Q-0455: Does an assignment through a chain of member accesses make a union member active when the member's class has a deleted default constructor?

**Status:** provisional
**Affects:** UNI-13, OBJ-2, `tests/classes/unions/chain-assign-deleted-default-constructor-constant.cpp`
**Source:** RV11-R-5 (`reports/review/2026-11-11-read-of-rv10-repair-diff.md`), with the round 15 writer grid `reports/clean-room/roadmap-15/writer-probes/rv11-rows.json`.
**Waits for:** the maintainer (pending.md row 4)

## Context

A union has an `int` member and a member of class type. The class
declares its default constructor as deleted at its first declaration,
so the default constructor is trivial (CTOR-23). The class has another
constructor that takes an `int`. A `constexpr` function makes the
`int` member active, then assigns to a scalar member of the class
member through the chain of member accesses, then reads it back:

```cpp
struct Gauge { Gauge() = delete; constexpr Gauge(int v) : level(v) {} int level; };
union Panel { int raw; Gauge reading; };
constexpr int refill() { Panel p{.raw = 1}; p.reading.level = 4; return p.reading.level; }
static_assert(refill() == 4);
```

| Form | Clang 18, 22, host | GCC 14, 16, host | EDG 7.0 |
|---|---|---|---|
| default constructor deleted | accepts | rejects, "non-constant condition for static assertion" | accepts |
| default constructor defaulted | accepts | accepts | accepts |

## Evidence

The test in **Affects** and the grid in **Source**. The read of the
RV10 repair diff reads the C++23 text as putting a class member on the
chain only when its default constructor is trivial and not deleted, so
the assignment would make no member active and the write would fall
outside a lifetime (OBJ-2).

## Options

1. The assignment makes the member active, because its default
   constructor is trivial, deleted or not (Clang, EDG).
2. A deleted default constructor keeps the member off the chain, so
   the write is undefined and constant evaluation rejects it (GCC, the
   read of the RV10 repair diff).

## Provisional answer

The member becomes active, and the evaluation is constant (option 1).
UNI-13 asks for a trivial default constructor, and CTOR-23 makes a
default constructor deleted at its first declaration trivial. Clang 22
and EDG 7.0 agree, and GCC 16 alone differs. The read of the RV10
repair diff reads against the answer, so the question waits with the
other items of `pending.md` row 4.

Opened on 2026-11-11 (decision 0066, item F8). UNI-13's first item now
says that the default constructor can be deleted. UNI-P1 lists an
assignment through the chain among the actions that make a member
active, and does not name the default constructor, so no principle
gives the answer.

## Opinions

### Read of the RV10 repair diff, 2026-11-11

The source-reader read of the RV10 repair diff (`reports/review/2026-11-11-read-of-rv10-repair-diff.md`) reads OBJ-2's sentence on the chain and UNI-13 against this form (RV11-R-5). The chain collects a union member of class type only when its default constructor is trivial and usable, so not deleted. A default constructor deleted at its first declaration can be trivial, so OBJ-2 and UNI-13 cover one class that the C++23 text leaves out. The read calls the doubt small.
