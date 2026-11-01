# Front end update: round 9, constraints, deduction of class arguments, and the run outcome

Date: 2026-11-01. Decisions 0029, 0035, 0047, 0048, 0050 (item FE).
Role: clean-room reader. Inputs: the files under `specification/`,
`documentation/`, `tests/`, `questions/`, and `frontend/`, and the
previous reports in this directory. g++ was run once to print its
predefined macros, for the default new alignment of NEW-27. No crates
were added.

The new code is `frontend/src/cons.rs` (concepts, constraints,
requires expressions, abbreviated function templates, the use checks of
packs), `frontend/src/ctad.rs` (deduction of class template arguments),
and `frontend/src/dcmp.rs` (defaulted comparison operators). The
interpreter gained placement `new`, allocation functions called by name,
implicit objects in allocated storage, explicit destructor calls, and
the linking of template specializations across translation units.
`tidal-fe conform -v` now reports the run outcome in its status column.

## 1. Conformance

### Checks

`tidal-fe conform ../tests`. Other writers added tests during the round,
and the suite grew from 5260 to 5385 tests. The row for the 5260 tests
of the baseline isolates the change in the front end. The pass column
counts "Differs from EDG only" as a pass. It also counts a test that
expects a run as a pass when its check passed, whatever the run gave,
so that the rows compare the checks of both rounds.

| Run | Tests | Pass | False reject | Missed reject | Unsupported |
|---|---|---|---|---|---|
| before | 5260 | 4268 | 81 | 451 | 460 |
| after, same 5260 tests | 5260 | 4576 | 66 | 323 | 295 |
| after, whole suite | 5385 | 4652 | 74 | 347 | 312 |

No test whose check passed in the baseline fails its check now.

`python3 -I tools/frontend_check.py` (unchanged, without `--update`)
reports 4421/5385 pass, 220 worse, 302 better, 125 new (54 not
passing), and exits 1. All 220 worse tests expect a run. Their checks
pass as before. Their status column now gives the run outcome (section
1, "The status column"), and the script counts only `pass` and
`EDG-ONLY` as passing:

| Status of the 220 | Tests |
|---|---|
| `run-unsupported` | 212 |
| `RUN-WRONG-RESULT` | 6 |
| `RUN-UNEXPECTED-UB` | 2 |

None of the 220 passed its run in the baseline. The six wrong results
were wrong results in the baseline too. The two stops at undefined
behavior were unsupported in the baseline (finding G78). With the
check statuses alone, the count is 4652/5385, 0 worse.

The new and changed pages of the brief:

| Directory | Tests before | Pass before | Tests now | Pass now, same tests | Pass now, all |
|---|---|---|---|---|---|
| templates/packs | 68 | 20 | 69 | 34 | 35 |
| templates/class-template-deduction | 107 | 12 | 112 | 83 | 86 |
| templates/concepts | 114 | 14 | 117 | 104 | 107 |
| templates/constraints | 112 | 28 | 113 | 101 | 102 |
| templates/dependent-names | 108 | 77 | 116 | 77 | 82 |
| templates/instantiation | 125 | 116 | 127 | 116 | 118 |
| foundations/storage | 57 | 46 | 59 | 48 | 50 |
| foundations/language-support | 23 | 10 | 23 | 17 | 17 |
| foundations/odr | 64 | 64 | 67 | 64 | 66 |
| expressions/new-delete | 103 | 76 | 106 | 90 | 92 |
| expressions/additive | 53 | 51 | 53 | 51 | 51 |
| expressions/multiplicative | 54 | 52 | 54 | 52 | 52 |
| expressions/comparison | 84 | 60 | 84 | 75 | 75 |

The front end left constraints and the deduction of class template
arguments unknown in round 8. It models both now.

### Run tests

| Run | Tests | Pass | Wrong result | Missed undefined behavior | Unexpected undefined behavior | Unsupported | Check did not accept |
|---|---|---|---|---|---|---|---|
| before | 1173 | 874 | 8 | 0 | 0 | 263 | 28 |
| after, same 1173 tests | 1173 | 923 | 6 | 0 | 2 | 220 | 22 |
| after, whole suite | 1203 | 942 | 6 | 0 | 2 | 225 | 28 |

No run test that passed in the baseline fails now. By directory, on the
whole suite:

| Directory | Tests | Pass | Wrong result | Missed undefined behavior | Unexpected undefined behavior | Unsupported | Check did not accept |
|---|---|---|---|---|---|---|---|
| classes | 285 | 216 | 1 | 0 | 0 | 62 | 6 |
| declarations | 183 | 161 | 2 | 0 | 0 | 19 | 1 |
| exceptions | 95 | 84 | 0 | 0 | 0 | 11 | 0 |
| expressions | 295 | 212 | 2 | 0 | 0 | 76 | 5 |
| foundations | 154 | 111 | 1 | 0 | 2 | 36 | 4 |
| statements | 85 | 70 | 0 | 0 | 0 | 14 | 1 |
| templates | 106 | 88 | 0 | 0 | 0 | 7 | 11 |
| **total** | 1203 | 942 | 6 | 0 | 2 | 225 | 28 |

The run tests of the pages of the brief:

| Directory | Run tests before | Pass before | Run tests now | Pass now, same tests | Pass now, all |
|---|---|---|---|---|---|
| templates/packs | 1 | 1 | 1 | 1 | 1 |
| templates/concepts | 1 | 0 | 2 | 1 | 2 |
| templates/constraints | 1 | 1 | 1 | 1 | 1 |
| templates/dependent-names | 45 | 37 | 48 | 37 | 39 |
| templates/instantiation | 16 | 16 | 18 | 16 | 16 |
| foundations/storage | 49 | 1 | 51 | 23 | 23 |
| foundations/language-support | 9 | 0 | 9 | 1 | 1 |
| foundations/odr | 37 | 30 | 39 | 34 | 36 |
| expressions/new-delete | 49 | 10 | 51 | 11 | 11 |
| expressions/additive | 7 | 7 | 7 | 7 | 7 |
| expressions/multiplicative | 13 | 12 | 13 | 12 | 12 |
| expressions/comparison | 16 | 13 | 16 | 14 | 14 |

Templates in the interpreter (goal 2). 158 run tests use templates, in
the test or in a header or unit that it includes. 145 of them were in
the baseline. Of those 145, 98 passed their run before and 111 pass now.
Of all 158, 119 pass. Of the 39 that do not pass, 13 are not accepted by
the check, 25 stop at a construct the interpreter does not run, and 1
gives a wrong result. The interpreter met no template-specific
construct in the 25. The constructs are overloaded operators that the
front end leaves unknown, constructor calls through
`std::initializer_list`, structured bindings with `get`, and `new` with
a replaced allocation function. The run gains come from these changes:

- A specialization of a variable template is one object in the
  program, also when it is `const` (ODR-12). Link names carry the
  template arguments of a specialization, and of a class
  specialization in a qualified name.
- A later declaration of an explicit specialization redeclares it, so a
  specialization declared in a header and defined in one unit links
  (ODR-13).
- `&f<int>` names the specialization when the explicit arguments give
  every template argument.
- A template parameter object is not destroyed (TPL-49).
- The body of a defaulted `==`, `<=>`, or secondary comparison is made
  at its first use. A defaulted `<=>` declares a defaulted `==`.
- The built-in `<=>` gives the static data member `less`, `equal`,
  `equivalent`, `greater`, or `unordered` of the comparison category
  class (CMP-12, CMP-17).

### The status column of `conform -v` (goal 3)

Each test line of `tidal-fe conform ../tests -v` and of `--out FILE`
starts with a status. A test that expects `run` or `run-fail` takes the
run's outcome as its status when its check passed. The column holds one
of these strings:

| Status | Meaning |
|---|---|
| `pass` | The check matches the expectation. For a `run` or `run-fail` test, the run also passed. A check that rejects as an EDG override expects also gives `pass`. |
| `EDG-ONLY` | The check differs from the EDG override and matches the base `tidal-expect` line. |
| `FALSE-REJECT` | The check rejected a program that the test expects valid. |
| `MISSED-REJECT` | The check accepted a program that the test expects rejected. |
| `unsupported` | The check gave up on a construct and found no error. |
| `run-unsupported` | The check passed. The interpreter met a construct it does not model. |
| `RUN-WRONG-RESULT` | The check passed. A `run` test ended with a status other than 0 modulo 256, or terminated. A `run-fail` test whose rules name no undefined behavior ended with status 0. |
| `RUN-MISSED-UB` | The check passed. A `run-fail` test ended with status 0. |
| `RUN-UNEXPECTED-UB` | The check passed. The run stopped at undefined behavior that the test does not expect: a `run` test, or a `run-fail` test whose rules the stop does not name. |

`-v` prints the lines that are not `pass`. The run tests also keep their
own lines, which start with `RUN` and give the run status second:
`run-pass`, `run-not-run` (the check did not accept a unit, so nothing
ran), and the run statuses above. `tools/frontend_check.py` reads the
status column only, so it records the run outcome with `--update`. The
integrator can decide whether the run statuses count as passing.

## 2. What the front end models now

### Concepts and constraints

- A concept definition gives a normal form over atomic constraints
  (CNS-P1). An atom keeps its tokens, its scope stack, the template
  parameters it can name, and their arguments (the parameter mapping of
  CNS-4). A concept-id expands to its concept's normal form with the
  arguments mapped (CNS-2). A negation, comparison, call, fold, or
  requires expression is one atom (CNS-3).
- A check substitutes the mapping by parsing the atom's tokens again with
  the parameters bound, as a specialization is made. An error in that
  parse makes the atom not satisfied (CNS-7, CNS-8). An instantiation
  that the parse needs reports its own errors (CNS-33). A type other than
  `bool` or a value that is not constant is an error of the program
  (CNS-9), also during deduction. A check that needs its own result is an
  error (CNS-35). Conjunctions and disjunctions stop early (CNS-5,
  CNS-6). Results are cached by atom and mapped arguments (CNS-10).
- The associated constraints of a declaration join the type constraints
  of the head, the requires clause, the placeholder parameters, and the
  trailing clause, in that order (CNS-1).
- Uses: function templates in a call, before the function type is formed
  (CNS-11); functions that are not templates, in calls, constructors, and
  operators (OVL-24); the address of a function and a call of a single
  function (CNS-12); template-ids of classes, variables, and aliases, and
  explicit instantiations and specializations (CNS-13); partial
  specializations (CNS-17, CNS-36); constructors that still count as
  declared (CNS-16); prospective destructors (CNS-19, CNS-20).
- Subsumption compares atoms by their place, scope, and mapped arguments
  (CNS-4, CNS-22, CNS-23). It orders function templates after partial
  ordering (ORD-13, CNS-34), functions with the same parameters (OVL-30),
  and partial specializations with equivalent patterns (ORD-14).
- Equivalence of constraints for redeclarations renders the normal form
  with each name replaced by what it names (CNS-26 to CNS-29). Members of
  a class template can differ only in their trailing clauses.
- Requires expressions, with parameters and the four kinds of
  requirement (CPT-24 to CPT-35). Outside a template an invalid
  requirement is an error (CPT-27). In a specialization it makes the
  value `false` (CPT-26).
- Type constraints on type parameters, packs (CPT-19), and constant
  parameters with a placeholder (CPT-21), and constrained placeholders of
  variables and return types (AUTO-16, CPT-22, CPT-23). The checks of
  CPT-4 to CPT-7, CPT-10, CPT-12 to CPT-15, CPT-18, CPT-29, CPT-34,
  CPT-36, and CPT-37.
- Abbreviated function templates: each placeholder parameter adds an
  invented type parameter after those of the head (DED-34, CPT-20).

### Deduction of class template arguments

Each guide becomes a hidden function template whose result type is the
specialization (CTAD-P1). The guides come from the constructors, with
the parameters of a constructor template after those of the class, with
the constraints of both, and with a conditional `explicit` decided for
the deduced arguments (CTAD-5, CTAD-29). Then the guide with no
parameters, the copy candidate, the aggregate candidate with brace
elision and packs of bases, and the user guides declared before the use
(CTAD-6, CTAD-7, CTAD-9 to CTAD-11, CTAD-27). An alias template and a
class that inherits constructors adapt the guides of the class template
they name (CTAD-14, CTAD-16, CTAD-28). The call machinery chooses one
guide. CTAD-19 to CTAD-22 break the ties that remain after the usual
rules. The sites are variables, static data members, `new`, and
functional casts (CTAD-1). The checks of CTAD-2, CTAD-3, CTAD-12,
CTAD-13, CTAD-15, CTAD-23, and CTAD-24 apply.

### Packs

The checks of PACK-2, PACK-4, PACK-6, PACK-8, PACK-20, and PACK-28. For
PACK-8 the pattern parse records each use of a pack, and the end of each
statement or declaration requires a `...` after it. A use inside a
lambda waits for the enclosing statement.

### Objects in raw storage

- Placement `new` with the non-allocating form (STG-5): in a byte array,
  a new object with its own place, checked for size and alignment
  (STG-17); in the place of an object of the same type, a replacement
  that the old names reach (STG-9); in the place of an object of another
  type, or of a base subobject, a new object that the old names do not
  reach (STG-19). A `const` complete object that is not dynamic cannot
  host a new object (STG-20).
- Global allocation and deallocation functions called by name give
  storage with no object. A conversion of `void*` to a pointer to an
  implicit-lifetime type there creates an array of that type that fills
  the storage (STG-7, STG-8). A member call on storage with no object of
  the class stops (STG-18, STG-22).
- An explicit destructor call ends the object. A virtual destructor
  called through a base ends the complete object. The end of a block
  stops if the storage of a variable holds another type (STG-21).
- A pointer to a new object in a byte array compares equal to a pointer
  to the bytes it occupies (STG-1).

### Other checks

- The built-in `<=>`: CMP-13, CMP-18, CMP-24, CMP-25, and LSUP-2.
- LSUP-1 for `auto x = {...}`, LSUP-5 for `std::align_val_t`, and the
  default new alignment of NEW-27.
- NEW-25: the deallocation function that a throw from a `new`
  expression's initialization would call.
- Pack expansions in the initializer of `new`.
- A `>>` that closes a template argument list inside a nested name.

## 3. Undefined behavior (goal 4)

56 rules have the kind `undefined` (PFX-21 is new since the brief). The
table gives, for each rule, the run-fail tests whose run stops at
undefined behavior that names the rule, and the reject tests that
constant evaluation rejects. Constant evaluation runs on the same
machine with the limits of `constant-evaluation.md`, so it detects the
same operations. The interpreter reports undefined behavior and gives no
result for it (decision 0047).

| Rule | Run-fail tests detected | Reject tests detected | Result |
|---|---|---|---|
| CTOR-12 | none | 3 of 3 | Detected in constant evaluation. |
| DTOR-7 | none | 2 of 2 | Detected in constant evaluation (new this round: explicit destructor calls). |
| DER-17 | 1 of 1 | 1 of 1 | Detected. |
| UNI-6 | none | 3 of 8 | Partly. Four tests use designated initializers and one uses `new`, which constant evaluation does not run. |
| VIRT-12 | none | 2 of 3 | Partly. `nonvirtual-dtor-delete-constant.cpp` is a missed reject. |
| ATTR-3 | 1 of 1 | none | Detected. |
| ATTR-10 | 1 of 1 | 1 of 1 | Detected. |
| CV-5 | none | 1 of 1 | Detected in constant evaluation. |
| ENUM-6 | 1 of 1 | 1 of 1 | Detected. |
| FN-3 | 1 of 1 | 1 of 1 | Detected. |
| RET-6 | none | 1 of 1 | Detected in constant evaluation. |
| INIT-1 | none | 5 of 5 | Detected in constant evaluation. |
| REF-5 | none | 4 of 4 | Detected in constant evaluation. |
| STO-9 | 1 of 1 | none | Detected. |
| VOL-6 | none | none | Not detected. Its one test expects `run`, and the interpreter gives the result 0 for the access. |
| TRY-11 | none | none | No test runs the behavior. The one test is an `accept` test. |
| ADD-9 | 1 of 1 | 3 of 3 | Detected. |
| ADD-10 | 1 of 1 | 6 of 6 | Detected. |
| ADD-11 | none | 2 of 2 | Detected in constant evaluation. |
| ADD-14 | none | 3 of 3 | Detected in constant evaluation. |
| ADD-17 | none | 2 of 2 | Detected in constant evaluation. |
| ADD-18 | none | 0 of 1 | Not detected. Constant evaluation stops at pointer arithmetic it does not model in the ILP32 test. |
| CAST-5 | none | 1 of 1 | Detected in constant evaluation. |
| CAST-14 | 2 of 2 | 2 of 2 | Detected. The stop names ENUM-6 or DER-17, the cases that CAST-14 restates. |
| LAM-19 | 0 of 1 | 1 of 1 | Partly. The run-fail test needs `new` with a capture, which the interpreter does not run. |
| MPTR-6 | none | 0 of 2 | Not detected. Constant evaluation does not model casts of pointers to members. |
| MUL-9 | 1 of 1 | 2 of 2 | Detected. |
| MUL-10 | 3 of 3 | 5 of 5 | Detected. |
| MUL-12 | none | 1 of 1 | Detected in constant evaluation. |
| MUL-14 | none | 2 of 2 | Detected in constant evaluation. |
| NEW-3 | none | 1 of 1 | Detected in constant evaluation. |
| NEW-5 | 3 of 3 | 5 of 5 | Detected. |
| NEW-32 | 0 of 1 | none | Not detected. The test replaces an allocation function, which `new` in the interpreter does not call. |
| PFX-4 | 2 of 2 | 3 of 3 | Detected. |
| PFX-8 | none | 1 of 1 | Detected in constant evaluation. |
| PFX-21 | 1 of 1 | 1 of 1 | Detected (new this round: the stop names PFX-21 and UN-7). |
| PFX-19 | 0 of 1 | none | Not detected. The test needs `reinterpret_cast` of a function pointer. |
| PFX-18 | 1 of 1 | 3 of 3 | Detected. |
| PRIM-8 | none | 1 of 1 | Detected in constant evaluation. |
| SHF-7 | 2 of 2 | 3 of 3 | Detected. |
| UN-4 | 1 of 1 | 1 of 1 | Detected. |
| UN-7 | 1 of 1 | 2 of 2 | Detected. |
| UN-21 | 1 of 1 | 3 of 3 | Detected. |
| CONV-5 | 1 of 1 | 2 of 2 | Detected. |
| SEQ-4 | none | 1 of 1 | Detected in constant evaluation. |
| OBJ-2 | 2 of 2 | 15 of 16 | Detected. One reject test uses designated initializers. |
| OBJ-10 | none | 0 of 1 | Not detected. The test uses designated initializers. |
| STG-17 | 2 of 2 | none | Detected (new this round). |
| STG-18 | 1 of 1 | none | Detected (new). Its `run` test reads the bytes of storage that holds an object, which the interpreter does not model. |
| STG-19 | 2 of 2 | 0 of 1 | Detected at run time (new). The reject test uses designated initializers. The `run` test `replace-base-old-pointer.cpp` stops at STG-19 (G78). |
| STG-20 | 1 of 1 | none | Detected (new). The `run` test `reuse-const-automatic-object.cpp` stops at STG-20 (G78). |
| STG-21 | 1 of 1 | none | Detected (new). |
| STG-22 | 2 of 3 | 1 of 2 | Partly (new). One run-fail test needs `reinterpret_cast` of a byte array, and one reject test uses designated initializers. |
| STG-23 | none | none | Not detected. Its three tests expect `run` and need `reinterpret_cast` to another object type (G78). |
| TR-8 | none | 2 of 2 | Detected in constant evaluation. |
| INST-15 | none | 1 of 2 | Partly. `recursion-endless-function.cpp` is a missed reject. |

At the start of the round the interpreter detected none of the STG
rules. The rules without a run-fail test are CTOR-12, DTOR-7, UNI-6,
VIRT-12, CV-5, RET-6, INIT-1, REF-5, VOL-6, TRY-11, ADD-11, ADD-14,
ADD-17, ADD-18, CAST-5, MPTR-6, MUL-12, MUL-14, NEW-3, PFX-8, PRIM-8,
SEQ-4, OBJ-10, STG-23, TR-8, and INST-15. Item FE of decision 0050 asks
for a run-fail test for each of them.

## 4. What the front end does not model

- Generic lambdas. A lambda with a placeholder parameter or a template
  head stays unknown, and so does a call of it.
- The object representation. `reinterpret_cast` between object pointer
  types, and the bytes of an object read through `unsigned char`, stay
  unsupported (STG-11 to STG-14, STG-23).
- `new` with a class-specific or replaced allocation function, and the
  deallocation after a throw (NEW-21 to NEW-24).
- Designated initializers in constant evaluation.
- Explicit template arguments for a pack followed by deduced elements of
  the same pack.
- The access context of a requires expression inside a member (CPT-26).
  The front end checks the concept's atoms in the concept's own scope.
- The mapping of a concept argument that is an expression, as in
  `Inner<sizeof(T) + 1>`. The concept-id is then one atom (G77).
- Eligibility of special members by constraints (CNS-21).
- A class template as the type of a constant template parameter (G68).

## 5. Findings

### G76. CNS-9 and an operand that is never substituted

- Page and rules: templates/constraints, CNS-9, CNS-5.
- Kind: contradictory.
- What happened: in `atomic-not-bool-skipped.cpp`, the concept is
  `(sizeof(T) > 100) && (sizeof(T))`. CNS-9 applies "after
  substitution". CNS-5 says the right operand is not substituted when
  the left one fails, which it does for `int`. By the text the program
  is valid, as GCC finds. The test expects a rejection, as Clang and EDG
  give. The front end rejects at the definition, because the type of
  `(sizeof(T))` does not depend on `T`.
- Suggestion: state that an atomic constraint whose type does not depend
  on template parameters must have the type `bool`, and that the check
  happens at the definition.

### G77. Equivalent mappings when the argument is an expression

- Page and rules: templates/constraints, CNS-4, CNS-27.
- Kind: ambiguous.
- What happened: `mapping-same-arguments.cpp` expects that two concepts,
  each with the body `Inner<sizeof(T) + 1>`, give one atomic constraint.
  CNS-4 requires mappings that "give equivalent arguments". CNS-27
  defines equivalence for constraints, by tokens and by the entities the
  names denote. No rule says when two argument expressions in a mapping
  are equivalent. The front end keeps such a concept-id as one atom, and
  the call is ambiguous.
- Suggestion: say that two mapped arguments that are expressions are
  equivalent under the same test as CNS-27, with each template parameter
  replaced by its argument.

### G78. Run tests whose behavior is undefined

- Page and rules: foundations/storage, STG-18, STG-19, STG-20, STG-23;
  declarations/volatile, VOL-6; decision 0047.
- Kind: contradictory.
- What happened: `reuse-const-automatic-object.cpp` (STG-20),
  `replace-base-old-pointer.cpp` (STG-19), the three `access-*` tests of
  STG-23, `implicit-objects-no-set-works.cpp` (STG-18), and
  `nonvolatile-access-to-volatile-constant.cpp` (VOL-6) expect `run`.
  Their notes say the rule "allows any result". Each rule makes the
  behavior undefined. Decision 0047 has the interpreter stop at
  undefined behavior and give no result, so a run of these tests stops.
  The harness then reports an unexpected stop. The first two tests do
  this now.
- Suggestion: give these tests the expectation `run-fail`, or add an
  expectation that accepts a stop at the test's own rule as well as any
  result.

### G79. PFX-21 and UN-7 for a member call through a null pointer

- Page and rules: expressions/postfix, PFX-21; expressions/unary, UN-7.
- Kind: ambiguous.
- What happened: PFX-21 says `p->f()` with a null `p` is undefined, and
  that `(*p).f()` is the same call (UN-7). Both rules apply to one
  operation. `member-call-null-pointer-sanitizer.cpp` lists PFX-21 only.
  The interpreter first stopped at the indirection with UN-7. It now
  names both rules.
- Suggestion: state that PFX-21 is the case of UN-7 for a member call,
  and list both rules on the tests.

### G80. The value that the built-in `<=>` gives

- Page and rules: expressions/comparison, CMP-12, CMP-17;
  foundations/language-support, LSUP-2.
- Kind: missing.
- What happened: LSUP-2 requires the program to declare the ordering
  classes with static data members `less`, `equal`, and the others.
  CMP-17 says `a <=> b` "gives `less`, `equal`, or `greater`". No rule
  says that the result is a copy of the static data member of that name,
  nor what the result is if a member's initializer gives another value.
  The front end makes the result a copy of the static data member.
- Suggestion: say that the result is a prvalue initialized from the
  static data member that CMP-17 names.

### G81. The choice of implicit objects

- Page and rules: foundations/storage, STG-8, STG-24.
- Kind: needs example.
- What happened: STG-8 has the implementation choose the set of objects
  that gives defined behavior, looking ahead to later uses. An
  interpreter cannot look ahead. The front end creates, at the first
  conversion of a pointer to the storage to `T*`, an array of `T` that
  fills the storage. This choice runs `implicit-array-for-arithmetic.cpp`
  and both paths of `implicit-objects-follow-later-use.cpp`. No rule or
  example says whether the choice is always allowed, or where it gives
  another result than the set of STG-8.
- Suggestion: add an example of a single object and of pointer
  arithmetic over the same storage. State whether an array of `T` that
  fills the storage is a valid choice for a program whose first use
  names `T`.

### G82. When a program needs `std::initializer_list`

- Page and rules: foundations/language-support, LSUP-1.
- Kind: ambiguous.
- What happened: LSUP-1 applies when "a program needs an
  `std::initializer_list<E>`". In `initializer-list-no-members.cpp` the
  need arises from `take({1, 2})`, where `take` has a parameter of that
  type. The rule does not say whether the need arises when the parameter
  is declared, when overload resolution forms the sequence of ICS-16, or
  when the list initializes the parameter. The front end checks the shape
  for `auto x = {...}` only, and misses this test.
- Suggestion: name the operations that need the class: the
  initialization of an object of type `std::initializer_list<E>` from a
  braced list, and the deduction of AUTO-7.

### G83. The guides of an alias template and a fixed argument

- Page and rules: templates/class-template-deduction, CTAD-14, CTAD-28.
- Kind: needs example.
- What happened: CTAD-14 builds the guide of an alias from the match of
  the guide's result with the alias's type. The parameters of the new
  guide are "the alias parameters that the match uses, then the guide's
  parameters that the match leaves". The rule does not say what a guide
  parameter that the match fixes to a type becomes in the parameter types
  of the new guide. The front end substitutes it, so the argument for it
  converts and is not deduced. With this reading
  `alias-template-fixed-argument.cpp`, where `1L` converts to the fixed
  `int`, matches the base expectation. EDG rejects it (Q-0240).
- Suggestion: add an example with an alias such as `Box<int, U>` and a
  guide `Box(T, U)`, and say that `T` becomes `int` in the new guide's
  parameters.

### G84. The place of a pack expansion for PACK-8

- Page and rules: templates/packs, PACK-8, PACK-3.
- Kind: needs example.
- What happened: PACK-3 says that in `T x...` the `...` is the variable
  argument part, and the pack in `T` stays unexpanded. PACK-8 then makes
  the declaration invalid. `named-parameter-before-ellipsis.cpp` tests
  this. The text of PACK-8 counts a use as expanded when it is "in the
  pattern of an expansion". The `...` after `x` follows the use, as an
  expansion would, and only PACK-3 says it is no expansion. The front end
  needs a special case for it.
- Suggestion: add `template <class... T> void take(T x...);` to the
  examples of PACK-8, with the note that it is invalid.
