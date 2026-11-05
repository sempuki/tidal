# Q-0196: Can a friend declaration of a class template give a default template argument?

**Status:** provisional
**Affects:** TPL-37
**Waits for:** a principle (TPL-P3)

## Context

A class declares `template <class T = int> friend struct Box;` for a
class template `Box` declared before it.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | rejects: default argument on a friend template |
| GCC 14, 16, host | accepts |
| EDG 7.0 | rejects: default argument on a friend |

All compilers reject a default on a friend function template
declaration that is not a definition.

## Evidence

`tests/templates/declarations/default-friend-class.cpp`, with an
override for GCC, and `default-friend-declaration.cpp`.

## Options

1. A friend declaration of a class template must not give a default
   (Clang, EDG).
2. The default is valid (GCC).

## Provisional answer

No; Clang and EDG reject, GCC accepts (option 1). Clang and EDG agree, and the rule then matches the rule for
friend function template declarations, which all compilers apply.

Kept open on 2026-11-03 under decision 0054, because no principle on the page gives the answer. TPL-P3 lets the defaults of several declarations add up and does not name friend declarations, and TPL-37 is an exception to it.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). TPL-P3 fails the test, because TPL-P3 names TPL-37 in its `:exceptions:` line. The matrix shows no model that explains why a friend declaration that is not a definition cannot give a default, beyond the rule itself. The question stays provisional.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) agrees with option 1. A default template argument in a friend declaration of a class template is forbidden outright. For a friend function template a default is allowed only on a definition that is the only declaration. The rule is not exempt from diagnosis, so GCC's acceptance is a defect.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
