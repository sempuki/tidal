# Template model for the rewrite

This document gives the model of templates that the rewrite of the
front end needs (decision 0060, decision 0062 item RP). It comes from
the templates chapter of the specification, from the 32 tests that
fail on the current front end because of its model, and from the three
facts that the I6 component could not compute
(`reports/clean-room/I6/report.md`, section "Score").
`reports/clean-room/rust-frontend/rewrite-marks.tsv` (the marks file)
names the 32 tests and the section of this document that passes each.
The rewrite takes them over, and the gate of decisions 0062 and 0063
does not count them (decision 0063, item GC). `DESIGN-current.md`
describes the current template record, and this document refers to it
as "the current model".

The model says what the rewrite represents and when it decides each
fact. It does not fix the Rust types. The structures below are
sketches.

## The current model and its limits

The current model keeps the tokens of each template. It parses them
once as a pattern, with each parameter as a dependent type, and again
for each specialization, with each parameter bound to its argument
(`replay`). The two parses share only facts recorded under token
positions.

| Limit | Effect | Rules | Tests that fail |
|---|---|---|---|
| In a pattern, the type of an object of the current instantiation is `Ty::Unknown`. | A member access on such an object cannot find the member template, and `<` reads as the less-than operator. A constructor that takes the class by value is not seen (CPY-15). | DEP-2, DEP-15, DEP-24, CPY-15 | 5 |
| A base of a pattern class is a known class or an unknown one (`BaseInfo::class` is `None`). The current instantiation of an enclosing class is an unknown base. | Unqualified lookup in the class is uncertain, and a name that the base declares is unknown. | DEP-11 | 1 |
| A template-id with independent arguments, inside the template's own definition, gets no special form. | `Meter<0>` inside `Meter<N>` is treated as dependent, so its members are not looked up at the definition. | DEP-1, DEP-15, DEP-30 | 2 |
| An expression has no value-dependence mark. Any template argument that names a parameter makes the template-id dependent. | `Slot<sizeof(N)>` is treated as dependent, though DEP-4 does not make `sizeof(N)` value-dependent, so a call binds late and `Box<sizeof(N)>::type` needs `typename`. | DEP-4, DEP-5 | 2 |
| A specialization's lookup has no point of instantiation. | Lookup in a specialization sees declarations after its point, and a `static` member's initializer is made with the class instead of at its use. | INST-5, INST-8, INST-30 to INST-33 | 7 |
| The members of a local class in a specialization are made at their use. | Errors in a default argument or an exception specification of such a member are missed when nothing uses it. | INST-6, INST-12 | 2 |
| Function bodies made at the end of the unit have no `busy` state. | Endless nesting of such bodies is not caught. | INST-15, TR-13 | 1 |
| A substitution is a replay with every diagnostic muted, and any muted error is a failure. | An error outside the immediate context, such as in a lambda body, becomes a quiet failure instead of an error. A member of the wrong kind, or a conditional `explicit`, stops the replay as unsupported. | DED-24, DED-29 | 4 |
| The check of an expansion pattern scans tokens. | The scan does not know that a fold or `sizeof...` inside the pattern uses up the pack. | PACK-8 | 2 |
| The pattern's parse does not resolve an independent call in a discarded statement. | A call with too few arguments there is accepted. | IF-22, DEP-6 | 1 |
| The pattern of an operator function is not checked for a pack that only an empty list fits. | The definition is accepted. | DEP-37 | 2 |
| The trailing clause of a member of a class template is not kept for the definition outside the class. | A definition with another clause, or none, matches the member. | CNS-29 | 2 |
| A variable partial specialization with no primary takes a path that does not reach the TSP-31 check. | The program is accepted. | TSP-31 | 1 |

## 1. A template definition is a typed tree with dependence

The rewrite parses a template definition once, into the same typed
trees as other code. A node whose type or value depends on a parameter
says so. The tree is the template. A specialization is made from the
tree (section 3), and the tokens are not parsed again.

Each expression node carries these marks:

| Mark | Meaning | Rules |
|---|---|---|
| type-dependent | the node's type differs between specializations | DEP-2, DEP-3 |
| value-dependent | the type is fixed, and the value differs; the operand of `sizeof`, `alignof`, `noexcept`, or `typeid` passes on no value-dependence, only type-dependence | DEP-4 |
| unexpanded packs | the packs that the node names and that no expansion inside the node expands | PACK-8, PACK-13, PACK-19 |
| binding | for a name: the declarations found at the definition, or "dependent name"; a qualified call with an independent qualifier, and an unqualified call whose name finds a member, bind at the definition also with a dependent argument | DEP-5, DEP-7, DEP-8, DEP-40 |

A type is a known type or a dependent type. The dependent forms extend
the current `Dep` (`types.rs`) with the forms that the current model
reads as `Ty::Unknown`:

```text
DepType = Param(depth, index)                  a template parameter (DEP-1)
        | CurrentInst(class, nesting)          the current instantiation (DEP-15)
        | Id(template, [Arg])                  another specialization with a
                                               dependent argument (DEP-1)
        | Member(DepType, name, reading)       T::name, with the reading that the
                                               first phase chose (DEP-19, DEP-20,
                                               DEP-22, DEP-26)
        | Decltype(expression)                 decltype of a type-dependent
                                               expression (DEP-1)
        | Built(kind, DepType, ...)            a pointer, reference, array,
                                               function, or member pointer built
                                               from a dependent type (DEP-1)
reading = type | value | template | class template
```

`Id` with no dependent argument is no `DepType`. Inside `Meter<N>`,
`Meter<0>` is an ordinary class specialization (DEP-15, last
paragraph), and the definition looks up its members.

The first phase only chooses a reading for `T::name`. The second phase
reports the error when what the name finds does not fit the reading
(DEP-P5, DEP-19, DEP-22, DEP-26).

A member access through `this`, or through an object of type
`CurrentInst`, is type-dependent only when the member's type is
dependent, or when the name finds nothing and the class has a dependent
base (DEP-2, the paragraph after the list). So `this->n`, where `n` has
the type `int`, has the type `int` at the definition.

## 2. The current instantiation is a class with known members

Inside a class template, its members, and its nested classes, the
class being defined is an entity whose members the definition knows.
Its record holds:

```text
CurrentInst {
  template, nesting               which class template, which nested class
  members: name -> [declaration]  every member declared so far (NAM-6 for bodies)
  independent_bases: [class]      complete classes, and the current
                                  instantiation of an enclosing class (DEP-11)
  dependent_bases: [DepType]      not searched at the definition (DEP-11, DEP-12)
  partial_args: [Arg] or none     the own list of a partial specialization (DEP-15)
}
```

The names of DEP-15 give `CurrentInst`: the injected class name, the
template name with its own parameters in order, the list of a partial
specialization, a nested class's name, and an alias that gives one of
these. `this` has type pointer to `CurrentInst` (DEP-2). A parameter or
local declared with the class's own name, such as `const Wrap& w`, has
type reference to `CurrentInst`.

Lookup of a name in `CurrentInst` at the definition follows one
procedure, for `X::name`, for `this->name`, and for `obj.name` where
`obj` has type `CurrentInst`:

```text
search members, then independent_bases (DEP-11, DEP-36)
  found          ──► bind it (DEP-14, DEP-16); a member template
                     needs no `template` keyword (DEP-24)
  not found:
    no dependent base  ──► reject (DEP-17)
    dependent bases    ──► dependent name, looked up again at the
                           instantiation, also in the bases (DEP-18);
                           a member template needs `template` (DEP-24)
```

At the instantiation, the name is looked up again in the
specialization. An ambiguity is an error (DEP-13), and a different
member is a no-diagnostic error (DEP-35).

A member access on a type-dependent object that is not `CurrentInst`
binds nothing at the definition (DEP-39).

The I6 caller had to supply the members of the independent bases
(`base_members`). With this record, `independent_bases` gives them.
Each is a complete class at the definition (an independent base must
be complete, DER-1), or `CurrentInst` of an enclosing class, whose
members are known in the same way.

## 3. A specialization is the definition plus a substitution

A specialization is made by substituting the arguments into the tree
of the definition:

- A node with no dependence keeps its type and its binding from the
  definition (DEP-5). The substitution copies it.
- A dependent type takes the arguments (DEP-7). `Member(T, name,
  reading)` becomes a lookup in the class that `T` became (section 4).
- A dependent name is looked up again (DEP-7). A dependent unqualified
  call adds argument-dependent lookup at the point of instantiation
  (DEP-9, DEP-10).
- Each dependent construct is checked for these arguments (DEP-28).

The current model gets the first item from a limit on lookup
(`Source::limit`) and from the pattern facts. The substitution makes
both unnecessary.

The record of a specialization:

```text
Specialization {
  template, args: [Arg]                   the key (TARG-P2)
  source: explicit | partial(index, deduced args) | primary   (TSP-13 to TSP-16)
  point: position in the unit             (INST-30, INST-31)
  state: declared | defined | busy | failed
  members: name -> declaration, each with its own definition state
}
```

The parts of a class specialization are made in the order of INST-P2:
the member declarations with the class (INST-5, INST-6), and each
definition at its first use (INST-7 to INST-10). A function or a
`static` data member has a point after the declaration that uses it
and a point at the end of the unit (INST-31). Lookup at the
instantiation sees only the declarations before the point (INST-33).
`busy` marks a specialization whose definition uses itself, and
nesting deeper than the limit of TR-13 is an error (INST-15).

A partial specialization is a template of its own with a head, an
argument pattern, and a link to its primary (TSP-9). A partial
specialization whose primary is not declared is an error when it is
declared (TSP-31). Selection uses deduction against the pattern
(TSP-14) and partial ordering (TSP-15, ORD-14), as the current model
does.

## 4. Substitution failure needs the immediate context

Deduction substitutes into the declaration of a function template
(DED-24). The substitution has two results, kept apart:

| Where the error is | Result | Rules |
|---|---|---|
| the function type, a template parameter's type or default, the operand of a conditional `explicit` | a failure: the template adds no candidate | DED-24, DED-25, DED-28 |
| access to a name in those places | a failure | DED-26 |
| the body of a class that the substitution instantiates | an error of the program | DED-29 |
| the body of another function template that the type needs | an error of the program | DED-29 |
| the body of a lambda in the function type or in a template parameter | an error of the program | DED-29 |
| the `noexcept` operand of the chosen specialization | an error of the program | DED-27, DED-29 |

So the substitution of a declaration returns a failure value for the
first three rows. Any instantiation it causes goes through section 3,
whose errors are errors of the program, also during deduction. A
diagnostic switch that mutes every error cannot make this split.

`Member(T, name, reading)` is where most failures come from. The
substitution makes the class that `T` names complete (INST-1), looks up
`name` in it, and compares the kind of the result with the reading:

| Reading | Finds a type | Finds a value | Finds a template | Finds nothing |
|---|---|---|---|---|
| type (`typename`) | the type | failure, else DEP-22 | failure, else DEP-22 | failure, else DEP-22 |
| value | failure, else DEP-19 | the value | failure, else DEP-19 | failure, else a lookup error |
| template (`template`) | failure, else DEP-26 | failure, else DEP-26 | the template | failure, else DEP-26 |

"Failure, else" means a substitution failure inside the immediate
context of deduction (DED-24, the bullet on a member of the wrong
kind), and an error of the program elsewhere. A `T` that is not a class
or an enumeration, or an incomplete class, is a failure in the same
way (DED-24).

This table is the second of the facts that the I6 caller supplied
(`member_kind` in `at_instantiation()`): what a qualified name finds
for given arguments. In the model, the substitution computes the fact.

## 5. Checks at the definition

The typed tree lets the definition check each construct that does not
depend on the parameters. The marks of section 1 decide which:

| Construct | Check at the definition | Rules |
|---|---|---|
| an independent name | must be declared before the definition | DEP-6 |
| an independent construct, also in a discarded statement | must be valid; a no-diagnostic rule, and every baseline compiler checks | DEP-30, IF-22 |
| a name in `CurrentInst` with no dependent base | must be found | DEP-17 |
| a member template after `.`, `->`, or `::` | the keyword, by section 2 | DEP-24 |
| a declaration that the grammar does not derive with the chosen readings | rejected | DEP-29 |
| the pattern of an expansion | must hold a pack in its unexpanded packs | PACK-8 |
| an operator function that only an empty pack makes valid | rejected | DEP-37 |
| a definition of a member outside its class | its trailing clause must be equivalent to the member's | CNS-29 |
| a `static_assert` | waits for the instantiation | SA-5 |

The unexpanded-packs mark makes PACK-8 a property of the tree. A fold
(PRIM-27) and `sizeof...` (PACK-19) remove their pack from the mark of
the node that holds them, so `(N + ...)...` has an outer pattern whose
mark is empty.

CNS-29 needs the member's trailing clause in its declaration record,
as a normal form (CNS-P1). The definition outside the class then
compares its own clause with that form when it matches the member, as
CNS-26 compares template heads.

The first of the I6 caller's facts is whether some argument list makes
a construct valid (`valid=` in `Construct`). In the model, the marks
give it for each independent construct: the check at the definition
says valid for every list or for none. A dependent construct counts as
valid for some list. The model does not search for a list that makes a
dependent construct valid. DEP-32 and DEP-33 need no diagnostic, so an
implementation that accepts follows the text (TR-P1). DEP-37 is the
case of that kind that the text requires the implementation to reject.

## 6. The 32 tests

The marks file, `reports/clean-room/rust-frontend/rewrite-marks.tsv`,
names the tests that fail on the current front end because of its
model. Report 25 section 7 marked 30 of them, and report 26 marked two more, and `tools/frontend_check.py`
counts them apart on its completion line (decision 0063). This table
follows the marks file, by the section of this model that passes each
test. A test path is under `tests/`.

| Test | Rules | Now | Section | Part of the model |
|---|---|---|---|---|
| `templates/dependent-names/other-specialization-constant-argument-member-missing.cpp` | DEP-30, DEP-15 | missed reject | 1 | `o.missing` is independent and checked at the definition (sections 1 and 5) |
| `templates/dependent-names/other-specialization-constant-argument-no-typename.cpp` | DEP-15, DEP-1 | unsupported | 1 | `Meter<0>` is no `DepType`; `Meter<0>::unit` is a type found at the definition |
| `templates/dependent-names/sizeof-value-dependent-operand-call-bound-early.cpp` | DEP-4, DEP-5 | EDG-AGREES | 1 | `sizeof(N)` is not value-dependent, so `Slot<sizeof(N)>` is no `DepType` and the call binds at the definition |
| `templates/dependent-names/sizeof-value-dependent-operand-no-typename.cpp` | DEP-4 | EDG-AGREES | 1 | the same, so `Box<sizeof(N)>::type` is a type found at the definition |
| `templates/dependent-names/current-instantiation-enclosing-base.cpp` | DEP-11 | unsupported | 2 | `Outer<T>` is the enclosing current instantiation, an independent base |
| `templates/dependent-names/member-template-current-instantiation-pointer.cpp` | DEP-24, DEP-2 | false reject | 2 | `p->get` and a local `l.get` have type `CurrentInst`; lookup finds `get` |
| `templates/dependent-names/member-template-current-instantiation-qualifier-dependent-base.cpp` | DEP-24 | missed reject | 2 | `Wrap::make` finds nothing, the base is dependent, so `<` needs `template` |
| `templates/dependent-names/member-template-non-dependent-base.cpp` | DEP-24 | false reject | 2 | `get` and `make` are in an independent base |
| `templates/dependent-names/member-template-through-current-instantiation-object.cpp` | DEP-24, DEP-2 | false reject | 2 | `w` has type `CurrentInst`; lookup finds `get` |
| `classes/copy-move/class-template-ctor-by-value.cpp` | CPY-15 | missed reject | 2 | the parameter `Wrapper` has type `CurrentInst`, so CPY-15 rejects the constructor at the definition |
| `templates/instantiation/class-point-not-later-member-type.cpp` | INST-33, INST-30 | missed reject | 3 | lookup sees only the declarations before the point |
| `templates/instantiation/class-point-not-later-type.cpp` | INST-33 | missed reject | 3 | the same |
| `templates/instantiation/class-point-not-later.cpp` | INST-32, INST-31 | unsupported | 3 | the same, for a `static` member's initializer with the points of INST-31 |
| `templates/instantiation/class-point-static-const-member.cpp` | INST-33, INST-5 | unsupported | 3 | the initializer is a part of the class, made in the order of INST-P2 |
| `templates/instantiation/class-point-static-constexpr-made-at-use.cpp` | INST-33, INST-5, INST-8 | unsupported | 3 | a `static constexpr` member's initializer waits for its first use |
| `templates/instantiation/class-static-constexpr-member-initializer-waits.cpp` | INST-5 | EDG-AGREES | 3 | the same |
| `templates/instantiation/static-inline-member-incomplete-argument.cpp` | INST-5 | EDG-AGREES | 3 | an `inline` static member's definition and initializer wait for a use (Q-0231), so `Cursor<Derived>` is not made with the class |
| `templates/instantiation/local-class-default-argument.cpp` | INST-12, INST-6 | missed reject | 3 | a local class's members are made with the enclosing function's specialization |
| `templates/instantiation/local-class-exception-specification.cpp` | INST-12, INST-6 | missed reject | 3 | the same |
| `templates/instantiation/recursion-endless-function.cpp` | INST-15, TR-13 | missed reject | 3 | the `busy` state and the nesting limit |
| `templates/specialization/partial-no-primary-variable.cpp` | TSP-31 | missed reject | 3 | a partial specialization links to its primary when declared |
| `templates/deduction/substitution-explicit-specifier.cpp` | DED-24 | unsupported | 4 | the operand of a conditional `explicit` is in the immediate context |
| `templates/deduction/substitution-lambda-body.cpp` | DED-29 | missed reject | 4 | the lambda body is outside the immediate context |
| `templates/deduction/substitution-lambda-default.cpp` | DED-29 | missed reject | 4 | the same, in a template parameter's default |
| `templates/deduction/substitution-member-wrong-kind.cpp` | DED-24 | unsupported | 4 | the kind table gives a failure, and the fallback wins |
| `statements/if/constexpr-template-nondependent-call-checked.cpp` | IF-22 | missed reject | 5 | the independent call `g()` is checked at the definition |
| `templates/constraints/out-of-class-other-clause.cpp` | CNS-29 | missed reject | 5 | the member's clause is in its record |
| `templates/constraints/out-of-class-without-clause.cpp` | CNS-29 | missed reject | 5 | the same, with no clause on the definition |
| `templates/dependent-names/binary-operator-pack-never-fits.cpp` | DEP-37 | missed reject | 5 | an operator function that only an empty pack makes valid is rejected |
| `templates/dependent-names/operator-pack-never-fits.cpp` | DEP-37 | missed reject | 5 | the same |
| `templates/packs/fold-in-outer-pattern.cpp` | PACK-8 | missed reject | 5 | the fold empties the outer mark |
| `templates/packs/size-operator-in-outer-pattern.cpp` | PACK-8 | missed reject | 5 | `sizeof...` empties the outer mark |

By section: 4 tests need section 1, 6 need section 2, 11 need section
3, 4 need section 4, and 7 need section 5.

Front-end round 26 passes the false rejects of Port suite P3 that a
base class template with the derived class as its argument caused:
`templates/instantiation/member-return-type-waits-for-definition.cpp`
(P3-7), `templates/instantiation/member-constructor-made-at-call.cpp`
(P3-8), and `templates/instantiation/member-parameter-type-waits-for-definition.cpp`
(P3-6). The current front end now completes the parameter and return
types of an instance's member function when it makes the body, which
waits for a use (`DESIGN-current.md`, "Lazy bodies of instances").
`programs/P3/pipeline-first-try.cpp` passes its check, and its run
stops at a range `for` that the interpreter does not model.

`programs/roadmap/r10-list-first-try.cpp` passes. The cause of its
TR-13 stop was in deduction, outside this model (report 25, section 2).

## 7. What the model keeps from the current front end

These parts of the current model work on types and argument lists,
and carry over to `DepType` with the new forms added:

- deduction of a type against a pattern (DED-P3, `deduce_ty`),
- partial ordering with invented types (ORD-P2),
- the selection of an explicit or partial specialization (TSP-P2,
  `select_partial`),
- constraints as normal forms with parameter mappings (CNS-P1).

## 8. Rules the model depends on that can still change

The model follows the text as it stands. These questions and findings
can change it. Each question is provisional.

| Item | Rules | What depends on it |
|---|---|---|
| Q-0427 | DEP-24, DEP-P5 | the member template found through an object of the current instantiation, with no `template` (section 2); waits for a use of DEP-P5 |
| Q-0432 | DEP-4, DEP-P2 | that the operand of `sizeof`, `alignof`, `noexcept`, or `typeid` passes on no value-dependence (section 1); waits for a use of DEP-P2 |
| Q-0434 | DEP-2, DEP-18 | that a member access through `this` or an object of `CurrentInst` with an independent member type is not type-dependent (section 1); waits for a review |
| P3-6 | INST-1, INST-5 | when the return type of a member definition is made (section 3, and the P3 tests of section 6) |
| RV8 | DEP-P5, DEP-4, DEP-6, DEP-30, DEP-40, INST-1 | the RV7 repair (b5d925e0) changed these blocks, which sections 1, 4, and 5 cite, and decision 0063's item RV8 reviews them |

Findings I6-4, I6-5, I6-6, and I6-9 are closed. DEP-P5 and DEP-40
give the text of I6-4 and I6-9, and Q-0434 and Q-0432 hold I6-5 and
I6-6.
