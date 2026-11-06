# Q-0337: Can direct initialization of an lvalue reference to `const` use an explicit conversion function that does not return an lvalue?

**Status:** provisional
**Affects:** ICS-45, OVL-15, `tests/overloading/conversion-sequences/user-defined-explicit-reference-value.cpp`, `tests/overloading/conversion-sequences/user-defined-explicit-reference-xvalue.cpp`
**Parent:** Q-0156, which covers rvalue references
**Waits for:** newest families that agree; a review (doubt)

## Context

A class `ToItem` has only `explicit operator Item()`, and a class
`ToItemRvalue` has only `explicit operator Item&&()`. A reference
`const Item&` is direct-initialized from an lvalue of one of these
classes, for example `const Item& r(s);` or
`static_cast<const Item&>(s)`.

| Compiler | from `explicit operator Item()` | from `explicit operator Item&&()` |
|---|---|---|
| Clang 18, Clang 22 | accepted | accepted |
| GCC 14, GCC 16 | rejected | rejected |
| EDG 7.0 | rejected | accepted |

Every compiler accepts `const Item&` from `explicit operator Item&()`,
where the reference binds directly to an lvalue. Every compiler accepts
`Item&&` from `explicit operator Item&&()`.

OVL-15 gives an lvalue reference only the conversion functions that
return an lvalue reference. Without such a candidate, the reference
binds to a temporary, and the temporary is copy-initialized, which
takes no `explicit` function. An lvalue reference to `const` can also
bind to an rvalue (REF-2), but no rule offers it a conversion function
that returns one when the function is `explicit`.

## Evidence

The two tests in **Affects**, and the probes of 2026-11-03 in Q-0156.

## Options

1. Only an explicit conversion function that returns an lvalue
   reference serves an lvalue reference to `const` (GCC, and EDG for a
   value result).
2. An explicit conversion function that returns a value or an rvalue
   reference also serves it, as for an rvalue reference (Clang, and EDG
   for an rvalue reference result).

## Provisional answer

No; only a function that returns an lvalue reference serves it (option 1). GCC and EDG reject the value result, and OVL-15 offers an lvalue reference only lvalue results.

The question was split from Q-0156 on 2026-11-03. The source-reader gap review of question cluster 2 (`reports/review/2026-11-03-gap-review-question-cluster-2.md`) reads the reference as supporting option 1, weakly, and calls the literal reading a possible gap, since an lvalue reference to `const` can bind to an rvalue. For the xvalue result, EDG joins Clang, so the answer there rests on GCC and OVL-15 alone.

Kept open on 2026-11-03 under decision 0054, because the gap review does not support the answer with no doubt (it reads with doubt, weakly, unclearly, or for only part of the question).

Kept open on 2026-11-05 under decision 0054, because the gap review of the text phase 9 wrote reads the answer with a doubt.

Noted on 2026-11-05 under decision 0058, item PK2. A test whose notes name the question has two newest families that differ from the answer, so the question also waits for newest families that agree (`pending.md` row 1). The check of `tools/question_index.py` found the case.

Kept open on 2026-11-05 under decision 0054, because the review reads the answer with a doubt, and the question also waits on newest families that agree (gap review of the text phase 10 wrote, item RV4).

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 2 (`reports/review/2026-11-03-gap-review-question-cluster-2.md`) reads the reference as supporting option 1, weakly. The candidates for a direct binding of an lvalue reference return lvalue references, and the fallback through copy-initialization takes no explicit function. The review sees a gap, because the binding rules for this kind of reference expect a conversion to an rvalue that the candidate set never offers.

### Gap review, 2026-11-05

The source-reader gap review of the text phase 9 wrote on classes, overloading, and exceptions (`reports/review/2026-11-05-gap-review-phase-9-classes-overloading-exceptions.md`) supports the answer with a doubt. The reader finds that only lvalue reference results are offered to an lvalue reference and that the fallback through a temporary takes no explicit function, which gives option 1. An example in the reference that binds a reference to const through a conversion by value leaves a doubt.

### Gap review, 2026-11-05 (phase 10 text, classes, declarations, foundations, overloading, and statements)

The source-reader gap review of the text phase 10 wrote on classes, declarations, foundations, overloading, and statements (`reports/review/2026-11-05-gap-review-phase-10-classes-declarations-foundations.md`) reads the question: supports with a doubt. The candidate set for direct binding gives an lvalue reference only functions that return an lvalue reference. The fallback then copy-initializes a temporary, which excludes explicit functions. An example in the reference binds an lvalue reference to const directly to a value result of a conversion function, which does not fit that candidate set.
