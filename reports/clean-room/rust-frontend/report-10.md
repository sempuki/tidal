# Front end update: round 10, part 1 (the check side)

Date: 2026-11-03. Decisions 0029, 0035, 0051 (item FE).
Role: clean-room reader. Inputs: the files under `specification/`,
`documentation/`, `tests/`, `questions/`, `frontend/`, and `tools/`,
the previous reports in this directory, and
`reports/clean-room/roadmap-3/implementer.md` (items 2 and 5). g++ and
clang++ were run twice on short programs of my own: once to see whether
a constructor's `noexcept` counts the destructors of its members
(finding G87), and once to see which `delete` expressions draw the
VIRT-13 warning (finding G88). No crates were added.

Part 1 covers items 2 and 5 of the roadmap. A previous reader started
this part and was interrupted. Its changes (TR-12 parse failures,
user-defined literals in the lexer, DEP-20 type-only positions, TPL-42,
TARG-11, TARG-16) are kept, and this report covers them with the rest.
The interpreter items (1, 4, 7) are left for part 2.

## 1. Conformance

### Checks

`tidal-fe conform ../tests`. The baseline is `conformance.tsv` of round
9, with 5471 tests. The suite now has 5480. "Pass" counts a passed
check, so it includes "Differs from EDG only" and a `run` test whose
check passed, whatever its run gave.

| Run | Tests | Pass | False reject | Missed reject | Unsupported |
|---|---|---|---|---|---|
| round 9 baseline | 5471 | 4722 | 79 | 349 | 321 |
| start of this part (the interrupted reader's changes) | 5480 | 4868 | 79 | 344 | 189 |
| after, same 5471 tests | 5471 | 5012 | 62 | 246 | 151 |
| after, whole suite | 5480 | 5018 | 63 | 246 | 153 |

By top directory, tests whose check passes:

| Directory | Baseline | Start of this part | After |
|---|---|---|---|
| classes | 793/960 | 808/960 | 888/960 |
| declarations | 912/1023 | 918/1023 | 932/1023 |
| exceptions | 116/146 | 118/146 | 146/146 |
| expressions | 1089/1191 | 1099/1197 | 1107/1197 |
| foundations | 675/823 | 763/826 | 769/826 |
| statements | 212/224 | 214/224 | 215/224 |
| templates | 925/1104 | 948/1104 | 961/1104 |

`python3 -I tools/frontend_check.py` (without `--update`) reports:

```
frontend: 4783/5480 pass, 5018/5480 pass their check, 0 worse, 299 better, 9 new (3 not passing)
frontend: 2/2 interpreter-only expectations met
```

It exits 0. At the start of this part it reported 4635 and 4868.

### Run tests

| Run | Tests | Pass | Wrong result | Missed undefined behavior | Unexpected undefined behavior | Unsupported | Check did not accept |
|---|---|---|---|---|---|---|---|
| start of this part | 1242 | 983 | 6 | 4 | 2 | 223 | 24 |
| after | 1242 | 986 | 6 | 4 | 2 | 225 | 19 |

The interpreter did not change in this part except for designated
lists (section 2). Five more run tests now reach the interpreter. Two
of them stop at a construct it does not model.

### Parse failures (item 2)

- 144 tests that expect a rejection now get a TR-12 error. 92 of them
  cite KW-1.
- No test that expects acceptance or a run fails with TR-12 as its first
  diagnostic. At the start of this part two did
  (`class-template-deduction/dependent-qualifier.cpp` and
  `dependent-qualifier-template-keyword.cpp`). Both were a parser gap:
  `>>` at the end of a template argument list inside a skipped
  `typename X::template Y<...>` closed both lists. The front end now
  splits the token.
- Two accept tests stopped at a parse as unsupported. Both were parser
  gaps in the current instantiation (DEP-15): a partial specialization
  that names itself with its own argument list, and an alias template
  for the class template. Both now run.
- One accept test still stops at a parse as unsupported:
  `dependent-names/current-instantiation-enclosing-base.cpp` (DEP-11).
  The parse stop is inside a member class of a class template defined
  outside it, where the front end does not search the enclosing class.
- Five accept tests show a TR-12 error after an earlier error of their
  own. The TR-12 error marks a parser gap in each:
  - init-capture packs `[...y = x]` (`packs/init-capture-pack.cpp`,
    `packs/size-operator.cpp`),
  - a lambda in the pattern of an expansion (`packs/lambda-in-pattern.cpp`),
  - a pack expansion in mem-initializers for pack bases
    (`packs/base-list-and-initializers.cpp`),
  - an alias template as a template template argument in a deduction
    site (`class-template-deduction/template-template-parameter-alias.cpp`).

## 2. What the front end models now

### Parse failures

A token sequence that the parser does not derive is a TR-12 error. The
parse stays unsupported for constructs the front end knows it does not
model (extensions, `asm`, named escapes, case ranges, compound literals)
and inside a class template or a scope where an earlier name was
unknown. In those places the stop can come from the front end, so a
TR-12 error there would be a guess.

### The `noexcept` operator and EXS-4

- Each function gets a "potentially throwing" answer: `noexcept` as
  written, then the deallocation functions (non-throwing), destructors
  (DTOR-4: the destructors of the bases and members decide, also for a
  destructor the program writes), and defaulted or implicit special
  members, which follow the same member of each base and member. A
  default constructor also follows the default member initializers it
  uses. A move that the class does not declare uses the copy.
- A defaulted comparison follows the same operator of each base and
  member (EXS-4, Q-0095).
- In the operand: calls, constructor calls, implicit value and
  aggregate initialization of a class without constructors, the
  implicit copy and move assignment, `delete` (the destructor decides),
  an explicit destructor call (also on a temporary), the allocation
  functions called by name (`operator new` throws, `operator delete`
  does not), a placement `new` whose allocation functions of that arity
  are all non-throwing, and the destructor of each class temporary.
- Not modeled: an operand that names a member declared later in the
  class (the specifier is evaluated where it is written), and the
  inherited constructors of EXS-4.

### Designated initializers

A designated list of a class without bases keeps the value of each
designator. Each member that the list does not name gets a marker that
means "as if the list ended before it". The interpreter and constant
evaluation then initialize the members in order, with the default
member initializer or value initialization for a marked member. For a
union, the named member is the active one (UNI-4). Two designators in a
union's list are an error (UNI-8).

### Virtual functions

At the end of a class definition the front end finds, for each member
function, the virtual functions of the bases that it overrides (same
name, same parameter types, same cv- and ref-qualifiers). It checks:

- VIRT-2: `virtual` outside the class definition, on a friend, on a
  member of a union, or on a data member.
- VIRT-3: `override` that overrides nothing, `final` on a function that
  is not virtual, an override of a `final` function, and `override` or
  `final` outside the class.
- VIRT-4 and VIRT-15: the return type of an override is the same or
  covariant. A covariant class must be complete (or be the class of the
  override), have the overridden class as an unambiguous base, add no
  cv-qualifier, and have the base accessible in the class of the
  override.
- VIRT-9: `= 0` on a friend.
- VIRT-14: an override and the overridden function agree on `consteval`.
- VIRT-19: an override and the overridden function agree on being
  deleted. An explicit object member function with the name and
  parameters of a base virtual function is an error.
- VIRT-21: a static member function with the name and parameters of a
  base virtual function.
- VIRT-13 under `-Wall`: `delete` of a single object through a pointer
  to a polymorphic class that is not `final` and whose destructor is not
  virtual.

### Exceptions

- TRY-3: a handler after `catch (...)`.
- TRY-4: a handler that an earlier handler of the same try block always
  takes (same type, a base class, a pointer to a base, `void*`).
- TRY-5: the handler parameter checks (storage class, `auto`, qualified
  name, `void`, rvalue reference, reference to an array of unknown
  bound, incomplete class through any form, abstract class by value).
- TRY-7: a `goto`, `case`, or `default` that enters a try block or a
  handler. Each try block and handler adds a marker to the jump sets
  that JMP-6 and SW-5 already use.
- TRY-12: a handler of a function try block that can reach its end, in
  a function that returns a value.
- TRY-13: a handler of a constructor's function try block that names a
  non-static data member.
- TRY-17: a `return` in a handler of a constructor's function try block.
- THR-3: a `throw` operand of incomplete or abstract class, a class
  whose destructor is deleted or not accessible, or an lvalue or xvalue
  whose copy or move constructor is deleted or not accessible.
- UNW-8: a `throw` in a function declared `noexcept` that no enclosing
  handler of the function catches, and a rethrow outside a `catch
  (...)` (TRY-8 keeps a handler out of its own try block).

### Classes

- CVF-6: a warning for a conversion function to the class, a base, or
  `void`. CVF-7: a return type before `operator`, parameters, `static`,
  a non-member, or a target of function or array type. CVF-11:
  copy-initialization from a class whose conversion functions are all
  `explicit`, and a condition from such a class when none converts to
  `bool`.
- DCMP-9 and DCMP-11: a use of a defaulted comparison that DCMP-2 or
  DCMP-5 deletes. Deletion is decided from a union, an anonymous union
  member, a reference member, a written return type of `<=>` that is not
  a comparison category, or an error while the body is built.
- DER-1: the same direct base twice, and a `final` base. DER-9: a
  using-declaration in a class that names a class that is not a base.
- CLS-14: an explicit object parameter that is not first, or on a
  function that is not a member. CLS-28: a static data member, a friend
  function definition, and the use of an automatic variable of the
  enclosing function in a member function of a local class.
- UNI-8: two mem-initializers or two default member initializers for
  variant members. UNI-9: an anonymous union that declares functions,
  static members, or types, or that has a member outside public access.
- BF-2: a width that is a floating value or a variable that is not
  constant, and an unnamed bit-field with a cv-qualifier. BF-8: a width
  wider than the type, and a constant assignment that the width changes.
- OPR-14: a chain of `operator->` that returns a type that is neither a
  pointer nor a class, or that returns to a class already in the chain.
- PFX-11 and NAM-37: member access that finds a type member.
- ACC-2 and ACC-4: a qualified name of a private or protected static data
  member.

### Templates

- Pack expansions in braced lists give one element per element of the
  pack (PACK-13), so `{x..., 0}` and `{(n = n * 10 + D)...}` work, and
  the bound of `int a[] = {D...}` follows.
- `R (*... fp)()`: `...` inside a parenthesized declarator makes a pack
  (PACK-1). DEFARG-8 does not apply to a function parameter pack or to
  the parameters it expands to (finding G90).
- DED-12: a constant parameter declared with a placeholder pattern
  (`auto*`, `const auto*`) keeps the pattern. An argument matches only
  when its type has that form with no conversion. A partial
  specialization `template <int V> struct Kind<V>` of a primary with
  `auto V` is a new argument list (TSP-11).
- ICS-47: a template-id with explicit arguments as an argument names the
  specializations that match the parameter type. Dropping `noexcept`
  from the selected function ranks below the identity.
- Generic lambdas: the body is parsed as a pattern (LAM-15, SA-5), so a
  static assertion that needs a call waits for it.

### Other

- PFX-20: `(x.f)(1)` calls the member function. The error names PFX-20
  and PFX-14 (finding G85).
- STMT-4 no longer warns for `delete p;`, `new T;`, or an explicit
  destructor call.
- An attribute after a bound in a `new` type (ATTR-19).

## 3. What the front end does not model

### Item 2

- Parse stops inside class templates and after unknown names stay
  unsupported. A front end that follows the productions closely enough
  to call those stops TR-12 needs the current-instantiation lookup of
  DEP-11 for members defined outside their class.
- The parser gaps in section 1: init-capture packs, a lambda in a pack
  pattern, pack expansions in mem-initializers, and an alias template
  as a template template argument.

### Item 5

- False rejects: 63. 36 are in `templates/`. The largest groups are
  packs (alignment specifiers with packs, using-declarations with packs,
  init-capture packs, nondeduced elements before the end), member
  templates of explicit and partial specializations (TSP-9, TSP-21,
  TSP-22), and CTAD sites. Outside templates: pointer comparisons in
  constant evaluation (CMP-7, CMP-20, CMP-26), constexpr evaluation of
  defaulted comparisons (DCMP-1, DCMP-4), `consteval` and
  allocation-function cases, and IEEE and wide-unsigned constants
  (MUL-8, MUL-20).
- Missed rejects: 246. In `classes/` 46 remain; `exceptions/` has none.
  The classes groups:
  - access through friends (ACC-2, ACC-3, ACC-6). The front end treats
    a class that declares any friend as open to everyone, because it
    does not track friends one by one.
  - inherited constructors (DER-20, DER-11 to DER-15).
  - the matching `!=` of DCMP-12 and DCMP-13 through bases, using
    declarations, and ref-qualifiers.
  - the narrowing of a union's first member (UNI-8), and a few single
    rules (VIRT-5, VIRT-8, VIRT-12, ILC-4 to ILC-9).
- The `same` idiom with arguments the front end cannot compute: 32
  tests, each a different construct (generic lambda calls, CTAD at
  several sites, member pointers in `?:`, rewritten comparisons).
- `noexcept`: an operand that names a later member (complete-class
  context, 3 tests), and inherited constructors (1 test).
- Generic lambdas: the call operator is not instantiated, so a call of
  a generic lambda stays unknown.
- `new` in constant evaluation (13 tests, including 4 of UNI-14).
- The rules after round 9 that still fail: SB-17 (structured bindings
  with `get`, 2 tests), INST-36 (a run test), UNI-14 (`new` in constant
  evaluation), and ICS-47 for an overload set passed to `...`.

## 4. Findings

### G85. PFX-14 and PFX-20 cover the same use

- Page and rules: expressions/postfix, PFX-14, PFX-20.
- Kind: overlap of two `reject` rules.
- What happened: the first item of PFX-14 forbids using a non-static
  member function reached through member access for anything other than
  a call. PFX-20 says the same thing, and adds the parenthesized call
  and the static case. `postfix/member-function-not-value.cpp` cites
  PFX-14, and `postfix/member-function-not-called.cpp` and
  `member-function-statement.cpp` cite PFX-20, for the same error. The
  front end names both rules in one diagnostic.
- Suggestion: remove the first item of PFX-14 and cite PFX-20 in
  `member-function-not-value.cpp`, or have PFX-14 refer to PFX-20.

### G86. EXS-6 is `no-diagnostic`, and its test expects a rejection

- Page and rules: declarations/exception-specifications, EXS-6, Q-0079.
- Kind: contradictory.
- What happened: EXS-6 says an implementation need not diagnose an
  operand of a `noexcept` specifier that depends on the specifier
  itself. `exception-specifications/operand-cycle.cpp` expects a
  rejection, with GCC and EDG overrides that accept. A front end that
  follows the text can accept, and the harness then counts a missed
  rejection against the base expectation. The front end gives
  unsupported, because it evaluates the operand where it is written and
  cannot see the later member.
- Suggestion: under decision 0034 a `no-diagnostic` rule allows either
  answer. The test could record that, or the rule could become `reject`
  with Q-0079 recording the GCC and EDG behavior.

### G87. Which functions a defaulted constructor calls, for EXS-4

- Page and rules: declarations/exception-specifications, EXS-4; DTOR-4.
- Kind: ambiguous.
- What happened: EXS-4 makes an implicit or defaulted special member
  non-throwing when each function it calls for bases and members is
  non-throwing. A constructor also runs the destructors of the members
  it has built when a later member throws. The text does not say
  whether those destructors count. I compiled a program where a member's
  destructor is `noexcept(false)` and its constructor is `noexcept`, and
  asked `noexcept` of a placement `new` of the enclosing class. g++ and
  clang++ both give `true`, so they count the member constructors only.
  The front end does the same.
- Suggestion: say that a constructor follows the constructors (and
  default member initializers) of its bases and members, and that the
  destructors it would run on a throw do not count.

### G88. VIRT-13 and a `final` class

- Page and rules: classes/virtual, VIRT-13; the warning-flag table of
  foundations/translation.
- Kind: missing.
- What happened: VIRT-13 asks for a warning on `delete` through a
  pointer to a polymorphic class whose destructor is not virtual. With
  `-Wall`, g++ and clang++ warn for such a class when it is abstract,
  and give no warning when it is `final`. A `final` class cannot be a
  base, so the deleted object has exactly that type and the behavior is
  defined. The flag table names a Clang flag for classes that are not
  abstract, and Clang uses a second flag for the abstract case. The
  front end warns for abstract classes and skips `final` ones.
- Suggestion: add "unless the class is `final`" to the first item, and
  list both Clang flags in the table.

### G89. TRY-5 lists forms that the handler production does not derive

- Page and rules: exceptions/handlers, TRY-5, the `exception_decl`
  production; foundations/translation, TR-12.
- Kind: overlap.
- What happened: `exception_decl` is type specifiers with an optional
  declarator. It does not derive a storage class or a default argument.
  TRY-5 still lists both, so TR-12 and TRY-5 each reject
  `catch (int e = 1)` and `catch (static int e)`. The front end reports
  TR-12 for the default argument, because its handler parse stops at
  `=`, and TRY-5 for the storage class, because its decl-specifier
  parse accepts `static` and checks it later.
- Suggestion: drop the storage class and default argument items from
  TRY-5 and let TR-12 cover them, or widen the production and keep the
  items. Either way one rule owns each form.

### G90. DEFARG-8 and a function parameter pack

- Page and rules: declarations/functions/default-arguments, DEFARG-8;
  templates/packs, PACK-1, PACK-4.
- Kind: contradictory.
- What happened: DEFARG-8 requires a default argument on each parameter
  after one that has a default argument. `packs/function-pack-forms.cpp`
  (PACK-1, accept, no compiler overrides) declares
  `int first = 1, T... rest` and calls it with no arguments. A function
  parameter pack cannot have a default argument (PACK-4), so by the
  text the declaration breaks DEFARG-8. The front end exempts a
  function parameter pack, and the parameters that a pack expands to in
  a specialization.
- Suggestion: add to DEFARG-8 that a function parameter pack can follow
  the parameters with default arguments.
