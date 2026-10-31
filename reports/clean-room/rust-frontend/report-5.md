# Front end update: round 5

Date: 2026-10-31. Decisions 0029, 0035. Role: clean-room reader.
Inputs: the files under `spec/`, `docs/`, `tests/`, `questions/`, and
`frontend/`, and the previous reports in this directory. No crates were
added. The new code is a slice of preprocessing, the implicit conversion
sequences, and semantic checks, which decision 0035 keeps hand-written.

## 1. Conformance

`tidal-fe conform ../tests`. The suite grew from 3659 to 3750 tests
during the run (91 new tests in `foundations/overload-resolution`). The
row for the same 3659 tests isolates the change in the front end. The
pass column counts "Differs from EDG only" as a pass, as
`tools/frontend_check.py` does.

| Run | Tests | Pass | False reject | Missed reject | Unsupported |
|---|---|---|---|---|---|
| before | 3659 | 3028 | 17 | 329 | 285 |
| after, same 3659 tests | 3659 | 3118 | 0 | 295 | 246 |
| after, whole suite | 3750 | 3148 | 0 | 318 | 284 |

`python3 -I tools/frontend_check.py` reports 3148/3750 pass, 0 worse,
90 better, 91 new (61 not passing). The new tests belong to item M,
which has no page yet.

Rejected tests whose diagnostic names one of the test's rules: 828
before, 856 after.

| Directory | Pass before | Pass after | Missed reject before | Missed reject after |
|---|---|---|---|---|
| foundations/conversion-sequences | 23 | 72 | 5 | 0 |
| foundations/odr | 32 | 44 | 4 | 0 |
| classes/constructors | 136 | 147 | 12 | 1 |
| classes/conversion-functions | 35 | 38 | 21 | 18 |
| classes/access | 34 | 35 | 17 | 16 |
| classes/initializer-lists | 19 | 20 | 10 | 9 |
| declarations/attributes | 68 | 69 | 1 | 0 |
| expressions/new-delete | 40 | 41 | 1 | 0 |

The probe grids in `tests/foundations/conversion-sequences/grids/` give
1534 calls in pairs of overloads, plus 48 in the object-parameter grid.
A script outside the repository built each call and compared the front
end with the tables on the page. The front end agrees on 1124 cells
and disagrees on none. The other 400 cells, and 20 of the
object-parameter grid, stay unsupported. In all but 6 of them, both
rows of the pair reject, because the sequences tie or no function is
viable. In the other 6, the compilers disagree. G37 explains why the
front end does not reject those calls.

## 2. What changed in the front end

- `#include "file"` (INC-1 to INC-5). The lexer replaces the directive
  with the tokens of the file. The file is searched in the directory of
  the file that holds the directive (INC-2). A missing file is rejected
  under INC-3. `#ifndef`, `#define` of a name without a value, and
  `#endif` work as include guards, and a file that starts with
  `#pragma once` adds its text once. Other directives stay unsupported.
  The tokens of a header carry the line of the directive.
- ODR-1: a static data member defined twice outside its class.
- ODR-9 and ODR-11: a function with internal linkage that the unit calls
  and does not define is rejected under ODR-9 (G39).
- Implicit conversion sequences (ICS-1 to ICS-37), in a new file
  `src/ics.rs`:
  - standard sequences with their rank and the steps that the
    tie-breakers need,
  - reference bindings, with the cases of ICS-11 and ICS-12,
  - user-defined sequences through converting constructors and
    conversion functions, with ICS-5, ICS-9, ICS-10, and the ambiguous
    sequence of ICS-7,
  - the implicit object parameter (ICS-13) and the object slot of a
    static member function (ICS-14),
  - sequences from braced lists (ICS-15 to ICS-20), including
    `std::initializer_list<E>`, which the front end now models as a
    class with a known element type,
  - the comparison steps of ICS-P5, with ICS-37 for the choice between
    conversion functions.
- A call chooses its function by these sequences when the front end
  can form all of them (G37). A call where every sequence was formed and
  no candidate is viable is rejected under PFX-16. A chosen function
  that needs the ambiguous sequence is rejected under ICS-8 and CVF-12.
- An operator expression with a class operand chooses its operator
  function by the same comparison, when no class operand has a
  conversion function and no defaulted operator is a candidate. The
  left operand of `=` takes only a standard sequence (ICS-10), so a
  class with a conversion function can use its `operator=`. The comma
  operator uses a visible `operator,`.
- Conversion functions run in initializations, reference bindings,
  `static_cast`, and constant evaluation. A constant evaluation now
  returns a reference from a function that returns a reference to a
  variable.
- Repairs since round 4:
  - CTOR-25: an initialization whose arguments no constructor accepts,
    for direct initialization, braces, a functional cast, a
    mem-initializer, and `new` (G28).
  - DTOR-3: class prvalues in the operands of `sizeof`, `noexcept`, and
    `decltype`, except a call that is the operand of `decltype`, also as
    the right operand of a comma (G34).
  - NEW-20: each placement argument must convert to its parameter type
    (G35).
  - ACC-17: a qualified declaration outside the namespace of a friend
    that the namespace has not declared (G33).
  - ATTR-6: members that are arrays of classes count (G30).
  - REF-8: a non-`const` lvalue reference bound to a bit-field.
  - LIST-14: designators out of declaration order.
- Other fixes found on the way:
  - A conversion function to a pointer to member, as in
    `operator int Base::*()`, now parses.
  - An empty aggregate from `{}` with no members can be evaluated.
  - The new tests of item M showed five false rejects, which no longer
    occur:
    - A call of a class object whose class has a conversion function
      stays unknown, because a conversion to a function pointer adds a
      candidate that the front end does not model.
    - A subscript whose index is a class stays unknown.
    - The address of an overloaded member stays unknown.
    - A class conversion bound to a reference to a function stays
      unknown.
    - A member with a trailing requires-clause can overload a member
      with the same parameters.

Not done: `floating-equal-rank.cpp` needs the ranks of the extended
floating-point types (G42). `conversion-choice-nearest-source.cpp` and
`object-conversion-function-class.cpp` need constant evaluation of a
derived-to-base pointer conversion, a pointer-to-member constant, and a
temporary of a class with a base.

## 3. Findings

### G36. A braced list for a reference in REF-11

- Page and rules: declarations/initialization/references, REF-11;
  foundations/conversion-sequences, ICS-17, ICS-20.
- Kind: contradictory.
- What happened: REF-11 says that the initializer of a reference must
  hold exactly one expression. ICS-17 and ICS-20 give a sequence for a
  list of several elements to a reference to an array or to a class.
  `array-fewer-elements.cpp` passes `{1, 2}` to `int (&&)[2]`, and
  `user-defined-same-aggregate.cpp` passes `{1, 2}` to `const Point&`.
  The front end rejected both under REF-11. The front end now
  initializes a temporary of the referenced type when the type is an
  array, or a class that the single element does not already have.
- Suggestion: limit REF-11 to a list that binds the reference directly,
  and state that a list for a reference to an array or a class
  initializes a temporary.

### G37. Combining the sequences of several arguments

- Page and rules: foundations/conversion-sequences, ICS-P5, ICS-25,
  ICS-13, ICS-14.
- Kind: missing.
- What happened: the page compares two sequences for one argument.
  Many tests have two arguments and say "the second argument decides",
  for example `rank-class-value.cpp`, `reference-temporary.cpp`,
  `static-object-slot.cpp`, and `user-defined-different-functions.cpp`.
  The front end uses a rule from outside knowledge: one candidate is
  better than another when no argument has a worse sequence and one
  argument has a better one, and the call uses the candidate that is
  better than all others. The same rule chooses a constructor for
  ICS-19. When no candidate is best, no rule rejects the call, so the
  front end leaves the call unsupported. The grids have 400 such cells.
- Suggestion: item M can state the rule. Until then, one sentence in
  ICS-P5 would let the tests with two arguments stand on the page alone.
  Item M also needs a reject rule for a call where no candidate is best.

### G38. ICS-12 and an rvalue reference to an lvalue

- Page and rules: foundations/conversion-sequences, ICS-12, ICS-11.
- Kind: wrong.
- What happened: ICS-12 says that no sequence binds "an rvalue
  reference to an object type, to an lvalue". The references grid shows
  that a `short` lvalue binds `int&&` through a temporary and ties with
  `int`. The rule holds only when the lvalue has the referenced type or
  a type derived from it. The front end applies the rule only in that
  case.
- Suggestion: restrict the second bullet of ICS-12 to an lvalue whose
  type is related to the referenced type, and add the `short` case as an
  example.

### G39. ODR-11 in the warning-flag table

- Page and rules: foundations/odr, ODR-9, ODR-11; foundations/translation,
  Evidence.
- Kind: missing.
- What happened: ODR-11 asks for a warning, but the warning-flag table
  has no row for it. The ODR-11 tests pass `-Werror`.
  `internal-function-used-in-unused-inline.cpp` passes no flag and
  expects a reject, because `-pedantic-errors` makes the warning an
  error (ODR Evidence). The translation page does not say that
  `-pedantic-errors` turns some warnings into errors, or which ones.
  The front end rejects every case under ODR-9, which allows a
  diagnostic, and counts only calls as uses.
- Suggestion: add an ODR-11 row to the table. State in the convention
  which warnings `-pedantic-errors` makes errors, so a tool can tell
  ODR-11 apart from the other `recommended` rules.

### G40. The form of an include guard

- Page and rules: foundations/odr, Syntax, INC-4, INC-5.
- Kind: grammar gap.
- What happened: the `include_guard` production has "any text" between
  `#define` and `#endif`. The page does not say whether comments or
  blank lines can come before `#ifndef`, whether the text can hold
  another `#ifndef` and `#endif` pair, or what a `#define` with a value
  does. `#pragma once` has no production. The front end treats each
  `#ifndef` of a defined name as a skip to the matching `#endif`, skips
  comments before `#pragma once`, and leaves other directives
  unsupported.
- Suggestion: give `#pragma once` a production, and say that the guard
  is the first directive of the file and that nested conditional
  directives pair up.

### G41. A designated list for `std::initializer_list`

- Page and rules: foundations/conversion-sequences, ICS-16, ICS-18.
- Kind: missing.
- What happened: the lists grid passes `{.x = 1}` to
  `std::initializer_list<int>` against `Point`. The compilers choose
  `Point`, so the list has no sequence to the initializer list. ICS-16
  says only that each element converts to `E`. The front end gives a
  designated list no sequence to an initializer list, which is outside
  knowledge. Q-0136 covers the array case.
- Suggestion: say in ICS-16 that a designated list has no sequence to
  `std::initializer_list<E>`.

### G42. The ranks of extended floating-point types

- Page and rules: foundations/conversion-sequences, ICS-35; PRIM-22.
- Kind: untestable.
- What happened: ICS-35 depends on which floating-point types have
  equal ranks, and the page marks the extended types as referenced. The
  front end does not know the rank of the type of `1.0f64`, so
  `floating-equal-rank.cpp` stays unsupported.
- Suggestion: none for this page. The front end will cover the test
  when a page gives the ranks.

### G43. A chosen function with an ambiguous list sequence

- Page and rules: foundations/conversion-sequences, ICS-8, ICS-19.
- Kind: missing.
- What happened: ICS-8 names a specific rule for each fault. For the
  ambiguous sequence it names CVF-12, which covers conversion functions.
  ICS-19 also forms the ambiguous sequence when no constructor is best
  for a braced list. If the function with that sequence wins, no rule
  names the fault. The front end reports ICS-8 with CVF-12 in both
  cases.
- Suggestion: name a rule for a braced list that no constructor
  initializes unambiguously, for example CTOR-25, and add a `chosen-`
  test for it.

### G44. CTOR-25 and list initialization

- Page and rules: classes/constructors, CTOR-25; classes/initializer-lists,
  ILC-2.
- Kind: ambiguous.
- What happened: CTOR-25 says that a constructor accepts the arguments
  if their number fits and each argument converts. In list
  initialization, an initializer-list constructor takes the whole list
  as one argument, which CTOR-25 does not mention. The implicit copy
  and move constructors also accept one argument of the class. The
  front end skips the check when a class has an initializer-list
  constructor, and when the one argument is a class or a braced list.
- Suggestion: say in CTOR-25 that the arguments of list initialization
  are the elements, after ILC-2 has tried the initializer-list
  constructors, and that the implicit constructors count.

### G45. The left operand of a comma in `decltype`

- Page and rules: classes/constructors, DTOR-3.
- Kind: needs example.
- What happened: DTOR-3 exempts a call that is the operand of
  `decltype`, also as the right operand of a comma.
  `deleted-dtor-decltype-comma.cpp` tests `decltype(0, make())`. The
  rule does not say what happens to a class prvalue as the left operand,
  as in `decltype(make(), 0)`. The front end rejects it, because only
  the right operand is exempt.
- Suggestion: add a test with the prvalue on the left of the comma.
