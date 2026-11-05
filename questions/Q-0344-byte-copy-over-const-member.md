# Q-0344: Can a byte copy write over a live object that has a `const` member?

**Status:** provisional
**Affects:** CPY-19, CV-5, `tests/classes/copy-move/trivially-copyable-const-member.cpp`
**Waits for:** a review (against)

## Context

`struct Fixed { const int value; };` is trivially copyable (CPY-19).
The `const` member deletes the copy and move assignment operators
(CPY-7), and the constructors stay trivial.

A program defines `Fixed a{1}, b{2};` and copies the bytes of `a` into
`b`, for example with `std::memcpy`. Two rules apply:

- CPY-19 gives `b` the value of `a` after a byte copy into a second
  object of the same type.
- CV-5 makes a change to an object defined `const` undefined. The
  member `b.value` is declared `const`, and the byte copy changes it.

The source-reader gap review of item O
(`reports/review/2026-11-03-gap-review-item-o.md`) says the reference
has the same two rules and does not say which one decides.

No test can show the answer. Constant evaluation cannot copy bytes into
an object, and no sanitizer in the matrix reports a write to a `const`
member. The tests of CPY-19 copy into a byte buffer, or into an object
with no `const` member.

## Evidence

The test in **Affects** shows that the class is trivially copyable.
The question has no test of the byte copy itself.

## Options

1. The byte copy changes a `const` object, and the behavior is
   undefined (CV-5 decides). A program that wants a copy creates a new
   object in the storage instead.
2. The byte copy is allowed and gives `b` the value of `a` (CPY-19
   decides).

## Provisional answer

The copy is undefined (option 1). CPY-19 says that the behavior is undefined when the second
object is `const` or has a `const` subobject. The `const` rule then has
no exception, and an implementation can keep relying on a `const`
member that does not change.

Kept on 2026-11-03. The review reads against the answer in part, with doubt, and no test can show either result. The question stays open under decision 0054, because the review reads with doubt.

Kept open on 2026-11-04 under decision 0054, because the gap review reads against part of the answer with doubt.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) reads the copy through the library's byte copy functions as option 2, with doubt, and the copy byte by byte as option 1. The library's byte copy functions create objects in the destination when that gives the program defined behavior. A new object over the second object ends the old one, and the old object is not a `const` complete object, so the new object replaces it and the old name refers to the new object. No `const` object changes. A copy through a character pointer creates no objects, so the change to the `const` member stays undefined. The doubt is whether objects are created in a destination that already holds a live object of the same type.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
