# Front end update: items A, C, and D

Date: 2026-10-31. Decisions 0029, 0034, 0035. Role: clean-room reader.
Inputs: the files under `spec/`, `docs/`, `tests/`, `questions/`, and
`frontend/`, and the previous report in this directory. The front end
uses no crates (decision 0035 allows them, and no new code in this round
replaced work that a crate would do).

## 1. Conformance

`tidal-fe conform ../tests`. Expectations use `tidal-expect-edg` where
present. The suite grew from 3084 to 3127 tests during the run (43 new
tests in `foundations/types/`, all passing). The "after" row for the same
3084 tests isolates the change in the front end.

| Run | Tests | Pass | False reject | Missed reject | Unsupported | Differs from EDG only |
|---|---|---|---|---|---|---|
| before | 3084 | 2299 | 1 | 571 | 181 | 32 |
| after, same 3084 tests | 3084 | 2467 | 0 | 422 | 163 | 32 |
| after, whole suite | 3127 | 2510 | 0 | 422 | 163 | 32 |

Rejected tests whose diagnostic names one of the test's rules: 379 before,
657 after. A diagnostic can now name two rules (section 2.3); the count
accepts either.

Directories that changed most (same tests):

| Directory | Pass before | Pass after | Missed reject before | Missed reject after |
|---|---|---|---|---|
| expressions/constant-expressions | 38 | 50 | 6 | 1 |
| declarations/attributes | 21 | 45 | 27 | 3 |
| declarations/functions | 71 | 90 | 21 | 2 |
| classes/constructors | 51 | 87 | 61 | 26 |
| classes/virtual | 49 | 67 | 49 | 31 |
| classes/operators | 54 | 72 | 30 | 13 |
| classes/comparisons | 30 | 41 | 27 | 16 |
| classes/members | 65 | 76 | 17 | 7 |

## 2. What changed in the front end

### 2.1 The false reject (ENUM-10)

Inside the enumerator list, each enumerator now has the type that ENUM-10
gives it: the fixed underlying type, else the type of its initializer,
else the type of the enumerator before it if that type holds the value.
When it does not, the front end picks the first integer type of at least
the same rank that holds the value (ENUM-13 leaves the choice
unspecified). `enumerator-type-inside-list.cpp` passes.

### 2.2 CE-21 to CE-24

- The evaluator runs constructors. A mem-initializer is now stored with
  its arguments converted to the member's type. A constructor call is
  chosen only when every candidate of the right arity has scalar
  parameters and exactly one accepts the arguments; otherwise it stays
  unknown. The evaluator initializes members in declaration order from
  the mem-initializer, else the default member initializer, else by
  default initialization, then runs the body. It gives up on bases,
  delegating constructors, and classes whose destructor does something.
- CE-21: a constant whose value has a scalar part with no value is
  rejected. This includes `constexpr` and `constinit` variables with no
  initializer, which are now default-initialized and checked.
- CE-22: reading a whole class or array object (a copy or a move) with a
  part that has no value is rejected. A `return` of a local by value now
  copies the object, so this applies to it. Classes with a user-provided
  copy or move constructor are reported as unsupported.
- Union members: the evaluator tracks the active member. Reading another
  member is rejected under CE-6, and an assignment to a member makes it
  active (UNI-4 picks the member with a default member initializer).
- CE-23: a new literal-type test follows the list on the page (scalar,
  reference, array of literal, class with an implicit, defaulted, or
  `constexpr` destructor whose bases and members are literal and not
  `volatile`, with the union exception). CX-1 uses it, and constant
  evaluation that reaches a variable of a non-literal type is rejected
  under CE-20.
- CE-24: a constant initializer is the whole initialization, including
  constructor calls, so CINIT-1 now runs the constructor.

Remaining CE tests that do not pass: `literal-closure.cpp` (lambdas are
not modelled), three tests that bind a reference to a temporary, and
`literal-base.cpp` where the base class makes the constructor
unsupported (the CX-1 literal check still rejects it).

### 2.3 Reject rules in diagnostics

Every diagnostic that named a `required` rule now names its reject
partner on the same page (for example ENUM-4 to ENUM-17, INIT-4 to
INIT-7, NAM-1 to NAM-8, DCL-5 to DCL-19), 61 rules in all. Four required
rules remain because they have no reject partner: CAST-1, CAST-9, CE-11,
and PFX-7 (finding G8).

When a constant is needed and the evaluation fails, the error now names
the rule that needs the constant and the rule that the evaluation broke,
as in `error [CX-1, CE-22]`. Section 3, G7, explains why.

### 2.4 Missed rejects

New checks, each from the page's reject rule:

- Attributes: a parser for attribute specifiers and its argument checks
  (ATTR-2, ATTR-13, ATTR-17), placement of `fallthrough` and `assume`,
  `likely` with `unlikely`, `noreturn` and `nodiscard` on a variable,
  `no_unique_address` on a bit-field, `noreturn` and
  `carries_dependency` added on a later declaration, a false assumption
  in constant evaluation (ATTR-10), and the warnings of ATTR-4, ATTR-5,
  and ATTR-14.
- Default arguments: DEFARG-4, DEFARG-5, DEFARG-6, DEFARG-8, DEFARG-9.
  Default arguments are now converted to the parameter type.
- Deleted and defaulted functions: DEL-1, DEL-2, DEL-4.
- Return deduction: RET-3, RET-4, RET-5.
- Operator functions: OPR-2 (arity, member-only operators, class
  parameter, static, variadic, postfix `int`, operators that cannot be
  overloaded).
- Constructors and destructors: CTOR-1, DTOR-1, CTOR-9 (`explicit` in
  copy-initialization, `explicit(c)` with a constant, `explicit` on other
  functions), CTOR-20 (a default constructor that CTOR-3 deletes).
- Classes: CLS-9 (member declared twice, `mutable` static, static member
  named as the class or in an unnamed class, nested class named as the
  class, unnamed member of class type, parenthesized default member
  initializer, `= 0` on a function that is not virtual).
- Abstract classes: VIRT-8 for variables, arrays, members, parameters
  and return values of definitions, and calls.
- Defaulted comparisons: DCMP-3.
- `dynamic_cast`: VIRT-18.

## 3. Findings

### G1. The type of an enumerator inside its list

- Page and rules: declarations/enumerations, ENUM-10, ENUM-13.
- Kind: ambiguous.
- What happened: ENUM-10 says that an enumerator with an initializer has
  "the type of the initializer". The front end took the type of the
  expression without cv-qualifiers and without promotion, so `p = 'a'`
  gives `char`. The text does not say whether promotion applies, or what
  type results when the initializer has an enumeration type (an
  enumerator of an earlier, complete enumeration).
- Suggestion: say that the type is the cv-unqualified type of the
  initializer, with no promotion, and add a test with an initializer of
  another enumeration type.

### G2. CE-21 and unions

- Page and rules: foundations/constant-evaluation, CE-21; classes/unions,
  UNI-4.
- Kind: ambiguous.
- What happened: CE-21 asks for a value in each scalar part. In a union
  only the active member has a value. The front end checks only the
  active member, and accepts a union with no active member. The text
  says neither.
- Suggestion: state that only the active member of a union counts, and
  whether a union with no active member is a valid constant.

### G3. Which copies CE-22 covers

- Page and rules: foundations/constant-evaluation, CE-22; classes/copy-move,
  CPY-8, CPY-12.
- Kind: ambiguous.
- What happened: CE-22 cites CPY-8, which describes the implicit copy and
  move functions. A user-provided copy constructor need not read every
  part, so the front end applies CE-22 only to implicit copies and
  reports classes with a user-provided copy or move constructor as
  unsupported. `copy-unset-array-element.cpp` also depends on a `return`
  of a local by value being a move during constant evaluation. CPY-12
  covers that, but CE-22 does not point to it.
- Suggestion: say that CE-22 applies to the implicit copy and move
  functions (a user-provided one reads what its body reads), and cite
  CPY-12 for the `return` case.

### G4. The literal type list

- Page and rules: foundations/constant-evaluation, CE-23.
- Kind: outside knowledge.
- What happened: from outside the specification, the reader knows a
  further condition on literal class types that concerns constructors,
  and that `void` is usually counted as a literal type. CE-23 lists
  neither, and its Evidence says it lists only conditions that a test can
  observe. The front end follows the page.
- Suggestion: state in the rule that the list is complete for Tidal, or
  add the missing conditions with tests that observe them.

### G5. An implicit default constructor in a constant

- Page and rules: declarations/constexpr, CX-1; foundations/constant-evaluation,
  CE-21, CE-24.
- Kind: ambiguous.
- What happened: CX-1 lets a `constexpr` class variable have no
  initializer "if a `constexpr` default constructor gives every member a
  value". For a class with no user-declared constructor, the text does
  not say whether its implicit default constructor is `constexpr`. The
  front end accepts the variable when every member gets a value from a
  default member initializer (CE-21 decides).
- Suggestion: say when an implicit or defaulted default constructor
  counts as `constexpr`, with a test of a class whose members all have
  default member initializers.

### G6. No initializer and CE-24

- Page and rules: foundations/constant-evaluation, CE-24, CE-21;
  declarations/constexpr, CINIT-1.
- Kind: ambiguous.
- What happened: CE-24 defines a constant initializer as an initializer
  whose whole initialization is a constant expression.
  `result-unset-member.cpp` declares `constinit Pair pair;` with no
  initializer. Read literally, that variable has no initializer at all,
  so it is unclear whether CINIT-1 requires anything of it. The test
  expects a rejection under CE-21. The front end treats default
  initialization as the initializer.
- Suggestion: say in CE-24 that default initialization counts as the
  initialization of a variable with no initializer.

### G7. Two rules for one constant-evaluation rejection

- Page and rules: decision 0034; CX-1, CINIT-1, SA rules, DCL-19, SW-3,
  ENUM-14 with CE-21, CE-22, CE-23, and the `undefined` rules.
- Kind: needs example.
- What happened: when a constant is needed and the evaluation fails, two
  reject rules apply: the one that needs a constant and the one that the
  evaluation broke. Tests cite them inconsistently.
  `result-unset-member.cpp` cites only CE-21. The `literal-*` tests cite
  CE-23 and CX-1. `copy-unset-member.cpp` cites CE-22 and OBJ-2. A front
  end that names one rule matches some tests and not others. This front
  end now names both.
- Suggestion: add a convention to the style guide that such a test cites
  both the context rule and the cause rule.

### G8. Prohibitions still in required rules

- Page and rules: foundations/constant-evaluation, CE-11;
  expressions/casts, CAST-1, CAST-9; expressions/postfix, PFX-7.
- Kind: missing.
- What happened: these required rules imply a rejection with no reject
  partner. CE-11 needs an integer or enumeration type for an array
  bound, an enumerator, or a case label, and CE-19 covers only
  narrowing. CAST-1 and CAST-9 imply that a cast outside their lists is
  invalid. PFX-7 implies that a call needs a function or a pointer to
  one. Tests of a call of a non-function cite DER-19, MPTR-10, or NAM-12
  instead.
- Suggestion: add reject rules for these cases on the same pages, as
  decision 0034 does elsewhere.

### G9. Default arguments on operator functions

- Page and rules: declarations/functions/default-arguments, DEFARG-6;
  classes/operators, OPR-2, OPR-3.
- Kind: contradictory.
- What happened: DEFARG-6 allows default arguments only on `operator()`.
  OPR-2 and OPR-3 allow them on `operator()` and `operator[]`, and
  `subscript-default-arg.cpp` expects acceptance. The front end follows
  OPR-3.
- Suggestion: add `operator[]` to DEFARG-6.

### G10. Where `fallthrough` may stand

- Page and rules: declarations/attributes, ATTR-2, ATTR-7.
- Kind: ambiguous.
- What happened: ATTR-2 says that `fallthrough` is "immediately before a
  `case` or `default` label". The front end reads it as text order, so
  the next token must be the label. That rejects a `fallthrough` at the
  end of an `if` branch that a label follows, as in
  `case 0: if (x) { f(); [[fallthrough]]; } case 1:`. The text names only
  the loop body as invalid. It does not say whether a nested block that is
  not a loop is valid.
- Suggestion: say whether "immediately before" means the next statement
  in execution or the next token, and add a test with an `if` branch.

### G11. Arguments of other standard attributes

- Page and rules: declarations/attributes, ATTR-2.
- Kind: missing.
- What happened: ATTR-2 gives the argument forms of `deprecated`,
  `nodiscard`, and `noreturn` only. It does not say whether
  `fallthrough`, `likely`, `unlikely`, `maybe_unused`,
  `no_unique_address`, or `carries_dependency` can take an argument. The
  front end accepts an argument on them.
- Suggestion: say that these attributes take no argument, with a test.

### G12. RET-3 for member functions

- Page and rules: declarations/functions/return-deduction, RET-3;
  classes/members, CLS-2.
- Kind: ambiguous.
- What happened: RET-3 rejects a use of the function before its return
  type is known. In a class, member function bodies are parsed after the
  class is complete, so a body can call a member defined later in the
  class. The text does not say whether that member's type counts as
  known. The front end applies RET-3 only to functions that are not
  members.
- Suggestion: add an example of a member function body that calls a
  later member function with a deduced return type, with its outcome.

### G13. Tests without rule text

- Page and rules: foundations/types, TYP-2.
- Kind: outside the spec.
- What happened: during the run, `spec/foundations/types.md` held 11
  lines and no TYP-2 rule, while 43 tests in `tests/foundations/types/`
  cite TYP-2. One of them, `align-ilp32.cpp`, changed during the run
  (the expected alignment of `long long` and `double` under ILP32 went
  from 8 to 4). The front end passes all 43 with its earlier LP64 and
  ILP32 model, which it did not get from TYP-2.
- Suggestion: none for the text. This page should be checked again once
  its rules are written.

## 4. Crates

None added. Decision 0035 allows crates that encode no C++ knowledge.
This round's new code was semantic checks, constructor evaluation, and a
small attribute parser, which the decision keeps hand-written or which no
crate would shorten.

## 5. Files changed

- `frontend/src/ast.rs`: `EK::Construct`, `Stmt::Assume`, `walk_expr`.
- `frontend/src/parser.rs`: `MemInit`, `Attr`, enumerator types,
  attribute and pure-function tables, `error_because`, abstract-class
  and deleted-default-constructor tests.
- `frontend/src/diag.rs`, `frontend/src/main.rs`, `frontend/src/conform.rs`:
  the second rule of a diagnostic.
- `frontend/src/consteval.rs`: constructor evaluation, CE-21, CE-22,
  CE-23, unions, bit-field truncation on assignment, ATTR-10.
- `frontend/src/class.rs`: ENUM-10 types, enumerator and class
  attributes, CLS-9.
- `frontend/src/decl.rs`: attribute parser, `explicit(c)`, OPR-2 names,
  default argument conversion and DEFARG-4.
- `frontend/src/decl2.rs`: mem-initializers, default construction,
  CX-1 and CINIT-1 without an initializer, and the declaration checks
  of section 2.4.
- `frontend/src/expr.rs`, `frontend/src/expr2.rs`: constructor choice,
  CTOR-9, DEL-1, RET-3, ATTR-4, VIRT-8 calls, VIRT-18.
- `frontend/src/stmt.rs`: statement attributes, ATTR-5.
- `frontend/README.md`: diagnostic format.
- Every `required` rule ID used in a diagnostic was replaced by its
  reject partner across `frontend/src/`.
