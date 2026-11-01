# Front end update: round 8, templates

Date: 2026-10-31. Decisions 0029, 0035, 0047, 0048. Role: clean-room
reader. Inputs: the files under `specification/`, `documentation/`,
`tests/`, `questions/`, and `frontend/`, and the previous reports in
this directory. No crates were added. The new code is
`frontend/src/templ.rs` and changes across the parser, overload
resolution, and the interpreter. The front end now models class,
function, variable, and alias templates, explicit and partial
specialization, deduction, partial ordering, explicit instantiation,
and the lazy instantiation of the instantiation page. The round also
follows the repairs of G57 to G67.

## 1. Conformance

### Checks

`tidal-fe conform ../tests`. Other writers added tests during the round,
and the suite grew from 4348 to 4941 tests. The row for the 4348 tests
of the baseline isolates the change in the front end. The pass column
counts "Differs from EDG only" as a pass, as `tools/frontend_check.py`
does.

| Run | Tests | Pass | False reject | Missed reject | Unsupported |
|---|---|---|---|---|---|
| before | 4348 | 3500 | 18 | 424 | 406 |
| after, same 4348 tests | 4348 | 3798 | 19 | 278 | 253 |
| after, whole suite | 4941 | 4053 | 56 | 425 | 407 |

`python3 -I tools/frontend_check.py` reports 4053/4941 pass, 0 worse,
298 better, 593 new (338 not passing).

The five template pages of the brief, on the tests of the baseline:

| Page | Tests | Pass before | Pass after |
|---|---|---|---|
| arguments | 74 | 5 | 63 |
| declarations | 101 | 23 | 95 |
| specialization | 81 | 20 | 70 |
| deduction | 89 | 7 | 78 |
| ordering | 34 | 7 | 26 |

The new test directories: instantiation 109 of 117 pass, dependent
names 66 of 95. The directories for packs (14 of 54), deduction of class
template arguments (10 of 98), concepts (14 of 105), and constraints
(27 of 101) have no page yet. The front end leaves constraints and the
deduction of class template arguments unknown.

The 37 false rejects among new tests are mostly in those four
directories. In the five pages, 10 false rejects remain. Findings G68,
G69, and G71 explain three. The others are front-end limits: member
templates whose partial or explicit specializations are declared
outside the class (TSP-21, TSP-9), an explicit specialization of a
member class after the class was made (TSP-22), an enumeration of a
class template defined outside the class (TPL-42), placeholder function
parameters (DED-34), and a lambda in a function type (DED-29).

### Run tests

| Directory | Tests | Pass | Wrong result | Missed undefined behavior | Unexpected undefined behavior | Unsupported | Check did not accept |
|---|---|---|---|---|---|---|---|
| classes | 281 | 196 | 1 | 0 | 0 | 80 | 4 |
| declarations | 174 | 156 | 0 | 0 | 0 | 18 | 0 |
| exceptions | 95 | 84 | 0 | 0 | 0 | 11 | 0 |
| expressions | 251 | 183 | 1 | 0 | 0 | 63 | 4 |
| foundations | 77 | 71 | 0 | 0 | 0 | 6 | 0 |
| statements | 77 | 63 | 0 | 0 | 0 | 14 | 0 |
| templates | 92 | 76 | 0 | 0 | 0 | 7 | 9 |
| **total** | 1047 | 829 | 2 | 0 | 0 | 199 | 17 |

In round 7, 2 of 28 template run tests passed. Now 76 of 92 pass.

- The two wrong results are `list-ctor-preferred-over-better-match.cpp`
  (the same as in round 7) and `floating-divide-by-zero-infinity.cpp`.
  The interpreter holds `long double` in 64 bits, so the literal
  `1.1e4932L` is an infinity and one comparison fails. This is a limit
  of the interpreter.
- The five stops at unexpected undefined behavior of round 7 are gone.
  MUL-11 now gives a NaN or an infinity at run time, and
  `braced-order.cpp` was repaired.

## 2. Templates

### The model

A template keeps a record (`TemplInfo`): its parameters with their
kinds, defaults as tokens, the tokens of its declaration, and the scope
stack at the declaration. The parser reads the pattern once. Inside it a
type parameter is a dependent type (`Ty::Dep`), a template-id with
dependent arguments keeps its template and its argument patterns, and an
array bound or a member pointer class can be a parameter. These forms
are the patterns of DED-P3. The checks treat a dependent type as
unknown, so the checks of earlier rounds do not fire on it.

A specialization is made by a replay: the parser reads the template's
tokens again, with a scope that declares each type parameter as its
argument and each constant parameter as its value. The declaration then
goes through the ordinary code, so every check of the front end applies
to a specialization. A replay keeps the state of the outer parse and
puts it back. A specialization is hidden from name lookup and found
through its template.

### What the front end models

- Template-ids for classes, aliases, and variables (TARG-1 to TARG-27),
  with defaults (TARG-9, TPL-15), constant arguments and their
  conversions (TARG-12 to TARG-16), template template arguments
  (TARG-17, TARG-18, TPL-40), and the same specialization by TARG-19.
- Explicit specialization of classes, functions, variables, and members
  (TSP-1 to TSP-8, TSP-22 to TSP-28), partial specialization of class and
  variable templates, selection (TSP-13 to TSP-16), and ordering of
  partial specializations (ORD-14). Checks of TSP-11, TSP-12, and TSP-31.
- Deduction in a call (DED-1 to DED-29), from a target type (DED-30,
  OVL-19), for conversion function templates (DED-31), and for
  declarations of specializations (DED-32, DED-33). Packs at the end of
  the parameters, in function types, and in class arguments (DED-20,
  DED-37).
- Partial ordering for calls, operators, conversion functions, and
  addresses (ORD-1 to ORD-12), with OVL-27 and OVL-28 in the best viable
  function. The member object parameter of ORD-6 is added.
- Lazy instantiation (INST-1 to INST-10). A class is made complete only
  when a use needs it. A member function body, a default argument, a
  default member initializer, an out-of-class static data member
  definition, and a variable template initializer wait for a use. Their
  errors are kept and reported at the use. An unused static data member
  has no definition, so its initialization does not run (INST-8).
- Points of instantiation (INST-30, INST-31). A function specialization
  that constant evaluation can call is made at its first use. Any other
  is made at the end of the unit, where it sees later declarations.
- Explicit instantiation (INST-16 to INST-29).
- Two-phase lookup (DEP rules). In a replay, a namespace shows only the
  declarations before the template's definition (DEP-5).
  Argument-dependent lookup of a dependent call also sees later
  declarations (DEP-9). Unqualified lookup skips dependent bases
  (DEP-11). The pattern's parse records facts at token positions, which
  the replay reads: a qualified name read as a value (DEP-19), a `<` read
  as less-than after a dependent member name (DEP-24), and an independent
  call (DEP-3, DEP-5).
- Pack expansions in call arguments, template arguments, and function
  parameters, `sizeof...`, and fold expressions in specializations
  (PRIM-27 to PRIM-29).
- Definition-time checks of the declarations page: TPL-2 to TPL-4,
  TPL-7, TPL-14, TPL-16 to TPL-18, TPL-20, TPL-21, TPL-25, TPL-27,
  TPL-30, TPL-33, TPL-34, TPL-37, TPL-43.
- The interpreter runs specializations as ordinary functions. Link names
  of specializations carry their arguments. Constant initializers run
  after the variables they name, since a specialization's variable can
  be declared after a variable that reads it.

### What the front end does not model

- Constraints, concepts, and requires-clauses. A constrained template
  leaves its calls and selections unknown.
- Deduction of class template arguments, and a class template as the
  type of a constant parameter (G68).
- Placeholder function parameters (DED-34).
- Partial and explicit specializations of member templates declared
  outside the class (TSP-9, TSP-21), and nested expansions of packs.
- Rewritten comparison candidates with templates (ORD-7). Such a
  comparison stays unknown.
- The type-only positions of DEP-20.

## 3. The repairs of round 7

- MUL-11, MUL-20, CE-28: floating division by zero gives a NaN or an
  infinity at run time. Constant evaluation stops with CE-28.
- OBJ-2: the interpreter names OBJ-2 for every access outside a
  lifetime, and with INIT-1 or NEW-3 for a read of a scalar with no
  value. An implicit copy of an object with a scalar part that has no
  value stops with OBJ-2 at run time (G65).
- TRY-2: the match of a pointer to an object with a `void*` handler
  follows the rule now.
- VIRT-10: steps 1 and 2 of the rule decide the cast. A failed cast of a
  reference throws an exception that only `catch (...)` matches, and is
  not constant.
- UNI-13: an assignment through a chain of member accesses and
  subscripts makes each union member active, when its default
  constructor is trivial. The aggregate initialization of a union also
  gives the active member its parts first.
- TR-11: the harness judges the value of `main` modulo 256.
- OVL-25, LAM-13: against the user-defined first sequence of a surrogate
  candidate, the object slot of a static member is better. Lambdas
  declared `static` now get a closure type.

Fixes found on the way: a reference to an array of `const` elements now
binds an array of the plain elements in overload resolution, and an
immediate-escalating specialization takes on `consteval` (CEV-1).

## 4. Findings

### G68. A class template as the type of a constant parameter

- Page and rules: templates/declarations, TPL-11; templates/arguments,
  TARG-10.
- Kind: missing.
- What happened: TPL-11 lets `template <Wrap W>` take its type from the
  argument, as a variable does. For a class template this is the
  deduction of class template arguments, which no page specifies.
  `constant-parameter-class-template.cpp` needs `Hold<4>` to make a
  `Wrap<int>`. The front end cannot follow the rule.
- Suggestion: specify the deduction of class template arguments, or
  point TPL-11 at the page that will.

### G69. A static local in a constant pointer argument

- Page and rules: templates/arguments, TARG-14; constant-evaluation,
  CE-20.
- Kind: contradictory.
- What happened: `pointer-arguments.cpp` expects `Ptr<address()>` to be
  valid, where `address` returns the address of `static const int kept`.
  CE-20 says constant evaluation must not reach a `static` variable. The
  front end follows CE-20 and rejects the argument.
- Suggestion: allow a `static` variable that is usable in constant
  expressions in CE-20, or change the test.

### G70. The order of the qualifier removal in DED-31

- Page and rules: templates/deduction, DED-31.
- Kind: ambiguous.
- What happened: DED-31 removes top-level `const` and `volatile` from
  both types before the match. For `operator T&()` and a needed
  `const int&`, `context-conversion-exact.cpp` expects `T = const int`.
  For `operator T()` and the same needed type, `context-conversion.cpp`
  expects `T = int`. The text gives `T = int` in both. The front end
  keeps the qualifiers in the exact match when the result is a
  reference, from the tests.
- Suggestion: state that a reference result keeps the qualifiers of the
  referred type in the exact match.

### G71. Placeholder parameters of partial specializations

- Page and rules: templates/specialization, TSP-11, TSP-14;
  templates/deduction, DED-12; templates/ordering, ORD-14.
- Kind: missing.
- What happened: in `partial-specializations-placeholder.cpp`,
  `template <auto* P> struct Kind<P>` matches only pointer values. DED-12
  says a placeholder parameter takes the type of the class template's
  parameter. No rule says that the placeholder form `auto*` must match
  the argument's type, or that `Kind<P>` with such a head is not the
  primary template's list again (TSP-11). The front end selects by the
  values only and fails the test.
- Suggestion: state that the declared form of a placeholder constant
  parameter is a pattern that the argument's type must match.

### G72. "Still to be deduced" in DED-3

- Page and rules: templates/deduction, DED-3, DED-21.
- Kind: ambiguous.
- What happened: DED-3 says that a parameter whose type holds no
  template parameter "still to be deduced" takes no part. One reading
  skips a parameter whose template parameters an earlier pair already
  gave. Then `larger(1, 2L)` never meets the conflict of DED-21. The
  front end reads "still to be deduced" as "not fixed by an explicit
  argument", which `combine-conflict.cpp` needs.
- Suggestion: say "that explicit arguments do not fix".

### G73. TPL-14 and defaults from several declarations

- Page and rules: templates/declarations, TPL-14, TPL-15.
- Kind: ambiguous.
- What happened: in `defaults-merge.cpp` the second declaration gives a
  default to the first parameter only, and the earlier declaration gave
  the second one. TPL-14 on that declaration alone rejects it. The test
  expects the defaults of all declarations to count. The front end
  follows the test.
- Suggestion: state that TPL-14 applies to the defaults of the
  declaration together with the earlier ones.

### G74. Deduction from a base class and instantiation

- Page and rules: templates/deduction, DED-15; templates/instantiation,
  INST-1.
- Kind: missing.
- What happened: DED-15 searches the bases of the argument's class. The
  bases of a class template specialization are known only after its
  instantiation. INST-1 does not list deduction among the uses that
  instantiate a class. The front end instantiates the class there.
- Suggestion: add deduction from a derived class to the list of INST-1,
  or say that INST-3 covers it.

### G75. Requires-expressions in the tests of IF-13

- Page and rules: statements/if, IF-13; templates/dependent-names.
- Kind: outside knowledge.
- What happened: `constexpr-discard-in-template.cpp` uses a
  requires-expression as the condition of `if constexpr`. Requires
  expressions are not specified yet. In a specialization the front end
  evaluates each simple requirement as valid or not, from outside
  knowledge, so the test runs.
- Suggestion: specify requires-expressions, or use a condition that the
  specification covers.
