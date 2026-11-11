# Template model for the rewrite

This document gives the model of templates that the rewrite of the
front end needs (decision 0060, decision 0062 item RP). It comes from
the templates chapter of the specification, from the 16 tests that
fail on the current front end because of its model, and from the three
facts that the I6 component could not compute
(`reports/clean-room/I6/report.md`, section "Score"). `DESIGN-current.md`
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
| In a pattern, the type of an object of the current instantiation is `Ty::Unknown`. | A member access on such an object cannot find the member template, and `<` reads as the less-than operator. | DEP-2, DEP-15, DEP-24 | 4 |
| A base of a pattern class is a known class or an unknown one (`BaseInfo::class` is `None`). The current instantiation of an enclosing class is an unknown base. | Unqualified lookup in the class is uncertain, and a name that the base declares is unknown. | DEP-11 | 1 |
| A template-id with independent arguments, inside the template's own definition, gets no special form. | `Meter<0>` inside `Meter<N>` is treated as dependent, so its members are not looked up at the definition. | DEP-1, DEP-15, DEP-30 | 2 |
| A substitution is a replay with every diagnostic muted, and any muted error is a failure. | An error outside the immediate context, such as in a lambda body, becomes a quiet failure instead of an error. A member of the wrong kind stops the replay as unsupported. | DED-24, DED-29 | 3 |
| The check of an expansion pattern scans tokens. | The scan does not know that a fold or `sizeof...` inside the pattern uses up the pack. | PACK-8 | 2 |
| The pattern's parse does not resolve an independent call in a discarded statement. | A call with too few arguments there is accepted. | IF-22, DEP-6 | 1 |
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
| value-dependent | the type is fixed, and the value differs | DEP-4 |
| unexpanded packs | the packs that the node names and that no expansion inside the node expands | PACK-8, PACK-13, PACK-19 |
| binding | for a name: the declarations found at the definition, or "dependent name" | DEP-5, DEP-7, DEP-8 |

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
(DEP-19, DEP-22, DEP-26; finding I6-4).

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

## 6. The 16 tests

The 16 tests that the implementer's round 11 view and report 24
section 6 attribute to the current model, and the part of this model
that passes each:

| Test | Rules | Now | Part of the model |
|---|---|---|---|
| `templates/dependent-names/member-template-through-current-instantiation-object.cpp` | DEP-24, DEP-2 | false reject | `w` has type `CurrentInst`; lookup finds `get` (section 2) |
| `templates/dependent-names/member-template-current-instantiation-pointer.cpp` | DEP-24, DEP-2 | false reject | the same for `p->get` and a local `l.get` |
| `templates/dependent-names/member-template-non-dependent-base.cpp` | DEP-24 | false reject | `get` and `make` are in an independent base (section 2) |
| `templates/dependent-names/member-template-current-instantiation-qualifier-dependent-base.cpp` | DEP-24 | missed reject | `Wrap::make` finds nothing, the base is dependent, so `<` needs `template` (section 2) |
| `templates/dependent-names/current-instantiation-enclosing-base.cpp` | DEP-11 | unsupported | `Outer<T>` is the enclosing current instantiation, an independent base (section 2) |
| `templates/dependent-names/other-specialization-constant-argument-no-typename.cpp` | DEP-15, DEP-1 | unsupported | `Meter<0>` is no `DepType`; `Meter<0>::unit` is a type found at the definition (section 1) |
| `templates/dependent-names/other-specialization-constant-argument-member-missing.cpp` | DEP-30, DEP-15 | missed reject | `o.missing` is independent and checked at the definition (sections 1 and 5) |
| `templates/deduction/substitution-member-wrong-kind.cpp` | DED-24 | unsupported | the kind table of section 4 gives a failure, and the fallback wins |
| `templates/deduction/substitution-lambda-body.cpp` | DED-29 | missed reject | the lambda body is outside the immediate context (section 4) |
| `templates/deduction/substitution-lambda-default.cpp` | DED-29 | missed reject | the same, in a template parameter's default |
| `templates/constraints/out-of-class-other-clause.cpp` | CNS-29 | missed reject | the member's clause is in its record (section 5) |
| `templates/constraints/out-of-class-without-clause.cpp` | CNS-29 | missed reject | the same, with no clause on the definition |
| `templates/packs/fold-in-outer-pattern.cpp` | PACK-8 | missed reject | the fold empties the outer mark (section 5) |
| `templates/packs/size-operator-in-outer-pattern.cpp` | PACK-8 | missed reject | `sizeof...` empties the outer mark |
| `statements/if/constexpr-template-nondependent-call-checked.cpp` | IF-22 | missed reject | the independent call `g()` is checked at the definition (section 5) |
| `templates/specialization/partial-no-primary-variable.cpp` | TSP-31 | missed reject | a partial specialization links to its primary when declared (section 3) |

The test `programs/roadmap/r10-list-first-try.cpp` stops at the TR-13
limit, and its cause is open. It is not in the table.

## 7. What the model keeps from the current front end

These parts of the current model work on types and argument lists,
and carry over to `DepType` with the new forms added:

- deduction of a type against a pattern (DED-P3, `deduce_ty`),
- partial ordering with invented types (ORD-P2),
- the selection of an explicit or partial specialization (TSP-P2,
  `select_partial`),
- constraints as normal forms with parameter mappings (CNS-P1).

## 8. Rules the model depends on that can still change

The model follows the text as it stands. These findings and questions
can change it:

| Item | Rules | What depends on it |
|---|---|---|
| I6-4 | DEP-P5 | the first phase only reads, and the second reports (sections 1 and 4) |
| I6-5 | DEP-2 | whether `this->n` with an independent member type is type-dependent; the mark of section 1 |
| I6-6 | DEP-4 | whether `sizeof` of a value-dependent operand is value-dependent |
| I6-9 | DEP-8, DEP-9 | the binding of a qualified dependent call, and of a member call with a dependent argument |
| Q-0427 | DEP-24 | the member template found through an object of the current instantiation (section 2) |
