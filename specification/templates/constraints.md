# Constraints

This page says what the constraints of a declaration are, how the
implementation checks them, and how two sets of constraints compare.
It covers:

- normalization of constraints into atomic constraints,
- satisfaction, with its short circuit and substitution failure,
- the uses that check constraints, and what a failure does in each,
- subsumption, and the relation "more constrained" that overload
  resolution and partial ordering use,
- redeclarations of constrained templates, and special member functions
  with constraints.

{doc}`concepts` gives the ways to write a constraint. Overload
resolution uses constraints in {rule}`OVL-24`, {rule}`OVL-30`, and
{rule}`OVL-41`. Partial ordering uses them in {rule}`ORD-13` and
{rule}`ORD-14`.

Depends on: concepts and requires expressions, template declarations,
deduction, partial ordering, overload resolution, instantiation.

## Principles

:::{principle} CNS-P1
:rules: CNS-1 CNS-2 CNS-3 CNS-4

The implementation reduces the constraints of a declaration to a
formula over {term}`atomic constraints <atomic constraint>`. Both the
check of a constraint and the comparison of two declarations work on
this formula:

```text
a declaration
   |
   v
1. collect     one conjunction, in this order:                    CNS-1
   |           a. the type constraints of the template head, of
   |              type and constant parameters, in order
   |           b. the requires clause after the head
   |           c. the type constraints of placeholder parameters
   |           d. the trailing requires clause
   v
2. normalize   && makes a conjunction, || makes a disjunction,    CNS-2
   |           and a concept-id becomes the normal form of its
   |           concept, with the arguments mapped to its
   |           parameters
   v
3. atoms       every other expression is one atomic constraint,   CNS-3
               known by its place in the source and its mapping   CNS-4
               of the parameters that the expression uses
```

The result is the {term}`normal form` of the declaration's
{term}`associated constraints`. An atomic constraint is known by the
expression it comes from, and the expression is known by its place in
the source. Two copies of the same text are two atomic constraints.
The mapping holds only the parameters that the expression uses, so two
concept-ids that differ only in an unused argument give one atomic
constraint.
The implementation never compares the meaning of two expressions, so
the comparison of two declarations is always decidable. A concept
lets several declarations share one expression, and so share its
atomic constraints.
:::

:::{principle} CNS-P2
:rules: CNS-5 CNS-6 CNS-7 CNS-8
:exceptions: CNS-9 CNS-10 CNS-33 CNS-35

The implementation checks a normal form against template arguments in
these steps. A form that the check finds `true` is
{term}`satisfied`.

```text
P && Q    check P. If P is not satisfied, the result is false,      CNS-5
          and Q is not substituted. Otherwise the result is Q.
P || Q    check P. If P is satisfied, the result is true,           CNS-6
          and Q is not substituted. Otherwise the result is Q.
atom E    substitute the mapping into E                             CNS-7
            |
            +-- a type or expression is not valid  -> not satisfied CNS-7, CNS-8
            |
            +-- an instantiation that it needs fails -> error       CNS-33
            |
            +-- the check needs its own result     -> error         CNS-35
            |
            +-- E is not a constant of type bool   -> error         CNS-9
            |
            +-- E is true                          -> satisfied     CNS-7
                E is false                         -> not satisfied
```

A substitution failure gives "not satisfied" and no error. A
constraint can therefore test whether an operation exists, and a
declaration whose constraint fails is only left out. A failure is
local to its atomic constraint. A negation is part of its atomic
constraint, so a failure inside `!E` makes the constraint not
satisfied. A failure inside a concept makes the concept-id `false`,
and its negation is then `true` (CNS-8). Only a failure that the
substitution forms itself counts. An error inside a class or function
that the substitution instantiates is an error of the program (CNS-33).

An atomic constraint must be a constant of type `bool` exactly. Its
value is never converted, so a constraint means the same in every
use.
:::

:::{principle} CNS-P3
:rules: CNS-11 CNS-12 CNS-13 CNS-14 CNS-15 CNS-16 CNS-17 CNS-18 CNS-19 CNS-20 CNS-21 CNS-36 CNS-37 CNS-38

The implementation checks the constraints of a declaration when a use
of the declaration needs them. The use decides what a failure does:

| Use | Arguments | If the constraints are not satisfied | Rules |
|---|---|---|---|
| a call, through overload resolution | after deduction | the candidate is not viable | {rule}`OVL-24`, CNS-11 |
| the address of an overload set | after deduction | the function is removed | {rule}`OVL-19` |
| any other use of a function's name: a call of a single function, `&`, `decltype` | known | the program is not valid | CNS-12 |
| a template-id of a class, variable, or alias template | known | the program is not valid | CNS-13 |
| a template-id inside a template | depend on parameters | checked at instantiation | CNS-14 |
| a template-id inside a template | do not depend on parameters | the program is not valid | CNS-13 |
| a use of a template template parameter inside the template | do not depend on parameters | the program is not valid, with no diagnostic required | CNS-38 |
| an explicit instantiation or specialization | known | the program is not valid | CNS-13 |
| an explicit specialization of a member template, outside the class | the class's arguments | the member does not match | CNS-37 |
| the instantiation of a class | known | the member is declared and cannot be used | CNS-15, CNS-16 |
| the selection of a partial specialization | after matching | the partial specialization does not match | CNS-17, CNS-36 |
| a lambda's conversion to a pointer to function | known | the conversion does not exist | CNS-18 |
| the selection of the destructor, at the end of the class | known | the destructor is not a candidate | CNS-19, CNS-20 |
| a special member function | known | the function is not eligible | CNS-21 |
| a placeholder type | after deduction | the program is not valid | {rule}`CPT-22`, {rule}`AUTO-16` |

A constraint guards its declaration, and nothing else. For a function
template, the check comes after deduction and before the function
type is formed. A failed check therefore stops the substitution, and
errors in the return type and the parameters do not arise. The default
arguments of the template parameters are formed before the check
(CNS-11).
:::

:::{principle} CNS-P4
:rules: CNS-22 CNS-23 CNS-25 CNS-34
:exceptions: CNS-24

Overload resolution and partial ordering prefer the declaration with
more constraints. The comparison runs in these steps, on the normal
forms of CNS-P1:

```text
D1 with associated constraints P1, D2 with P2
   |
   v
0. correspond    for two function templates, a template parameter    CNS-34
   |             matches the one at the same position, and the
   |             function parameter types must agree
   v
1. at least as   holds from D1 to D2 if D2 has no constraints,      CNS-23
   constrained   or if P1 subsumes P2
   |
   v
2. subsumes      write P1 as a disjunction of conjunctions, and      CNS-22
   |             P2 as a conjunction of disjunctions. P1 subsumes
   |             P2 if each conjunction of P1 and each disjunction
   |             of P2 share an atomic constraint (CNS-4)
   v
3. more          the relation of step 1 holds from D1 to D2, and     CNS-23
   constrained   not from D2 to D1
```

Subsumption uses only the identity of atomic constraints. A size test
of more than 4 does not subsume a size test of more than 2. The
comparison is a check on the form of the constraints, and needs no
reasoning about values.

These uses compare declarations by their constraints:

| Use | Declarations compared | Rule |
|---|---|---|
| a call | two functions that are not templates, with the same parameters | {rule}`OVL-30` |
| a call, and other uses of partial ordering | two function templates that partial ordering does not order | {rule}`ORD-13`, {rule}`OVL-41` |
| the address of an overload set | two functions that are not templates | {rule}`OVL-19` |
| the selection of a partial specialization | two partial specializations with the same pattern | {rule}`ORD-14` |
| the selection of the destructor | the satisfied prospective destructors | CNS-19 |
| eligibility of a special member function | two special members of one kind | CNS-21 |
| a template template argument | the parameter against the argument | CNS-25 |

If a use needs one declaration, and neither of two is more
constrained, the use is not valid (CNS-24).
:::

:::{principle} CNS-P5
:rules: CNS-26 CNS-27 CNS-28 CNS-29 CNS-30 CNS-31 CNS-32

The constraints of a template are part of the template's identity.
All declarations of one template write the same constraints in the
same places, and a declaration with other constraints declares another
template. A trailing requires clause is part of the identity of a
function in the same way. Two members of a class template can differ
only in their trailing clauses, and a definition outside the class
repeats the clause. The clause is not part of the function's type. A
lambda expression in a constraint is unique, so a constraint with a
lambda is never equivalent to another.

A friend with a trailing clause that names the class's template
parameters is defined in the class. Each specialization of the class
then defines its own function, which no declaration outside the class
redeclares.
:::

## Constraints

### Normalization

:::{rule} CNS-1
:kind: required
:tests: templates/constraints/order.cpp templates/constraints/order-placeholder.cpp templates/constraints/spellings-same.cpp templates/constraints/order-constant-placeholder.cpp templates/constraints/order-constant-placeholder-before-clause.cpp templates/constraints/order-constant-placeholder-after-type.cpp templates/constraints/constant-placeholder-more-constrained.cpp

The {term}`associated constraints` of a declaration are one conjunction
of these parts, in this order ({question}`Q-0262`, {question}`Q-0282`):

1. the type constraints of the template parameters, in order. For a
   constant parameter, the type constraint of its placeholder counts
   ({question}`Q-0281`).
2. the requires clause after the template parameter list,
3. the type constraints of the placeholder parameters, in order,
4. the trailing requires clause.

A concept written in any of these places gives the same constraint.
:::

:::{rule} CNS-2
:kind: required
:tests: templates/constraints/normal-form-parentheses.cpp templates/constraints/concept-expansion.cpp

The {term}`normal form` of a constraint expression is found in these
steps:

- `P && Q` gives the conjunction of the normal forms of `P` and `Q`.
- `P || Q` gives their disjunction.
- `(P)` gives the normal form of `P`.
- A concept-id `C<A...>` gives the normal form of the expression of
  `C`. Each parameter of `C` maps to its argument in `A...`.
:::

:::{rule} CNS-3
:kind: required
:tests: templates/constraints/negation-atomic.cpp templates/constraints/fold-atomic.cpp templates/concepts/type-constraint-pack-one-atom.cpp

Every other expression gives one {term}`atomic constraint`. A negation,
a comparison, a call, a requires expression, and a fold expression are
each one atomic constraint, even if they contain `&&` or `||`.
:::

:::{rule} CNS-4
:kind: required
:tests: templates/constraints/same-text.cpp templates/constraints/equal-bodies.cpp templates/constraints/named-negation.cpp templates/constraints/concept-expansion.cpp templates/constraints/inline-negation.cpp templates/constraints/mapping-unused-parameter.cpp templates/constraints/mapping-unused-parameter-not-substituted.cpp templates/constraints/mapping-same-arguments.cpp

An atomic constraint has an expression and a {term}`parameter mapping`,
which maps each template parameter that the expression uses to an
argument ({question}`Q-0284`). Two atomic constraints are identical only
if both come from the same expression in the source and their mappings
give equivalent arguments. Two expressions with the same text in
different places give different atomic constraints.
:::

### Satisfaction

:::{rule} CNS-5
:kind: required
:tests: templates/constraints/and-short-circuit.cpp

A conjunction is {term}`satisfied` if both of its operands are
satisfied. The left operand is checked first. If the left operand is
not satisfied, the right operand is not substituted or checked.
:::

:::{rule} CNS-6
:kind: required
:tests: templates/constraints/or-short-circuit.cpp

A disjunction is satisfied if either of its operands is satisfied. The
left operand is checked first. If the left operand is satisfied, the
right operand is not substituted or checked.
:::

:::{rule} CNS-7
:kind: required
:tests: templates/constraints/substitution-failure.cpp templates/constraints/atomic-value.cpp templates/constraints/constant-evaluation-context.cpp templates/constraints/mapping-unused-parameter-not-substituted.cpp

To check an atomic constraint, the implementation substitutes the
arguments of its mapping, and no others, into its expression. Then:

- If the substitution forms a type or an expression that is not valid,
  the constraint is not satisfied.
- Otherwise the expression goes through {term}`constant evaluation`,
  after an lvalue-to-rvalue conversion, and the constraint is satisfied
  if the value is `true`. `if consteval` takes its first branch there.
:::

:::{rule} CNS-8
:kind: required
:tests: templates/constraints/negation-substitution-failure.cpp templates/constraints/negated-concept.cpp

A substitution failure inside a negation makes the whole atomic
constraint not satisfied. A failure inside a concept makes the
concept-id `false` (CNS-7), so the negation of the concept-id is
satisfied.
:::

:::{rule} CNS-9
:kind: reject
:tests: templates/constraints/atomic-not-bool.cpp templates/constraints/atomic-not-bool-class.cpp templates/constraints/atomic-not-bool-other-candidate.cpp templates/constraints/atomic-not-constant.cpp templates/constraints/atomic-not-bool-skipped.cpp templates/concepts/nested-requirement-parameter-evaluated.cpp

After substitution, the expression of an atomic constraint must be a
constant expression of type `bool`. Another type is not valid, even
one that converts to `bool`. The program is not valid even if another
candidate would be chosen ({question}`Q-0263`).
:::

:::{rule} CNS-33
:kind: reject
:tests: templates/constraints/instantiation-error-class.cpp templates/constraints/instantiation-error-return-type.cpp templates/constraints/default-argument-before-constraints.cpp templates/constraints/order-constant-placeholder-after-type.cpp

The substitution of CNS-7 can instantiate a class or the body of a
function, for example to read a member or to deduce a return type. An
error inside that instantiation is not a substitution failure. The
program is not valid, even if another candidate would be chosen.
:::

:::{rule} CNS-35
:kind: reject
:tests: templates/constraints/recursive-satisfaction.cpp

The check of an atomic constraint must not need the result of the same
atomic constraint with the same arguments. Such a check can arise
through overload resolution inside a requirement.
:::

### Uses that check constraints

:::{rule} CNS-11
:kind: required
:tests: templates/constraints/before-function-type.cpp templates/constraints/before-parameters.cpp templates/constraints/default-argument-before-constraints.cpp templates/concepts/placeholder-constant-parameter-fallback.cpp

For a function template, the constraints are checked when deduction
and the default template arguments have given every template argument
({rule}`DED-1`). The check comes before substitution into the function
type. If the constraints are not satisfied, deduction fails, and the
function type is not formed ({question}`Q-0261`).
:::

:::{rule} CNS-12
:kind: reject
:tests: templates/constraints/unsatisfied-call.cpp templates/constraints/unsatisfied-address.cpp templates/constraints/unsatisfied-decltype.cpp templates/constraints/unsatisfied-member-call.cpp templates/concepts/placeholder-constant-parameter-no-candidate.cpp

A program must not use the name of a function whose constraints are
not satisfied. A call, the operator `&`, and an operand of `decltype`
all use the name. {rule}`OVL-24` and {rule}`OVL-19` remove such a
function from a set of several.
:::

:::{rule} CNS-13
:kind: reject
:tests: templates/constraints/class-not-satisfied.cpp templates/constraints/variable-not-satisfied.cpp templates/constraints/alias-not-satisfied.cpp templates/constraints/dependent-checked-at-instantiation.cpp templates/constraints/explicit-instantiation-class.cpp templates/constraints/explicit-specialization-class.cpp templates/constraints/explicit-specialization-variable.cpp templates/constraints/explicit-instantiation-function.cpp templates/constraints/explicit-specialization-function.cpp templates/constraints/independent-template-id.cpp templates/constraints/independent-alias-template-id.cpp templates/constraints/template-template-argument-constraint-use.cpp templates/constraints/explicit-instantiation-member.cpp templates/constraints/explicit-instantiation-member-not-satisfied.cpp templates/concepts/placeholder-constant-parameter-not-satisfied.cpp templates/concepts/type-constraint-default-not-satisfied.cpp

The template arguments of these must satisfy the template's
constraints:

- a template-id that names a class, a variable, or an alias, also
  inside a template that is never instantiated if the arguments do not
  depend on template parameters,
- an explicit instantiation ({rule}`INST-16`), also of a single member
  function of a class template specialization ({question}`Q-0291`),
- an explicit specialization ({rule}`TSP-1`, {rule}`TSP-2`).

For a function template, a declaration of a specialization finds no
template whose constraints fail ({rule}`DED-32`).
:::

:::{rule} CNS-14
:kind: required
:tests: templates/constraints/dependent-postponed.cpp templates/constraints/dependent-checked-at-instantiation.cpp templates/constraints/template-template-argument-constraint-use.cpp

Inside a template, a template-id whose arguments depend on template
parameters is checked when the enclosing template is instantiated. A
template-id that names a template template parameter is checked then
against the constraints of the template argument.
:::

:::{rule} CNS-15
:kind: required
:tests: templates/constraints/members-declared.cpp templates/constraints/explicit-instantiation-members.cpp templates/constraints/unsatisfied-member-call.cpp

The instantiation of a class template declares each member, without a
check of the member's constraints. A member whose constraints are not
satisfied cannot be used (CNS-12). An explicit instantiation of the
class instantiates only the members whose constraints are satisfied.
:::

:::{rule} CNS-16
:kind: required
:tests: templates/constraints/constructors-count.cpp

A constructor whose constraints are not satisfied still counts as a
declared constructor. The class therefore gets no implicit default
constructor ({rule}`CTOR-2`).
:::

:::{rule} CNS-17
:kind: required
:tests: templates/constraints/partial-specialization.cpp templates/constraints/partial-specialization-same-arguments.cpp

A partial specialization matches an argument list only if its
constraints are satisfied for the arguments that {rule}`TSP-14` finds.
Its argument list can repeat the primary template's parameters if it is
more constrained than the primary template (CNS-36).
:::

:::{rule} CNS-36
:kind: reject
:tests: templates/constraints/partial-specialization-same-arguments.cpp templates/constraints/partial-specialization-not-more-constrained.cpp templates/constraints/partial-specialization-same-constraints.cpp

A partial specialization whose argument list repeats the primary
template's parameters must be more constrained than the primary
template ({question}`Q-0292`). The rule is a case of {rule}`TSP-11`
(decision 0042).
:::

:::{rule} CNS-18
:kind: required
:tests: templates/constraints/lambda-conversion.cpp

A lambda with no captures and no template parameters converts to a
pointer to function only if the constraints of its call operator are
satisfied ({rule}`LAM-10`).
:::

:::{rule} CNS-19
:kind: required
:tests: templates/constraints/destructor-selection.cpp templates/constraints/destructor-trivial.cpp

A class can declare several {term}`prospective destructors
<prospective destructor>` with different constraints. At the end of
the class, the destructor is chosen from those whose constraints are
satisfied. The chosen one is more constrained (CNS-23) than each of
the others.
:::

:::{rule} CNS-20
:kind: reject
:tests: templates/constraints/destructor-none.cpp templates/constraints/destructor-ambiguous.cpp

The choice of CNS-19 must give one destructor. The class is not valid
if no prospective destructor is satisfied, or if none is more
constrained than each of the others.
:::

:::{rule} CNS-21
:kind: required
:tests: templates/constraints/eligible-default.cpp templates/constraints/eligible-copy.cpp templates/constraints/eligible-more-constrained.cpp templates/constraints/destructor-trivial.cpp templates/constraints/eligible-unsatisfied-more-constrained.cpp

A special member function is {term}`eligible` if all of these hold
({question}`Q-0266`):

- It is not deleted.
- Its constraints are satisfied.
- No other special member of the same kind whose constraints are
  satisfied is more constrained. Two copy or move constructors are of
  the same kind if their first parameters have the same type. Two
  assignment operators also need the same cv-qualifiers and
  ref-qualifier.

Only the eligible special members decide whether a class has trivial
special members ({rule}`CTOR-23`, {rule}`DTOR-9`, {rule}`CPY-27`).
:::

### Subsumption

:::{rule} CNS-22
:kind: required
:tests: templates/constraints/subsumes-conjunction.cpp templates/constraints/subsumes-normal-forms.cpp templates/constraints/normal-forms-equivalent.cpp templates/constraints/identity-only.cpp

A constraint `P` {term}`subsumes <subsume>` a constraint `Q` if this
test holds. Write `P` as a disjunction of conjunctions of atomic
constraints, and `Q` as a conjunction of disjunctions. Each conjunction
of `P` and each disjunction of `Q` share an identical atomic
constraint (CNS-4).
:::

:::{rule} CNS-23
:kind: required
:tests: templates/constraints/constant-placeholder-more-constrained.cpp templates/constraints/unconstrained.cpp templates/constraints/subsumes-conjunction.cpp templates/constraints/spellings-same.cpp templates/constraints/same-constraints-both-ways.cpp templates/constraints/normal-form-parentheses.cpp templates/constraints/concept-expansion.cpp templates/constraints/named-negation.cpp

For two declarations `D1` and `D2`, the relation "at least as
constrained" holds from `D1` to `D2` in two cases. `D2` has no
associated constraints, or the constraints of `D1` subsume those of
`D2`. `D1` is {term}`more constrained` than `D2` if this relation
holds from `D1` to `D2` and fails from `D2` to `D1`.
:::

:::{rule} CNS-34
:kind: required
:tests: templates/constraints/parameter-correspondence.cpp

To compare the constraints of two function templates, each template
parameter of one corresponds to the parameter at the same position in
the other. The comparison uses this correspondence for the mappings.
If the function parameter types differ under it, the constraints are
not compared ({rule}`ORD-13`).
:::

:::{rule} CNS-24
:kind: reject
:tests: templates/constraints/negation-atomic.cpp templates/constraints/fold-atomic.cpp templates/constraints/same-text.cpp templates/constraints/equal-bodies.cpp templates/constraints/inline-negation.cpp templates/constraints/normal-forms-equivalent.cpp templates/constraints/identity-only.cpp templates/constraints/same-constraints-both-ways.cpp templates/constraints/different-spellings.cpp templates/constraints/parameter-correspondence.cpp templates/constraints/lambda-overload.cpp templates/constraints/friend-distinct-from-namespace.cpp templates/concepts/type-constraint-pack-one-atom.cpp

A use can need the more constrained of two declarations, by one of the
rules in {principle}`CNS-P4`. If neither is more constrained, and no
other rule decides, the use is not valid. The rule is a case of
{rule}`OVL-36` (decision 0042).
:::

:::{rule} CNS-25
:kind: reject
:tests: templates/constraints/template-template-constrained.cpp templates/constraints/template-template-more-constrained.cpp

A template template parameter that has constraints must be at least as
constrained as its argument ({question}`Q-0265`). A parameter with no
constraints accepts an argument with any constraints.
:::

### Redeclarations

:::{rule} CNS-26
:kind: reject
:tests: templates/constraints/redeclare-clause-differs.cpp templates/constraints/redeclare-constraint-missing.cpp templates/constraints/redeclare-type-constraint-as-clause.cpp templates/constraints/different-spellings.cpp templates/constraints/redeclare-constant-placeholder.cpp templates/constraints/lambda-redeclaration.cpp

All declarations of one template must have equivalent constraints in
the same places. A type constraint and the same concept-id in a
requires clause are not equivalent. A constant parameter of type
`C auto` and one of type `auto` differ. For a function template, a
declaration with other constraints declares another template
({rule}`TPL-22`).
:::

:::{rule} CNS-27
:kind: required
:tests: templates/constraints/redeclare-equivalent.cpp templates/constraints/redeclare-concept-arguments.cpp templates/constraints/redeclare-placeholder.cpp templates/constraints/lambda-redeclaration.cpp templates/constraints/lambda-overload.cpp

Two constraints are equivalent if their expressions have the same
tokens, and each name means the same entity. A template parameter
matches the parameter at the same position. Two concept-ids that name
the same specialization of a concept are equivalent. A placeholder
parameter matches its invented type parameter ({question}`Q-0269`). A
constraint that contains a lambda expression is not equivalent to any
other constraint.
:::

:::{rule} CNS-28
:kind: required
:tests: templates/constraints/overload-by-clause.cpp templates/constraints/out-of-class-definition.cpp

Two member functions of a class template can differ only in their
trailing requires clauses. The two are different functions, even if
both clauses are always `true`. A definition outside the class with an
equivalent clause defines the member with that clause.
:::

:::{rule} CNS-29
:kind: reject
:tests: templates/constraints/out-of-class-without-clause.cpp

A definition of a member outside its class template must repeat the
member's trailing requires clause. A definition without an equivalent
clause matches no member.
:::

:::{rule} CNS-37
:kind: required
:tests: templates/constraints/member-specialization-by-constraints.cpp

An explicit specialization of a member template of a class template
specialization can be declared outside the class. It specializes the
member whose constraints, after substitution of the class's template
arguments, are equivalent to its own. A member whose constraints are
not valid after that substitution does not match.
:::

:::{rule} CNS-30
:kind: required
:tests: templates/constraints/function-type.cpp

A trailing requires clause is not part of the function's type.
:::

:::{rule} CNS-31
:kind: reject
:tests: templates/constraints/friend-declaration.cpp templates/constraints/friend-template-declaration.cpp

A friend declaration must be a definition in these cases
({question}`Q-0267`):

- The friend is not a template and has a trailing requires clause.
- The friend is a template, and its constraints use a template
  parameter of the enclosing class.
:::

:::{rule} CNS-32
:kind: required
:tests: templates/constraints/friend-definition-per-specialization.cpp templates/constraints/friend-distinct-from-namespace.cpp

A friend definition of CNS-31 declares a different function in each
specialization of the class. The definitions in two specializations do
not conflict. The function differs from every declaration outside the
class, even one with the same name and form ({question}`Q-0268`).
:::

## Implementation Requirements

:::{rule} CNS-10
:kind: no-diagnostic
:tests: templates/constraints/satisfaction-changes.cpp

An atomic constraint with the same arguments must give the same result
at each point where it is checked. A program in which the result
changes is not valid, for example when a class is completed between
two checks. An implementation need not diagnose the change
({question}`Q-0264`).
:::

:::{rule} CNS-38
:kind: no-diagnostic
:tests: templates/constraints/template-template-parameter-constraint-use.cpp

Inside a template, a template-id that names a template template
parameter, with arguments that do not depend on template parameters,
must satisfy the parameter's own constraints. An implementation need
not diagnose a template-id that does not ({question}`Q-0283`).
:::

## Examples

```cpp
template <class T> concept Sized = sizeof(T) > 1;
template <class T> concept Small = Sized<T> && sizeof(T) < 16;

template <Sized T> int pick(T);            // 1
template <Small T> int pick(T);            // 2: Small subsumes Sized
pick(1);                                   // 2
pick('a');                                 // rejected: no viable function

template <class T> int test(T) requires (sizeof(T) > 1);
template <class T> int test(T) requires (sizeof(T) > 1) && true;
test(1);                                   // rejected: two different atomic constraints

template <class T> concept HasSize = requires (T t) { t.size(); };
template <class T> requires (!HasSize<T>) int count(T);   // satisfied for int
template <class T> requires (!T::size) int other(T);      // not satisfied for int
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 84
of 107 tests.

The clean-room writer probed each rule with Clang 18 and 22, GCC 14 and
16, the host Clang and GCC, and EDG 7.0, and checked the tests and the
grids on the same compilers. The rejections name the reason in each
compiler's own words, for example "atomic constraint must be of type
'bool'" and "requires clause differs in template redeclaration"
(clang-diag-sema).

- CNS-1: GCC and EDG check the requires clause after the template
  parameter list before the type constraints of placeholder parameters.
  Clang checks the placeholders first ({question}`Q-0262`). Every
  compiler checks the type constraints of the head first and the
  trailing clause last.
- CNS-1: Clang and GCC 16 check the constraints of the head in
  parameter order. GCC 14 and EDG check the constraint of a constant
  placeholder before the type constraint of an earlier type parameter
  ({question}`Q-0282`). Every compiler checks the constraint of a
  constant placeholder before the requires clause after the head.
- CNS-1, CNS-23: Clang 22 and GCC 16 find a template with a constrained
  constant parameter more constrained than the same template with
  `auto`. Clang 18, GCC 14, and EDG find the call ambiguous
  ({question}`Q-0281`).
- CNS-4, CNS-7: EDG keeps every parameter of a concept in the mapping,
  so two concept-ids that differ in an unused argument give two atomic
  constraints. EDG and Clang 18 also substitute an unused argument.
  Clang 22 and GCC do neither ({question}`Q-0284`).
- CNS-9: Clang and EDG reject an atomic constraint whose type is not
  `bool` for any argument, even if a short circuit skips it. GCC
  accepts it ({question}`Q-0263`).
- CNS-10: GCC 16 diagnoses a result that changes after a class is
  completed. Clang, EDG, and GCC 14 keep the first result. The rule is
  `no-diagnostic` because the change shows only against an earlier
  check, which an implementation need not keep ({question}`Q-0264`).
- CNS-11: Clang 18 forms the return type of a function template before
  it checks the constraints. The other compilers check the constraints
  first ({question}`Q-0261`). Every compiler forms the default template
  arguments before the check.
- CNS-13: Clang 18 accepts an explicit instantiation of a member
  function whose constraints fail ({question}`Q-0291`). Every compiler
  rejects a template-id with arguments that do not depend on template
  parameters and fail the constraints, inside a template that is never
  instantiated. The gap review describes that case as one that needs no
  diagnostic (item 6), and {rule}`DEP-30` covers the general case. The
  rule follows the compilers.
- CNS-18: EDG rejects the trailing clause on the lambda
  ({question}`Q-0260`).
- CNS-21: EDG does not treat a constructor as trivial when a less
  constrained or unsatisfied constructor of the same kind is
  user-provided. Clang and GCC do ({question}`Q-0266`). Every compiler
  chooses the trivial destructor in `destructor-trivial.cpp`. No test
  shows the kinds of the third condition apart from another split. A
  class with a defaulted copy constructor and a user-provided one with
  another first parameter type, or two copy assignment operators with
  different ref-qualifiers, makes a union member that Clang copies and
  GCC and EDG do not. The split appears without constraints too.
- CNS-25: Clang 18 and EDG accept a template template argument that is
  more constrained than its parameter. Clang 22 and GCC reject it
  ({question}`Q-0265`). Every compiler accepts a constrained argument
  for a parameter with no constraints (`template-template-constrained.cpp`).
- CNS-27: Clang treats `int f(A auto)` as a redeclaration of
  `template <A T> int f(T)`. GCC and EDG treat the two as different
  templates ({question}`Q-0269`). The `spellings` grid shows the same
  split as a rejection on every compiler, for different reasons.
- CNS-31: EDG accepts a friend template declaration whose constraint
  uses the class's template parameter ({question}`Q-0267`).
- CNS-32: EDG reports two definitions of a constrained friend from two
  specializations. EDG also treats the friend and a namespace template
  of the same form as one template ({question}`Q-0268`).
- CNS-36: EDG accepts a partial specialization that repeats the primary
  template's parameters and is not more constrained. Clang and GCC
  reject it ({question}`Q-0292`).
- CNS-38: the rule is `no-diagnostic`. A use that fails the parameter's
  own constraints can never be valid, and the compilers differ only in
  whether they diagnose it. EDG rejects the definition. Clang and GCC
  accept it, also after an instantiation with an argument template
  that has no constraints ({question}`Q-0283`).
- CNS-3: the inventory doubts whether newer compilers split a fold
  expression into its operands (gap E8). Every compiler in the matrix
  treats the fold as one atomic constraint.

Subsumption (CNS-4, CNS-22 to CNS-24). Each column declares two static
members `f` of `Box<T>`, with the two requires clauses. The first
returns 1 and the second 2. `int` satisfies every clause, so the row
"int → first" checks that the first is more constrained. A `reject` in
both rows means that neither is more constrained. `A2` has the same
text as `A`, `Z` is never satisfied for `int`, and `NotZ` is the
concept `!Z<T>`:

| | none / A | none / B | none / A and B | none / A or B | none / AB | none / A2 | none / not Z | none / A and not Z | none / NotZ | none / A and NotZ | A / B | A / A and B | A / A or B | A / AB | A / A2 | A / not Z | A / A and not Z | A / NotZ | A / A and NotZ | B / A and B | B / A or B | B / AB | B / A2 | B / not Z | B / A and not Z | B / NotZ | B / A and NotZ | A and B / A or B | A and B / AB | A and B / A2 | A and B / not Z | A and B / A and not Z | A and B / NotZ | A and B / A and NotZ | A or B / AB | A or B / A2 | A or B / not Z | A or B / A and not Z | A or B / NotZ | A or B / A and NotZ | AB / A2 | AB / not Z | AB / A and not Z | AB / NotZ | AB / A and NotZ | A2 / not Z | A2 / A and not Z | A2 / NotZ | A2 / A and NotZ | not Z / A and not Z | not Z / NotZ | not Z / A and NotZ | A and not Z / NotZ | A and not Z / A and NotZ | NotZ / A and NotZ |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| int → first | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | reject | reject | reject | reject | reject | reject | reject | accept | reject | reject | reject | reject | reject | reject | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| int → second | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | reject | accept | reject | accept | reject | reject | accept | reject | accept | accept | reject | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | reject | reject | accept | reject | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept |

Spellings (CNS-1, CNS-23, CNS-26, CNS-27, {rule}`ORD-13`). Each column
declares two function templates `f` with the two forms, and the row
checks the call `f(1)`. A `reject` in both rows means an ambiguous call,
or for `A T / A auto` and `AB T / AB auto` a redefinition on Clang
({question}`Q-0269`):

| | none / A T | none / A leading | none / A trailing | none / A auto | none / AB T | none / AB trailing | none / AB auto | A T / A leading | A T / A trailing | A T / A auto | A T / AB T | A T / AB trailing | A T / AB auto | A leading / A trailing | A leading / A auto | A leading / AB T | A leading / AB trailing | A leading / AB auto | A trailing / A auto | A trailing / AB T | A trailing / AB trailing | A trailing / AB auto | A auto / AB T | A auto / AB trailing | A auto / AB auto | AB T / AB trailing | AB T / AB auto | AB trailing / AB auto |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| int → first | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| int → second | accept | accept | accept | accept | accept | accept | accept | reject | reject | reject | accept | accept | accept | reject | reject | accept | accept | accept | reject | accept | accept | accept | accept | accept | accept | reject | reject | reject |

The probe grids are in `tests/templates/constraints/grids/`. This
table gives the source of each rule. Inventory items have the numbers
of `reports/review/2026-10-31-inventory-concepts-and-constraints.md`:

| Rule | Source |
|---|---|
| CNS-1 | grid spellings; probes; inventory E7, E9 |
| CNS-2 to CNS-4 | grid subsumption; probes; clang-diag-sema (similar constraint expressions not equivalent); inventory E1, E2, E5, E6, E8, H4, H5 |
| CNS-5, CNS-6 | probes; inventory F3, F4, D6 |
| CNS-7, CNS-8 | probes; clang-diag-sema (substituted constraint expression is ill-formed); inventory E4, F5, F6, F8, F9 |
| CNS-9 | probes; clang-diag-sema (atomic constraint must be of type bool, non-constant expression); inventory F1, F2 |
| CNS-10 | probes; GCC diagnostic (satisfaction value changed); inventory F7 |
| CNS-11 | probes; inventory G4, G5 |
| CNS-12 | probes; clang-diag-sema (invalid reference to function, constraints not satisfied); inventory G1, G15 |
| CNS-13, CNS-14 | probes; clang-diag-sema (constraints not satisfied for class template); inventory G2, G8, G10, G12 |
| CNS-15, CNS-16 | probes; inventory G6, G7, G11 |
| CNS-17 | probes; inventory G18 |
| CNS-18 | probes; inventory G13 |
| CNS-19, CNS-20 | probes; diagnostics (no viable destructor, destructor is ambiguous); inventory G9 |
| CNS-21 | probes; inventory G19 |
| CNS-22 to CNS-24 | grids subsumption and spellings; probes; inventory H1 to H5, H7, H11 |
| CNS-25 | probes; clang-diag-sema (template template argument more constrained); inventory H12 |
| CNS-26, CNS-27 | grid spellings; probes; clang-diag-sema (requires clause and type constraint differ in redeclaration); inventory I3, I10 |
| CNS-28 to CNS-30 | probes; inventory I2, I7, I8 |
| CNS-31, CNS-32 | probes; clang-diag-sema (friend declarations with a requires clause); inventory B9, I1 |
| CNS-33 to CNS-38 | probes; diagnostics (static assertion in an instantiation, satisfaction depends on itself, partial specialization not more constrained) |

The gap review `reports/review/2026-10-31-gap-review-concepts-and-constraints.md`
led to these changes, by its own numbering. Item 31 also changed
{rule}`TSP-11`. Item 37, on deduction guides, belongs to class
template argument deduction.

| Rule | Gap review items |
|---|---|
| CNS-1 | 2, 3 |
| CNS-4 | 10, 11 |
| CNS-7 | 9, 10 |
| CNS-11 | 7 |
| CNS-13, CNS-14 | 5, 6, 36 |
| CNS-17, CNS-36 | 31 |
| CNS-21 | 29, 30 |
| CNS-25 | 4 |
| CNS-26, CNS-27 | 33, 35 |
| CNS-32 | 34 |
| CNS-33 | 8 |
| CNS-34 | 12 |
| CNS-35 | 38 |
| CNS-37 | 32 |
| CNS-38 | 5 |

These inventory items are left out:

- E3, G16, G17, and H6 make a program not valid with no diagnostic
  required. The probes show only each compiler's own choice.
- G3, the constraints of a deduction guide made from an alias template,
  belongs to class template argument deduction.
- G14, the constraints of a member of a local class, depends on
  {question}`Q-0260`, because EDG rejects the clause.
- H8, H9, and H13 are {rule}`OVL-30`, {rule}`ORD-14`, and
  {rule}`ORD-13`, which use CNS-23.
- H10 is the reordering of template parameters for a swapped
  rewritten candidate. Every compiler finds the probe's call ambiguous,
  and {rule}`ORD-7` already has open questions.
- I4, I6, and I11 have no probe. A template cannot have C linkage
  ({rule}`TPL-3`), so I6 has no case to probe in that form.
- I5 needs `std::strong_ordering`, a library type.

Inputs: clang-diag-sema, clang-diag-parse.
