# 0025. The exception handling chapter

Date: 2026-10-31
Status: Accepted. Amends 0023 and the charter.

## Context

Decision 0023 deferred exception handling to a chapter of its own.
Every topic in the charter's scope now has a page, and several pages
refer to exceptions as *(referenced)*: `throw` as an operand of the
conditional operator, exception specifications, destructors that
throw, and what happens when an exception leaves `main` or a
`noexcept` function.

## Decision

The chapter `Exceptions` follows `Classes`. It has three pages, in this
order:

1. `throw` expressions and the exception object: the operand, how the
   exception object is made from it, its type and lifetime, rethrow,
   `throw` in constant evaluation.
2. Try blocks and handlers: the try block as a scope, how a handler is
   selected, by value and by reference, the catch-all handler, nested
   try blocks and rethrow, function try blocks, handlers in constructors
   and destructors.
3. Unwinding and termination: the order in which objects end when an
   exception leaves a block, partly constructed objects and the storage
   that `new` obtained, an exception that leaves a `noexcept` function,
   a destructor, or `main`, and an exception with no handler.

Each page goes through the full pipeline in the principles-first
format. Tests that end in termination are `run-fail` tests; the harness
cannot see the reason for the termination, and the Evidence segment
says so.

## Consequences

- The charter lists exception handling as in scope.
- The references that other pages mark *(referenced)* for exceptions
  resolve to these pages once they are drafted.
- `noexcept` as part of a function type stays on the exception
  specifications page. What happens when an exception leaves such a
  function is on the unwinding page.
