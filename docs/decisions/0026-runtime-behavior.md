# 0026. Rename Dynamic Semantics to Runtime Behavior

Date: 2026-10-31
Status: Accepted. Amends 0016 and 0022.

## Context

The segment that holds what a program does when it runs was called
`Dynamic Semantics`, after the Ada and Ferrocene skeletons. Its partner
segment is `Constraints` (decision 0022). The pair read unevenly, and
"dynamic semantics" needs explaining to a reader who does not know the
Ada vocabulary.

## Decision

The segment is `Runtime Behavior`. It holds the same rules: `required`
facts about what happens during execution. All pages, the style guide,
and `tools/check_spec.py` use the new name.

## Consequences

- A page reads Principles, Syntax, Constraints, Runtime Behavior,
  Undefined Behavior, Implementation Requirements, Recommended Practice,
  Examples, Evidence.
- Rule IDs and rule classes do not change.
