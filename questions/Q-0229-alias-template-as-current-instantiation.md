# Q-0229: Does an alias template specialization name the current instantiation?

**Status:** provisional
**Affects:** DEP-15, `tests/templates/dependent-names/current-instantiation-alias.cpp`

## Context

`template <class U> using Same = Box<U>;` and, inside `Box<T>`, the
name `Same<T>`.

| Compiler | Member type without `typename` | Missing member, no dependent base, unused | Member of a dependent base |
|---|---|---|---|
| Clang 18, 22, host | accepts | rejects | accepts |
| GCC 16, host | accepts | rejects | rejects |
| GCC 14 | accepts | accepts | accepts |
| EDG 7.0 | rejects: needs `typename` | accepts | accepts |

## Evidence

The test above and the grid `current-instantiation` (row `Same<T>`).
Inventory item C1 says that an equivalent alias template
specialization names the current instantiation.

## Options

1. The alias names the current instantiation (Clang; GCC except with
   a dependent base).
2. The alias names a dependent specialization that is not the current
   instantiation (EDG).

## Provisional answer

Yes; Clang and GCC agree, EDG differs (option 1). Clang treats every case as the current instantiation, and
the inventory agrees. GCC's rejection of a dependent base member
through the alias is a separate defect under DEP-18.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
