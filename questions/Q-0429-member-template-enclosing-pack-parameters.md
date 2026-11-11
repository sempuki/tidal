# Q-0429: Does a function parameter pack of a member template that expands only the enclosing class's pack take part in deduction as a pack?

**Status:** provisional
**Affects:** DED-20, INST-5, PACK-1, `tests/templates/packs/member-template-enclosing-pack-before-deduced.cpp`, `tests/templates/packs/member-template-enclosing-pack-after-deduced.cpp`
**Source:** the round 11 programmer report (`reports/clean-room/roadmap-11/programmer.md`, section 1.5), reductions `member_template_class_pack_before_deduced_parameter.cpp` and `member_template_class_pack_after_deduced_parameter.cpp`.
**Waits for:** a first review; a rule

## Context

A class template has a type parameter pack `Ts`. A member template of
the class has its own type parameter `Total` and the function
parameters `Ts... parts` and `Total start`, in either order. A program
calls the member template on an object of `Mixer<int, int>`.

`parts` is a function parameter pack (PACK-1). DED-20 says that a
function parameter pack before the end of the parameters is a
non-deduced position, and that such a pack is empty unless explicit
arguments give it elements. INST-5 says that the instantiation of the
class makes the declarations of its members, so in `Mixer<int, int>`
the elements of `Ts` are known before any call. The text does not say
whether DED-20 applies to a pack whose elements the enclosing class
already fixes.

| Form | Clang 18 and 22 | GCC 14 and 16 | EDG 7.0 | Tidal front end |
|---|---|---|---|---|
| `parts` before `start` | runs | runs | accepts | rejects (DED-35) |
| `parts` after `start` | runs | runs | accepts | stops (unsupported) |

## Evidence

The tests in **Affects**, on the matrix.

## Options

1. The instantiation of the class expands `parts` into one parameter
   for each element of `Ts`. The member template then has ordinary
   parameters in their place, and deduction finds only `Total`. DED-20
   applies only to a pack of the template that the call deduces (every
   compiler).
2. `parts` stays a pack in deduction. Before the end, DED-20 makes it a
   non-deduced position, and the call has no viable function.

## Provisional answer

The pack takes part as one ordinary parameter for each element of the
class's pack, and deduction finds only the member template's own
parameters (option 1). Every compiler agrees.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
