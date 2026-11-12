# Q-0226: Can the template keyword come before a member that has no argument list after it?

**Status:** provisional
**Affects:** DEP-26, `tests/templates/dependent-names/template-keyword-no-list.cpp`, `tests/templates/dependent-names/template-keyword-no-list-class.cpp`, `tests/templates/dependent-names/template-keyword-no-list-static.cpp`
**Waits for:** the maintainer (pending.md row 4)

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

Kept on 2026-11-03. The source-reader gap review of question cluster 6 (`reports/review/2026-11-03-gap-review-question-cluster-6.md`) agrees with option 1 on the keyword rule, and so with the rejection of the data member after `.` and of the static data member. The review rejects the class template with deduced arguments on two other rules. The keyword has no place before the name of a deduced class type, and the qualifier of a deduced class type must not be dependent. With those rules, the results match Clang 22 in every column (option 3). The second rule is the reading of Q-0243, which Tidal does not follow, because every compiler accepts `typename Outer<T>::In x(1);`. Against the first rule, DEP-P5 treats the keyword `template` only as a mark that the name is a template, and here the name is a class template. Q-0243 keeps the form with the keyword on that ground, and GCC 16 and EDG 7.0 accept it. DEP-26 keeps option 1. A decision on Q-0243 settles the deduced form of this question too (`documentation/status.md`).

Kept open on 2026-11-03 under decision 0054, because the question waits on the maintainer (`documentation/decisions/pending.md`).

The probes of 2026-11-03 added no case where Clang 22, GCC 16, and EDG 7.0 agree with the provisional answer. For the deduced form, the cluster 6 review relies in part on the reading of Q-0243 that Tidal does not follow. DEP-26 keeps the deduced form, as GCC 16 and EDG 7.0 do, and that form waits with Q-0243 (`documentation/decisions/pending.md`).

Kept in `pending.md` row 4 on 2026-11-11 (decision 0062, item RV7), because the review still reads against the deduced form.

Evidence on 2026-11-11 (decision 0062, item RV7): the DEP-P5 row for the deduced form stays, as GCC 16 and EDG 7.0 accept the form. The divergence line on the dependent-names page names the review, and `pending.md` row 4 holds the deduced form.

Kept in `pending.md` row 4 on 2026-11-11 (decision 0063, item RV8). The gap review of the phase 15 text still reads against the deduced form, and only Clang 22 follows the review. The row 4 item names the review.

Kept in `pending.md` row 4 on 2026-11-11 (decision 0065, item RV10). The gap review of the phase 17 text reads the `["template"] identifier` arm of `typename_specifier` against the C++23 text (RV10-10), which is the deduced form of this question. The writer's probe gives the same split: GCC 16 and EDG 7.0 accept `typename T::template B b(1);`, and Clang 22 rejects it (`reports/clean-room/roadmap-14/writer-probes/rv10-review.json`). The production and DEP-26 keep the deduced form, as before.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 6 (`reports/review/2026-11-03-gap-review-question-cluster-6.md`) reads the keyword rule as option 1. A data member after `.` and a static data member after a qualifier are not templates, so both need a diagnostic, and GCC's acceptance after `.` is a defect. The class template with deduced arguments meets the keyword rule and fails on two other rules. The keyword has no place before a deduced class type, and the qualifier of a deduced class type must not be dependent. The member alias template fails the same way. So every form in the table is not valid, as Clang finds, and only a template template argument can have the keyword without a list (option 3 for the results). The review has not checked whether a later change lifts the limit on a dependent qualifier.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Gap review, 2026-11-11 (phase 14 text, expressions, overloading, and templates)

The source-reader gap review of the text phase 14 wrote on expressions, overloading, and templates (`reports/review/2026-11-11-gap-review-phase-14-expressions-overloading-templates.md`) names the question in its reading of DEP-P5 (RV7-B-1). The new DEP-P5 row states as valid a member class template after `typename T::template` with no argument list, as the type of a declaration whose arguments are deduced. The review reads against that row on two grounds. The C++23 form of a deduced class type has an optional `typename`, an optional qualifier, and the template's name, with no place for `template`. And the qualifier of a deduced class type must not be dependent. The review reads the form after `typename` that uses `template` as one that needs an argument list.

### Read of the RV7 repair diff, 2026-11-11

The source-reader read of the RV7 repair diff (`reports/review/2026-11-11-read-of-rv7-repair-diff.md`) reads the RV7 repair of the deduction page: the divergence line names the RV7 review and row 4, which the read finds correct.

### Gap review, 2026-11-11 (phase 15 text, expressions, overloading, and templates)

The source-reader gap review of the phase 15 text on expressions, overloading, and templates (`reports/review/2026-11-11-gap-review-phase-15-expressions-overloading-templates.md`) reads the question: reads against the deduced form (RV8-B-2). The reference's form for a deduced class type has no place for the keyword `template`, and its qualifier must not be dependent, so the test that accepts the form reads against the reference. The review adds a small doubt on DEP-25: the keyword with no list before a template template argument is allowed and deprecated, and Tidal states it with no mark.

### Read of the RV8 repair diff, 2026-11-11

The source-reader read of the RV8 repair diff (`reports/review/2026-11-11-read-of-rv8-repair-diff.md`) names the question with RV8-B-2, which the repair left as it was. The record states the review's reading correctly, and the doubt on DEP-25 stays. The read gives no new reading of the question's answer.

### Gap review, 2026-11-11 (phase 17 text)

The source-reader gap review of the phase 17 text (`reports/review/2026-11-11-gap-review-phase-17.md`) reads `typename_specifier` and the sentence after it against the deduced form (RV10-10). The typename form of the reference ends in an identifier, or in an optional `template` keyword and a template-id. It has no keyword before a plain identifier. The keyword before a qualified name of a class template or alias template with no argument list is allowed and deprecated, but only where a production lets the keyword come before a plain name, such as a template template argument. The review names one doubt: the reference's rule on the keyword names class templates in general and does not exclude the typename form in words.
