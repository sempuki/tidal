# Q-0427: Does a member template named after `.` need `template` when the object's type is the current instantiation?

**Status:** provisional
**Affects:** DEP-24, DEP-39, DEP-2, `tests/templates/dependent-names/member-template-through-current-instantiation-object.cpp`
**Source:** the round 10 programmer report (`reports/clean-room/roadmap-10/programmer.md`, sections 1.5 and 6), reduction `member_template_through_current_instantiation_object.cpp`.
**Waits for:** a first review; a rule

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

The test in **Affects**, which expects a run.

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

## Opinions
