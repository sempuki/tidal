# Front end update: round 4

Date: 2026-10-31. Decisions 0029, 0035. Role: clean-room reader.
Inputs: the files under `spec/`, `docs/`, `tests/`, `questions/`, and
`frontend/`, and the previous reports in this directory. No crates were
added. The new code is name lookup and semantic checks, which decision
0035 keeps hand-written.

## 1. Conformance

`tidal-fe conform ../tests`. The suite grew from 3506 to 3548 tests
during the run (42 new tests, most of them in `foundations/odr`). The
row for the same 3506 tests isolates the change in the front end. The
pass column counts "Differs from EDG only" as a pass, as
`tools/frontend_check.py` does.

| Run | Tests | Pass | False reject | Missed reject | Unsupported |
|---|---|---|---|---|---|
| before | 3506 | 2873 | 12 | 372 | 249 |
| after, same 3506 tests | 3506 | 2953 | 0 | 306 | 247 |
| after, whole suite | 3548 | 2981 | 5 | 313 | 249 |

`python3 -I tools/frontend_check.py` reports 2981/3548 pass, 0 worse,
80 better, 42 new (14 not passing). The 5 false rejects in the whole
suite are new `foundations/odr` tests that use `#include`, which the
front end does not process.

Rejected tests whose diagnostic names one of the test's rules: 768
before, 815 after.

| Directory | Pass before | Pass after | Missed reject before | Missed reject after |
|---|---|---|---|---|
| foundations/names | 118 | 161 | 35 | 1 |
| classes/derived | 52 | 57 | 19 | 14 |
| classes/access | 26 | 29 | 19 | 16 |
| expressions/new-delete | 24 | 38 | 12 | 0 |
| declarations/namespaces | 34 | 37 | 7 | 4 |
| classes/constructors | 131 | 135 | 5 | 1 |
| declarations/attributes | 62 | 65 | 3 | 0 |

## 2. What changed in the front end

- Unqualified lookup (NAM-23 to NAM-26). The members of a namespace
  that a using directive names act as members of the innermost
  namespace around the directive and the named namespace. A local
  variable now hides them. Declarations from several places that name
  different entities, not all functions, make a use ambiguous.
- Qualified lookup (NAM-13, NAM-14). `N::x` searches `N` and its inline
  namespaces, and follows the using directives of `N` only when that
  finds nothing. Different entities at the ends of the paths are
  ambiguous.
- Qualifiers (NAM-15, NAM-16). The name before `::` finds only
  namespaces and types. A variable or a fundamental type before `::`
  is rejected.
- Member lookup tracks base subobjects (NAM-17, NAM-18, DER-4). A
  member found in two subobjects is ambiguous, unless every declaration
  found is a static member, a type, or an enumerator of one class. A
  declaration hides the same name in its virtual bases. The check runs
  for `C::x`, for `d.x`, for unqualified names in a class, and for type
  names.
- NAM-21: the parameters and the trailing return type after a
  qualified declarator name see the members of the named scope.
- The injected class name (NAM-34, NAM-36, ACC-14, ACC-18). `C::C`
  names the constructors outside a base clause and an elaborated type
  specifier. An unqualified name that is reachable only through a
  private base of a base is rejected.
- Friends (ACC-7, ACC-17, NAM-14). A friend function first declared in
  a class is hidden from ordinary and qualified lookup until a
  namespace-scope declaration.
- Argument-dependent lookup (NAM-27 to NAM-32) replaces the old
  approximation. The front end computes the associated namespaces and
  classes of each argument by the table of NAM-27, including pointers
  to members, function types, overload sets, closure types, local
  classes, and inline namespaces. The lookup finds namespace functions
  and the friends of associated classes. A call uses the lookup only
  when its name is unqualified, not in parentheses, and ordinary
  lookup finds nothing or only namespace-scope functions. When every
  argument type is modeled and the lookup finds nothing, the call is
  rejected.
- Repairs since round 3:
  - CPY-7: the destructor condition deletes only the copy and move
    constructors (G16).
  - CPY-7 and UNI-3: an rvalue of a union whose move UNI-3 deletes uses
    the copy function, unless the copy is also deleted (G17).
  - CTOR-24: a union, and a class with an anonymous union member, for
    a `const` object without an initializer (G15).
  - DTOR-3: a class prvalue in a `return` statement and in a `new`
    initializer (G22).
  - ATTR-6: a variable that is only assigned, a structured binding
    declaration, and a class variable (G18, G30).
  - NEW-19: a later declaration of a deallocation function is
    non-throwing like the first (G14).
  - The README now follows the warning-flag table of the translation
    page (G19).
- `new` expressions: NEW-7 (function type, incomplete class, abstract
  class, reference), NEW-8 (`delete` of `void*`), NEW-11 (`const`
  scalar without an initializer), NEW-16 (`new auto`), NEW-17 (absent
  or negative size, too many values), and NEW-20 (placement arguments
  that no declared function accepts).
- A functional cast whose class argument matches no constructor is
  rejected under PFX-16 (G28).
- Other fixes found on the way:
  - TYN-4: an elaborated type specifier in the outermost block of a
    function declares the class in that block.
  - ENUM-21: a value converted to an enumeration with a fixed
    underlying type no longer meets the ENUM-6 range check.
  - RET-3: a function whose deduced return type the front end cannot
    name is no longer reported as used before deduction.
  - LAM-19: a constant evaluation that calls a closure returned from a
    function, with reference captures of that function's variables, is
    rejected. The test `dangling-reference-capture-constant.cpp`
    passed in round 3 only through the wrong RET-3 report.

Not done: `adl-template-value-argument.cpp` needs class template
specializations (G29). The tests that need `new` or `delete` in
constant evaluation stay unsupported. The `#include` tests in
`foundations/odr` need a preprocessor.

## 3. Findings

### G26. Using directives inside a named namespace

- Page and rules: foundations/names, NAM-23, NAM-25.
- Kind: missing.
- What happened: NAM-13 says that qualified lookup follows the using
  directives of each namespace it reaches. NAM-23 gives the level where
  the members of a named namespace act for unqualified lookup. The page
  does not say what happens when the named namespace has its own using
  directive. The front end follows the chain and puts the members of
  each namespace on it at the innermost namespace around the first
  directive and that namespace. That choice is outside knowledge.
- Suggestion: state the level for a namespace reached through two
  directives, with a test where a declaration between the two levels
  hides one of them.

### G27. Unnamed namespaces in qualified lookup

- Page and rules: foundations/names, NAM-13; declarations/namespaces,
  NS-4.
- Kind: ambiguous.
- What happened: NAM-13 step 1 searches `N` and its inline namespaces.
  Step 2 searches the namespaces that using directives in `N` name. NS-4
  makes the members of an unnamed namespace visible in the enclosing
  scope, but the names page does not say which step covers them. The
  difference shows when `N` and an unnamed namespace in `N` both
  declare `x`. The front end treats the unnamed namespace as a using
  directive, so the declaration in `N` wins.
- Suggestion: name the step for an unnamed namespace in NAM-13, with a
  test.

### G28. No constructor accepts the argument

- Page and rules: classes/constructors; foundations/names, NAM-32.
- Kind: missing.
- What happened: in `adl-suppressed-by-type.cpp`, ordinary lookup finds
  a class, and the call becomes a functional cast with an argument that
  no constructor of the class takes. The constructors page has no reject
  rule for a set of arguments that matches no constructor. The front
  end reports PFX-16, which the expressions pages give for a function
  call with no viable function. The test cites only NAM-32.
- Suggestion: add a reject rule for an initialization whose arguments
  match no constructor, or say that PFX-16 covers constructors.

### G29. The template rows of NAM-27 and NAM-28

- Page and rules: foundations/names, NAM-27, NAM-28.
- Kind: untestable.
- What happened: NAM-27 has a row for a specialization of a class
  template, marked *templates referenced*. NAM-28 lists a template
  argument that is a value. `adl-template-value-argument.cpp` needs the
  front end to form the type `t::box<1>` and find that its argument
  adds nothing. The specification does not give the rules for
  specializations, so the front end cannot tell what the argument type
  is and does not reject the call.
- Suggestion: none for the names page. The front end will cover the
  test when a templates page exists.

### G30. Class variables in ATTR-6

- Page and rules: declarations/attributes, ATTR-6.
- Kind: ambiguous.
- What happened: ATTR-6 recommends a warning for "a variable of a class
  type without a user-provided constructor or destructor". The rule
  does not say whether a member or base with a user-provided
  constructor or destructor matters. Such a variable runs code when it
  is created or destroyed. The front end warns only when the class, its
  bases, and its members have no user-provided constructor or
  destructor. That choice is outside knowledge.
- Suggestion: say whether the condition covers bases and members, with
  a test of a class whose only member has a user-provided constructor.

### G31. Gaps in the warning-flag table

- Page and rules: foundations/translation, Evidence; ATTR-6, SEQ-7,
  STMT-4, COMMA-5.
- Kind: ambiguous.
- What happened: the table settles G19 for most rules. Two gaps remain.
  The ATTR-6 tests pass `-Wunused`, which the table does not list. For
  some rules the columns differ, for example SEQ-7 is on in Clang and
  needs `-Wall` in GCC. A test with `-Werror` and no `-W` flag then
  expects different results from the two compilers, and the page does
  not say which one a tool should follow. The front end accepts
  `-Wunused` for ATTR-6 and needs `-Wall` for SEQ-7.
- Suggestion: add `-Wunused` to the ATTR-6 row. Say which column the
  tests follow when the columns differ, or give those tests an override
  per compiler.

### G32. Overlapping reject rules on the names page

- Page and rules: NAM-24 and NAM-26; NAM-18 and DER-4; NAM-36, DER-4,
  and ACC-14; NAM-34 and CTOR-1; NAM-8, NAM-28, and NAM-30.
- Kind: needs example.
- What happened: each group has two or more reject rules for one
  program. Some examples follow.
  - A name that two directives bring to the level of a third
    declaration matches NAM-24 and NAM-26.
  - `&C::C` matches NAM-34 and CTOR-1.
  - A call that argument-dependent lookup cannot resolve matches NAM-8
    and the NAM-28 or NAM-30 case that caused it.

  The front end names the rule it can tell apart and the general one,
  for example `[NAM-18, DER-4]` and `[NAM-8, NAM-30]`. For a call that
  finds nothing, the front end cannot tell which NAM-28 or NAM-30 case
  applies, so it names NAM-30.
- Suggestion: extend the convention of G7 and G23 to this page. Say
  which rule a call that finds nothing breaks.

### G33. When a hidden friend becomes visible

- Page and rules: classes/access, ACC-7, ACC-17; foundations/names,
  NAM-14.
- Kind: needs example.
- What happened: ACC-7 says only argument-dependent lookup finds a
  friend first declared in a class, "unless the namespace also declares
  the function". The rule does not say whether a namespace declaration
  after the use counts, or whether a declaration in a block counts. The
  front end makes the friend visible from a namespace-scope declaration
  onward, and a block-scope declaration does not change it.
- Suggestion: add a test with the namespace declaration after the class
  and before the use, and one with the declaration after the use.

### G34. Unevaluated operands in DTOR-3

- Page and rules: classes/constructors, DTOR-3.
- Kind: ambiguous.
- What happened: DTOR-3 rejects a prvalue of a class whose destructor is
  deleted, "outside the operand of `decltype`". The rule does not say
  whether `sizeof(S{})` or another unevaluated operand is allowed. The
  front end checks a prvalue in an expression statement, a `return`
  statement, and a `new` initializer, and skips unevaluated operands.
- Suggestion: say whether `sizeof` and the other unevaluated operands
  are exceptions, with a test.

### G35. What "accepts them" means in NEW-20

- Page and rules: expressions/new-delete, NEW-20, NEW-10.
- Kind: needs example.
- What happened: NEW-20 asks for a declared allocation function that
  accepts the placement arguments. The page does not say whether the
  check uses the argument types or only their number. The front end
  checks only the number. `placement-form-undeclared.cpp` has no
  declared placement form at all, so the test does not decide the
  question.
- Suggestion: add a test with a placement form whose parameter type
  does not take the argument.
