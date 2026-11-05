# Q-0294: How does a braced list bind a reference, initialize an array, and form a conversion sequence?

**Status:** provisional
**Affects:** REF-P3, ICS-P4, REF-6, REF-11, REF-14, ICS-15 to ICS-20, ICS-24, ICS-38, ICS-46, LIST-6, ILC-3
**Waits for:** child questions (Q-0165)

## Context

Fourteen questions ask how a braced list behaves for a reference, an
array, or a parameter. Each child question below keeps its own context,
evidence, and compiler split. This parent question names the model that
their provisional answers follow together.

The model has three outcomes for a braced list and a reference:

- **direct**: the reference binds to the list's one element, as after
  `=`,
- **temporary**: the list initializes a temporary of the referenced
  type,
- **none**: a declaration is rejected, and a parameter gets no
  conversion sequence.

The table of REF-P3 gives the outcome for each shape of list and each
referenced type. ICS-P4 gives the sequence for each outcome. Probes on
every compiler agree with the table where no child question records a
split, for example `{}` for `const int&`, `{.x = 1}` for an aggregate,
and several values for `const int&` or a function reference.

| Child | Point of the model | Provisional answer | Compilers that differ |
|---|---|---|---|
| Q-0164 | `{a}`, where `a` is an `E[N]` lvalue, for `E (&)[]` | direct | GCC makes a temporary |
| Q-0157 | `{x}`, where `x` converts to the referenced type through a conversion function | temporary | GCC binds through the conversion function |
| Q-0165 | `{f}` for a reference to a function | direct for a function of the referenced type or an overload set; none for a `noexcept` function and a type without `noexcept` | GCC rejects an overload set; Clang and GCC bind a `noexcept` function |
| Q-0166 | `{s}`, a string literal of another character array type | temporary | GCC rejects |
| Q-0149 | values for `E (&&)[]` | temporary of one element for each value, and the reference keeps its type; `{}` is none | Clang gives the reference the list's bound; GCC accepts `{}` |
| Q-0136 | `{}` and a designated list for a reference to an array | none | GCC and EDG form a sequence in part |
| Q-0150 | a temporary that needs an `explicit` constructor | none | Clang accepts braces without `=` |
| Q-0155 | `{s}` for a parameter of a character array reference | the identity exactly when `s` can initialize the array | each compiler on one case |
| Q-0167 | a direct binding against a temporary from one string literal | a tie | GCC chooses the direct binding |
| Q-0188 | two direct bindings to `E[N]` and `E[]` | the known bound is better | Clang finds a tie |
| Q-0135 | `{1}` for `int` against a temporary `int[2]` | a tie | GCC prefers `int` |
| Q-0154 | `{d}` of a derived class for a class parameter by value | the element's sequence | EDG keeps only the rank |
| Q-0132 | a class parameter by value whose chosen constructor is `explicit` | a sequence that fails if chosen | Clang and EDG form no sequence |
| Q-0081 | `{a}` of the same class for an object with an initializer-list constructor | the initializer-list constructor | Clang 18 uses the copy constructor |

## Evidence

The tests that each child question names. The probes for the cells with
no child question compiled declarations with `-std=c++23
-pedantic-errors` (Clang, GCC) and `--c++23 --strict` (EDG), on Clang 18
and 22, GCC 14 and 16, and EDG 7.0.

## Options

1. The model of REF-P3: a braced element binds directly only if the
   element is already what the reference names, and every other list
   makes a temporary or has none.
2. A model that follows one compiler throughout. No compiler agrees with
   the other two on every child, and each compiler differs on several
   children.

## Provisional answer

The table of REF-P3: direct, temporary, or none; 14 child questions keep each split (option 1). The model gives one condition for a direct binding for every
referenced type. In each child except Q-0136, two of three
implementations agree with the model. Q-0136 follows the rules that
need elements for the bound and members for the designators. A change
to one child's answer changes the table of REF-P3.

Kept open on 2026-11-03 under decision 0054, because several children stay open, and Q-0165 changed for the `noexcept` function on the same day. The table of REF-P3 already gives none for that case.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on overloading, references, and list initialization (`reports/review/2026-11-03-gap-review-questions-overloading.md`) agrees with option 1 in the model's central condition: a braced element binds directly when the referenced type is related to the element's type, which matches the model for object types. The children Q-0136, Q-0154, Q-0155, and Q-0165 carry the exceptions. The review did not read Q-0081 and Q-0188.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
