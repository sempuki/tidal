# 0023. Defer exception handling to its own chapter

Date: 2026-10-30
Status: Accepted. Amends the charter.

## Context

The gap review lists `throw` expressions as a missing expression topic.
A `throw` expression depends on try blocks, handlers, stack unwinding,
function try blocks, and the behavior of `noexcept` when an exception
leaves a function. These topics are statements and run-time rules as
much as expressions. The charter's scope lists none of them.

## Decision

Exception handling is a later chapter of its own. Until then, `throw`,
try blocks, and handlers are referenced and not specified. Pages that
mention them mark them *(referenced)*. Rules that depend on whether an
exception can leave a function, such as the value of `noexcept(e)`, stay
where they are.

## Consequences

- The charter lists exception handling as out of scope for this draft.
- The list of topics in scope with no page no longer includes `throw`.
- The chapter needs its own topic order when work on it starts.
