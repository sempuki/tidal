# Concepts and requires expressions

A {term}`constraint` is a condition on template arguments that the
implementation checks during translation. This page gives the ways to
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
:rules: CPT-8 CPT-16 CPT-17 CPT-19 CPT-20 CPT-21 CPT-23
:exceptions: CPT-18 CPT-22 CPT-37

A constraint can be written in these places:

| Place | Example | What it constrains | Rules |
|---|---|---|---|
| before a {term}`type parameter` | `template <Big T>` | the template | CPT-17, CPT-19 |
| before `auto` in a {term}`constant parameter` | `template <Big auto N>` | the template | CPT-21 |
| after the template parameter list | `template <class T> requires Big<T>` | the template | CPT-8 |
| before `auto` in a function parameter | `int f(Big auto x)` | the {term}`function template` | CPT-20 |
| after the {term}`declarator` of a function | `int f() requires Big<T>` | the function | CPT-11 |
| on a lambda | `[]<class T> requires Big<T> (T) {}` | the call operator | CPT-16 |
| before `auto` in a variable or a return type | `Big auto n = 1;` | the deduced type | CPT-22, CPT-23 |
| after `->` in a compound requirement | `{ e } -> Big;` | the type of `e` | CPT-33 |
| in a nested requirement | `requires Big<T>;` | the requires expression | CPT-35 |
:::

:::{principle} CPT-P4
:rules: CPT-9
:exceptions: CPT-10

A requires clause sits between two parts of a declaration, so the end
of its expression must be plain to see. Its operands are therefore
primary expressions joined by `&&` and `||`. Any other expression, a
negation or a comparison for example, goes in parentheses (CPT-10).
:::

:::{principle} CPT-P5
:rules: CPT-11
:exceptions: CPT-12 CPT-13 CPT-14 CPT-15

A trailing requires clause constrains a function that has {term}`template arguments <template argument>`
to check. The function is a template, or the function is
declared inside a templated entity. A generic lambda is a template. A
clause on any other function is not valid. A clause on a virtual
function is not valid either, because a call through a base class does
not check the constraints (CPT-14, CPT-15). The clause comes after the
declarator of a function and after any trailing return type (CPT-11 to
CPT-13).
:::

:::{principle} CPT-P2
:rules: CPT-1 CPT-2 CPT-3
:exceptions: CPT-4 CPT-5 CPT-6 CPT-7 CPT-36

A {term}`concept` gives a name to a constraint, so that declarations
can share the constraint. A concept has template parameters and a
constraint expression. A {term}`concept-id`, the name of the concept
with a template argument list, is a constant of type `bool`. Its value
says whether the arguments satisfy the concept ({rule}`CNS-7`). The
arguments must fit the concept's parameters in kind and number. A
concept-id with other arguments is not valid and has no value (CPT-36).

A concept is only a name for a constraint:

- A concept is defined once, at namespace scope. It has no
  {term}`specializations <specialization>` and no {term}`instantiations <instantiation>` (CPT-4, CPT-7).
- A concept has no constraints of its own (CPT-5).
- A concept cannot use its own name (CPT-6).

A named constraint also orders declarations. Two uses of one concept
give the same {term}`atomic constraints <atomic constraint>`, and two copies of the same text do
not ({rule}`CNS-4`).
:::

:::{principle} CPT-P3
:rules: CPT-24 CPT-25 CPT-28 CPT-29 CPT-30 CPT-31 CPT-32 CPT-33 CPT-35
:exceptions: CPT-34

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
ends the check (CPT-25). No requirement is evaluated (CPT-30).

The optional parameters are names with types, and no objects exist for
them (CPT-28). The list has the form of a function's parameter list,
inside a template as well as outside one:

- The parameter types adjust as in a function.
- `(void)` means no parameters.
- A later parameter can use an earlier one.

Nothing calls a requires expression, so the list has none of the parts
that serve a call. A parameter has no default argument and no
placeholder type, and the list does not end with an ellipsis (CPT-29).
:::

:::{principle} CPT-P6
:rules: CPT-26
:exceptions: CPT-27

A requirement that depends on template parameters is checked when the
arguments are substituted. If the requirement is not valid for the
arguments, the expression is `false` (CPT-26). A template uses the
result to test its arguments. A requirement that depends on no template
parameter has nothing to substitute. If the requirement is not valid,
the program is not valid, inside or outside a template (CPT-27).
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
:tests: templates/concepts/concept-id-value.cpp templates/concepts/concept-id-constant-uses.cpp

A {term}`concept-id` is a prvalue constant expression of type `bool`.
Its value is `true` if the template arguments satisfy the constraint
expression of the concept ({rule}`CNS-7`). A concept-id can appear
wherever a constant expression can.
:::

:::{rule} CPT-2
:kind: required
:tests: templates/concepts/parameter-kinds.cpp templates/concepts/parameter-kinds-constant-pack.cpp

A concept can have type parameters, constant parameters, {term}`template template parameters <template template parameter>`, and packs. The constraint expression can be any
`logical_or_expr`. The restricted grammar of CPT-10 does not apply to
it.
:::

:::{rule} CPT-3
:kind: required
:tests: templates/concepts/attribute.cpp templates/concepts/attribute-deprecated-used.cpp

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

The {term}`template head` of a concept must not have a requires clause or a
template parameter with a {term}`type constraint`.
:::

:::{rule} CPT-6
:kind: reject
:tests: templates/concepts/self-reference.cpp templates/concepts/self-reference-requirement.cpp

A concept definition must not use the concept's own name, directly or
in a template argument.
:::

:::{rule} CPT-36
:kind: reject
:tests: templates/concepts/concept-id-argument-kind.cpp templates/concepts/concept-id-argument-count.cpp

The template arguments of a concept-id must match the concept's
template parameters in kind and number. A concept-id with arguments
that do not match makes the program not valid, also where a `false`
value would be expected.
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
:tests: templates/concepts/leading-clause-places.cpp templates/concepts/leading-clause-guide.cpp templates/concepts/leading-clause-template-template.cpp

A {term}`requires clause` can follow the template parameter list of
these templates, and constrains the template ({rule}`CNS-1`):

- a {term}`class template`, function template, {term}`variable template`, alias
  template, or {term}`member template`,
- a {term}`partial specialization`,
- a template template parameter,
- a {term}`deduction guide`, which {term}`deduction` then uses only if the clause is
  satisfied.
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
:tests: templates/concepts/trailing-clause.cpp templates/concepts/trailing-clause-member.cpp templates/concepts/trailing-clause-friend.cpp templates/concepts/trailing-clause-lambda.cpp templates/concepts/trailing-clause-local-class.cpp templates/concepts/trailing-clause-member-initializer-lambda.cpp templates/concepts/trailing-clause-generic-lambda.cpp

A requires clause can follow the declarator of a function, after any
trailing return type. The function must be a {term}`templated entity`,
such as one of these ({question}`Q-0260`):

- a function template, including the call operator of a generic
  lambda,
- a member function whose class is a class template, or lies inside
  one,
- a friend defined in a class template,
- a lambda or a member of a local class inside a function template,
- a lambda inside a class template, such as one in a default member
  initializer.
:::

:::{rule} CPT-12
:kind: reject
:tests: templates/concepts/trailing-clause-before-return.cpp templates/concepts/trailing-clause-before-return-member.cpp

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
:tests: templates/concepts/trailing-clause-plain-function.cpp templates/concepts/trailing-clause-plain-member.cpp templates/concepts/trailing-clause-block-scope.cpp

A function that is not a {term}`templated entity` must not have a
trailing requires clause. A function declared at block scope inside a
function template is not a templated entity.
:::

:::{rule} CPT-15
:kind: reject
:tests: templates/concepts/virtual.cpp templates/concepts/virtual-override.cpp templates/concepts/virtual-constraint-not-satisfied.cpp

A virtual function must not have a trailing requires clause. A function
that overrides a virtual function is virtual, and the trailing clause
does not stop it from overriding ({question}`Q-0258`).
:::

:::{rule} CPT-16
:kind: required
:tests: templates/concepts/lambda-clauses.cpp templates/concepts/lambda-both-clauses.cpp

A lambda with a template parameter list can have a requires clause
after that list, and a trailing requires clause after its parameters.
Both clauses constrain the call operator. Attributes after the first
clause appertain to the lambda.
:::

### Type constraints and placeholders

:::{rule} CPT-17
:kind: required
:tests: templates/concepts/type-constraint-arguments.cpp templates/concepts/type-constraint-qualified.cpp templates/concepts/type-constraint-default.cpp templates/concepts/type-constraint-default-not-satisfied.cpp

A type constraint `C` or `C<A...>` before a type parameter `T`
constrains the template with `C<T>` or `C<T, A...>`. The name of the
concept can be qualified. The parameter can have a default argument,
which is checked only when a {term}`template-id` uses it ({rule}`CNS-13`).
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
:tests: templates/concepts/type-constraint-pack.cpp templates/concepts/type-constraint-pack-one-atom.cpp templates/concepts/type-constraint-variadic-concept.cpp templates/concepts/type-constraint-pack-same-concept-two-declarations.cpp

A type constraint `C` before a type parameter pack `T` gives the
constraint `(C<T> && ...)`, which is one atomic constraint
({rule}`CNS-3`, {question}`Q-0285`). The atom comes from the type
constraint where it is written, so the folds of two declarations are
two atoms ({rule}`CNS-4`, {question}`Q-0411`). An empty pack satisfies it. If
the first parameter of `C` is a pack, each check gives it one element.
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
:tests: templates/concepts/placeholder-constant-parameter.cpp templates/concepts/placeholder-constant-parameter-fallback.cpp templates/concepts/placeholder-constant-parameter-no-candidate.cpp templates/concepts/placeholder-constant-parameter-not-satisfied.cpp

A constant template parameter can have a constrained {term}`placeholder type`.
The type constraint, applied to the type that the argument gives, is a
constraint of the template ({rule}`CNS-1`). If the constant argument
does not satisfy the constraint, the function template has a
{term}`deduction failure` ({rule}`CNS-11`).
:::

:::{rule} CPT-22
:kind: reject
:tests: templates/concepts/placeholder-return-not-satisfied.cpp templates/concepts/placeholder-decltype-not-satisfied.cpp templates/concepts/placeholder-return-void.cpp templates/concepts/placeholder-return-void-expression.cpp

The type deduced for a constrained placeholder return type must satisfy
its type constraint, as {rule}`AUTO-16` requires for a variable. A
function with no `return` statement, or one that returns only
expressions of type `void`, deduces `void` for this check. CPT-21
covers a constant template parameter.
:::

:::{rule} CPT-23
:kind: required
:tests: templates/concepts/placeholder-deduced-part.cpp templates/concepts/placeholder-decltype-not-satisfied.cpp

The type constraint checks the type deduced for the placeholder alone.
References, pointers, and qualifiers written around the placeholder take
no part. For `decltype(auto)`, the deduced type is the type that
{rule}`AUTO-10` gives, with its reference.
:::

:::{rule} CPT-37
:kind: reject
:tests: templates/concepts/placeholder-parameter-decltype-auto.cpp templates/concepts/placeholder-structured-binding.cpp templates/concepts/placeholder-functional-cast.cpp templates/concepts/placeholder-functional-cast-braces.cpp

A constrained placeholder must not appear in these places:

- `decltype(auto)` as the type of a function parameter,
- a structured binding declaration ({question}`Q-0286`),
- a functional cast, such as `C auto(x)` or `C auto{x}`.
:::

### Requires expressions

:::{rule} CPT-24
:kind: required
:tests: templates/concepts/requires-expression-value.cpp statements/if/constexpr-discard-in-template.cpp

A requires expression is a prvalue constant expression of type `bool`.
Its value is `true` if every requirement holds. A requires expression
can appear wherever a constant expression can, inside or outside a
template.
:::

:::{rule} CPT-25
:kind: required
:tests: templates/concepts/requirement-order.cpp templates/concepts/requirement-order-after-hold.cpp

The requirements are checked in the order written. When one does not
hold, the value is `false`, and the requirements after it are not
checked or substituted.
:::

:::{rule} CPT-26
:kind: required
:tests: templates/concepts/requirement-failure-false.cpp templates/concepts/requirement-access-concept.cpp templates/concepts/requirement-access-member.cpp templates/concepts/requirement-access-concept-in-member.cpp templates/concepts/requirement-access-concept-kept.cpp templates/concepts/requirement-access-concept-kept-false.cpp templates/concepts/requirement-false-outside-template.cpp templates/concepts/requires-expression-lookup-outside-constraint.cpp templates/concepts/requirement-deleted-function-false.cpp templates/concepts/requirement-narrowing-in-braces-false.cpp templates/concepts/requirement-too-many-values-nested-aggregate.cpp statements/if/constexpr-requirement-too-many-values-discards-binding.cpp

A requirement that depends on template parameters does not hold if
substitution forms a type or an expression that is not valid. The
program stays valid. A name that is not accessible where the check
happens, and a call that selects a deleted function ({rule}`DEL-1`),
are not valid in this sense ({question}`Q-0288`). A
{term}`narrowing conversion` in braces ({rule}`LIST-2`) is also not
valid in this sense. If the same
concept-id is checked again where the access differs, and the result
would change, the program is not valid ({rule}`CNS-10`). In a
constraint, {rule}`CNS-7` gives the lookup. A requires expression
elsewhere, such as in the condition of `if constexpr` or in a variable
template, looks up its names as any expression of the template does
({rule}`DEP-9`).
:::

:::{rule} CPT-27
:kind: reject
:tests: templates/concepts/requires-expression-outside-template.cpp templates/concepts/requirement-deleted-function-independent.cpp templates/concepts/requirement-independent-in-template.cpp templates/concepts/requirement-independent-with-parameters.cpp templates/concepts/type-requirement-independent-constraints.cpp

A requirement that does not depend on template parameters must form
only valid types and expressions. The rule applies inside a template
too, even if the template is never instantiated.
:::

:::{rule} CPT-28
:kind: required
:tests: templates/concepts/requires-parameters.cpp templates/concepts/requires-parameters-adjusted.cpp templates/concepts/requires-parameters-void.cpp templates/concepts/requires-parameters-void-template.cpp templates/concepts/requires-parameter-iterator-type.cpp templates/concepts/requires-parameter-type-not-valid-false.cpp

A parameter of a requires expression is a name with a type, which the
requirements can use. No object is made, so the type need not be
constructible or destructible. A later parameter can use an earlier one.
The parameter types adjust as in a function ({rule}`DCL-8`), and
`(void)` means no parameters ({question}`Q-0290`). If substitution
into a parameter type forms a type that is not valid, the requires
expression is `false`, as for a requirement ({rule}`CPT-26`).
:::

:::{rule} CPT-29
:kind: reject
:tests: templates/concepts/requires-parameter-default.cpp templates/concepts/requires-parameter-ellipsis.cpp templates/concepts/requires-parameter-placeholder.cpp templates/concepts/requires-parameter-template-parameter-name.cpp templates/concepts/requires-parameter-enclosing-parameter-name.cpp

A parameter of a requires expression must not have a default argument
or a placeholder type, and must not have the name of an enclosing
template parameter ({rule}`TPL-18`). The parameter list must not end
with an ellipsis ({question}`Q-0259`).
:::

:::{rule} CPT-30
:kind: required
:tests: templates/concepts/requirements-unevaluated.cpp templates/concepts/requirements-unevaluated-constant.cpp

The expressions in a requires expression are not evaluated.
{rule}`UNEV-1` applies to them as to an operand of `sizeof`.
:::

:::{rule} CPT-31
:kind: required
:tests: templates/concepts/simple-requirement.cpp templates/concepts/simple-requirement-false-value.cpp statements/if/constexpr-discard-in-template.cpp

A simple requirement holds if its expression is valid.
:::

:::{rule} CPT-32
:kind: required
:tests: templates/concepts/type-requirement.cpp templates/concepts/type-requirement-not-instantiated.cpp templates/concepts/type-requirement-constraints.cpp templates/concepts/type-requirement-independent-constraints.cpp

A type requirement holds if its name forms a valid type. The name can
be a member type, a member template specialization, a class template
specialization, or an {term}`alias template` specialization. A class template
specialization is not instantiated, so the class can be incomplete. A
specialization whose arguments fail the template's constraints or its
default arguments is not valid. If those arguments depend on no
template parameter, CPT-27 applies.
:::

:::{rule} CPT-33
:kind: required
:tests: templates/concepts/compound-requirement-type.cpp templates/concepts/compound-requirement-noexcept.cpp templates/concepts/compound-requirement-order.cpp templates/concepts/compound-requirement-order-noexcept.cpp templates/concepts/compound-requirement-extra-argument.cpp templates/concepts/compound-requirement-extra-argument-unused.cpp

A compound requirement `{ E } noexcept -> C<A...>` is checked in these
steps. A step that fails ends the check, and the later steps are not
substituted ({question}`Q-0287`).

1. `E` must be valid.
2. With `noexcept`, `noexcept(E)` must be `true` ({rule}`UN-18`).
3. The arguments `A...` must be valid.
4. `C<decltype((E)), A...>` must be satisfied.
:::

:::{rule} CPT-34
:kind: reject
:tests: templates/concepts/requirement-body-empty.cpp templates/concepts/compound-requirement-type-name.cpp templates/concepts/simple-requirement-noexcept.cpp templates/concepts/type-requirement-keyword.cpp

The body of a requires expression must have at least one requirement.
`noexcept` and `->` appear only in a compound requirement, and `->` is
followed by a type constraint. A type after `->` is not valid, and a
type requirement must not name a keyword type such as `int`.
:::

:::{rule} CPT-35
:kind: required
:tests: templates/concepts/nested-requirement.cpp templates/concepts/nested-requirement-parameter-unevaluated.cpp templates/concepts/nested-requirement-parameter-evaluated.cpp templates/concepts/requirement-false-outside-template.cpp templates/concepts/nested-requirement-never-satisfied.cpp

A nested requirement holds if its constraint expression is satisfied
({rule}`CNS-5` to {rule}`CNS-7`). The parameters of the requires
expression have no values, so the expression can use them only in an
unevaluated operand ({rule}`CNS-9`, {question}`Q-0289`). A requirement
that starts with `requires` is always a nested requirement. A nested
requirement that is valid and false for every argument, such as
`requires false;`, is valid. The requires expression is then false for
every argument ({question}`Q-0400`).
:::

## Examples

% tidal-example: reject
```cpp
template <class T> concept Whole = T(1) / 2 == 0;
template <class T> concept Sized = requires (T t) {
  { t.size() } -> Whole;      // a compound requirement
  typename T::value_type;     // a type requirement
};

template <Sized T> int count(const T& c);                        // a type constraint
template <class T> requires Sized<T> int first(const T& c);     // a requires clause
int last(const Sized auto& c);                                   // a constrained placeholder
template <Whole auto N> int half();                              // constrains the template

template <class T> struct Box {
  int get() requires (sizeof(T) > 1);   // a trailing clause on a member
};

template <class T> requires !Sized<T> int bad(T);        // rejected: needs (!Sized<T>)
constexpr bool b = requires (int n) { n.size(); };        // rejected: depends on no template parameter
```

Outside a template, a requirement that is not valid makes the program
not valid (CPT-27). Two forms test such a requirement and give `false`
instead. A nested requirement holds a concept-id, which is valid and
only has the value `false` (CPT-35). A concept gives the requirement a
template parameter (CPT-26).

% tidal-example: reject
```cpp
struct Label {};
template <class T> concept Shape = requires (const T& s) { s.area(); };
static_assert(!requires (const Label& l) { l.area(); });   // rejected: depends on no template parameter
static_assert(!requires { requires Shape<Label>; });        // valid: the concept-id is false
static_assert(!Shape<Label>);                               // valid: the requirement depends on T

template <class T, int N> requires (N > 0) struct Stack {};
static_assert(!requires { typename Stack<int, 0>; });      // rejected: Stack<int, 0> is not valid
template <class T, int N> concept Makes = requires { typename Stack<T, N>; };
static_assert(!Makes<int, 0>);                             // valid
```

A concept cannot be a template argument ({rule}`TARG-7`). A class
template that holds the value of the concept-id can be one:

% tidal-example: reject
```cpp
template <class T> concept Integral = T(1) / 2 == 0;
template <class T> struct IsIntegral { static constexpr bool value = Integral<T>; };
template <template <class> class P, class... T> constexpr bool all = (P<T>::value && ...);
all<Integral, int>;                        // rejected: Integral is a concept
all<IsIntegral, int, long>;                // true
```

```cpp
template <class T> concept Wide = sizeof(T) >= 4;
static_assert(Wide<int> && !Wide<char>);   // a concept-id is a bool constant
char cells[Wide<int> + 1];                 // 2 elements

template <class T> concept Tagged = requires {
  typename T::tag;                         // checked first
  T::tag::size;                            // not checked when the first fails
};
static_assert(!Tagged<int>);               // false, and no error
```

A compound requirement checks the type of an expression through a
concept (CPT-33). A program that includes no header writes its own
same-type concept. A variable template with a partial specialization
gives the test ({rule}`TSP-10`):

```cpp
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class A, class B> concept SameAs = same<A, B>;

unsigned long hash(int);
template <class K> concept Key = requires (const K& k) {
  { hash(k) } -> SameAs<unsigned long>;  // SameAs<decltype((hash(k))), unsigned long>
  { k == k } -> SameAs<bool>;
};
static_assert(Key<int>);
static_assert(!Key<double*>);              // hash(k) is not valid
```

% tidal-example: diverges
```cpp
template <class T> int size_of(const T& t) {
  if constexpr (requires { t.size(); })    // true if t.size() is valid
    return t.size();
  else
    return 1;
}

template <class T> concept Valid = requires { sizeof(T) == 100; };
static_assert(Valid<char>);                // holds: the expression is valid, though false

template <class T> concept Old [[deprecated]] = true;   // the attribute appertains to Old
```

A simple requirement can ask that an operator's result converts to
`bool`, with no library concept ({rule}`CPT-31`):

```cpp
template <class T>
concept Ordered = requires(const T& a, const T& b) {
  bool(a < b);
  bool(a == b);
};
struct Opaque {};
static_assert(Ordered<int>);
static_assert(Ordered<const char*>);
static_assert(!Ordered<Opaque>);
```

A parameter of a requires expression can have a type that no object
of the program has yet, such as the iterator type of a range. `decltype`
of a call to a function that is only declared names the type
({doc}`/expressions/decltype`). No iterator is made ({rule}`CPT-28`):

```cpp
template <class T> T&& value_of();          // declared only
template <class R> using IteratorOf = decltype(value_of<R&>().begin());

template <class R>
concept Sequence = requires(IteratorOf<R> it, R& range) {
  ++it;
  *it;
  bool(it != range.end());
};

struct Digits {
  struct Step {
    int i;
    int operator*() const { return i; }
    Step& operator++() { ++i; return *this; }
    bool operator!=(const Step& other) const { return i != other.i; }
  };
  Step begin() { return {0}; }
  Step end() { return {10}; }
};
static_assert(Sequence<Digits>);
static_assert(!Sequence<int>);             // false: IteratorOf<int> is not valid
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 112
of 130 tests.

### Divergences

Each settled question below keeps its test's override for the family
that still differs. Reporting the EDG differences of Q-0260, Q-0281,
Q-0285, and Q-0286 to the EDG project waits on the maintainer
(`documentation/decisions/pending.md`).

- {question}`Q-0270` (CPT-3; settled): whether a concept definition can
  have an attribute after the concept's name. It can. Clang 18 rejects
  the attribute.
- {question}`Q-0260` (CPT-11; settled on CPT-P1): whether a lambda or a
  member of a local class inside a function template can have a
  trailing requires clause. It can, as Clang and GCC say. EDG rejects
  the clause on a lambda that is not generic, inside a function
  template or in a default member initializer of a class template, and
  on a member of a local class inside a function template.
- {question}`Q-0258` (CPT-15; settled on CPT-P5): whether a virtual
  function can have a trailing requires clause, or a constrained
  function override a virtual function. Neither can, as Clang and EDG
  say. GCC accepts both.
- {question}`Q-0285` (CPT-19, CNS-3; settled on {principle}`CNS-P1`):
  whether two variadic templates are ordered by the type constraints
  `A` and `AB` on their packs. They are not, and the call is ambiguous,
  as Clang and GCC say. EDG orders them in its C++23 mode.
- {question}`Q-0281` (CPT-21, CNS-1, CNS-23; settled on
  {principle}`CNS-P1`): whether a template with a constrained constant
  parameter is more constrained than the same template with `auto`. It
  is, as Clang 22 and GCC 16 say. EDG, Clang 18, and GCC 14 find the
  call ambiguous.
- {question}`Q-0288` (CPT-26; settled): whether a requires expression
  inside a member function can use a private member of the class. It
  can. GCC 14 gives `false`, so the `static_assert` of
  `requirement-access-member.cpp` fails.
- {question}`Q-0290` (CPT-28; open): whether `(void)` gives a requires
  expression no parameters inside a template. It does, as GCC and EDG
  say. Clang finds the expression `false`.
- {question}`Q-0259` (CPT-29; settled on CPT-P3): whether a parameter
  of a requires expression can have a default argument, or the list
  end in an ellipsis. Neither can, as Clang and EDG say. GCC accepts
  both.
- {question}`Q-0287` (CPT-33; open): whether the extra arguments of a
  compound requirement's type constraint are substituted only after the
  expression and the `noexcept` test hold, and also when the concept
  does not use them. Both, as EDG does. Clang substitutes the extra
  arguments first, so a class whose body has an error makes the program
  not valid ({rule}`CNS-33`), and Clang rejects programs that GCC and
  EDG accept. GCC never substitutes an extra argument that the concept
  does not use.
- {question}`Q-0289` (CPT-35, CNS-9; settled): whether a nested
  requirement that evaluates a parameter of the requires expression is
  an error. It is. Clang 18 gives `false`.
- {question}`Q-0400` (CPT-35; settled): whether a concept is valid when
  its requirement is satisfied for no argument. It is. No family
  differs.
- {question}`Q-0286` (CPT-37; settled on CPT-P1): whether a structured
  binding declaration can have a constrained placeholder. It cannot, as
  Clang and GCC say. EDG accepts it.
- {question}`Q-0411` (CPT-19, CNS-4; open): whether the folds that one
  concept makes on a type parameter pack in two declarations are one
  atom or two. Two atoms, so neither declaration is more constrained,
  as Clang and GCC say. EDG treats them as one atom.

### Split tests

| Test | Rules | Expected | Clang | GCC | EDG | Question |
|---|---|---|---|---|---|---|
| `attribute-deprecated-used.cpp` | CPT-3 | accept | Clang 22: accept; Clang 18: reject | accept | accept | {question}`Q-0270` |
| `attribute.cpp` | CPT-3 | accept | Clang 22: accept; Clang 18: reject | accept | accept | {question}`Q-0270` |
| `compound-requirement-extra-argument-unused.cpp` | CPT-33 | accept | accept | reject | accept | {question}`Q-0287` |
| `compound-requirement-order-noexcept.cpp` | CPT-33 | accept | reject | accept | accept | {question}`Q-0287` |
| `compound-requirement-order.cpp` | CPT-33 | accept | reject | accept | accept | {question}`Q-0287` |
| `nested-requirement-parameter-evaluated.cpp` | CPT-35, CNS-9 | reject | Clang 22: reject; Clang 18: accept | reject | reject | {question}`Q-0289` |
| `placeholder-structured-binding.cpp` | CPT-37 | reject | reject | reject | accept | {question}`Q-0286` |
| `requirement-access-member.cpp` | CPT-26 | accept | accept | GCC 16: accept; GCC 14: reject | accept | {question}`Q-0288` |
| `requires-parameter-default.cpp` | CPT-29 | reject | reject | accept | reject | {question}`Q-0259` |
| `requires-parameter-ellipsis.cpp` | CPT-29 | reject | reject | accept | reject | {question}`Q-0259` |
| `requires-parameters-void-template.cpp` | CPT-28 | accept | reject | accept | accept | {question}`Q-0290` |
| `trailing-clause-lambda.cpp` | CPT-11 | accept | accept | accept | reject | {question}`Q-0260` |
| `trailing-clause-local-class.cpp` | CPT-11 | accept | accept | accept | reject | {question}`Q-0260` |
| `trailing-clause-member-initializer-lambda.cpp` | CPT-11 | accept | accept | accept | reject | {question}`Q-0260` |
| `type-constraint-pack-one-atom.cpp` | CPT-19, CNS-3, CNS-24 | reject | reject | reject | accept | {question}`Q-0285` |
| `type-constraint-pack-same-concept-two-declarations.cpp` | CPT-19, CNS-4, OVL-36 | reject | reject | reject | accept | {question}`Q-0411` |
| `virtual-override.cpp` | CPT-15 | reject | reject | accept | reject | {question}`Q-0258` |
| `virtual.cpp` | CPT-15 | reject | reject | accept | reject | {question}`Q-0258` |

### Notes

- CPT-28: every compiler names the iterator type of a range with
  `decltype` of a member call through a null pointer of the range's
  type, and through a call to a function that is only declared, and
  accepts a parameter of that type in a requires expression
  (`requires-parameter-iterator-type.cpp`). Under `-Wall`, GCC 14 warns
  that the `this` pointer is null for the null-pointer form, and no
  compiler warns for the declared function
  (`reports/clean-room/roadmap-13/writer-probes/p4-4-warnings.json`).
  So the example uses the declared function. The P4 Port exercise
  asked for the example (P4-4).
- CPT-28: every compiler gives `false` for a requires expression whose
  parameter type is not valid after substitution, in a concept, in a
  variable template, and in an `if constexpr` condition
  (`requires-parameter-type-not-valid-false.cpp`,
  `reports/clean-room/roadmap-13/writer-probes/rv9-points.json`). The
  gap review of the phase 16 text (RV9-14) reaches `false` for the
  concept through the failed substitution into its constraint, and
  finds no sentence in the reference for a failure in the parameter
  list. The read of the RV9 repair diff (RV10-R-5) finds that the
  reference is silent for a variable template and an `if constexpr`
  condition, where no constraint is checked. The test's second and
  third assertions rest on CPT-28 alone. The round 14 writer probe
  (`reports/clean-room/roadmap-14/writer-probes/rv10-points.json`)
  gives `false` on every compiler in those two places and in a
  requires clause.
- CPT-14: every compiler rejects a trailing clause on a function
  declared at block scope inside a function template. The gap review
  expected a split there (item 27).
- CPT-15: in `virtual-constraint-not-satisfied.cpp` the constraint
  fails, and every compiler rejects the class. GCC rejects the use of
  the function that the class's virtual functions need.
- CPT-21: every compiler drops a function template whose constant
  argument fails the placeholder's constraint, and chooses another
  candidate.
- CPT-26: in `requirement-access-member.cpp`, `Box` is not a template,
  so the requirement depends on no template parameter, and CPT-27
  applies to it. The member is accessible there, so the requirement is
  valid. Every compiler judges access from the place of the check, also
  for a concept-id first checked inside a member. The gap review
  expected a concept to be checked in its own context (item 21). No
  compiler does this, and the rule follows the compilers. A check at
  another place with another result falls under {rule}`CNS-10`. When a
  concept-id is checked first inside the member and again at namespace
  scope, or in the other order, every compiler keeps the first result
  and none diagnoses the change (`requirement-access-concept-kept.cpp`,
  `requirement-access-concept-kept-false.cpp`).
- CPT-27: every compiler rejects a requirement that uses no template
  parameter inside a template that is never instantiated. The gap
  review expected the compilers to differ (item 20).
- CPT-27: every compiler rejects a requires expression outside a
  template whose requirement is not valid. The inventory names a doubt
  that some compilers give `false` (gap D4). None does.
- CPT-23: every compiler checks only the type deduced for the
  placeholder, so the inventory's doubt (gap C10) does not arise.
- CPT-35: inventory item D1 says that a requirement that fails for
  every argument makes the program not valid, with no diagnostic
  required. CPT-35 reads a nested requirement that is valid and false
  for every argument as valid ({question}`Q-0400`). Every compiler
  accepts such a concept, used or not, and gives `false`
  (`nested-requirement.cpp`, `nested-requirement-never-satisfied.cpp`).
  Every compiler also accepts, and gives `false` for, a concept whose
  expression requirement is not valid for any argument, such as the sum
  of two pointers. The page has no rule for that case.

### Sources

The clean-room writer probed each rule with Clang 18 and 22, GCC 14 and
16, the host Clang and GCC, and EDG 7.0, and checked each test on the
same compilers, the eight {ref}`baseline compilers <baseline-compilers>`.
The rejections name the reason in each compiler's own words, for
example "concept cannot have associated constraints" and
"non-templated function cannot have a requires clause"
(clang-diag-sema, clang-diag-parse).

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
| CPT-36, CPT-37 | probes; diagnostics (template argument kind, constrained placeholder in a structured binding or a cast) |

The gap review `reports/review/2026-10-31-gap-review-concepts-and-constraints.md`
led to these changes, by its own numbering:

| Rule | Gap review items |
|---|---|
| CPT-8 | 26 |
| CPT-11, CPT-14 | 27 |
| CPT-17 | 15 |
| CPT-19 | 13, 14 |
| CPT-21, CPT-22 | 1, 18 |
| CPT-26, CPT-27 | 20, 21 |
| CPT-28, CPT-29 | 23, 24 |
| CPT-32, CPT-34 | 25 |
| CPT-33 | 19 |
| CPT-35 | 22 |
| CPT-36 | 28 |
| CPT-37 | 16, 17 |

Inputs: clang-diag-sema, clang-diag-parse.
