# Front end update: round 7, the interpreter

Date: 2026-10-31. Decisions 0029, 0035, 0047, 0048. Role: clean-room
reader. Inputs: the files under `spec/`, `docs/`, `tests/`, `questions/`,
and `frontend/`, and the previous reports in this directory. No crates
were added. The new code is roadmap item FI: an interpreter of the
abstract machine in `frontend/src/interp/`, constant evaluation on top of
it, a `run` command, and run tests in the conformance harness.

## 1. Conformance

### Checks

`tidal-fe conform ../tests`. Other writers added tests during the round,
and the suite grew from 4042 to 4295 tests. The row for the 4041 tests of
the baseline that still exist isolates the change in the front end. The
pass column counts "Differs from EDG only" as a pass, as
`tools/frontend_check.py` does.

| Run | Tests | Pass | False reject | Missed reject | Unsupported |
|---|---|---|---|---|---|
| before | 4042 | 3345 | 4 | 338 | 354 |
| after, same 4041 tests | 4041 | 3415 | 4 | 330 | 292 |
| after, whole suite | 4295 | 3476 | 15 | 420 | 384 |

`python3 -I tools/frontend_check.py` reports 3476/4295 pass, 0 worse, 70
better, 254 new (193 not passing). The 70 better tests come from
constant evaluation on the interpreter (section 3) and from the trees that
the front end now builds for the interpreter (section 4).

The 11 false rejects outside the baseline are new tests. Eight are in
`templates/` and in areas the front end does not model. Three are
resolution questions in tests from the repairs of round 6:
`list-string-literal-binds-literal.cpp`,
`tie-conversion-result-constructor.cpp`, and
`ne-not-matching-friend.cpp`. The interpreter does not take part in any
of them.

Rejected tests whose diagnostic names one of the test's rules: 922 in
round 6, 942 now.

### Run tests

`tidal-fe conform` now runs each test whose `tidal-expect` is `run` or
`run-fail`, after every translation unit passes the check. In round 6 no
run test ran.

| Directory | Tests | Pass | Wrong result | Missed undefined behavior | Unexpected undefined behavior | Unsupported | Check did not accept |
|---|---|---|---|---|---|---|---|
| classes | 277 | 192 | 1 | 0 | 0 | 80 | 4 |
| declarations | 174 | 155 | 0 | 0 | 1 | 18 | 0 |
| exceptions | 92 | 81 | 0 | 0 | 0 | 11 | 0 |
| expressions | 233 | 178 | 0 | 0 | 4 | 50 | 1 |
| foundations | 73 | 67 | 0 | 0 | 0 | 6 | 0 |
| statements | 77 | 61 | 0 | 0 | 0 | 15 | 1 |
| templates | 28 | 2 | 0 | 0 | 0 | 19 | 7 |
| **total** | 954 | 736 | 1 | 0 | 5 | 199 | 13 |

- 40 of the 42 `run-fail` tests pass. 26 stop at undefined behavior that
  names one of the test's rules, 12 terminate (UNW-5, THR-7, THR-9,
  TRY-18), and 2 end with a nonzero status. The other 2 are unsupported.
- 41 of the 42 run tests with `tidal-sources` run linked and pass.
- 37 passing runs passed a choice point: SEQ-8 (18), SEQ-1 (12), CMP-8
  (2), CMP-20 (2), PRIM-9 (2), and SINIT-3 (1).
- The one wrong result is `list-ctor-preferred-over-better-match.cpp`.
  The front end chooses `S(int, double)` for `S s{1, 2.0}`, because it
  does not recognize the class template `std::initializer_list` of the
  test as the list type that ILC-3 prefers.
- The five stops at unexpected undefined behavior are findings G57, G58,
  and G59.

The constructs that the interpreter stops at most often:

| Construct | Tests |
|---|---|
| a defaulted `==` or `<=>`, and the comparison category types | 18 |
| a pointer to member | 18 |
| a call the front end does not resolve: templates, an explicit destructor call, a global allocation function called by name | 15 |
| `new` with placement arguments or a class allocation function | 14 |
| `reinterpret_cast` | 12 |
| a class with an anonymous union, an inherited constructor, or a `std::initializer_list` parameter | 10 |
| a structured binding | 8 |
| a user-defined literal | 7 |
| a virtual base class | 7 |

## 2. The interpreter

`frontend/src/interp/` is a component of its own (decision 0048). It runs
the trees that the check builds.

| File | Contents |
|---|---|
| `mod.rs` | values, objects, places, the stops, and the objects that end with a block, a full expression, or the program |
| `expr.rs` | expressions, conversions, initialization, constructors, copies, assignments, calls, `new`, `delete`, `throw`, and `dynamic_cast` |
| `stmt.rs` | statements, blocks, `goto`, `switch`, try blocks, and static locals |
| `run.rs` | linking the units, static initialization, `main`, and the end of the static objects |

A run ends in one of these ways:

| Outcome | Meaning |
|---|---|
| result | `main` returned. The run gives the status. |
| unspecified | `main` returned, and the run passed a choice point. The run gives the status under its fixed choices and lists each choice point with its rule and line. |
| undefined | The run reached an operation that an `undefined` rule names. It stops, names the rule, and gives no result (decision 0047). |
| terminated | The program ended without returning from `main` (UNW-5, THR-7, THR-9, TRY-18). On the matrix targets the status is nonzero. |
| unsupported | The interpreter met a construct it does not model. |

`tidal-fe run FILE [FILE...] [--m32]` prints the outcome on one line.
`frontend/README.md` gives the exit codes: the program's status for a
result, 125 for a rejected unit, 126 for undefined behavior, 127 for
unsupported, and 134 for termination.

### What the interpreter covers

- Integer and floating arithmetic, conversions, shifts, and comparisons,
  with the checks of ADD-9, ADD-10, ADD-11, ADD-17, MUL-9 to MUL-12,
  SHF-7, UN-4, UN-21, PFX-18, CONV-5, and ENUM-6.
- Pointers and arrays: bounds (PFX-4, ADD-10), null pointers (UN-7,
  PFX-8), pointer equality with the addresses of union members and first
  members, and the relational order of CMP-7.
- Object lifetimes (OBJ rules): automatic objects end at the end of their
  block, also through `return`, `break`, `goto`, and exceptions, in the
  reverse order. Temporaries end at the end of their full expression
  (VC-9). Lifetime extension follows REF-4 for references, members of
  aggregates, init captures (LAM-7), and the range of a range-based `for`
  (RFOR-4). Static objects end after `main` in the reverse order of their
  construction (OBJ-4).
- Reads of objects with no value (INIT-1, NEW-3), of ended objects
  (OBJ-2, UN-7, DTOR-7), and of union members that are not active
  (UNI-6), with the common initial sequence of UNI-10.
- Classes with bases that are not virtual: constructors with member
  initializers, base initializers, delegation, and default member
  initializers; destructors; copy and move constructors and assignments,
  implicit, defaulted, or written, with the choices of CPY-5 and CPY-9.
- Virtual functions: the final overrider (VIRT-1, VIRT-5), covariant
  results (VIRT-4), the dynamic type during construction and
  destruction, qualified calls without dispatch, and `dynamic_cast`
  (VIRT-10). `delete` through a base needs a virtual destructor
  (VIRT-12).
- `new` and `delete` without placement arguments or class allocation
  functions, with NEW-5.
- Exceptions: `throw`, rethrow, handlers in order with the matches of
  TRY-2, handler parameters by value and by reference, function try
  blocks (TRY-9, TRY-10), unwinding of blocks, full expressions, and
  partly built objects (UNW-1, UNW-2, UNW-9), and termination (UNW-5,
  THR-9).
- Statements: `if`, `if consteval` (IF-18), loops, `switch`, `goto` within
  a function, range-based `for` over arrays and over classes with
  `begin` and `end`, `[[assume]]` (ATTR-10, ATTR-15), and `[[noreturn]]`
  (ATTR-3).
- Block-scope statics with STO-9 and UNW-4, and static initialization in
  two phases: constant initializers first (CE-24), then the dynamic ones
  in the order of their definitions.
- Lambdas with captures by copy, by reference, init captures, `this`,
  and `*this`, and the conversion of a lambda with no captures to a
  function pointer (LAM-10).
- Programs of several translation units (TR-4). Each declaration with
  external linkage names the first definition. An inline function or
  variable is one entity. Internal linkage keeps one entity per unit.
- `main` with `argc` and `argv` (TR-9).

### What the interpreter does not cover

- Virtual base classes, pointers to members, structured bindings,
  user-defined literals, `reinterpret_cast`, placement `new`, class
  allocation functions, and the backing array of `std::initializer_list`.
- Defaulted comparison operators and the comparison category types.
- Templates, generic lambdas, and lambdas with a `static` call operator.
  The front end leaves these unknown.
- SEQ-4: the interpreter does not detect unsequenced accesses. It
  evaluates operands from left to right and records SEQ-8 or SEQ-1 when
  two operands or arguments have side effects.
- Every choice point takes one fixed choice. The interpreter does not
  try the other choices, so an `unspecified` outcome gives one member of
  the allowed set.
- The trace of `volatile` accesses (TR-7) is not recorded. Threads are
  not modeled.
- VC-7: the interpreter makes no extra copies of trivially copyable
  arguments.

## 3. Constant evaluation on the interpreter

`src/consteval.rs` is now a client of the interpreter. Each entry point
makes a machine with `constant` set and runs the expression or the
initializer as a full expression. The machine then stops in two ways:

- `Stop::Limit` names a constant-evaluation rule: CE-2 for a call of a
  function that is not `constexpr`, CE-3 for a variable that is not
  usable, CE-5 for a change to an object from before the evaluation,
  CE-6, CE-8, CE-10, CE-12, CE-13, CE-15, CE-16, CE-20, and CE-22.
- `Stop::Undefined` names the `undefined` rule, as at run time (CE-9,
  TR-10).

Both become "not a constant expression" with the rule. The client checks
CE-14, CE-21, CE-7 (storage from `new` freed in the same evaluation), and
CX-2 at the end. At run time the same operations either run or record a
choice: a relational comparison of unrelated pointers records CMP-8, and
string literal addresses record PRIM-9.

Constant evaluation gained what the interpreter brought: base
subobjects, destructors that are `constexpr`, `new` and `delete`,
`dynamic_cast`, temporaries that end with their full expression, and
references at namespace scope that extend a temporary (CE-14). The
static initialization of a run uses the same client: a variable whose
initializer is constant runs in the first phase with `constant` set, so
`if consteval` takes its first branch there.

## 4. Changes to the check

The interpreter needs trees for constructs that the check left unknown.
Each new tree is built only when it raises no diagnostic. Otherwise the
check keeps its old unknown node. The changes:

- Constructor calls use the constructor that resolution chose, with each
  argument converted to its parameter. A copy or move from the same class
  keeps its source, and the interpreter chooses the copy or move
  constructor.
- Functional casts make class temporaries. A prvalue of the same class
  initializes the result directly (VC-4).
- Aggregates with bases, and aggregates with members that are
  aggregates, when the list needs no brace elision.
- Base and delegating member initializers.
- A reference bound to a temporary binds a materialized prvalue (REF-2,
  REF-14), also for a braced list with one class element (Q-0157).
- `new`, `delete`, try blocks with handlers, function try blocks,
  `dynamic_cast`, range-based `for` lowered to its loop (RFOR-1, RFOR-2),
  the implicit class assignment, and closure classes for lambdas.
- Qualified member names (DER-3): `d.B::f()` looks in `B`, and a
  qualified call does not dispatch.
- Overload resolution of constructors no longer counts an implicit copy
  or move constructor that the class declares itself (CPY-5).
- `noexcept` judges constructor calls, `new`, and `dynamic_cast` of a
  reference, and stays unknown for a temporary whose destructor or
  default member initializer runs code.
- The parser records `noexcept(false)` (UNW-5) and the internal linkage
  of namespace-scope variables.

## 5. Findings

### G57. Floating division by zero at run time

- Page and rules: expressions/multiplicative, MUL-11;
  expressions/comparison, CMP-5.
- Kind: contradictory.
- What happened: MUL-11 makes `0.0 / 0.0` undefined. The run tests
  `nan.cpp`, `three-way-floating.cpp`, and `nan-to-bool.cpp` compute
  `zero / zero` from a `volatile double` at run time and expect a NaN.
  The interpreter follows MUL-11 and stops, so the three tests count as
  unexpected undefined behavior.
- Suggestion: decide whether IEEE division is the run-time behavior on
  the matrix targets. If it is, MUL-11 can apply to constant evaluation
  only, or become implementation-defined. If it is not, the three tests
  need a NaN from another source.

### G58. Floating overflow and extra precision

- Page and rules: expressions/multiplicative, MUL-12, MUL-17.
- Kind: contradictory.
- What happened: `excess-precision-either.cpp` computes
  `big * 10.0 / 10.0` with `big` equal to `1e308` and accepts either
  result. Under MUL-12 the product overflows `double`, so the run is
  undefined and has no result to accept. MUL-12 is provisional (Q-0001).
- Suggestion: settle Q-0001 and change either MUL-12 or the test.

### G59. A run test that writes outside its array

- Page and rules: declarations/initialization/list, LIST-7;
  expressions/postfix, PFX-4.
- Kind: contradictory.
- What happened: `braced-order.cpp` declares `int order[3]` and calls
  `note` five times, and each call writes `order[n++]`. The fourth call
  writes `order[3]`, which PFX-4 makes undefined. The interpreter stops
  there.
- Suggestion: make the array five elements long.

### G60. A handler for `void*`

- Page and rules: exceptions/handlers, TRY-2.
- Kind: contradictory.
- What happened: TRY-2 lets a pointer handler match through a
  qualification conversion or a conversion to a base, or from the type
  of `nullptr`. `match-void-pointer.cpp` throws `&x` for an `int x` and
  expects `catch (void* p)` to match. The interpreter also matches a
  pointer to an object with a `void*` handler, from outside knowledge.
- Suggestion: add the conversion to `void*` to TRY-2, or change the test.

### G61. `dynamic_cast` from a private base

- Page and rules: classes/virtual, VIRT-10.
- Kind: missing.
- What happened: VIRT-10 says that a cast to a derived class succeeds if
  the most derived object contains one unambiguous public subobject of
  the target class. In `dynamic-cast-private-base.cpp` the most derived
  object is a `D`, and the operand points to its private base `A`. By the
  text the cast succeeds, and the test expects a null pointer. The
  interpreter also requires that the operand's subobject is a public base
  of the most derived object, from outside knowledge.
- Suggestion: state the condition on the operand's subobject.

### G62. A failed `dynamic_cast` of a reference

- Page and rules: classes/virtual, VIRT-10.
- Kind: missing.
- What happened: VIRT-10 gives the result of a failed cast of a pointer.
  It does not say what a failed cast of a reference does. The interpreter
  reports such a run as unsupported.
- Suggestion: state that the cast throws an exception, and name its type
  as the language-support page gives it.

### G63. An assignment to a member of a union member

- Page and rules: classes/unions, UNI-5, UNI-6, UNI-10.
- Kind: missing.
- What happened: `common-initial-sequence.cpp` default-initializes a
  union, then assigns `u.a.tag = 7` and `u.a.a = 1`. UNI-5 says that an
  assignment to a member makes that member active. The assignment here
  is to a member of the member `a`. Without a rule for it, the write
  reaches an object that does not exist yet, and UNI-6 can apply. The
  interpreter makes `u.a` active, from outside knowledge.
- Suggestion: say whether an assignment through a chain of member
  accesses makes each union member on the chain active.

### G64. Three rules for a use after a lifetime ends

- Page and rules: foundations/objects, OBJ-2; expressions/unary, UN-7;
  classes/constructors, DTOR-7; expressions/new-delete, NEW-5.
- Kind: ambiguous.
- What happened: a read through a pointer to an object whose lifetime
  ended breaks OBJ-2, UN-7, and DTOR-7 when a destructor ran. The tests
  cite different subsets. `use-after-delete-sanitizer.cpp` cites DTOR-7,
  and `read-after-block-ends-sanitizer.cpp` cites OBJ-2 and UN-7. The
  interpreter names all three rules at run time, so either test passes.
  Constant evaluation names one rule, as before.
- Suggestion: name one rule for the operation and let the others refer
  to it, or cite every rule in each test.

### G65. An implicit copy of a member with no value

- Page and rules: foundations/constant-evaluation, CE-22;
  foundations/objects, OBJ-2.
- Kind: missing.
- What happened: CE-22 says that an implicit copy reads each scalar part,
  and constant evaluation rejects the copy of a part with no value. At
  run time OBJ-2 makes a read of a scalar with no value undefined. No
  rule says whether an implicit copy at run time reads the parts in that
  sense. The interpreter copies the missing value along and does not
  stop.
- Suggestion: state whether an implicit copy or move at run time is a
  read for OBJ-2.

### G66. An exit status outside 0 to 255

- Page and rules: foundations/translation, TR-2.
- Kind: missing.
- What happened: TR-2 says that the value `main` returns is the exit
  status. On the matrix targets the environment keeps only the low eight
  bits, so `return 256;` gives status 0. The interpreter reports the
  value. `tidal-fe run` exits with the low eight bits, from outside
  knowledge.
- Suggestion: add an implementation-defined rule for the status that the
  environment sees.

### G67. A static call operator and the conversion to a function pointer

- Page and rules: expressions/lambdas, LAM-10, LAM-13;
  foundations/overload-resolution, OVL-5, OVL-6.
- Kind: ambiguous.
- What happened: `static-call-operator.cpp` calls `g(1)` on a lambda
  with a static call operator. The closure also converts to a function
  pointer (LAM-10), which gives a surrogate candidate with the same
  parameter types. A static member function has no object parameter that
  could break the tie, so the rules as the front end reads them make the
  call ambiguous. The test expects the call to work. The front end keeps
  such lambdas unknown.
- Suggestion: say which candidate wins, or that the surrogate is not a
  candidate for a closure with a static call operator.
