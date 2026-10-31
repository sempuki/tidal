# Front end update: round 3

Date: 2026-10-31. Decisions 0029, 0035. Role: clean-room reader.
Inputs: the files under `spec/`, `docs/`, `tests/`, `questions/`, and
`frontend/`, and the previous reports in this directory. No crates were
added. The new code is semantic checks, which decision 0035 keeps
hand-written.

## 1. Conformance

`tidal-fe conform ../tests`. The suite grew from 3218 to 3332 tests
during the run (96 new tests in `foundations/keywords`). The row for
the same 3218 tests isolates the change in the front end.

| Run | Tests | Pass | False reject | Missed reject | Unsupported | Differs from EDG only |
|---|---|---|---|---|---|---|
| before | 3218 | 2572 | 3 | 443 | 167 | 33 |
| after, same 3218 tests | 3218 | 2696 | 0 | 323 | 166 | 33 |
| after, whole suite | 3332 | 2723 | 0 | 328 | 248 | 33 |

Rejected tests whose diagnostic names one of the test's rules: 663
before, 765 after.

| Directory | Pass before | Pass after | Missed reject before | Missed reject after |
|---|---|---|---|---|
| classes/constructors | 91 | 126 | 34 | 1 |
| classes/copy-move | 58 | 96 | 39 | 1 |
| declarations/attributes | 46 | 56 | 8 | 0 |
| expressions/conversions | 53 | 58 | 5 | 0 |
| expressions/postfix | 38 | 44 | 6 | 0 |
| expressions/casts | 42 | 44 | 2 | 0 |

## 2. What changed in the front end

- The false reject. A call of `::operator new[](size)` was checked
  only against the program's own placement `operator new[]`. A call of
  a global allocation or deallocation function now has an unknown
  callee (G14).
- Two other false rejects came from the rule changes. JMP-6 now uses
  CTOR-23 and DTOR-9, so a class with a defaulted default constructor
  can be skipped. ATTR-2 now checks that the next statement that can
  run after `fallthrough` has a label, so a `fallthrough` can end an
  `if` or `else` branch.
- TYP-1 to TYP-13: the sizes, alignments, and ranks already matched the
  tables, and every `foundations/types` test passes. `--m32` follows
  the `-m32` column with the GCC and Clang alignments.
- CTOR-23, DTOR-9, CPY-27: triviality. The "first declaration has
  `= default`" condition is now tracked. JMP-6, SW-5 and UNI-3 use it,
  and UNI-11 rejects a use of a union special member that UNI-3
  deletes: default construction, destruction of a variable, and copy
  or move by initialization, argument, or assignment. This also covers
  anonymous union members.
- A copy and move model following CPY-4 to CPY-7, CPY-14 and CPY-1 to
  CPY-3. It drives CPY-20 to CPY-24 and CPY-26 at initializations,
  arguments, returns, and assignments, plus CPY-15 and CPY-17 at
  declarations, CPY-22 hiding, and the CPY-13 warnings.
- Constructors: CTOR-1 and DTOR-8 (address, missing call, wrong type
  after `~`), CTOR-4, CTOR-8, CTOR-15, CTOR-16, CTOR-19, CTOR-20 for
  `new`, CTOR-9 and CTOR-22 in implicit conversions, DTOR-3 (variable,
  temporary, `delete`), CTOR-13 warning. Member access is now tracked
  for special members, so a member or base whose special member is not
  accessible deletes the implicit one (CTOR-3, CPY-7, DTOR-3). CAST-2
  checks the base on a downcast.
- RET-3 now applies to member functions. Member bodies are checked in
  declaration order.
- PFX-17 replaces PFX-7, and a call of a class object with no
  `operator()` and no conversion function is rejected. PFX-11 covers
  `->` on a class without `operator->`. PFX-14 covers an uncalled
  member function and incomplete return or parameter types.
- PRIM-20 with an explicit object parameter. CONV-10 for pointers to
  member functions. CONV-17 for ambiguous or inaccessible bases and for
  pointer-to-member conversions.
- CE-25 replaces CE-11 in diagnostics. CAST-19 replaces CAST-1 and
  CAST-9. CAST-5 writes to `const` objects during constant evaluation.
- ATTR-2 rejects arguments on `fallthrough`, `likely`, `unlikely`,
  `maybe_unused`, and `no_unique_address`.
- Recommended warnings that a `-W` flag enables: ATTR-5 for a
  `nodiscard` constructor, ATTR-6, ATTR-7, SEQ-7, CTOR-13, CPY-13.

Not done: INIT-9 tests that need base classes or `new` in constant
evaluation stay unsupported. CTOR-14 (needs ranking `S('a')` between
`S(int)` and `S(char)`), CE-19 for template arguments, and the
pessimizing-move test (it calls a template) stay missed rejects.

## 3. Findings

### G14. The global allocation functions

- Page and rules: expressions/new-delete, NEW-10, NEW-12, NEW-13.
- Kind: missing.
- What happened: `new-placement-and-bound-order.cpp` declares
  `operator new[](size, int)` and calls `::operator new[](size)`. The
  page does not say that the global allocation and deallocation
  functions exist in every program, or that a program's own
  declarations add to them. The front end found only the program's
  function and rejected the call. It now treats these calls as calls
  of an unknown function.
- Suggestion: add a rule that names the global allocation and
  deallocation functions every program has, and say that they can be
  called by name.

### G15. CTOR-4 and unions

- Page and rules: classes/constructors, CTOR-4.
- Kind: ambiguous.
- What happened: CTOR-4 asks that each data member has a default member
  initializer. For a union, only one member can have a value. The text
  does not say whether one initialized variant member is enough. The
  front end does not apply CTOR-4 to unions.
- Suggestion: state the condition for a union and for a class with an
  anonymous union member, with a test.

### G16. The destructor condition of CPY-7

- Page and rules: classes/copy-move, CPY-7.
- Kind: outside knowledge.
- What happened: CPY-7 deletes any implicit copy or move function when a
  base or member has a deleted or inaccessible destructor. As written,
  this includes the assignment operators. From outside the
  specification, the reader knows the condition only for the
  constructors. The front end follows the page.
- Suggestion: say which functions the destructor condition deletes, with
  a test of an assignment.

### G17. A UNI-3 deleted move and overload resolution

- Page and rules: classes/unions, UNI-3, UNI-11; classes/copy-move,
  CPY-7.
- Kind: ambiguous.
- What happened: CPY-7 says that a defaulted move function that is
  deleted takes no part in overload resolution, so an rvalue uses the
  copy function. UNI-3 deletes the move functions of a union whose
  variant member has a non-trivial move. The text does not say whether
  that deleted move takes part. The front end rejects the move under
  UNI-11, as `trivial-move-user-provided.cpp` expects. In that test the
  copy is also deleted, so the test does not decide the question.
- Suggestion: say whether a move that UNI-3 deletes takes part, with a
  test where the union's copy is usable.

### G18. Which unused variables ATTR-6 covers

- Page and rules: declarations/attributes, ATTR-6.
- Kind: ambiguous.
- What happened: ATTR-6 recommends a warning for "a local variable that
  the program never uses". It does not say whether this covers class
  objects with constructors or destructors, condition variables,
  range-for variables, or structured bindings. The front end warns only
  for local scalar variables declared in a block. This avoids false
  rejects in the 37 tests that use `-Wall -Werror`.
- Suggestion: list the kinds of variables the warning covers, with a
  test of a class object that is never named.

### G19. Which flag enables each recommended warning

- Page and rules: ATTR-6, ATTR-7, CTOR-13, CPY-13, SEQ-7; foundations/translation.
- Kind: outside knowledge.
- What happened: the tests for these rules pass `-Wall`, `-Wextra`,
  `-Wunused`, or `-Wimplicit-fallthrough` with `-Werror`. Other tests
  use `-Werror` alone and expect no warning. No page says which flag
  turns on which warning. The front end now receives the `-W` flags.
  It uses outside knowledge for the mapping: `-Wextra` enables the
  fallthrough and deprecated-copy warnings, and `-Wall` enables the
  order, sequencing, unused-variable, and pessimizing-move warnings.
- Suggestion: add a table to the translation page that gives the flag
  for each `recommended` rule on the matrix compilers.

### G20. What counts as falling through for ATTR-7

- Page and rules: declarations/attributes, ATTR-7.
- Kind: needs example.
- What happened: ATTR-7 says an implementation can warn when the
  attribute is missing. It does not say what counts as control reaching
  the next label. The front end does not warn after an empty label
  (`case 0: case 1:`), after `break`, `return`, `continue`, `goto`,
  `throw`, a `noreturn` call, or an `if` whose two branches all leave.
- Suggestion: add an accept test with consecutive labels and one with
  an `if` and `else` that both `break`.

### G21. A cast to an rvalue in CPY-13

- Page and rules: classes/copy-move, CPY-13.
- Kind: ambiguous.
- What happened: CPY-13 recommends a warning for "a cast to an rvalue in
  a `return` statement". `pessimizing-move-warns.cpp` writes the cast
  as a call of a function template named `move`. The front end does not
  instantiate templates, so it cannot see the cast. From outside the
  specification, the reader knows that compilers warn for the call and
  not for a written `static_cast`.
- Suggestion: say which forms the warning covers, and add a test that
  does not need a template.

### G22. The temporary in DTOR-3

- Page and rules: classes/constructors, DTOR-3.
- Kind: ambiguous.
- What happened: DTOR-3 rejects creating a temporary of a class whose
  destructor is deleted. The page does not say which class prvalues
  create a temporary. The front end rejects only a class prvalue that is
  a whole expression statement. It does not reject `S s = S{};` or
  `return S{};`.
- Suggestion: list where a prvalue initializes an object directly
  without a temporary, with a test of `return S{};`.

### G23. Overlapping reject rules

- Page and rules: CAST-2 and CAST-19; CPY-21, CPY-23, CPY-24 and JMP-15;
  CTOR-9 and PFX-16; DCL-19, SW-3 and CE-25.
- Kind: needs example.
- What happened: each group has two or more reject rules for one
  program. For example, `static_cast<long*>(int_ptr)` matches CAST-2,
  which lists unrelated pointers, and CAST-19. Tests cite one rule each,
  and not the same one:
  - `move-ctor-deletes-copy.cpp` cites CPY-21.
  - `lvalue-move-only-rejected.cpp` cites CPY-24.
  - `return-static-local-move-only.cpp` cites JMP-15.

  All three reject an lvalue copy of a class whose move constructor
  deletes its copy. The front end names the specific rule and the
  general one, as in `[CPY-21, CPY-24]`.
- Suggestion: extend the convention suggested in G7. A test cites
  every reject rule that its program breaks, or the general rule points
  to the specific one.

### G24. Keyword tests without rule text

- Page and rules: KW-1.
- Kind: outside the spec.
- What happened: during the run, 96 tests appeared in
  `tests/foundations/keywords/`. They cite KW-1, and `spec/` has no
  page with that rule. Most use a keyword as a name. The front end
  reports 79 of them as unsupported parse errors.
- Suggestion: none for the text. The front end will add the rule when
  the page exists.

### G25. CTOR-14 needs overload ranking

- Page and rules: classes/constructors, CTOR-14.
- Kind: outside the spec.
- What happened: `delegating-cycle.cpp` forms a cycle only if `S('a')`
  selects `S(char)` over `S(int)`. Choosing between them needs the
  ranks of overload resolution, which the specification does not give.
  The front end detects a constructor that delegates to itself (CTOR-8)
  but not this cycle.
- Suggestion: add a cycle test whose constructors differ in the number
  of parameters.
