# Q-0339: Does a part with a trivial and a non-trivial copy constructor make the enclosing copy constructor non-trivial when the copy selects the trivial one?

**Status:** provisional
**Affects:** CPY-27, CPY-19, UNI-3, `tests/classes/copy-move/trivially-copyable-member-selected.cpp`, `tests/classes/copy-move/trivially-copyable-member-template-selected.cpp`, `tests/classes/unions/union-copy-two-copy-constructors.cpp`, `tests/classes/copy-move/grids/trivially-copyable.json`
**Waits for:** newest families that agree

## Context

`Twin` has two copy constructors: `Twin(const Twin&) = default`, which
is trivial, and `Twin(Twin&) {}`, which is user-provided. A class
`Holder { Twin part; }` has an implicit copy constructor that takes
`const Holder&`, so it copies `part` from a `const` source with the
trivial constructor. A union that holds a `Twin` copies the member from
a `const` source too.

| Program | Clang 18, 22 | GCC 14, 16 | EDG 7.0 |
|---|---|---|---|
| copy a union that holds `Holder` | accepted | rejected: deleted | rejected: deleted |
| copy a union that holds `Twin` | accepted | rejected: deleted | rejected: deleted |
| builtin query: is `Holder` trivially copyable | yes | no | no |

A second case selects a constructor template. `Part` has a trivial
implicit copy constructor and `template <class T> Part(T&) {}`. A class
`Pair` holds a `Part` and a member that copies only from a non-`const`
source, so the implicit copy constructor of `Pair` copies `part` from a
non-`const` `Part`, and the template is selected.

| Program | Clang 18, 22 | GCC 14, 16 | EDG 7.0 |
|---|---|---|---|
| copy a union that holds `Pair` | rejected: deleted | rejected: deleted | accepted |
| builtin query: is `Pair` trivially copyable | no | yes | yes |

GCC's union and GCC's query disagree in this case.

Every compiler says `Twin` itself is not trivially copyable. `Twin` has
an eligible copy constructor that is not trivial (CPY-19).

The class-properties inventory
(`reports/review/2026-11-03-inventory-class-properties.md`, items
TCC-member-selected and TRIV-copy-conditions) describes the
triviality of a copy function as decided by the function that the copy
selects for each part. So the copy constructor of `Holder` is trivial,
and `Holder` is trivially copyable.

## Evidence

The three tests in **Affects**, with overrides for the compilers that
differ from option 1. The grid rows "two copy constructors", "member
with two copy constructors", and "member template wins".

## Options

1. The selected function decides. The copy constructor of `Holder` is
   trivial, and a union that holds `Twin` or `Holder` can be copied.
   The copy constructor of `Pair` is not trivial (Clang).
2. The copy constructors of the part decide, and the selected function
   does not. `Holder` is not trivially copyable, and `Pair` is (EDG, and
   GCC's query).

## Provisional answer

The selected function decides (option 1). The inventory describes this
rule, CPY-27 already says "copied or moved by a non-trivial one", and
the byte copy of `Holder` does what its copy constructor does. Clang
follows option 1 in every case, EDG follows option 2 in every case, and
GCC splits, so the question stays provisional.

Kept open on 2026-11-03 under decision 0054, because two of the newest families differ from the answer in at least one form, and the rule records one differing family.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of item O (`reports/review/2026-11-03-gap-review-item-o.md`) reads the reference as supporting option 1, the provisional answer. The function selected to copy each base and each member decides, as Clang finds.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
