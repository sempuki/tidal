# Q-0458: Can a friend with two template heads name a member of a class template specialization whose arguments are a pattern of the first head's parameters?

**Status:** provisional
**Affects:** TPL-4, `tests/templates/declarations/friend-two-heads-pointer-pattern.cpp`
**Source:** RV12-4 (`reports/review/2026-11-12-gap-review-phase-18.md`), with the test in **Affects** run on the matrix.
**Waits for:** the maintainer (pending.md row 4)

## Context

A class declares a friend with two template heads. The qualifier of
the friend's name is `Shelf<T*>`, so its argument is a pattern of the
first head's parameter `T`. `T` can be deduced from `Shelf<int*>`:

```cpp
template <class T> struct Shelf { template <class U> constexpr int peek(U); };
class Vault {
  int code = 7;
  template <class T> template <class U> friend constexpr int Shelf<T*>::peek(U);
};
template <class T> template <class U> constexpr int Shelf<T>::peek(U) { return Vault{}.code; }
static_assert(Shelf<int*>{}.peek('a') == 7);
```

| Form | Clang 18 and 22 | GCC 14 and 16 | EDG 7.0 |
|---|---|---|---|
| the friend `Shelf<T*>::peek` | accepts, and `peek` of `Shelf<int*>` reads `code` | rejects, "declared as friend before type 'Shelf<T*>' defined" | rejects, "declaration is incompatible with" |
| the friend `Shelf<T>::peek` (`friend-two-heads-member-template.cpp`) | accepts | accepts | accepts |
| the friend `Shelf<int>::peek` (`friend-two-heads-fixed-qualifier.cpp`) | rejects | rejects | rejects |

## Evidence

The test in **Affects** expects the rejection and records Clang as an
override.

## Options

1. The qualifier must give the first head's parameters themselves as
   its arguments, so the friend is not valid (GCC, EDG).
2. The first head's parameters need only be deducible from the
   qualifier's template-id. The friend is the member of each
   specialization that matches the pattern, here `Shelf<int*>` (Clang,
   the gap review of the phase 18 text).

## Provisional answer

The friend is not valid (option 1). GCC 16 and EDG 7.0 reject it, and
TPL-4 keeps their answer. Clang 22 alone follows the review, so the
question waits with the other items of `pending.md` row 4.

Opened on 2026-11-12 (decision 0066, item RV12). The review reads
TPL-4's second condition as narrower than the C++23 text (RV12-4). The
writer's test splits the newest families. TPL-P6 names TPL-4 in
`:exceptions:` and says nothing of a friend with two heads, so no
principle gives the answer.

## Opinions

### Gap review, 2026-11-12 (phase 18 text)

The source-reader gap review of the phase 18 text (`reports/review/2026-11-12-gap-review-phase-18.md`) reads TPL-4's second condition with a doubt (RV12-4). The reference lets a friend template name a member of a dependent specialization when the qualifier ends in a template-id that names a class template, and the friend's own template parameters can be deduced from that template-id. So the template-id may give a pattern of the parameters, such as a pointer to the parameter, and the friend then covers the specializations that match the pattern. TPL-4 asks for the parameters themselves as the arguments, which leaves out these patterns.
