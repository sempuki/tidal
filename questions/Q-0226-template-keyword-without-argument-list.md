# Q-0226: Can the template keyword come before a member that has no argument list after it?

**Status:** provisional
**Affects:** DEP-26, `tests/templates/dependent-names/template-keyword-no-list.cpp`, `tests/templates/dependent-names/template-keyword-no-list-class.cpp`, `tests/templates/dependent-names/template-keyword-no-list-static.cpp`

## Context

`object.template field` names a data member with the keyword
`template` and no argument list. The object has a dependent type.

`typename T::template box b(1);` names a member class template with
the keyword and no argument list, and deduces the arguments from the
initializer.

`T::template field` names a static data member with the keyword and
no argument list.

| Compiler | data member after `.` | static data member after `T::` | class template, arguments deduced |
|---|---|---|---|
| GCC 14, 16, host | accepts | rejects: not a template | accepts |
| Clang 18, 22, host | rejects: an argument list is expected after the keyword | rejects | rejects |
| EDG 7.0 | rejects: an argument list for a template is expected | rejects | accepts |

## Evidence

The test above and the grid `template-keyword` (row "data member that
is not a template"). Inventory item F10 allows the keyword without a
list only before a class template or alias template. Item 3 of the gap
review `reports/review/2026-10-31-gap-review-dependent-names-and-instantiation.md`
says the same. Clang 22 reports the class template case under the
warning group `-Wmissing-template-arg-list-after-template-kw`, which is
an error by default.

## Options

1. The keyword without a list is valid only before a template that is
   a template template argument, or a class or alias template (EDG).
2. The keyword is ignored before a data member after `.` (GCC).
3. The keyword without a list is valid only before a template template
   argument (Clang).

## Provisional answer

Only before a template template argument or a class or alias template; Clang and GCC each differ on one case (option 1). For the data member, Clang and EDG agree. For the class
template, GCC and EDG agree. The inventory and the gap review support
option 1.

The question stays open under decision 0051 (2026-11-03). GCC 16 accepts the data member after `.`, and Clang 22 rejects the class template with deduced arguments, so two families disagree in each case. A probe of 2026-11-03 gives the class template split for a member alias template with deduced arguments too. GCC 16 rejects `T::template field` for a static data member (`template-keyword-no-list-static.cpp`), so GCC ignores the keyword only after `.`. DEP-P5 supports the provisional answer. The keyword states that a template follows, and a name with no argument list after the keyword must still stand for a template.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
