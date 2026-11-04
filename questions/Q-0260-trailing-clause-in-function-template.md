# Q-0260: Can a lambda or a member of a local class inside a function template have a trailing requires clause?

**Status:** settled
**Affects:** CPT-11, CNS-18, `tests/templates/concepts/trailing-clause-lambda.cpp`, `tests/templates/concepts/trailing-clause-local-class.cpp`, `tests/templates/constraints/lambda-conversion.cpp`, `tests/templates/concepts/trailing-clause-member-initializer-lambda.cpp`

## Context

The lambda has no template parameter list of its own. The local class
is not a template. Both are declared in the body of a function
template, and the clause names the function template's parameter.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | accepts |
| GCC 14, 16, host | accepts |
| EDG 7.0 | rejects: a requires-clause is not allowed here (not a templated function) |

## Evidence

The four tests above. EDG also rejects the clause on a lambda in a default member initializer of a class template. Every compiler accepts the clause on a member of a class nested in a class template (`trailing-clause-member.cpp`).

EDG rejects a trailing requires clause on each lambda that is not a template, also after a trailing return type.

## Options

1. Valid (Clang, GCC). The lambda and the local class are part of the
   function template, so the clause has template arguments to check.
2. Not valid (EDG).

## Answer

Yes; Clang and GCC agree, EDG differs (option 1). Two of three implementations agree, and the case is the same as a member of a class nested in a class template.

The question stays open under decision 0051 (2026-11-03). Clang 22 and GCC 16 accept the clause on the lambda and on the member of the local class, and EDG 7.0 rejects it, so two families disagree. CPT-P1 supports the provisional answer. A trailing requires clause constrains a function that is declared inside a templated entity, and the body of a function template is one (CPT-11). EDG's behavior is on the cluster 5 list of EDG differences that waits on the maintainer (`documentation/decisions/pending.md`).

Kept on 2026-11-03. The source-reader gap review of question cluster 5 agrees with option 1.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of question cluster 5 (`reports/review/2026-11-03-gap-review-question-cluster-5.md`) supports this answer with no doubt, and CPT-P1 gives the same answer: a trailing requires clause constrains a function declared inside a templated entity. EDG 7.0 still differs. Tidal records EDG's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for EDG.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 5 (`reports/review/2026-11-03-gap-review-question-cluster-5.md`) reads the reference as supporting option 1. A trailing requires clause is valid on a templated function. A local class in a templated entity, its members, and a closure type made in a templated entity are templated, so their functions are templated functions. A closure type in the declaration of a class template is templated too.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
