# Front end update: round 6

Date: 2026-10-31. Decisions 0029, 0035. Role: clean-room reader.
Inputs: the files under `spec/`, `docs/`, `tests/`, `questions/`, and
`frontend/`, and the previous reports in this directory. No crates were
added. The new code is overload resolution, a slice of preprocessing,
and semantic checks, which decision 0035 keeps hand-written.

## 1. Conformance

`tidal-fe conform ../tests`. The suite grew from 3815 to 4042 tests
during the run: 142 tests in a new `templates/` directory, 37 in
`foundations/overload-resolution`, and 48 elsewhere. The row for the
same 3815 tests isolates the change in the front end. The pass column
counts "Differs from EDG only" as a pass, as `tools/frontend_check.py`
does.

| Run | Tests | Pass | False reject | Missed reject | Unsupported |
|---|---|---|---|---|---|
| before | 3815 | 3181 | 8 | 330 | 296 |
| after, same 3815 tests | 3815 | 3271 | 0 | 269 | 275 |
| after, whole suite | 4042 | 3345 | 4 | 339 | 354 |

`python3 -I tools/frontend_check.py` reports 3345/4042 pass, 0 worse,
89 better, 228 new (153 not passing). The four false rejects are new
tests. Three are in `templates/` and use alias templates and template
parameters of reference type, which the front end does not model. The
fourth, `attribute-positions-base-and-new-bound.cpp`, has an attribute
in a `new` bound that the parser reads as a lambda capture.

Rejected tests whose diagnostic names one of the test's rules: 861
before, 922 after.

| Directory | Pass before | Pass after | Missed reject before | Missed reject after |
|---|---|---|---|---|
| foundations/overload-resolution | 31 | 77 | 26 | 0 |
| foundations/conversion-sequences | 90 | 98 | 2 | 0 |
| foundations/odr | 47 | 50 | 0 | 0 |
| classes/constructors | 149 | 155 | 7 | 1 |
| classes/access | 35 | 41 | 16 | 10 |
| classes/comparisons | 41 | 46 | 16 | 11 |
| classes/operators | 72 | 76 | 13 | 9 |
| classes/conversion-functions | 38 | 41 | 18 | 15 |
| classes/initializer-lists | 20 | 23 | 9 | 6 |
| expressions/comparison | 48 | 50 | 11 | 9 |
| expressions/decltype | 44 | 46 | 4 | 2 |

The table counts the tests that existed at the start. With its 37 new
tests, `foundations/overload-resolution` has 101 of 137 tests passing
and 3 missed rejects.

A script outside the repository built each cell of the probe grids and
compared the front end with the tables on the two pages:

| Grids | Agree | Disagree | Unsupported | Compilers split |
|---|---|---|---|---|
| overload resolution, 12 grids | 448 | 25 | 307 | 22 |
| conversion sequences, 11 grids | 1750 | 0 | 46 | 44 |

In all 25 disagreements the table rejects and the front end accepts.
They involve templates (15), the narrowing check of the built-in `<=>`
(6), an explicit object member called with no object (2), an explicit
constructor in copy-list-initialization (1), and a parameter pack (1).
No cell is a false reject. The unsupported cells are mostly templates
and calls on objects whose conversion functions name an alias template.

## 2. What changed in the front end

- Overload resolution for calls (OVL-1 to OVL-4, OVL-22 to OVL-25,
  OVL-34 to OVL-37), in `src/ics.rs`:
  - The best viable function follows OVL-25 and OVL-34. This rule
    replaces the outside knowledge of round 5 (G37).
  - A call with no best function is rejected under OVL-36. A call where
    every sequence was formed and no candidate is viable is rejected
    under OVL-35, or PFX-5 for the count. Round 5 used PFX-16 for this
    case, which is the rule for `explicit` constructors.
  - The chosen function is checked for deletion (DEL-1), for access
    (ACC-3, for a member of the object's own class), and for the faults
    of ICS-8: an ambiguous conversion (CVF-12), an ambiguous braced list
    (CTOR-25), and a deleted constructor or conversion function (DEL-1).
    A deleted function now forms a sequence.
  - OVL-2 and OVL-3: a call by name gets `*this` only if its class is
    the members' class or derived from it. A call with no object that
    chooses a member with an implicit object parameter is rejected
    under OVL-3. A default member initializer has an object.
  - OVL-21 and OVL-44: `(&f)(a)` resolves `f` with no implied object
    argument. The front end rejects the call when the set holds a
    non-static member and the operand of `&` is unqualified or in
    parentheses.
  - OVL-39: an argument that names an overload set has the identity
    sequence for a parameter whose type selects one of its functions.
  - A function that two using-declarations bring in twice with C
    linkage is one candidate (LSP-4). A derived member hides the base
    member with the same parameters that a using-declaration brings in
    (DER-3).
  - A candidate with a trailing requires-clause stops the resolution,
    because constraints are not modeled (OVL-24, OVL-30).
- Operator expressions (OVL-7 to OVL-13, OVL-32, OVL-33, OVL-43), in a
  new file `src/ovl.rs`:
  - the member, non-member, and argument-dependent candidates, with
    members skipped in ordinary lookup,
  - the built-in candidates of the OVL-P4 table for the arithmetic,
    bitwise, shift, comparison, logical, unary, increment, subscript,
    and compound assignment operators, with OVL-10,
  - the rewritten and swapped candidates of DCMP-6, the exclusion of an
    `==` whose scope declares a matching `!=`, and DCMP-12 for an `==`
    that returns a type other than `bool`,
  - OVL-11: after a built-in candidate wins, each class operand
    converts by its conversion function, and the operator's own page
    checks the result,
  - OVL-12: with no viable function, `,` and unary `&` keep their
    built-in meaning,
  - OVL-13 and OVL-43: a rewritten candidate evaluates as the table
    gives, and the comparison with `0` has no rewritten candidates,
  - an explicit object parameter as the object of an operator function
    (ICS-41).
  - An operator with no viable function is rejected under OVL-35, and
    an ambiguous one under OVL-36. Both name OPR-15.
- Initialization of a class (OVL-14, OVL-40, CTOR-25, ILC-2 to ILC-4,
  ILC-8):
  - The constructor is chosen by the sequences. Phase 1 of ILC-2 takes a
    single viable initializer-list constructor.
  - An ambiguous or impossible constructor call is rejected under
    CTOR-25. An explicit constructor chosen in copy-list-initialization
    is rejected under ILC-8. A narrowing element is rejected under
    ILC-4.
  - Copy-initialization from another class with no best constructor or
    conversion function is rejected under OVL-36, or OVL-35 when none
    applies. A scalar initialized from a class with two equally good
    conversion functions is rejected under ICS-8 and CVF-12.
  - Copy-initialization of a reference from a class with no
    non-explicit conversion is rejected under OVL-35 (OVL-15).
- Calls on a class object (OVL-5, OVL-6): the call operators and the
  surrogate candidates, with the cv-qualifier condition of Q-0144.
- The address of an overload set (OVL-18 to OVL-20): a target type
  selects one function, for a pointer, a reference, or a pointer to
  member. No match or several matches are rejected under OVL-20, also
  with no target in `auto` or `decltype`. `decltype(f)` for an overload
  set is rejected under DTY-4.
- The comparison order of ICS-P5 step 4: the same-rank tie-breakers
  (ICS-32 to ICS-36, and ICS-42 to ICS-44 for the third parts), then
  ICS-28, ICS-29, ICS-31, and ICS-30. ICS-37 also applies between two
  conversion functions to a class.
- Braced lists (ICS-20, ICS-38, REF-6, REF-14, REF-11):
  - A list with one element of the referenced type binds the reference
    directly. The cv-qualifiers of array elements do not count.
  - Other lists initialize a temporary, which a non-`const` lvalue
    reference cannot bind (REF-9). An empty list for an array of
    unknown bound is rejected under REF-11.
  - One string literal for a reference to an array of characters has
    the identity only if the literal fits and has the right character
    type. Its element count includes the null character, for ICS-24.
- Preprocessing (INC-4, INC-5): the conditional groups of the ODR page's
  grammar, `#define` with a value, object-like replacement, `#undef`,
  and `#pragma once` after other text.
- ODR-11 is named with ODR-9 in the error for an internal function that
  is used and not defined. The warning-flag table makes the warning an
  error under `-pedantic-errors`.
- Other fixes found on the way:
  - A pointer to an array converts to a pointer to an array of unknown
    bound, or with more cv-qualifiers on the elements. A reference to
    `const int[2]` binds an `int[2]` lvalue.
  - The built-in `<=>` gives `std::strong_ordering` or
    `std::partial_ordering` when the program declares them (CMP-12),
    and rejects a `bool` with another type (CMP-13).
  - Two function templates whose parameters depend on template
    parameters and whose return types differ are separate declarations
    (OVL-41).
  - A private or protected data member of the object's class is
    rejected outside the class under ACC-2 or ACC-4.
  - A class with a member declaration that failed to parse takes part
    in no resolution.
  - Several arguments in `[]` stay unknown, since a member `operator[]`
    can take them (OPR-3).

Not done:

- Templates as candidates: the front end gives up unless a plain
  function matches every argument with no conversion.
- Inherited constructors (OVL-17, OVL-31) and the deleted defaulted
  move functions of OVL-42.
- `explicit` conversion functions in direct initialization (ICS-45,
  OVL-40). The front end gives up on these calls.
- Constant evaluation of class temporaries with bases, of conversion
  functions in initializations, and of rewritten comparisons on class
  temporaries. Many new tests stay unsupported for this reason.

## 3. Findings

The repairs resolve round 5 findings G36, G37, G39, G40, G43, G44, and
G45. The new ICS-12 text resolves G38.

### G46. One function found twice

- Page and rules: foundations/overload-resolution, OVL-1;
  declarations/linkage-specifications, LSP-4; classes/derived, DER-3.
- Kind: missing.
- What happened: `default-from-two-scopes-passed.cpp` finds one C
  function through two using-declarations. `using-derived-wins.cpp`
  finds a derived member and the base member that a using-declaration
  brings in. OVL-1 says that the candidates are the functions that
  lookup finds. The front end at first counted each declaration as a
  candidate, so the two tied and OVL-36 rejected both programs. LSP-4
  and DER-3 settle each case on their own pages.
- Suggestion: say in OVL-1 that a function found through several
  declarations is one candidate, and point to DER-3 for the
  using-declaration case.

### G47. The object of a call with no object

- Page and rules: foundations/overload-resolution, OVL-3, OVL-23,
  OVL-25.
- Kind: missing.
- What happened: OVL-3 says that a call with no object still compares
  the members. OVL-23 counts the implied object argument, but such a
  call has none. `no-object-static-default-ambiguous.cpp` needs the
  non-static member's object parameter to tie with the static member's
  object slot. The front end gives the missing object the slot of
  ICS-14, which ties with every sequence. This is outside knowledge.
- Suggestion: state that a member's object parameter takes no part when
  the call has no object, as ICS-14 does for a static member.

### G48. The built-in `!`, `&&`, and `||` with an explicit conversion

- Page and rules: foundations/overload-resolution, OVL-P4, OVL-9;
  foundations/conversion-sequences, ICS-4; expressions/logical.
- Kind: ambiguous.
- What happened: the table gives these operators the candidates
  `(bool)` and `(bool, bool)`. Their operands convert to `bool`
  contextually, which can use an `explicit operator bool`. ICS-4 forms
  sequences only through conversion functions that are not `explicit`,
  so the candidate has no sequence for such an operand. The front end
  leaves these operators to their own page when no operator function is
  a candidate.
- Suggestion: say whether the built-in candidates of `!`, `&&`, and
  `||` take `explicit` conversion functions, and add a test with an
  operator function that competes with them.

### G49. One string literal for a reference to an array

- Page and rules: declarations/initialization/references, REF-6;
  foundations/conversion-sequences, ICS-38, ICS-24.
- Kind: contradictory.
- What happened: a string literal is an lvalue array. For
  `const char (&)[3]` and `{"ab"}`, the second bullet of REF-6 binds the
  reference to the literal. For `const char (&)[]`, the third bullet
  does. `list-string-literal-bound.cpp` needs both calls to form the
  list sequence of ICS-38, so that ICS-24 can prefer the known bound.
  With direct binding the two sequences tie. The front end treats one
  string literal as a list that initializes the array, in both the
  sequence and the initialization.
- Suggestion: exclude one string literal for an array of characters
  from REF-6, and point to ICS-38.

### G50. Array bounds in qualification conversions

- Page and rules: declarations/declarators, CV-7;
  foundations/conversion-sequences, ICS-31;
  declarations/initialization/references, REF-1.
- Kind: missing.
- What happened: `qualification-array-bound.cpp` converts `int (*)[3]`
  to `const int (*)[]` and to `const int (*)[3]`. CV-7 speaks only of
  cv-qualifiers on the pointed-to type. It does not say that `const` on
  the elements of an array counts, or that a pointer to an array of
  known bound converts to one of unknown bound. ICS-31 mentions the
  bound only to say that it does not count. The same gap stopped
  `const int (&r)[2] = a` for an `int[2]` lvalue, which
  `braced-list-array-binds-directly.cpp` needs. The front end allows
  both conversions, from outside knowledge.
- Suggestion: add both cases to CV-7, with an example, and say in REF-1
  that a reference to an array of `const T` binds an array of `T`.

### G51. A braced list and the ellipsis

- Page and rules: foundations/conversion-sequences, ICS-6, ICS-15.
- Kind: missing.
- What happened: ICS-6 gives an ellipsis sequence to any argument that
  matches `...`. The classes grid passes `{d}` to `f(T)` and `f(...)`
  with a `T` that `d` does not convert to. Every compiler rejects both
  rows, so the list has no sequence to the ellipsis and no function is
  viable. The front end makes a braced list match no `...`, from the
  grid.
- Suggestion: say in ICS-6 that a braced list has no ellipsis sequence,
  and add a test.

### G52. OVL-26 between a constructor and a conversion function

- Page and rules: foundations/overload-resolution, OVL-26, OVL-14;
  foundations/conversion-sequences, ICS-37, ICS-42.
- Kind: ambiguous.
- What happened: OVL-26 applies "in an initialization by a conversion
  function". `conversion-choice-class-value.cpp` needs ICS-42 to choose
  between two conversion functions to a class.
  `init-constructor-conversion-tie.cpp` needs a constructor and a
  conversion function to tie, although the constructor's result needs
  no conversion and the conversion function's result does. ICS-37 names
  two conversion functions, but OVL-26 does not. The front end applies
  the tie-breaker only between two conversion functions.
- Suggestion: say in OVL-26 that both functions are conversion
  functions.

### G53. A surrogate from a conversion function template

- Page and rules: foundations/overload-resolution, OVL-6.
- Kind: missing.
- What happened: `surrogate-template-not-candidate.cpp` expects that a
  conversion function template gives no surrogate. The four conditions
  of OVL-6 do not exclude templates. The test's note gives the reason,
  that nothing fixes the result type. The front end skips templates.
- Suggestion: add the condition to OVL-6.

### G54. The matching `!=` of DCMP-6

- Page and rules: classes/comparisons, DCMP-6, DCMP-12.
- Kind: ambiguous.
- What happened: DCMP-6 removes an `==` from the rewritten candidates
  if its scope declares a matching `!=`. The page does not say what
  matches: the parameter types, the cv-qualifiers and ref-qualifier of
  a member, or the return type. `ne-declared-blocks-reversed.cpp` has a
  member `!=` with the same parameter and qualifiers. The front end
  requires the same parameter types, qualifiers, and class, in the same
  scope. DCMP-12 also allows a cv-qualified `bool` for a reversed `==`
  (Q-0158), but says only "`bool`" for an `==` that rewrites `!=`. The
  front end requires `bool`, without a reference, in both cases.
- Suggestion: define a matching `!=`, and state the result type for
  `!=` in the words used for `==`.

### G55. Conditions and values in an included header

- Page and rules: foundations/odr, Syntax, INC-4.
- Kind: outside the spec.
- What happened: the grammar gives `if_line` and `guard_value` as "any
  tokens". `guard-nested-conditionals.cpp` uses `#if NAME > 2` and
  then the name as a value in code. `guard-define-with-value.cpp` uses
  the guard name in a `static_assert`. The page does not say how the
  condition is evaluated or that a name with a value is replaced. The
  front end evaluates integer expressions with `defined`, and replaces
  object-like names, from outside knowledge.
- Suggestion: none for the page, if preprocessing stays outside the
  spec. The two tests then depend on that outside knowledge, which the
  test notes could say.

### G56. The positions in OVL-8 for a swapped candidate

- Page and rules: foundations/overload-resolution, OVL-8, DCMP-6.
- Kind: needs example.
- What happened: with no class operand, a non-member is a candidate only
  if a parameter has the type of "the matching enumeration operand".
  For a swapped candidate, the first parameter meets the right operand.
  The page does not say whether the match uses the written order or the
  swapped one. The front end uses the swapped order for a swapped
  candidate.
- Suggestion: add a test with an enumeration operand and a swapped
  candidate whose parameters match only in the swapped order.
