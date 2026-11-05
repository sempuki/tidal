# Q-0343: Is a class trivially copyable when its destructor is deleted, or when all its copy and move functions are deleted?

**Status:** provisional
**Affects:** CPY-19, `tests/classes/copy-move/trivially-copyable-at-least-one.cpp`, `tests/classes/copy-move/trivially-copyable-deleted-destructor.cpp`, `tests/classes/copy-move/grids/trivially-copyable.json`
**Waits for:** the maintainer (pending.md row 6)

## Context

Two classes have only trivial functions apart from the deleted ones:

- `Pinned` has trivial copy and move functions and a deleted
  destructor.
- `None` has a trivial default constructor and destructor, and deletes
  all four copy and move functions.

| Class | Clang 18, 22 query | GCC 14, 16 query | EDG 7.0 query | CPY-19 |
|---|---|---|---|---|
| `Pinned` | trivially copyable | trivially copyable | trivially copyable | not |
| `None` | trivially copyable | trivially copyable | trivially copyable | not |

The class-properties inventory
(`reports/review/2026-11-03-inventory-class-properties.md`, items
TCC-all-deleted, TCC-deleted-destructor, and TCC-at-least-one)
describes both classes as not trivially copyable. A class needs at
least one eligible copy or move function, and a destructor that is
trivial and not deleted.

No core-language check shows the difference. A union that holds
either class behaves the same way under both answers, because UNI-3
and CPY-7 decide the union's functions from the class's functions,
not from the property.

## Evidence

The grid `tests/classes/copy-move/grids/trivially-copyable.json`, rows
"deleted destructor" and "four copy and move deleted", column "builtin
trivially copyable". The builtin queries are Evidence only (decision
0051). The two tests in **Affects** show the union's behavior.

## Options

1. Not trivially copyable. A class that forbids every copy, or forbids
   its own end, does not promise that a byte copy makes a valid object.
2. Trivially copyable, as the builtin queries of all three compilers
   say.

## Provisional answer

No; neither class is trivially copyable (option 1). The inventory
describes this limit, and CPY-P6 says that a class that forbids copies
does not get the byte-copy promise. The compilers' queries disagree,
and no test can show the effect, so the question stays provisional.

Kept open on 2026-11-03 under decision 0054, because the question waits on the maintainer (`documentation/decisions/pending.md`).

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of item O (`reports/review/2026-11-03-gap-review-item-o.md`) reads the reference as supporting option 1, the provisional answer. A trivially copyable class needs at least one eligible copy or move function and a destructor that is trivial and not deleted, and a deleted function is never eligible. The review describes the builtin queries of the three compiler families as following an older definition.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
