# Q-0241: Must an implementation diagnose a deduction guide whose template parameter cannot be deduced?

**Status:** provisional
**Affects:** CTAD-13, `tests/templates/class-template-deduction/guide-not-deducible.cpp`

## Context

`template <class T, class U> Box(T) -> Box<U>;` declares a guide in
which no parameter gives `U` a value, and `U` has no default.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | rejects the declaration: a template parameter cannot be deduced |
| GCC 14, 16, host | accepts |
| EDG 7.0 | accepts |

Such a guide is never viable, so the guide changes no deduction.

## Evidence

The test above, and the inventory item H4. Item 38 of the gap review
`reports/review/2026-10-31-gap-review-packs-and-ctad.md` reads the rule
as one that gives no exemption from diagnosis, so an implementation
must reject the guide. Clang's rejection matches that reading.

## Options

1. The guide is not valid, and an implementation need not diagnose it
   (`no-diagnostic`).
2. The guide is not valid and must be rejected (`reject`).
3. The guide is valid and is never chosen.

## Provisional answer

Yes; Clang agrees, GCC and EDG differ (option 2). The inventory and the gap review say that the guide is not
valid, and the gap review finds no exemption from diagnosis. Clang
rejects the guide. GCC and EDG accept it, and the tests record their
results as overrides.

Kept open on 2026-11-03 under decision 0054, because two of the newest families differ from the answer, and the rule records one differing family. GCC 16 and EDG 7.0 accept the guide.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) agrees with option 2. Each template parameter of a guide that has no default must be deducible from the guide's parameter types, and the rule is not exempt from diagnosis. GCC's and EDG's acceptance is a defect.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Gap review, 2026-10-31

The source-reader gap review of packs and class template argument deduction (`reports/review/2026-10-31-gap-review-packs-and-ctad.md`) reads every template parameter of a guide template that has no default as required to be deducible from the guide's parameters, with no exemption from diagnosis. Clang's rejection matches. The reading supports the provisional answer, and the review asks the question to record that GCC and EDG accept.
