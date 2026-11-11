# I6: dependent names and the checks before instantiation, from the rules alone

Date: 2026-11-10
Commit read: `98efc40f37190ddba31c5548cacfd49061c9a8f0`
Kind: Implement (decision 0061, item E)
Role: clean-room reader in the role of an implementer

The reader returned this report as a message, because the write of `report.md` was refused. The integrator saved it here with the reader's files.

## Inputs

Blind phase. I read these pages only through `filter.py`, which is a copy of the I5 filter. The filter drops each `## Evidence` segment with its subsections and each line with `:tests:`.

- `templates/dependent-names.md`, all of it
- `templates/instantiation.md`: INST-P1 to INST-P5, INST-1 to INST-13 with INST-36, INST-3, INST-11, INST-30, INST-31, INST-33 and INST-15
- DED-29 from `templates/deduction.md`
- the glossary entries on the dependence terms, the current instantiation, the point of instantiation, the templated context and the discarded statement
- the linked rules NAM-27, NAM-37, TARG-2, TARG-23, SA-5, IF-13 and CNS-13

I did not read `tests/`, `questions/` or `frontend/` in the blind phase. `notes.md` lists 28 steps that the text did not give me. I wrote them before I read any test.

The component is `depnames.py`. It takes a template definition as a small tree of types, expressions and a context. The context holds the parameters, the bases, the members, the member aliases and the locals. The component answers these questions:

- whether a type is dependent (DEP-1), and whether an expression is type-dependent or value-dependent (DEP-2 to DEP-4)
- whether a type names the current instantiation (DEP-15)
- where a name binds: at the definition, at the instantiation, or at the instantiation with argument-dependent lookup (DEP-5 to DEP-18, DEP-39)
- which function a call finds, for a set of declarations with positions (DEP-8 to DEP-10, NAM-27, INST-31)
- whether `typename` or `template` is needed, how the parser reads a qualified name, and what the second phase checks (DEP-16 to DEP-27, DEP-34)
- which checks run before any instantiation, as "must", "may" (a no-diagnostic rule) or "none" (DEP-28 to DEP-33, DEP-37, SA-5, IF-13, CNS-13)
- which uses instantiate a class, a function, a variable, a default argument or a default member initializer, and which parts the class makes (INST-1 to INST-13)

The sha1 of `depnames.py` is `633d6346de48f9c5ce96155ddf784400dae7c51c`, recorded in `depnames.sha1`. `score.py` checks the hash before it runs. The file is read-only and did not change after the blind phase.

Scoring phase. I read these tests:

- the 128 tests in `tests/templates/dependent-names/`
- the three probe grids and `grids/tables.md`
- the 70 tests elsewhere whose tidal-rules line cites a rule from INST-1 to INST-13 or INST-36

I built the front end into my scratch directory with no build failure. I wrote 14 programs in `programs/`. Each ran under g++ and clang++ with `-std=c++23 -pedantic-errors`, and under `tidal-fe run`. Two reductions are in `reductions/`.

## Score

352 of 358 assertions are right in the strict reading (`score-output.txt`). 354 of 358 are right when `at_instantiation()` decides the second phase (`score-output-second-phase.txt`).

| Source | Tests scored | Assertions | Right (strict) | Right (second phase) |
|---|---|---|---|---|
| `tests/templates/dependent-names/*.cpp` | 128 | 196 | 190 | 192 |
| probe grids (`typename-positions`, `template-keyword`, `current-instantiation`) | 3 | 86 | 86 | 86 |
| tests that cite INST-1 to INST-13 or INST-36 | 63 | 76 | 76 | 76 |

The two readings differ in one way. `keywords()` marks some errors "at-instantiation", such as DEP-19 for `T::type` in a position where the parser reads a value. `at_instantiation()` then takes the member that the arguments give and says whether the error happens. The strict reading rejects an instantiated template whenever `keywords()` marks such an error. The second-phase reading lets `at_instantiation()` decide when the case calls it. I wrote the second reading after the first run, so I report both.

The score leaves out 7 tests whose assertions the component cannot express:

- `class-declared-not-defined` (INST-4). The component has no call for a class template that is declared and not defined.
- `member-partial-specializations-collide` (INST-6, last sentence). The component has no model of member partial specializations.
- `local-entities-per-specialization` and `local-entities-thread-local-and-nested-lambda` (INST-36). The component has no model of local statics or closure types.
- the three `local-class-requires-clause-*` tests that need constraint satisfaction at the use (CNS-12). The fourth, `local-class-requires-clause-waits`, is scored.

A score this high needs a caveat. Many assertions pass because the caller supplies a fact that the component cannot work out from the tree:

- whether some argument list makes a construct valid (`valid=` in `Construct`)
- what a qualified name finds for the arguments (`member_kind` in `at_instantiation()`)
- the members of the independent bases (`base_members`). The component never searches a base that the caller leaves out, so part of DEP-11 sits in the caller.

The score shows that the component reads the rules as the tests do once those facts are given. It does not show that the component can find those facts.

These are the wrong assertions:

- `independent-call-early` and `member-alias-independent`. In each test, an independent call has an argument of class type, and the function is in that class's namespace. The component runs no argument-dependent lookup for an independent call, so it finds no function (I6-1).
- `template-keyword-destructor` and `template-keyword-destructor-dot`. `keywords()` checks DEP-34 only on the path for a template-id. `p->template ~T()` has no argument list, so the check never runs (I6-2).
- Strict reading only: `template-keyword-no-list-class`. `keywords()` marks DEP-26 for `typename T::template box` with no argument list, although DEP-26 allows a class template whose arguments are deduced (I6-3).
- Strict reading only: `parameter-position-variable-template`. `keywords()` marks DEP-19 because the tree has a parameter declaration. `Limit::type` is a value, so no error happens (I6-4).

Every test that expects a template to be rejected before instantiation comes out "must" or "may". Every no-diagnostic test comes out "may". Every test that expects a template to be accepted comes out "none" or "may".

## Findings

### I6-1. DEP-5 does not say that an independent call uses argument-dependent lookup at the definition

- Pages and rules: `templates/dependent-names.md`, DEP-5, DEP-P1 (the lookup table), DEP-9, NAM-27.
- Kind: missing.
- What happened: DEP-5 says a name that is not dependent is "looked up at the definition". The DEP-P1 table lists argument-dependent lookup only for a call with a dependent argument. I read that as ordinary lookup alone for an independent call. In `independent-call-early`, the template is in the global namespace, and `probe(lib::Tag{}, sizeof(T))` finds `lib::probe` only through argument-dependent lookup at the definition. The component found nothing.
- Suggestion: say in DEP-5 that an independent unqualified call uses both ordinary lookup and argument-dependent lookup, both at the definition. Give the DEP-P1 table a row for the independent call.

### I6-2. The component missed DEP-34 for a destructor with no argument list

- Pages and rules: DEP-34, DEP-P5.
- Kind: none (a mistake in the component).
- What happened: DEP-34 is clear. My `keywords()` tested it only on the template-id path, and `p->template ~T()` has no list.
- Suggestion: none for the text. The DEP-P5 table could add a row for `x.template ~T()`, to put the rule next to the other forms.

### I6-3. DEP-26 has no example of `template` with no list before a deduced class template

- Pages and rules: DEP-26, the DEP-P5 table, Q-0226.
- Kind: needs example.
- What happened: the DEP-P5 table shows `typename T::template name<...>` with a list in each row. DEP-26 allows no list "for a class template or alias template whose arguments are deduced". In `keywords()` I treated "no list" as a DEP-26 error, while `at_instantiation()` follows DEP-26 and accepts. The two parts of the component disagree.
- Suggestion: add a DEP-P5 row for `typename T::template box b(1);`, read as a type whose arguments are deduced.

### I6-4. The two phases of a qualified name need one place that decides

- Pages and rules: DEP-19, DEP-22, DEP-26, DEP-P5.
- Kind: unclear.
- What happened: the DEP-P5 table has a column "Checked at the instantiation". DEP-19, DEP-22 and DEP-26 each say what the second phase checks. My component split the first phase (`keywords()`) from the second (`at_instantiation()`), and I let `keywords()` predict a second-phase error from the program's intent. That gave the two wrong assertions in the strict reading. The text is right. The table puts the first-phase reading and the second-phase check in one row, which made the split easy to get wrong.
- Suggestion: in DEP-P5, add a sentence saying the first phase only chooses a reading, and only the second phase reports an error, from what the name finds for the arguments.

### I6-5. A member access through `this` and type dependence

- Pages and rules: DEP-2 (the bullets on `this`, on a member access, and on a subexpression), DEP-13, DEP-17 (notes 1).
- Kind: unclear.
- What happened: DEP-2 makes `this` type-dependent in a member of a class template. The last bullet then makes every `this->n` type-dependent, so the bullet "a member access names a member whose type is dependent" adds nothing. I read the member bullet as the rule for a member of the current instantiation. Then `this->n` with `int n` is not type-dependent, and `f(this->n)` binds at the definition. No test decides this reading.
- Suggestion: say in DEP-2 that a member access through `this`, or through an object of the current instantiation, is type-dependent only when the member's type is dependent, or when the name finds nothing and the class has a dependent base (DEP-18). Add a test with `f(this->n)` and a later `f(int)`.

### I6-6. `sizeof` of a value-dependent operand

- Pages and rules: DEP-4 (the `sizeof` bullet and the last bullet) (notes 2).
- Kind: unclear.
- What happened: DEP-4's last bullet makes an expression value-dependent if "an operand is value-dependent". Read literally, `sizeof(N)` with a constant parameter `N` is value-dependent. The `sizeof` bullet names only a dependent type or a type-dependent operand. I excluded `sizeof`, `alignof` and `noexcept` from the last bullet. No test decides this reading.
- Suggestion: write "an operand is value-dependent, except the operand of `sizeof`, `alignof` and `noexcept`".

### I6-7. DEP-30 against DEP-6 and DEP-12 before instantiation

- Pages and rules: DEP-6, DEP-12, DEP-17, DEP-30, DEP-P6, the example "A dependent base" (notes 10).
- Kind: unclear.
- What happened: DEP-30 lists "a name in the body" among the constructs that need no diagnostic when no specialization is made. DEP-17 points to DEP-30 by name, and DEP-6 and DEP-12 do not. The example "A dependent base" is marked reject with no instantiation. I gave DEP-17 "may", and DEP-6 and DEP-12 "must". The tests `independent-error-unused-name` and `independent-error-unused-base-member` cite DEP-30 with DEP-12. So the tests read DEP-30 as covering these names, but no test note says whether a compiler may accept. My answer matched the tests' outcome, but I could not tell from the text whether "must" or "may" was meant.
- Suggestion: say in DEP-6 and DEP-12 whether DEP-30 applies when no specialization is made. If every baseline compiler rejects, say that the rule is "may" and the compilers reject, as the CNS-13 row of DEP-P6 does.

### I6-8. CNS-13 and the DEP-P6 table give different strengths

- Pages and rules: DEP-P6 (the CNS-13 row), CNS-13 (notes 11).
- Kind: unclear.
- What happened: the DEP-P6 row says "need not diagnose; every baseline compiler rejects". CNS-13 is kind reject and applies "also inside a template that is never instantiated". I chose "must". No test in my set decides it.
- Suggestion: make the two agree. If CNS-13 is a reject rule there, change the table's requirement to "rejects".

### I6-9. Qualified dependent calls, and member calls with a dependent argument

- Pages and rules: DEP-8, DEP-9, DEP-14, DEP-31 (third bullet) (notes 5 and 7).
- Kind: missing.
- What happened: DEP-8 and DEP-9 cover an unqualified call. The pages do not say when `ns::f(t)` binds with a dependent argument, or that it takes no argument-dependent lookup. DEP-31's third bullet hints at the answer through `qualified-call-later-default` (Q-0249). Also, DEP-14 keeps a member that unqualified lookup finds, and DEP-8 makes the call's name dependent when an argument is type-dependent. The text does not say whether argument-dependent lookup then adds functions. GCC, Clang and the front end agree on `programs/p09-member-hides-adl.cpp`: the member wins and no argument-dependent lookup runs.
- Suggestion: add to DEP-9 that a qualified call with a dependent argument looks up its name at the definition and at the instantiation in the named scope, with no argument-dependent lookup. Add that an unqualified call whose name finds a class member takes no argument-dependent lookup. Link the names page.

### I6-10. Which point of instantiation argument-dependent lookup uses

- Pages and rules: DEP-9, DEP-P1 table, INST-31, INST-32 (notes 6).
- Kind: unclear.
- What happened: DEP-9 says "before the point of instantiation", as if there were one. INST-31 gives a function specialization several points, the last at the end of the translation unit, and lets a use see declarations up to the last point. I let lookup see everything before the end, and flag INST-32 for a function after the first point. The tests do not separate the two readings.
- Suggestion: in DEP-9, write "before a point of instantiation (INST-31)", and say that the points must agree (INST-32).

### I6-11. The front end stops on `this->template` with a dependent base

- Pages and rules: DEP-24, DEP-11, DEP-18.
- Kind: front end.
- What happened: `reductions/r1-this-template-dependent-base.cpp` has `template <class T> struct Derived : T { int run() { return this->template get<3>(); } };` and is never instantiated. GCC and Clang accept it. The front end prints "unsupported at line 4: check: parse: expected an expression, found ')'". With a type argument, `get<int>`, it prints "expected '(' or '{' after a type name in an expression". The same call runs when `get` is in the class itself or in an independent base, and `Base<T>::template get<3>()` also runs. The front end seems to ignore `template` when lookup at the definition finds nothing, and to read `<` as less-than.
- Suggestion: after `.template` or `->template`, always parse a template argument list (DEP-24). Add the reduction as an accept test and `programs/p02` as a run test.

### I6-12. The front end does not find an operator in the namespace of a base class

- Pages and rules: NAM-27 (the row "A class with bases"), DEP-9.
- Kind: front end.
- What happened: `reductions/r2-operator-base-namespace.cpp` declares `geo::Shape` with `int operator+(Shape, Shape)` in `geo`, and `struct Circle : geo::Shape {}` in the global namespace. `Circle{} + Circle{}` runs on GCC and Clang. The front end rejects it with "[ADD-3] no operator '+' for class operand". No template is needed. A named call `probe(Circle{})` finds `geo::probe` correctly, and the operator works when `Circle` is itself in `geo`. So the front end's operator lookup leaves out the namespaces of the bases. I found the gap with the template program `programs/p05-operator-adl-base-namespace.cpp`.
- Suggestion: give operator lookup the same associated namespaces as a named call. Add the reduction as a run test.

### I6-13. Which uses instantiate a class: `decltype` of a call, and a call by value (I5 notes item 4)

- Pages and rules: INST-1, INST-P1 table, INST-7 (notes 19, 20, 22).
- Kind: missing.
- What happened: the text now decides the case in I5 notes item 4. INST-1 lists a qualified name that names a member, and a deduction that searches the bases. Two cases are still open.
  - INST-1 lists "a call to a function that returns the class" and does not say whether the call in `decltype` counts. GCC, Clang and the front end do not instantiate the class (`programs/p13-decltype-call-returning-class.cpp`).
  - The INST-P1 table lists "a call with the class by value", and INST-1 does not list a call that passes the class by value. GCC, Clang and the front end instantiate the class (`programs/p14-call-passes-class-by-value.cpp`).
- Suggestion: add to INST-1 "an argument that the call passes by value", and add to INST-2 "a call inside `decltype` whose result is the class". Add p13 and p14 as tests.

### I6-14. A hard error in an instantiated class, and the other candidates (I5 notes item 23)

- Pages and rules: INST-6, INST-P2, DED-29 (notes 21).
- Kind: needs example.
- What happened: the text decides most of I5 notes item 23. INST-6 and INST-P2 say that an error in a declaration made with the class makes the use not valid. An error in a waiting definition makes only its own use not valid. DED-29 makes the first kind a hard error during deduction. The text does not say in so many words that the hard error stands when another candidate would win the call. "Makes the program not valid" implies it. One example would settle it.
- Suggestion: add an example to DED-29 with a fallback `f(...)` and a template whose function type instantiates a class with a `static_assert` that fails. Mark it reject.

### I6-15. INST-5 and INST-33 on a `static constexpr` member

- Pages and rules: INST-5, INST-33, INST-P2 (notes 23).
- Kind: unclear.
- What happened: INST-5 says the initializers of `static` data members wait, "also `inline` ones". INST-33 says that the initializer of a `static constexpr` data member in the class body is instantiated with the class. A reader of INST-5 alone gets the `constexpr` case wrong. No test in my set covers it.
- Suggestion: add to INST-5 "except a `static constexpr` member whose initializer is in the class (INST-33)", and add a row to the INST-P2 table.

### I6-16. When a friend function defined in a class template is made

- Pages and rules: INST-13, INST-5, INST-P2 (notes 24).
- Kind: missing.
- What happened: INST-13 says that each instantiation defines the friend again. INST-5 and the INST-P2 table do not list friends, so I could not tell whether the friend's body is made with the class or waits for a use. I made the definition with the class. `friend-defined-twice` agrees for the redefinition. The text does not say whether an error in the friend's body is checked without a call.
- Suggestion: add a row for a friend function defined in the class to the INST-P2 table.

### I6-17. DEP-37 needs the arity of each operator

- Pages and rules: DEP-37 (notes 9).
- Kind: hard to find.
- What happened: DEP-37 needs the number of parameters that each operator function takes, as a member and as a non-member. The page neither gives the numbers nor links them. I used my own knowledge.
- Suggestion: link the rule on operator function parameters from DEP-37.

### I6-18. Why `T::x * p;` and `T::x p;` are rejected at the definition

- Pages and rules: DEP-19, DEP-29, DEP-6, the example "The keywords" (notes 12, 13).
- Kind: needs example.
- What happened: the example marks `C::value_type* q = nullptr;` rejected, though the template is never instantiated. DEP-19 makes the line a product, but the text does not say why the definition is rejected. I took the reason to be the undeclared `q` (DEP-6). For `T::type value = 0;`, two expressions stand side by side, so the grammar breaks (DEP-29). The tests `typename-needed-block` and `other-specialization-needs-typename` cite DEP-19, and EDG reports `expected a ";"`. So the compilers report a grammar error.
- Suggestion: add a comment to the example: "rejected: a product with the undeclared name q (DEP-6)". Add one line for `T::type value;` with DEP-29.

### I6-19. DEP-20 and the return type of a member function

- Pages and rules: DEP-20 (notes 14).
- Kind: unclear.
- What happened: DEP-20 lists "the type of a member" and "the parameter types of a member function". I read "the type of a member" to cover a member function's return type. The grid row "member function return, in class" agrees. A member function's return type outside the class is a namespace-scope return type, which the third bullet covers.
- Suggestion: write "the type of a member and the return type of a member function".

### I6-20. Knowledge the pages did not give

- Pages and rules: DEP-9, DEP-37, DEP-2, INST-12 (notes 4, 8, 9, 28).
- Kind: outside knowledge.
- What happened: I took these from my own knowledge:
  - the ranking of the conversions that overload resolution uses among the found functions (notes 8),
  - the scope that ordinary lookup searches from the definition (notes 4),
  - the operator arities (I6-17).
  - I also had to choose whether `this` in a member of a local class of a function template is type-dependent (notes 28). DEP-2 names only a member of a class template. I said no.
- Suggestion: DEP-2 could say whether `this` in a local class of a function template is type-dependent.

## Programs

Each program ran under g++ and clang++ with `-std=c++23 -pedantic-errors`, and under `tidal-fe run`.

| Program | Rules | g++ | clang++ | tidal-fe |
|---|---|---|---|---|
| p01-adl-through-template-argument | DEP-9, NAM-27 | 0 | 0 | 0 |
| p02-this-template-member | DEP-24, DEP-11 | 0 | 0 | unsupported (I6-11) |
| p03-typename-template-rebind | DEP-19, DEP-24 | 0 | 0 | 0 |
| p04-using-value-from-dependent-base | DEP-11, DEP-23 | 0 | 0 | 0 |
| p05-operator-adl-base-namespace | DEP-9, NAM-27 | 0 | 0 | rejected ADD-3 (I6-12) |
| p06-independent-binds-early | DEP-5 | 0 | 0 | 0 |
| p07-nested-member-outside | DEP-15, DEP-16 | 0 | 0 | 0 |
| p08-discarded-dependent | IF-13, DEP-28 | 0 | 0 | 0 |
| p09-member-hides-adl | DEP-14 | 0 | 0 | 0 |
| p10-arrow-template-constant | DEP-24 | 0 | 0 | 0 |
| p11-hidden-friend-adl | DEP-9, INST-13 | 0 | 0 | 0 |
| p12-decltype-dependent-return | DEP-1, DEP-2 | 0 | 0 | 0 |
| p13-decltype-call-returning-class | INST-1, INST-7 | 0 | 0 | 0 |
| p14-call-passes-class-by-value | INST-1, INST-P1 | rejects | rejects | rejects SA-1 |

p14 is meant to be rejected. The three tools agree on it.

## Reductions

| Reduction | From | Stop |
|---|---|---|
| `reductions/r1-this-template-dependent-base.cpp` | p02 | unsupported at line 4: check: parse: expected an expression, found ')' |
| `reductions/r2-operator-base-namespace.cpp` | p05 | rejected [ADD-3] at line 5: no operator '+' for class operand |

r1 stops in the same way with no instantiation. r2 needs no template.

## What worked well

- The DEP-P5 table and DEP-20's list gave all 42 cells of the `typename-positions` grid, the split cells included.
- DEP-15's list, with the alias bullets, gave every row of the `current-instantiation` grid. The `Same<T>` and `Pointer<T>` rows came out right from the alias template rule alone.
- The INST-P1 and INST-P2 tables gave all 76 scored instantiation assertions, the `decltype`, `sizeof` and braced-initializer cases of INST-7 included.
- The DEP-P6 table gave the strength of each check before instantiation. Every no-diagnostic test came out "may".
- DEP-8's four bullets and NAM-27's table gave every argument-dependent lookup test. That covers the pack, the braced list, explicit template arguments, the function type with a pack, and the `noexcept` condition.
- Twelve of the fourteen programs ran the same on GCC, Clang and the front end.
