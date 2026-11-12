# Q-0318: Must constant evaluation reject a member function call through a null pointer?

**Status:** provisional
**Affects:** PFX-21, CE-9, `tests/expressions/postfix/member-call-null-pointer-constant.cpp`
**Waits for:** a review (doubt)

## Context

A `constexpr` function calls `p->one()` where `p` is a null pointer to
a class, and `one` reads no member. A `static_assert` evaluates the
function.

| Compiler | Result |
|---|---|
| Clang 22, 18 | rejects: member call on dereferenced null pointer |
| GCC 16, 14 | rejects: dereferencing a null pointer |
| EDG 7.0 | accepts |

At run time, UBSan on Clang and GCC reports "member call on null
pointer" for the same call.

## Evidence

The test above and
`tests/expressions/postfix/member-call-null-pointer-sanitizer.cpp`.

## Options

1. The call is undefined (PFX-21), so constant evaluation rejects it
   (CE-9). EDG misses the case.
2. The call is valid when the function does not touch the object.

## Provisional answer

Yes, undefined (PFX-21); EDG accepts (option 1). Clang and GCC reject the call in constant evaluation, and both
sanitizers report it at run time. The test expects a rejection and
records EDG's result with an override.

The question stays open under decision 0051 (2026-11-03). Clang 22 and GCC 16 reject the call, and EDG 7.0 accepts it, so two families disagree. A probe where the member function reads a member is rejected by every compiler. CE-P3 supports the provisional answer, because constant evaluation rejects any operation that is undefined (CE-9, PFX-21). The front end's interpreter rejects the call with PFX-21.

Kept on 2026-11-03. The source-reader gap review of question cluster 4 agrees with option 1, with doubt.

Kept open on 2026-11-03 under decision 0054, because the gap review does not support the answer with no doubt (it reads with doubt, weakly, unclearly, or for only part of the question).

Kept open on 2026-11-05 under decision 0054, because the review reads the answer with a doubt (gap review of the text phase 9 wrote, item RV3).

Kept open on 2026-11-11 under decision 0054 (decision 0066, item DQ). The second review supports the answer with a narrower doubt (DQ-17), and EDG 7.0 alone differs. The doubt is only that C++23 has no rule that names indirection through a null pointer as undefined.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 4 (`reports/review/2026-11-03-gap-review-question-cluster-4.md`) reads the reference as supporting option 1, with doubt. The arrow form of member access is an indirection followed by the dot form, and a null pointer points to no object. A member access is undefined when the left operand gives an object of a type that does not match. The doubt is that the reference does not say plainly that the call through a null pointer is undefined. A later core change makes an indirection through a pointer that points to no object or function undefined, which settles the case for option 1.

### Gap review, 2026-11-05 (declarations, statements, and foundations)

The source-reader gap review of the text phase 9 wrote on declarations, statements, and foundations (`reports/review/2026-11-05-gap-review-phase-9-declarations-statements-foundations.md`) reads the question: supports with a doubt. C++23 gives no explicit rule that indirection through a null pointer is undefined. The answer rests on the indirection result naming the object the pointer points to, and a null pointer points to none. A later defect resolution (CWG2823) makes such an indirection explicitly undefined, which would remove the doubt.

### Gap review, 2026-11-05 (expressions and templates)

The source-reader gap review of the text phase 9 wrote on expressions and templates (`reports/review/2026-11-05-gap-review-phase-9-expressions-templates.md`) reads the question: supports with a doubt. No rule names a member call through a null pointer as undefined. The answer rests on the lack of any object, and constant evaluation rejects only undefined operations that the text specifies.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Second review of the doubt-held questions, 2026-11-11

The source-reader second review of the questions held only by a doubt (`reports/review/2026-11-11-second-review-doubt-held-questions.md`) reads the question: supports with a doubt that remains (DQ-17). C++23 has no rule that names indirection through a null pointer, or a member call through one, as undefined. The arrow form is defined as indirection followed by the dot form. Indirection gives the object that the pointer points to, and a null pointer points to no object. The rule that makes a member access on an object of an unrelated type undefined assumes that an object exists. The doubt is now only that missing rule, and the answer rests on the lack of any object for the access and the call.
