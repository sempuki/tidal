# Q-0227: Can the template keyword follow the qualifier of a name that a declaration declares?

**Status:** provisional
**Affects:** DEP-27, `tests/templates/dependent-names/template-keyword-in-declarator.cpp`, `tests/templates/dependent-names/template-keyword-in-declarator-static.cpp`, `tests/templates/dependent-names/template-keyword-in-declarator-class.cpp`

## Context

The definition `template <class T> template <class U> void
Box<T>::template put(U) {}` writes `template` before the declared name.
The same form defines a static member function template and a member
class template, `struct Box<T>::template Inner { ... };`.

| Compiler | member function | static member function | member class template |
|---|---|---|---|
| Clang 18, 22, host | rejects: expected an unqualified-id | rejects | rejects: expected an identifier |
| GCC 14, 16, host | accepts | accepts | rejects: the keyword is not allowed in the class name |
| EDG 7.0 | accepts | accepts | accepts |

## Evidence

The tests above. Inventory item F7 says that the keyword must not
follow the qualifier of a name being declared.

GCC also accepts the keyword in the definition of a member variable template. GCC and EDG accept the keyword after a namespace qualifier. Each form that the probes tried splits the compilers.

## Options

1. The keyword is not valid there (Clang).
2. The keyword is allowed and has no effect (EDG; GCC for a function).

## Provisional answer

No; Clang agrees, GCC agrees for a class, and EDG differs (option 1). The inventory supports it, and the keyword has no purpose in
a declarator, where the name must already be a member template. The
provisional answer goes against two of three implementations for a
function, so a contributor can argue for option 2.

The question stays open under decision 0051 (2026-11-03). Clang 22 rejects the keyword in each form. GCC 16 and EDG 7.0 accept it before a member function template. A probe of 2026-11-03 found that GCC 16 rejects it before a member class template, and EDG 7.0 accepts it (`template-keyword-in-declarator-class.cpp`). So two families disagree in each form. DEP-P5 lists DEP-27 as an exception. The keyword tells the parser that a template name follows a dependent qualifier, and the name that a declaration declares must already be a member template of the class.

Kept on 2026-11-03. The source-reader gap review of question cluster 6 (`reports/review/2026-11-03-gap-review-question-cluster-6.md`) agrees with option 1 for each form. DEP-27 stays a `reject` rule.

Kept open on 2026-11-03 under decision 0054, because two of the newest families differ from the answer in at least one form, and the rule records one differing family.

The probes of 2026-11-03 added no case where Clang 22, GCC 16, and EDG 7.0 agree with the provisional answer.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 6 (`reports/review/2026-11-03-gap-review-question-cluster-6.md`) reads the reference as supporting option 1. The keyword must not come right after a qualifier that names the scope of the declared entity. The rule covers a member function, a static member function, and a member class template alike, and a diagnostic is required.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
