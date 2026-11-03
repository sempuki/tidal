# Q-0235: Can an explicit instantiation outside the template's namespace use an unqualified name?

**Status:** provisional
**Affects:** INST-21, `tests/templates/instantiation/explicit-unqualified-outside-namespace.cpp`, `tests/templates/instantiation/explicit-unqualified-outside-namespace-variable.cpp`

## Context

`namespace lib { template <class T> struct Box {}; }`, then
`using namespace lib;` and `template struct Box<int>;` in the global
namespace. The other columns name a function template `lib::get` and
a variable template `lib::value` in the same way.

| Compiler | class | function | variable |
|---|---|---|---|
| Clang 18, 22, host | rejects: must occur in namespace `lib` | rejects | accepts |
| GCC 14, 16, host | rejects under `-pedantic-errors`: must use a nested-name-specifier | rejects | rejects |
| EDG 7.0 | accepts | rejects | accepts |

## Evidence

The tests above, `explicit-unqualified-outside-namespace-function.cpp`, and `explicit-enclosing-namespace.cpp`, which every
compiler accepts with a qualified name.

## Options

1. The name must be qualified (GCC; Clang except for a variable).
2. An unqualified name that lookup finds is enough (EDG except for a
   function).

## Provisional answer

No; Clang and GCC agree for a class, Clang and EDG differ for a variable (option 1). Two of three implementations agree for a class, and all three agree for a function. For a variable, the provisional answer goes against two of three implementations, so a contributor can argue for option 2.

The question stays open under decision 0051 (2026-11-03). Clang 22 and GCC 16 reject the unqualified name, and EDG 7.0 accepts it, so two families disagree. INST-P3 supports the provisional answer through INST-21. An explicit instantiation stands in the namespace of the template or in an enclosing namespace, and in an enclosing namespace it names the template with a qualified name. EDG's behavior is on the cluster 5 list of EDG differences that waits on the maintainer (`documentation/status.md`).

Probes of 2026-11-03 add two forms after the same using-directive. Every compiler rejects `template int get(int);` for a function template `lib::get` (`explicit-unqualified-outside-namespace-function.cpp`). For a variable template, `template int value<int>;` is accepted by Clang 22 and EDG 7.0 and rejected by GCC 16 (`explicit-unqualified-outside-namespace-variable.cpp`). The question stays open under decision 0051, and two families now disagree with the provisional answer for a variable.

Kept on 2026-11-03. The source-reader gap review of question cluster 5 (`reports/review/2026-11-03-gap-review-question-cluster-5.md`) reads the reference as supporting option 2, with doubt. The review finds no rule in the reference that puts an explicit instantiation in the namespace of its template, and no rule that its name be qualified. The review's doubt is that the reference may have dropped both rules without intent. The matrix keeps the first of the two rules. Every compiler rejects a qualified explicit instantiation in a namespace that does not enclose the template (`explicit-other-namespace.cpp`), and every compiler rejects the unqualified function form. Option 2 for the class form rests on the same missing rule that every compiler still applies. INST-P3 and INST-21 keep option 1. The review matches EDG 7.0 for the class form and Clang 22 and EDG 7.0 for the variable form, so the question is on the maintainer's list in `documentation/status.md`.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 5 (`reports/review/2026-11-03-gap-review-question-cluster-5.md`) reads the reference as supporting option 2, with doubt. The reference has no rule that places an explicit instantiation in the namespace of its template and no rule that its name be qualified. Earlier standards had both rules, and the reference dropped them when it rewrote scope and lookup. Ordinary lookup then finds the class template through the using-directive, and the program is valid, as EDG finds. The reference keeps a lookup limit for a function template specialization, which considers only templates that belong to the scope of the declaration, so the function form stays not valid. The doubt is that Clang and GCC still apply the old rule, and the reference may have dropped the rule without intent.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
