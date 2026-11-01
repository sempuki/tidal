# Concepts and requires expressions

A {term}`constraint` is a condition on template arguments that the
implementation checks at compile time. This page gives the ways to
write a constraint:

- concept definitions and concept-ids,
- requires clauses, before and after a declaration,
- type constraints on template parameters and on placeholders,
- requires expressions and their four kinds of requirement.

{doc}`constraints` says how a constraint is checked, where the checks
happen, and how two constraints compare.

Depends on: template declarations, placeholder types, deduction,
constant evaluation.

## Principles

:::{principle} CPT-P1
:rules: CPT-8 CPT-9 CPT-11 CPT-16 CPT-17 CPT-19 CPT-20 CPT-21 CPT-23
:exceptions: CPT-10 CPT-12 CPT-13 CPT-14 CPT-15 CPT-18 CPT-22

A constraint can be written in these places:

| Place | Example | What it constrains | Rules |
|---|---|---|---|
| before a type parameter | `template <Big T>` | the template | CPT-17, CPT-19 |
| after the template parameter list | `template <class T> requires Big<T>` | the template | CPT-8 |
| before `auto` in a function parameter | `int f(Big auto x)` | the function template | CPT-20 |
| after the declarator of a function | `int f() requires Big<T>` | the function | CPT-11 |
| on a lambda | `[]<class T> requires Big<T> (T) {}` | the call operator | CPT-16 |
| before `auto` in a variable, a return type, or a constant parameter | `Big auto n = 1;` | the deduced type | CPT-21 to CPT-23 |
| after `->` in a compound requirement | `{ e } -> Big;` | the type of `e` | CPT-33 |
| in a nested requirement | `requires Big<T>;` | the requires expression | CPT-35 |

A requires clause sits between two parts of a declaration, so the end
of its expression must be plain to see. Its operands are therefore
primary expressions joined by `&&` and `||`. Any other expression, a
negation or a comparison for example, goes in parentheses (CPT-10).

A trailing requires clause constrains a function that has template
arguments to check. The function is a template, or the function is
declared inside a templated entity. A clause on any other function is
not valid. A clause on a virtual function is not valid either, because
a call through a base class does not check the constraints (CPT-14,
CPT-15).
:::

:::{principle} CPT-P2
:rules: CPT-1 CPT-2 CPT-3
:exceptions: CPT-4 CPT-5 CPT-6 CPT-7

A {term}`concept` gives a name to a constraint, so that declarations
can share the constraint. A concept has template parameters and a
constraint expression. A {term}`concept-id`, the name of the concept
with a template argument list, is a constant of type `bool`. Its value
says whether the arguments satisfy the concept ({rule}`CNS-7`).

A concept is only a name for a constraint:

- A concept is defined once, at namespace scope. It has no
  specializations and no instantiations (CPT-4, CPT-7).
- A concept has no constraints of its own (CPT-5).
- A concept cannot use its own name (CPT-6).

A named constraint also orders declarations. Two uses of one concept
give the same atomic constraints, and two copies of the same text do
not ({rule}`CNS-4`).
:::

:::{principle} CPT-P3
:rules: CPT-24 CPT-25 CPT-26 CPT-28 CPT-30 CPT-31 CPT-32 CPT-33 CPT-35
:exceptions: CPT-27 CPT-29 CPT-34

A {term}`requires expression` asks whether some code is valid for the
template arguments. The answer is a `bool` constant. The body lists
requirements of four kinds:

| Requirement | Example | Holds when | Rule |
|---|---|---|---|
| simple | `t.run();` | the expression is valid | CPT-31 |
| type | `typename T::tag;` | the type can be named | CPT-32 |
| compound | `{ t.size() } noexcept -> Big;` | the expression is valid, cannot throw if `noexcept` is written, and its type satisfies the concept | CPT-33 |
| nested | `requires sizeof(T) > 2;` | the constraint is satisfied | CPT-35 |

The requirements are checked in order, and the first one that fails
ends the check (CPT-25). No requirement is evaluated (CPT-30). The
optional parameters are names with types, and no objects exist for
them (CPT-28).

Inside a template, a requirement that is not valid for the arguments
makes the expression `false` (CPT-26). A template uses the result to
test its arguments. Outside a template, nothing is substituted, so a
requirement that is not valid is an error (CPT-27).
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   concept_definition: `template_head` "concept" `identifier` {`attribute_spec`} "=" `logical_or_expr` ";"
   requires_clause: "requires" `constraint_or`
   constraint_or: `constraint_and` {"||" `constraint_and`}
   constraint_and: `primary` {"&&" `primary`}
   type_constraint: [`nested_name`] `identifier` ["<" [`template_args`] ">"]
   requires_expression: "requires" ["(" `parameters` ")"] "{" `requirement` {`requirement`} "}"
   requirement: `simple_requirement` | `type_requirement`
              : | `compound_requirement` | `nested_requirement`
   simple_requirement: `expression` ";"
   type_requirement: "typename" [`nested_name` ["template"]] (`identifier` | `template_id`) ";"
   compound_requirement: "{" `expression` "}" ["noexcept"] ["->" `type_constraint`] ";"
   nested_requirement: "requires" `logical_or_expr` ";"
```

A `concept_definition` is a `declaration`. A `requires_clause` comes
after the `>` of a `template_head`, after the declarator of a function,
and in a lambda. A `type_constraint` comes before a type parameter and
before a `placeholder`. A `requires_expression` is a `primary`. A
`simple_requirement` never starts with `requires` (CPT-35).

## Constraints

### Concept definitions

:::{rule} CPT-1
:kind: required
:tests: templates/concepts/concept-id-value.cpp

A {term}`concept-id` is a prvalue constant expression of type `bool`.
Its value is `true` if the template arguments satisfy the constraint
expression of the concept ({rule}`CNS-7`). A concept-id can appear
wherever a constant expression can.
:::

:::{rule} CPT-2
:kind: required
:tests: templates/concepts/parameter-kinds.cpp

A concept can have type parameters, constant parameters, template
template parameters, and packs. The constraint expression can be any
`logical_or_expr`. The restricted grammar of CPT-10 does not apply to
it.
:::

:::{rule} CPT-3
:kind: required
:tests: templates/concepts/attribute.cpp

Attributes after the name of the concept appertain to the concept
({question}`Q-0270`).
:::

:::{rule} CPT-4
:kind: reject
:tests: templates/concepts/namespace-scope-class.cpp templates/concepts/namespace-scope-block.cpp templates/concepts/friend.cpp

A concept must be defined at namespace scope. A concept definition must
not appear in a class or a block, and a friend declaration must not
declare a concept.
:::

:::{rule} CPT-5
:kind: reject
:tests: templates/concepts/head-clause.cpp templates/concepts/head-type-constraint.cpp

The template head of a concept must not have a requires clause or a
template parameter with a {term}`type constraint`.
:::

:::{rule} CPT-6
:kind: reject
:tests: templates/concepts/self-reference.cpp

A concept definition must not use the concept's own name, directly or
in a template argument.
:::

:::{rule} CPT-7
:kind: reject
:tests: templates/concepts/declaration-only.cpp templates/concepts/redefinition.cpp templates/concepts/explicit-specialization.cpp templates/concepts/partial-specialization.cpp templates/concepts/explicit-instantiation.cpp

A concept has one declaration, which is its definition. A program must
not declare the concept again, specialize it explicitly or partially,
or instantiate it explicitly.
:::

### Requires clauses

:::{rule} CPT-8
:kind: required
:tests: templates/concepts/leading-clause-places.cpp

A {term}`requires clause` can follow the template parameter list of a
class template, function template, variable template, alias template,
member template, or partial specialization. The clause constrains the
template ({rule}`CNS-1`).
:::

:::{rule} CPT-9
:kind: required
:tests: templates/concepts/clause-operands.cpp templates/concepts/requires-requires.cpp

The expression of a requires clause is one or more primary expressions
joined by `&&` and `||`. A primary expression can be a name, a
concept-id, a literal, an expression in parentheses, or a requires
expression. A clause with a requires expression therefore has the
keyword `requires` twice.
:::

:::{rule} CPT-10
:kind: reject
:tests: templates/concepts/clause-comparison.cpp templates/concepts/clause-call.cpp templates/concepts/clause-negation.cpp templates/concepts/trailing-clause-comparison.cpp

Each operand of `&&` and `||` at the top level of a requires clause
must be a primary expression. A negation, a comparison, a call, and an
arithmetic expression need parentheses.
:::

:::{rule} CPT-11
:kind: required
:tests: templates/concepts/trailing-clause.cpp templates/concepts/trailing-clause-member.cpp templates/concepts/trailing-clause-friend.cpp templates/concepts/trailing-clause-lambda.cpp templates/concepts/trailing-clause-local-class.cpp

A requires clause can follow the declarator of a function, after any
trailing return type. The function must be one of these
({question}`Q-0260`):

- a function template,
- a member function whose class is a class template, or lies inside
  one,
- a friend defined in a class template,
- a lambda or a member of a local class inside a function template.
:::

:::{rule} CPT-12
:kind: reject
:tests: templates/concepts/trailing-clause-before-return.cpp

A trailing requires clause must not come before a trailing return type.
:::

:::{rule} CPT-13
:kind: reject
:tests: templates/concepts/trailing-clause-variable.cpp templates/concepts/trailing-clause-parameter.cpp templates/concepts/trailing-clause-function-pointer.cpp templates/concepts/trailing-clause-new-type.cpp

A trailing requires clause must follow the declarator of a function
declaration. The clause must not follow the declarator of a variable,
a parameter, or a pointer to function, or the type in a `new`
expression.
:::

:::{rule} CPT-14
:kind: reject
:tests: templates/concepts/trailing-clause-plain-function.cpp templates/concepts/trailing-clause-plain-member.cpp

A function that CPT-11 does not list must not have a trailing requires
clause. Such a function is not a template, and it is not declared in a
templated entity.
:::

:::{rule} CPT-15
:kind: reject
:tests: templates/concepts/virtual.cpp templates/concepts/virtual-override.cpp

A virtual function must not have a trailing requires clause. A function
that overrides a virtual function is virtual, and the trailing clause
does not stop it from overriding ({question}`Q-0258`).
:::

:::{rule} CPT-16
:kind: required
:tests: templates/concepts/lambda-clauses.cpp

A lambda with a template parameter list can have a requires clause
after that list, and a trailing requires clause after its parameters.
Both clauses constrain the call operator. Attributes after the first
clause appertain to the lambda.
:::

### Type constraints and placeholders

:::{rule} CPT-17
:kind: required
:tests: templates/concepts/type-constraint-arguments.cpp templates/concepts/type-constraint-qualified.cpp

A type constraint `C` or `C<A...>` before a type parameter `T`
constrains the template with `C<T>` or `C<T, A...>`. The name of the
concept can be qualified.
:::

:::{rule} CPT-18
:kind: reject
:tests: templates/concepts/type-constraint-not-type-concept.cpp templates/concepts/placeholder-not-type-concept.cpp templates/concepts/type-constraint-missing-arguments.cpp templates/concepts/placeholder-without-auto.cpp

A type constraint must name a {term}`type concept`, whose first
template parameter is a type parameter. With the type as the first
argument, the type constraint must give the concept a valid argument
list. A concept name before a declarator needs `auto` or
`decltype(auto)` after it.
:::

:::{rule} CPT-19
:kind: required
:tests: templates/concepts/type-constraint-pack.cpp

A type constraint before a type parameter pack constrains each element
of the pack. An empty pack satisfies the constraint.
:::

:::{rule} CPT-20
:kind: required
:tests: templates/concepts/placeholder-parameter.cpp templates/concepts/placeholder-parameter-pack.cpp

A type constraint before `auto` in a function parameter constrains the
invented type parameter of {rule}`DED-34`. Before `auto...`, the type
constraint constrains each element of the invented pack.
:::

:::{rule} CPT-21
:kind: required
:tests: templates/concepts/placeholder-constant-parameter.cpp

A constant template parameter can have a constrained placeholder type.
The type that the argument gives must satisfy the type constraint.
:::

:::{rule} CPT-22
:kind: reject
:tests: templates/concepts/placeholder-return-not-satisfied.cpp templates/concepts/placeholder-constant-parameter-not-satisfied.cpp templates/concepts/placeholder-decltype-not-satisfied.cpp

The type deduced for a constrained placeholder must satisfy its type
constraint. The rule applies to a return type and a constant template
parameter, as {rule}`AUTO-16` does to a variable.
:::

:::{rule} CPT-23
:kind: required
:tests: templates/concepts/placeholder-deduced-part.cpp templates/concepts/placeholder-decltype-not-satisfied.cpp

The type constraint checks the type deduced for the placeholder alone.
References, pointers, and qualifiers written around the placeholder take
no part. For `decltype(auto)`, the deduced type is the type that
{rule}`AUTO-10` gives, with its reference.
:::

### Requires expressions

:::{rule} CPT-24
:kind: required
:tests: templates/concepts/requires-expression-value.cpp

A requires expression is a prvalue constant expression of type `bool`.
Its value is `true` if every requirement holds. A requires expression
can appear wherever a constant expression can, inside or outside a
template.
:::

:::{rule} CPT-25
:kind: required
:tests: templates/concepts/requirement-order.cpp

The requirements are checked in the order written. When one does not
hold, the value is `false`, and the requirements after it are not
checked or substituted.
:::

:::{rule} CPT-26
:kind: required
:tests: templates/concepts/requirement-failure-false.cpp

In a requires expression that depends on template parameters, a
requirement does not hold if substitution forms a type or an expression
that is not valid. The program stays valid.
:::

:::{rule} CPT-27
:kind: reject
:tests: templates/concepts/requires-expression-outside-template.cpp

A requires expression that does not depend on template parameters must
contain only valid types and expressions.
:::

:::{rule} CPT-28
:kind: required
:tests: templates/concepts/requires-parameters.cpp

A parameter of a requires expression is a name with a type, which the
requirements can use. No object is made, so the type need not be
constructible or destructible. A later parameter can use an earlier one.
:::

:::{rule} CPT-29
:kind: reject
:tests: templates/concepts/requires-parameter-default.cpp templates/concepts/requires-parameter-ellipsis.cpp

A parameter of a requires expression must not have a default argument.
The parameter list must not end with an ellipsis ({question}`Q-0259`).
:::

:::{rule} CPT-30
:kind: required
:tests: templates/concepts/requirements-unevaluated.cpp

The expressions in a requires expression are not evaluated.
{rule}`UNEV-1` applies to them as to an operand of `sizeof`.
:::

:::{rule} CPT-31
:kind: required
:tests: templates/concepts/simple-requirement.cpp

A simple requirement holds if its expression is valid.
:::

:::{rule} CPT-32
:kind: required
:tests: templates/concepts/type-requirement.cpp templates/concepts/type-requirement-not-instantiated.cpp

A type requirement holds if its name forms a valid type. The name can
be a member type, a member template specialization, a class template
specialization, or an alias template specialization. A class template
specialization is not instantiated, so the class can be incomplete.
:::

:::{rule} CPT-33
:kind: required
:tests: templates/concepts/compound-requirement-type.cpp templates/concepts/compound-requirement-noexcept.cpp

A compound requirement `{ E }` holds if `E` is valid, and these hold
too:

- With `noexcept`, `noexcept(E)` is `true` ({rule}`UN-18`).
- With `-> C<A...>`, the type `decltype((E))` satisfies `C`, as the
  first argument before `A...`.
:::

:::{rule} CPT-34
:kind: reject
:tests: templates/concepts/requirement-body-empty.cpp templates/concepts/compound-requirement-type-name.cpp templates/concepts/simple-requirement-noexcept.cpp

The body of a requires expression must have at least one requirement.
`noexcept` and `->` appear only in a compound requirement, and `->` is
followed by a type constraint. A type after `->` is not valid.
:::

:::{rule} CPT-35
:kind: required
:tests: templates/concepts/nested-requirement.cpp

A nested requirement holds if its constraint expression is satisfied
({rule}`CNS-5` to {rule}`CNS-7`). The expression can use the
parameters of the requires expression. A requirement that starts with
`requires` is always a nested requirement.
:::

## Examples

```cpp
template <class T> concept Whole = T(1) / 2 == 0;
template <class T> concept Sized = requires (T t) {
  { t.size() } -> Whole;      // a compound requirement
  typename T::value_type;     // a type requirement
};

template <Sized T> int count(const T& c);                        // a type constraint
template <class T> requires Sized<T> int first(const T& c);     // a requires clause
int last(const Sized auto& c);                                   // a constrained placeholder

template <class T> struct Box {
  int get() requires (sizeof(T) > 1);   // a trailing clause on a member
};

template <class T> requires !Sized<T> int bad(T);        // rejected: needs (!Sized<T>)
constexpr bool b = requires (int n) { n.size(); };        // rejected: not a template
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 60
of 67 tests.

The clean-room writer probed each rule with Clang 18 and 22, GCC 14 and
16, the host Clang and GCC, and EDG 7.0, and checked each test on the
same compilers. The rejections name the reason in each compiler's own
words, for example "concept cannot have associated constraints" and
"non-templated function cannot have a requires clause"
(clang-diag-sema, clang-diag-parse).

- CPT-3: Clang 18 rejects an attribute after the name of a concept.
  The other compilers accept the attribute ({question}`Q-0270`).
- CPT-11: EDG rejects a trailing clause on a lambda that is not
  generic, and on a member of a local class, inside a function
  template. Clang and GCC accept both ({question}`Q-0260`).
- CPT-15: GCC accepts a virtual function with a trailing clause, and a
  constrained function that overrides a virtual function. Clang and
  EDG reject both ({question}`Q-0258`).
- CPT-29: GCC accepts a default argument and an ellipsis in the
  parameters of a requires expression. Clang and EDG reject them
  ({question}`Q-0259`).
- CPT-27: every compiler rejects a requires expression outside a
  template whose requirement is not valid. The inventory names a doubt
  that some compilers give `false` (gap D4). None does.
- CPT-23: every compiler checks only the type deduced for the
  placeholder, so the inventory's doubt (gap C10) does not arise.

This table gives the source of each rule. Inventory items have the
numbers of `reports/review/2026-10-31-inventory-concepts-and-constraints.md`:

| Rule | Source |
|---|---|
| CPT-1, CPT-2 | probes; inventory A2, A4, A9 |
| CPT-3 | probes; inventory A1 |
| CPT-4 to CPT-7 | probes; clang-diag-sema and clang-diag-parse (the concept diagnostics); inventory A5 to A8 |
| CPT-8, CPT-9 | probes; inventory B2, B4, B6 |
| CPT-10 | probes; clang-diag-parse (parentheses in a requires clause); inventory B5, B10 |
| CPT-11 to CPT-14 | probes; clang-diag-sema and clang-diag-parse (the placement of a trailing clause); inventory B7, B8, B11 |
| CPT-15 | probes; clang-diag-sema (a virtual function with a clause); inventory B12, I9 |
| CPT-16 | probes; inventory B1, B3 |
| CPT-17 to CPT-19 | probes; clang-diag-sema (type concepts and missing arguments); inventory C4, C7, C11, C12 |
| CPT-20 to CPT-23 | probes; diagnostics; inventory C1, C2, C5, C6, C8 to C10 |
| CPT-24 to CPT-27 | probes; inventory D2 to D4, D9 |
| CPT-28, CPT-29 | probes; clang-diag-sema and clang-diag-parse (parameters of a requires expression); inventory D5 |
| CPT-30, CPT-31 | probes; inventory A3, D11, D14 |
| CPT-32 to CPT-35 | probes; clang-diag-parse (the forms of a requirement); inventory C3, D6 to D8, D10, D12, D13 |

Inventory item D1, a requirement that fails for every argument, is
left out. The program needs no diagnostic, and the compilers differ
only in whether they diagnose a template that is never used, as
{rule}`DEP-30` allows.

Inputs: clang-diag-sema, clang-diag-parse.
