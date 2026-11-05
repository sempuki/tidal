# Q-0364: Does a block declaration of an operator function turn off argument-dependent lookup for an operator?

**Status:** provisional
**Affects:** NAM-38, `tests/foundations/names/operator-lookup-block-declaration-keeps-adl.cpp`
**Waits for:** a review (doubt)

## Context

For a function call, a function declared in a block turns off
argument-dependent lookup (NAM-32). NAM-38 gives the lookup of operator
functions for an operator expression, and says that argument-dependent
lookup always adds its functions.

```cpp
namespace n { struct X {}; int operator+(X, X); }
struct Y {};
int g() { int operator+(Y, Y); n::X a; return a + a; }
```

The block declaration does not fit the operands. `n::operator+` fits
and is reachable only by argument-dependent lookup.

| Probe | Clang 22 | GCC 16 | EDG 7.0 |
|---|---|---|---|
| `a + a` above | accepts | accepts | rejects |

The source-reader gap review of names and lookup
(`reports/review/2026-11-03-gap-review-names.md`) says that the
reference builds the operator candidates by the lookup for a call, so a
strict reading turns argument-dependent lookup off. The review states
the reading with doubt and expects the compilers to keep
argument-dependent lookup.

## Evidence

The test in **Affects** expects acceptance and records EDG's result
with an override.

## Options

1. Argument-dependent lookup always adds its functions for an operator
   (Clang, GCC).
2. A block declaration turns argument-dependent lookup off for an
   operator, as for a call (EDG).

## Provisional answer

Argument-dependent lookup always adds its functions (option 1). NAM-P9 gives operators their own
row: members and non-members are candidates together, and
argument-dependent lookup is part of the non-member search. An operator
has no form that turns off argument-dependent lookup on purpose, as
parentheses do for a call.

Kept open on 2026-11-04 under decision 0054, because the gap review of the rules that phase 7 added finds the reference unclear, and its literal text supports option 2. EDG 7.0 is the one newest family that differs.

## Opinions

### Gap review, 2026-11-04

The source-reader gap review of the rules that phase 7 added (`reports/review/2026-11-04-gap-review-phase-7-additions.md`) finds the reference unclear, with a literal reading for option 2. The non-member candidates of an operator come from the unqualified lookup of the operator function name in the call that the expression becomes, under both the ordinary rules and the argument-dependent rules. The argument-dependent rules turn off when ordinary lookup finds a function declared in a block, so a literal reading turns them off for the operator too, as EDG does. The reference also says to ignore member functions in that lookup and does not fix the order of that step and the test. The review knows of no resolution on the point. Clang and GCC keep argument-dependent lookup, and the answer is a defensible reading of the intent.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
