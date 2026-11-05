# Q-0045: Must an anonymous union in an unnamed namespace be declared `static`?

**Status:** provisional
**Affects:** STO-11, UNI-9, `tests/declarations/storage-linkage/anonymous-union-unnamed-namespace.cpp`, `tests/classes/unions/anonymous-union-unnamed-namespace.cpp`
**Waits for:** newest families that agree

## Context

A namespace-scope anonymous union must be declared `static`. In an
unnamed namespace, the members already have internal linkage. Clang 22
and 18 accept an anonymous union there without `static`. GCC 16 and 14
and EDG 7.0 reject the union. The gap review summary says the unnamed
namespace form is valid.

STO-11 states the rule. UNI-9 lists the other limits on an anonymous
union and refers to STO-11 for this one. Q-0057 asked the same question
for UNI-9 and is a duplicate of this question.

## Evidence

The two tests in **Affects**. Each has an override for Clang.

## Options

1. `static` is necessary at every namespace scope, including an unnamed
   namespace, as GCC and EDG require.
2. An unnamed namespace is enough, as Clang does and the review summary
   says.

## Provisional answer

No; an anonymous union in an unnamed namespace needs no `static` (option 2). Clang accepts the union. GCC and EDG reject it.

Changed on 2026-11-03 to option 2. The review reads option 2, Clang 22 accepts, and STO-P2 with STO-P3 gives the same answer: an anonymous union must be `static` because it has no name to share, and a name in an unnamed namespace is shared with nothing already. STO-11 and UNI-9 now exempt an unnamed namespace, and the two tests expect acceptance, with overrides for GCC and EDG. The question stays open under decision 0054, because two newest families differ from the answer.

Kept open on 2026-11-04 under decision 0054, because two newest families, GCC 16 and EDG 7.0, differ from the answer.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) reads option 2. `static` is required only for an anonymous union in a namespace with external linkage. An unnamed namespace, and any namespace inside one, has internal linkage, so an anonymous union there needs no `static`. Clang follows this reading. GCC's and EDG's rejection is a defect.
