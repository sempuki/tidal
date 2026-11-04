# Q-0242: Can the result of a deduction guide have a cv-qualifier?

**Status:** provisional
**Affects:** CTAD-12, `tests/templates/class-template-deduction/guide-result-qualified.cpp`

## Context

`template <class T> Box(T*) -> const Box<T>;`

| Compiler | Result |
|---|---|
| Clang 18, 22, host | rejects: the result is not a specialization of the template |
| EDG 7.0 | rejects: the return type must name a specialization directly |
| GCC 14, 16, host | accepts |

## Evidence

The test above.

## Options

1. The result must be a template-id of the class template, with no
   cv-qualifier (Clang, EDG).
2. A cv-qualifier is allowed and has no effect (GCC).

## Provisional answer

No; Clang and EDG agree, GCC differs (option 1). Two of three implementations agree, and the result of a guide
names the specialization that a declaration gets. A declaration adds
its own cv-qualifiers (CTAD-1).

Reopened as provisional on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) supports this answer with no doubt. CTAD-P1 fails step 3 of the test. The model says that a guide returns a specialization. The model does not say whether the guide can write a cv-qualifier on that specialization, so option 2 also fits the model. No other principle on the page gives the answer. GCC 16 differs, and the tests keep their overrides for GCC 16.

On 2026-11-04, item PQ widened CTAD-P1. A user guide writes its result as the name of the class template with an argument list, the result only names the specialization, and a declaration that uses the guide adds its own cv-qualifiers. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses CTAD-P1.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) agrees with option 1. The grammar of a deduction guide allows only a template name with an argument list after the arrow. A cv-qualifier has no place there, so the form is a syntax error. GCC's acceptance is a defect.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
