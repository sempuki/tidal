# Q-0427: Does a member template named after `.` need `template` when the object's type is the current instantiation?

**Status:** provisional
**Affects:** DEP-24, DEP-39, DEP-2, `tests/templates/dependent-names/member-template-through-current-instantiation-object.cpp`, `tests/templates/dependent-names/member-template-current-instantiation-pointer.cpp`, `tests/templates/dependent-names/member-template-current-instantiation-dependent-base-needs-template.cpp`
**Source:** the round 10 programmer report (`reports/clean-room/roadmap-10/programmer.md`, sections 1.5 and 6), reduction `member_template_through_current_instantiation_object.cpp`.
**Waits for:** a review (doubt)

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

## Provisional answer

No; the program runs without `template` (option 1). Every compiler
accepts the form, and DEP-39 already treats an object of the current
instantiation as a case for lookup at the definition.

Rule text on 2026-11-10 (decision 0061, item F3): DEP-24's bullet for
`.` and `->` leaves out an object whose type is the current
instantiation when lookup at the definition finds the member template
in the class. The question waits for a first review.

Kept open on 2026-11-10 under decision 0054, because the review reads the answer with a doubt on DEP-24's wording (item RV6).

## Opinions

### Gap review, 2026-11-10 (phase 13 text, expressions, overloading, and templates)

The source-reader gap review of the text phase 13 wrote on expressions, overloading, and templates (`reports/review/2026-11-10-gap-review-phase-13-expressions-overloading-templates.md`) reads the question: supports with a doubt (RV6-B-9). The name after the dot is looked up in the type of the object, and the lookup runs at the definition when that type is the current instantiation. A member template found there makes the `<` open an argument list, so no `template` is needed (option 1). The doubt is DEP-24's phrase "in the class": the lookup also searches the bases that are not dependent, and a member template that such a base declares needs no `template` either.
