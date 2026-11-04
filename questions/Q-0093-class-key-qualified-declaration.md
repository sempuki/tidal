# Q-0093: Is a declaration of only a class key and a qualified name valid?

**Status:** provisional
**Affects:** TYN-7, `tests/declarations/type-names/elaborated-qualified-declaration-only.cpp`

## Context

After `namespace n { struct S; }`, the declaration `struct n::S;`
consists only of a class key and a qualified name. Clang and EDG reject
it. GCC accepts it.

## Options

1. The declaration is not valid.
2. The declaration is valid and has no effect.

## Provisional answer

No; the declaration is not valid (option 1). Clang and EDG agree.

Kept open on 2026-11-03 under decision 0054, because no principle on the page gives the answer. TYN-P2 and TYN-P3 say how a class key restricts lookup and when a class is complete, and neither says which forms a declaration of only a class key and a name can take. GCC 16 is the only newest family that differs.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). TYN-P2 and TYN-P3 fail step 3, because neither says which forms a declaration of only a class key and a name can take, so option 2 also fits. Item PQ widened TYN-P2: a declaration of only a class key and a plain name declares the class in the current scope, and with a qualified name the declaration declares nothing and is rejected. TYN-P2 now lists TYN-7. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses TYN-P2.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. A declaration of only a class key and a name is valid only in a few forms, and each form has a plain identifier or a template name with arguments. A qualified name is not one of them, except in an explicit specialization or an explicit instantiation. A diagnostic is required.
