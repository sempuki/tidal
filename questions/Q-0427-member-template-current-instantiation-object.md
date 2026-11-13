# Q-0427: Does a member template named after `.` need `template` when the object's type is the current instantiation?

**Status:** settled
**Affects:** DEP-24, DEP-39, DEP-2, `tests/templates/dependent-names/member-template-through-current-instantiation-object.cpp`, `tests/templates/dependent-names/member-template-current-instantiation-pointer.cpp`, `tests/templates/dependent-names/member-template-current-instantiation-dependent-base-needs-template.cpp`, `tests/templates/dependent-names/member-template-non-dependent-base.cpp`, `tests/templates/dependent-names/member-template-current-instantiation-qualifier-dependent-base.cpp`
**Source:** the round 10 programmer report (`reports/clean-room/roadmap-10/programmer.md`, sections 1.5 and 6), reduction `member_template_through_current_instantiation_object.cpp`.

## Context

Inside the class template `Wrap<T>`, a static member function takes a
parameter `w` of type `const Wrap&` and calls `w.get<int>()`, where
`get` is a member template of `Wrap`. The type of `w` is the current
instantiation, so DEP-2 makes `w` type-dependent. DEP-24's bullet for
`.` and `->` asks for `template` before a member template after any
type-dependent object. DEP-24's bullet for `::` and DEP-39 leave out
the current instantiation, and the bullet for `.` and `->` does not.

| Form | Clang 18 and 22 | GCC 14 and 16 | EDG 7.0 | Tidal front end |
|---|---|---|---|---|
| `w.get<int>()` with no `template` | accepts | accepts | accepts | rejects (DEP-24) |

The front end follows the text as it stands.

## Evidence

The tests in **Affects**. The two run tests call the member template
with no `template` through a reference, a pointer, and a local object
of the current instantiation. The reject test names a member template
that only a dependent base declares, so lookup at the definition does
not find it in the class, and every compiler asks for `template`. The
writer probe `reports/clean-room/roadmap-10/writer-probes/f3b-dep-24-object-forms.json`
gives each form.

## Options

1. The keyword is not needed when the object's type is the current
   instantiation and lookup at the definition finds a member template
   in the class. DEP-24's bullet for `.` and `->` gains the exception
   that its `::` bullet and DEP-39 have (every compiler).
2. The keyword is needed after any type-dependent object, as DEP-24
   says now. The `<` is then a less-than operator, and the program is
   rejected.

## Answer

No; the program runs without `template` (option 1). Every compiler
accepts the form, and DEP-39 already treats an object of the current
instantiation as a case for lookup at the definition.

Rule text on 2026-11-10 (decision 0061, item F3): DEP-24's bullet for
`.` and `->` leaves out an object whose type is the current
instantiation when lookup at the definition finds the member template
in the class. The question waits for a first review.

Kept open on 2026-11-10 under decision 0054, because the review reads the answer with a doubt on DEP-24's wording (item RV6).

Rule text on 2026-11-10 (decision 0061, item RV6): DEP-24 and the DEP-P5 row say that the member template can come from the class or from a base that is not dependent. A new run test shows the base form after `.` and after `::`, and a reject test shows `::` with the current instantiation for a member of a dependent base. The doubt stays until a review reads the new text.

Kept open on 2026-11-11 under decision 0054 (item RV7). The review supports the answer with no doubt, and every compiler agrees. The DEP-P5 row for `x.name<...>`, where `x` has the type of the current instantiation, gives a template-id, so the model rules out option 2. DEP-P5 changed in phase 14, so it gives no answer until a reader or a review uses it (decision 0055). The review reads DEP-24 and does not use DEP-P5.

Principle text on 2026-11-11 (decision 0062, item RV7): the DEP-P5 sentence on the middle column now says that a row whose middle column says "not valid" reports an error at the definition (RV7-B-10). The row for this question is unchanged.

Kept open on 2026-11-12 under decision 0054 (decision 0066, item RV12). The gap review of the phase 18 text uses DEP-P5 on the question (RV12-15). It names DEP-P5 and the question's program and reaches option 1 by DEP-P5's rows for a member access, so DEP-P5 passes step 3 of the principle test. The review supports the answer with no doubt, and every compiler agrees. The question meets decision 0054's settling rule. A commit cannot name its own hash, and the settlements table needs one, so the next commit settles the question and names this commit in its row. Until then the **Waits for:** line names newest families that agree, was a use of DEP-P5.

Settled on 2026-11-12 under decision 0054 (decision 0066, item RV12). The gap review of the phase 18 text (`reports/review/2026-11-12-gap-review-phase-18.md`) uses DEP-P5 on the question and supports the answer with no doubt (RV12-15), and every compiler agrees. DEP-P5 gives the answer. Its row for `x.name<...>`, where `x` has the type of the current instantiation and the class or a base that is not dependent declares the member template, gives a template-id, so no `template` is needed. The model rules out option 2. DEP-P5 last changed in phase 14 (decision 0062, item RV7), and the review is the use that decision 0055 asks for.

## Opinions

### Gap review, 2026-11-10 (phase 13 text, expressions, overloading, and templates)

The source-reader gap review of the text phase 13 wrote on expressions, overloading, and templates (`reports/review/2026-11-10-gap-review-phase-13-expressions-overloading-templates.md`) reads the question: supports with a doubt (RV6-B-9). The name after the dot is looked up in the type of the object, and the lookup runs at the definition when that type is the current instantiation. A member template found there makes the `<` open an argument list, so no `template` is needed (option 1). The doubt is DEP-24's phrase "in the class": the lookup also searches the bases that are not dependent, and a member template that such a base declares needs no `template` either.

### Gap review, 2026-11-11 (phase 14 text, expressions, overloading, and templates)

The source-reader gap review of the text phase 14 wrote on expressions, overloading, and templates (`reports/review/2026-11-11-gap-review-phase-14-expressions-overloading-templates.md`) reads the question: supports with no doubt (RV7-B-9). After the dot or arrow, the name is looked up in the class of the object. When that class is the current instantiation, the lookup runs at the definition and also searches the bases that are not dependent. A member template found there makes the `<` open an argument list, so no `template` is needed (option 1). DEP-24's two bullets now say this, and the RV6 doubt is closed. A base that is the current instantiation of an enclosing class counts as a base that is not dependent.

### Gap review, 2026-11-12 (phase 18 text)

The source-reader gap review of the phase 18 text (`reports/review/2026-11-12-gap-review-phase-18.md`) uses DEP-P5 on the question: supports with no doubt (option 1, RV12-15). The name after a dot or an arrow is looked up in the class of the object. When that class is the current instantiation, the lookup runs at the definition and also searches the bases that are not dependent, and a member template found there makes the following `<` open an argument list. When the class is dependent in any other way, no lookup runs at the definition, and the `<` is the less-than operator. DEP-P5's two rows for a member access say this. The row for the current instantiation gives a template-id, so DEP-P5 gives option 1, and the answer agrees with the reference.
