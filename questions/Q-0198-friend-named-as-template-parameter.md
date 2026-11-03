# Q-0198: Can a friend function in a class template have the name of a template parameter?

**Status:** provisional
**Affects:** TPL-48, TPL-18

## Context

A class template `template <class T> struct Holder` declares
`friend void T();`, and the program uses `Holder<int>`. A second form
defines the friend in the class: `friend void T(Holder) {}`.

| Compiler | Friend declaration | Friend definition |
|---|---|---|
| Clang 18, 22, host | accepts | accepts |
| GCC 14, 16, host | accepts | rejects: shadows template parameter |
| EDG 7.0 | rejects at the use of `Holder<int>`: the parameter is redeclared | rejects at the use |

Without a use of `Holder<int>`, all compilers accept the declaration.

## Evidence

`tests/templates/declarations/parameter-friend-name.cpp`, with an
override for EDG. The friend definition has no test.

## Options

1. A friend function declaration with the name is valid, because the
   friend binds no name in the template's scope (Clang, GCC).
2. The friend redeclares the parameter and is not valid (EDG).

## Provisional answer

Yes for the friend declaration (option 1), where Clang and GCC agree. The
friend definition stays open: Clang accepts it, and GCC and EDG reject
it. Tidal states no rule for the definition.

The question stays open under decision 0051 (2026-11-03). Clang 22 and GCC 16 accept the friend declaration when the program uses `Holder<int>`, and EDG 7.0 rejects it, so two families disagree. A probe of the friend definition gives the recorded split: Clang 22 accepts it, and GCC 16 and EDG 7.0 reject it. TPL-P4 supports the provisional answer for the declaration. No declaration in the template can reuse a parameter's name, and a friend function declaration declares no name in the template's scope (TPL-48). EDG's behavior is on the cluster 5 list of EDG differences that waits on the maintainer (`documentation/decisions/pending.md`).

Kept on 2026-11-03. The source-reader gap review of question cluster 5 agrees with option 1. The review also reads the friend definition as valid, as Clang accepts it. Tidal still states no rule for the friend definition.

Kept open on 2026-11-03 under decision 0054, because the friend definition has no answer and no rule.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 5 (`reports/review/2026-11-03-gap-review-question-cluster-5.md`) reads the reference as supporting option 1. A later declaration in the scope of a template parameter must not bind the parameter's name, and a friend declaration binds no name. A friend that is a definition binds no name either, so the friend definition fits Clang's acceptance, and the rejection by GCC and EDG has no support.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
