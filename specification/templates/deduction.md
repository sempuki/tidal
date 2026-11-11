# Template argument deduction

{term}`Deduction <deduction>` finds the template arguments of a function template
from the way a program uses it. A call gives the most common case: the
types of the call arguments give the template arguments. This page
covers:

- the contexts that deduce, and the steps of a deduction in a call,
- the adjustments to a parameter type and an argument type before they
  are matched,
- the positions in a type that give a value, and those that give none,
- the differences that a match allows,
- braced lists, overload sets, and packs as arguments,
- how the results of all arguments combine,
- substitution, and the {term}`deduction failure` that removes a
  template from the candidates.

Partial ordering of templates is on {doc}`ordering`. Dependent names
are on {doc}`dependent-names`, and instantiation is on
{doc}`instantiation`. Packs are on {doc}`packs`, and the deduction of
the arguments of a class template is on {doc}`class-template-deduction`.
Constraints are on {doc}`concepts` and {doc}`constraints`.

Depends on: template declarations, template arguments, implicit
conversion sequences, overload resolution, placeholder types.

## Principles

:::{principle} DED-P1
:rules: DED-1 DED-30 DED-31 DED-32 DED-33 DED-34 DED-39

Deduction compares a type that holds template parameters with a type
that holds none. The context decides the two types:

| Context | Type with template parameters | Type without | Rules |
|---|---|---|---|
| a call | each function parameter that takes part | the type of its argument | DED-1 to DED-29 |
| a {term}`function template` name with a target type | the function type of the template | the function type of the target | DED-30, {rule}`OVL-18` |
| a conversion function template | its result type | the type that the initialization needs | DED-31 |
| an {term}`explicit specialization`, {term}`explicit instantiation`, or friend | the function type of each template with the name | the declared function type | DED-32, DED-33 |
| a placement deallocation function template | its parameter types after the first | the parameter types of the placement allocation function after the first | DED-39 |
| a {term}`partial specialization` | its argument list | the argument list of the use | {rule}`TSP-14` |
| {term}`partial ordering` | the types of one template | the invented types of the other | {principle}`ORD-P2` |
| a variable with a {term}`placeholder type` | the placeholder, as an invented {term}`type parameter` | the type of the initializer | {rule}`AUTO-1` to {rule}`AUTO-4` |

A function parameter with a placeholder type makes a function template
of its own (DED-34). Each context uses the matching of DED-P3. The
contexts differ in the adjustments before the match and in the
differences that the match allows.
:::

:::{principle} DED-P2
:rules: DED-2 DED-3 DED-4 DED-5 DED-6 DED-7 DED-8 DED-9 DED-16 DED-17 DED-18 DED-19 DED-20 DED-21 DED-22 DED-23 DED-35

A deduction in a call runs in five steps. Each step takes what the
previous step gives:

```text
a call, with explicit template arguments or without
   |
   v
1. explicit     the explicit arguments fix the leading         DED-2 to DED-5
   |            template parameters, in order
   v
2. pairs        each argument and its function parameter       DED-6 to DED-9,
   |            make a pair; both types are adjusted           DED-16 to DED-20
   v
3. match        each pair alone gives values to the template   DED-10 to DED-15,
   |            parameters in its deducible positions          DED-36, DED-37
   v
4. combine      the values from all pairs must agree;          DED-21 to DED-23
   |            defaults fill the rest
   v
5. substitute   the values replace the template parameters,    DED-24 to DED-29,
   |            in this order:                                 DED-38
   |            a. the defaults of the template parameters
   |            b. the check of each parameter that holds no
   |               template parameter against its argument
   |            c. the function type
   v
a specialization, which is a candidate of the call ({rule}`OVL-1`)
```

A failure in any step is a {term}`deduction failure`. A deduction
failure removes the template from the candidates, and the program stays
valid. The call then goes on with the other candidates. A call is not valid only when no candidate remains
(DED-35), or when an error comes from outside the function type
(DED-29).
:::

:::{principle} DED-P3
:rules: DED-10 DED-11 DED-12 DED-13 DED-36 DED-37

Matching takes a type apart along its structure. Where the parameter
type has a template parameter, the template parameter takes the part of
the argument type at that place:

| Form in the parameter type | Gives a value to |
|---|---|
| `T`, `const T`, `T*`, `T&`, `T&&` | `T` |
| `T[N]` | `T` and `N` |
| `T C::*` | `T` and `C` |
| `R(A1, A2)`, `R(*)(A1, A2)`, `R(&)(A1, A2)` | `R`, `A1`, `A2` |
| `void() noexcept(B)` | `B` |
| `Box<T, N>`, with `Box` a {term}`class template` | `T` and `N`, from the arguments of the {term}`specialization` |
| `TT<T>`, with `TT` a {term}`template template parameter` | `TT` and `T` |
| `TT<int>`, with `TT` a template template parameter | `TT` |
| `Hold<TT>`, with `Hold` a class template and `TT` a template template parameter | `TT` |
| `void (*)(A...)` and `List<H, A...>`, with `A` a pack | each element of `A`, from the remaining parameters or arguments |

The forms nest, so `Box<T*>&` gives `T` from `const Box<int*>`.

A value keeps the type it has at its place. A constant parameter
declared with `auto` takes that type:

| Place of the value | Type of an `auto` parameter |
|---|---|
| an array bound | the type of a `sizeof` expression, because a bound counts elements |
| a constant argument of `Box<N>` | the type of the parameter of `Box` |

Some positions give no value, because no single argument determines them:

| Position | Example | Why |
|---|---|---|
| a qualified name with a template parameter before `::`, in every part of the name | `typename Id<T>::type`, `typename Outer<T>::template In<U>` | many `T` can give one member, and the member depends on `T` |
| the operand of `decltype` | `decltype(T{})` | many `T` can give one type |
| an expression of a {term}`constant parameter` | `Fixed<N + 1>` | the value of `N` would need to be solved |
| the first bound of an array parameter | `int values[N]` | the parameter is a pointer ({rule}`DCL-8`) |
| a parameter of an {term}`alias template` that the aliased type does not use | `Id<Tag>`, with `Id<Tag>` the type `int` | the specialization is the aliased type ({rule}`TPL-28`), and `Tag` is not part of it |

A template parameter in such a position takes its value from another
position, from an explicit argument, or from a default.
:::

:::{principle} DED-P4
:rules: DED-14 DED-15

A match looks first for values that make the substituted parameter type
the same as the argument type. If no such values exist, the match accepts a near type. Each difference
that the match accepts is one that an implicit conversion makes without
user code:

| Difference | Example parameter | Example argument | Rule |
|---|---|---|---|
| more `const` on the referred type of a reference | `const Box<T>&` | `Box<int>` | DED-14 |
| a qualification adjustment of a pointer | `const T* const*` | `int**` | DED-14 |
| a function pointer without `noexcept` | `R (*)()` | `int (*)() noexcept` | DED-14 |
| a base class that is a specialization | `Base<T>&`, `Base<T>*` | a class derived from `Base<int>` | DED-15 |
| a base class and more `const` | `const Base<T>*` | a pointer to a class derived from `Base<int>` | DED-14, DED-15 |

The near step runs only when no exact values exist, and it can also
fail:

| Parameter | Argument | Result | Rule |
|---|---|---|---|
| `const T&` | an `int` lvalue | no `T` makes `const T` the same as `int`, so the near step gives `T = int` | DED-14 |
| `const T**` | `int**` | `T = int` gives `const int**`, which `int**` does not convert to ({rule}`CONV-9`), so the pair fails | DED-14 |

The match finds a base class by the class relation alone. Deduction
does not check that the base is unambiguous or accessible. The call
checks the conversion after it chooses the specialization (DED-15).

The arguments that have no type of their own deduce in their own ways:
a braced list (DED-16 to DED-18), an {term}`overload set` (DED-19), and the
arguments for a pack (DED-20).
:::

:::{principle} DED-P5
:rules: DED-24 DED-25 DED-26 DED-27 DED-28 DED-38
:exceptions: DED-29

Substitution puts the values into the function type. A result that is
not a valid type or expression is a {term}`substitution failure`. The
template is removed, and the program stays valid. A program can therefore offer a template only for
the arguments whose types support it. The removal covers the function
type itself and nothing that the function type causes elsewhere:

| An error in | Effect | Rule |
|---|---|---|
| a parameter type, the result type, or a template parameter's type or default | the template is removed | DED-24 |
| the operand of a conditional `explicit` | the template is removed | DED-24 |
| access to a name in those places | the template is removed | DED-26 |
| a call in those places with no {term}`viable function`, also a call of a lambda | the template is removed | DED-24, DED-29 |
| the body of a class that substitution instantiates | the program is not valid | DED-29 |
| the body of another function template that the function type needs, such as for its deduced result type | the program is not valid | DED-29 |
| the body of a lambda expression in the function type or in a template parameter | the program is not valid | DED-29 |
| the `noexcept` operand of the chosen specialization | the program is not valid | DED-27, DED-29 |

Substitution goes through the function type in source order and stops
at the first failure (DED-25), so a later part never causes an error.
The defaults of the template parameters come before the function type
(DED-28).
:::

## Constraints

### Contexts and explicit arguments

:::{rule} DED-1
:kind: required
:tests: templates/deduction/call-each-argument.cpp templates/deduction/call-member-and-operator.cpp classes/constructors/return-owner-converts-to-base-owner.cpp classes/operators/call-operator-template.cpp classes/operators/static-call-operator-template.cpp declarations/initialization/list/braced-constructor-template.cpp templates/deduction/array-bound-from-converted-elements.cpp classes/copy-move/assign-from-lambda-through-constructor-template.cpp templates/packs/member-template-enclosing-pack-after-deduced.cpp templates/packs/member-template-enclosing-pack-before-deduced.cpp

In a call that names a function template, {term}`deduction` finds the
{term}`template arguments <template argument>` from the call arguments. The specialization that
deduction gives is a candidate ({rule}`OVL-1`). A constructor template
gets its template arguments in the same way.
:::

:::{rule} DED-2
:kind: required
:tests: templates/deduction/explicit-leading.cpp templates/deduction/explicit-pack-then-deduced.cpp templates/deduction/pack-not-last-explicit-then-deduced.cpp

Explicit template arguments go to the template parameters in order
({rule}`TARG-20`), and they are put in place before deduction. A pack
takes the rest of them, and deduction can add more elements to the
pack. Deduction finds the template parameters that the explicit
arguments leave.
:::

:::{rule} DED-3
:kind: required
:tests: templates/deduction/explicit-fixed-parameter-converts.cpp templates/deduction/explicit-fixed-not-deduced.cpp templates/deduction/explicit-fixed-later-pair.cpp

A function parameter takes no part in deduction if explicit arguments
fix every template parameter in its type. A parameter whose type holds
no template parameter takes no part either. Its argument converts by
any implicit conversion ({rule}`ICS-1`). The argument for a
parameter that takes part converts only as DED-14 and DED-15 allow.
:::

:::{rule} DED-4
:kind: required
:tests: templates/deduction/explicit-empty-list.cpp templates/deduction/explicit-empty-list-default.cpp

A call whose function name has a template argument list, even `<>`,
has only function templates as candidates. A function with the name
that is not a template is not a candidate.
:::

:::{rule} DED-5
:kind: required
:tests: templates/deduction/explicit-invalid-removes.cpp templates/deduction/explicit-invalid-template-template.cpp

If the explicit arguments do not fit the parameters of a template
({rule}`TARG-6`, {rule}`TARG-7`), the template adds no candidate. A
constant argument that narrows ({rule}`TARG-13`) does not fit either.
:::

### Pairs and their adjustments

:::{rule} DED-6
:kind: required
:tests: templates/deduction/adjust-value-parameter.cpp templates/deduction/adjust-value-parameter-more.cpp

If a function parameter's type is not a reference, the argument type is
adjusted before the match:

- an array type becomes a pointer to its element type,
- a function type becomes a pointer to the function type,
- any other type loses its top-level `const` and `volatile`.
:::

:::{rule} DED-7
:kind: required
:tests: templates/deduction/adjust-reference-parameter.cpp templates/deduction/adjust-reference-rvalue.cpp templates/deduction/function-type-const-dropped.cpp

The top-level `const` and `volatile` of a parameter type take no part
in the match. For a reference parameter, the referred type is matched.
The argument then keeps its qualifiers and its array or function type.
A cv-qualifier on a function type has no effect ({rule}`CV-6`), so
`const T` matches a function type, and `T` takes that type. Except
under DED-8, the {term}`value category` of the argument takes no part.
:::

:::{rule} DED-8
:kind: required
:tests: templates/deduction/forwarding-reference.cpp templates/deduction/forwarding-const-not.cpp templates/deduction/forwarding-class-parameter-not.cpp templates/deduction/forwarding-nested-not.cpp templates/deduction/forwarding-nested-rvalue.cpp classes/members/explicit-object-deduced-forwarding.cpp classes/members/explicit-object-deduced-constant.cpp templates/deduction/forwarding-const-scalar-prvalue.cpp templates/deduction/forwarding-const-class-prvalue.cpp templates/deduction/forwarding-const-array-prvalue.cpp

A {term}`forwarding reference` is a function parameter of type `T&&`,
with `T` an unqualified type parameter of the template itself. `T&&`
inside another type, such as a function pointer type,
is no forwarding reference. For an argument of type `A`:

- if the argument is an lvalue, `T` is `A&`, and the parameter is an
  lvalue reference ({rule}`DCL-4`),
- otherwise, `T` is `A`.

A prvalue of a type that is neither a class nor an array has no
cv-qualifiers ({rule}`VC-2`). So for a call of a function that returns
`const int`, `A` is `int`. A class prvalue keeps its cv-qualifiers in
`A`, and an array prvalue keeps those of its elements.
:::

:::{rule} DED-9
:kind: required
:tests: templates/deduction/default-argument-passed.cpp templates/deduction/default-argument-not-deduced.cpp

A function parameter whose default argument the call uses takes no part
in deduction. The type of the default argument gives no value.
:::

### Matching

:::{rule} DED-10
:kind: required
:tests: templates/deduction/match-compound-types.cpp templates/deduction/match-shape-mismatch.cpp

The match compares the parameter type with the argument type part by
part, by the forms of {principle}`DED-P3`. A template parameter takes
the part of the argument type at its place. If the types differ in a
part with no template parameter, the pair fails, except as DED-14 and
DED-15 allow.
:::

:::{rule} DED-11
:kind: required
:tests: templates/deduction/match-constant-parameters.cpp templates/deduction/match-bound-type.cpp templates/deduction/match-argument-type.cpp templates/deduction/match-noexcept.cpp

A constant template parameter takes its value from these positions:

- an array bound, as the number of elements,
- a constant argument of a class template specialization,
- the `noexcept` of a function type, as a `bool` ({question}`Q-0208`).

A placeholder type for a bound becomes the type of a `sizeof`
expression, `decltype(sizeof 0)` ({question}`Q-0209`). If the
parameter's type is a type parameter, that type parameter takes the type of the value.
:::

:::{rule} DED-12
:kind: required
:tests: templates/deduction/match-constant-type-same.cpp templates/deduction/match-constant-type-differs.cpp templates/deduction/match-constant-placeholder-type.cpp templates/deduction/match-constant-placeholder-pattern.cpp templates/specialization/partial-constant-type-exact.cpp templates/specialization/partial-placeholder-pattern-const.cpp

A constant parameter can take its value from an argument of a class
template specialization. If its type has a placeholder, the parameter
takes the type that the class template's parameter has for that
argument. That type must have
the form of the placeholder type, such as `auto*`, with no conversion
({question}`Q-0295`). Otherwise the two parameters must have the same
type, apart from top-level `const`. If a type does not fit, the pair
fails.
:::

:::{rule} DED-13
:kind: required
:tests: templates/deduction/nondeduced-qualified-name.cpp templates/deduction/nondeduced-qualified-only.cpp templates/deduction/nondeduced-expression.cpp templates/deduction/nondeduced-decltype.cpp templates/deduction/nondeduced-array-by-value.cpp templates/deduction/nondeduced-explicit.cpp templates/deduction/nondeduced-member-template-arguments.cpp templates/deduction/nondeduced-member-template-explicit.cpp templates/deduction/nondeduced-alias-unused-parameter.cpp templates/deduction/nondeduced-alias-unused-other-position.cpp

A template parameter in a {term}`non-deduced position` takes no value
from the pair. The table in {principle}`DED-P3` lists these positions,
and a qualified name with a template parameter before `::` is one in
all its parts. The parameter's type is checked after substitution,
with the values from other positions.
:::

:::{rule} DED-36
:kind: required
:tests: templates/deduction/match-template-template.cpp templates/deduction/difference-derived-template-template.cpp

A template template parameter takes its value from these positions:

- the template of a class template specialization, as in `TT<T>` or
  `TT<int>`,
- a template argument of a class template specialization, as in
  `Hold<TT>`.
:::

### Allowed differences

:::{rule} DED-14
:kind: required
:tests: templates/deduction/difference-qualification.cpp templates/deduction/difference-exact-first.cpp templates/deduction/difference-qualification-not-reached.cpp templates/deduction/difference-qualification-not-reached-no-fallback.cpp

The match first looks for values that make the substituted parameter
type the same as the argument type. Only if no such values exist, the
substituted type can differ in these ways:

- The referred type of a reference parameter has more `const` or
  `volatile` than the argument.
- A pointer or pointer to member argument reaches the parameter type
  by a qualification adjustment or by dropping `noexcept`
  ({principle}`ICS-P2`).

If no values give a type that the argument reaches in these ways, the
pair fails, as for `const T**` and an `int**` argument ({rule}`CONV-9`).
:::

:::{rule} DED-15
:kind: required
:tests: templates/deduction/difference-derived.cpp templates/deduction/difference-derived-two-bases.cpp templates/deduction/difference-derived-nearest.cpp templates/deduction/difference-derived-exact-first.cpp templates/deduction/difference-derived-template-template.cpp templates/instantiation/class-deduction-from-derived.cpp templates/instantiation/class-deduction-from-derived-error.cpp templates/deduction/two-equal-bases-with-fallback.cpp templates/deduction/private-base-with-fallback.cpp templates/deduction/protected-base-with-fallback.cpp

A parameter type can be a template name with arguments, such as
`Base<T>` or `TT<T>`, or a pointer to one. If the argument does not
match such a parameter exactly, a class derived from a matching
specialization matches, also with the qualifiers that DED-14 lets a
parameter add. If two bases match with different values, the pair
fails, and a base derived from another matching base wins over that
base. Two base subobjects that match with the same values give those
values.

The search asks only whether the class is derived from the
specialization. Deduction does not check that the base is unambiguous
or accessible. If the call chooses the specialization, {rule}`OVL-37`
checks the conversion, and an ambiguous or inaccessible base makes the
program not valid ({rule}`CONV-17`, {rule}`ACC-12`).

The search needs the bases of the argument's class. If that class is a
specialization of a class template, the search instantiates it
({rule}`INST-1`), and an error in its body makes the program not valid
(DED-29).
:::

### Braced lists, overload sets, and packs

:::{rule} DED-16
:kind: required
:tests: templates/deduction/braced-initializer-list.cpp templates/deduction/braced-elements-differ.cpp templates/deduction/braced-empty.cpp templates/deduction/braced-overload-set-element.cpp

Some parameter types are `std::initializer_list<P>` ({rule}`LSUP-1`)
after the reference and top-level qualifiers are removed. For such a
type, each element of a braced list is an argument for `P` on its own.
An empty list gives no value. An element that names an overload set can
give no value (DED-19).
:::

:::{rule} DED-17
:kind: required
:tests: templates/deduction/braced-array-reference.cpp templates/deduction/braced-array-reference-conflict.cpp

For a parameter of type reference to an array of `P`, each element of
a braced list is an argument for `P`. A constant parameter in the bound
takes the number of elements. Nested lists give the inner bounds in the
same way.
:::

:::{rule} DED-18
:kind: required
:tests: templates/deduction/braced-other-parameter.cpp templates/deduction/braced-other-parameter-fixed.cpp

For any other parameter type, a braced list argument gives no value.
The list initializes the parameter after substitution.
:::

:::{rule} DED-19
:kind: required
:tests: templates/deduction/overload-set-one-match.cpp templates/deduction/overload-set-two-match.cpp templates/deduction/overload-set-template.cpp templates/deduction/overload-set-later.cpp templates/deduction/overload-set-default.cpp

If an argument names an overload set, each function of the set is
matched with the parameter type. The pair gives values only if the set
has no function template and exactly one function matches. Otherwise
the pair gives no value, and the set resolves after substitution
({rule}`OVL-39`). The parameter can then take its type from a {term}`default template argument`
(DED-23).
:::

:::{rule} DED-20
:kind: required
:tests: templates/deduction/pack-trailing.cpp templates/deduction/pack-not-last.cpp templates/deduction/pack-not-last-call.cpp templates/deduction/pack-not-last-other-position.cpp templates/deduction/pack-not-last-explicit-then-deduced.cpp

A {term}`function parameter pack` at the end of the parameters takes the
remaining arguments. Each of them makes a pair with the pattern of the
pack, as for one parameter. A pack before the end is a non-deduced
position. Such a pack is empty unless explicit arguments give it
elements, even when a later parameter type names the same pack.
:::

:::{rule} DED-37
:kind: required
:tests: templates/deduction/pack-function-type.cpp templates/deduction/pack-class-arguments.cpp templates/deduction/pack-class-arguments-reference-pattern.cpp templates/deduction/pack-class-arguments-pointer-pattern.cpp

A {term}`pack expansion` at the end of a list in a parameter type takes the
remaining elements of the matching list in the argument type, one by
one. Such a list is the parameter list of a function type, or the
template argument list of a class template specialization. The
expansion can take no element.
:::

### Combining the pairs

:::{rule} DED-21
:kind: required
:tests: templates/deduction/combine-agree.cpp templates/deduction/combine-conflict.cpp templates/deduction/combine-conflict-one-pair.cpp

Each pair gives values on its own, and one pair can give values to
several template parameters. All values for one template parameter must
be the same. Otherwise the result is a {term}`deduction failure`.
:::

:::{rule} DED-22
:kind: required
:tests: templates/deduction/combine-bound-yields.cpp templates/deduction/combine-bound-alone.cpp

A type parameter can be the type of a constant parameter in an array
bound. The bound gives the type parameter a value only if no other pair
does. The bound then converts to that type.
:::

:::{rule} DED-23
:kind: required
:tests: templates/deduction/combine-defaults.cpp templates/deduction/combine-missing.cpp templates/deduction/combine-leading-pack-empty.cpp templates/deduction/overload-set-default.cpp

A template parameter that has no value from a pair or an explicit
argument takes its default template argument ({rule}`TARG-9`). A
{term}`template parameter pack` with no value is empty, wherever the pack is in
the {term}`template head`. Any other template parameter without a value gives
a {term}`deduction failure`.
:::

### Substitution

:::{rule} DED-24
:kind: required
:tests: templates/deduction/substitution-failure-removes.cpp templates/deduction/substitution-failure-forms.cpp templates/deduction/substitution-adjusted-type.cpp templates/deduction/substitution-explicit-specifier.cpp templates/deduction/substitution-lambda-call-fails.cpp templates/deduction/specialization-distinct-functions.cpp templates/deduction/substitution-failure-undefined-operation.cpp templates/deduction/substitution-failure-evaluation-limit.cpp templates/deduction/substitution-failure-negative-bound.cpp templates/deduction/substitution-failure-negative-bound-computed.cpp templates/deduction/substitution-failure-negative-bound-ellipsis.cpp templates/deduction/constraint-failure-in-return-type.cpp templates/deduction/constraint-failure-in-default-template-argument.cpp templates/deduction/address-declared-result-body-not-instantiated.cpp templates/deduction/qualified-name-in-incomplete-class.cpp templates/deduction/substitution-failure-array-of-function-or-void.cpp templates/deduction/substitution-enumeration-qualifier.cpp templates/deduction/substitution-member-wrong-kind.cpp

The values replace the template parameters in these places, and the
function type is then adjusted as a declared one is ({rule}`DCL-8`):

- the function type,
- the types and defaults of the template parameters,
- the operand of a conditional `explicit` of a constructor or
  conversion function.

A type or expression that substitution makes and that is not valid is
a {term}`substitution failure`. The template then adds no candidate.
These types are among those that are not valid:

- a pointer to a reference, or a reference to `void`,
- an array of references, of functions, or of `void`,
- a pointer to a member of a type that is not a class,
- a qualified name such as `T::type`, when `T` is neither a class nor
  an enumeration, is an incomplete class, or has no such member,
- a qualified name whose member has the wrong kind: a value where a
  type is needed, a type where a value is needed, or a name that is
  not a template after `template`,
- a function that returns an array or a function,
- an array whose bound is zero or less ({rule}`DCL-19`,
  {question}`Q-0399`),
- a template-id whose arguments do not satisfy the constraints of its
  template ({rule}`CNS-13`).

A template argument whose {term}`constant evaluation` fails, by {rule}`CE-9` or
by a limit of {rule}`CE-10`, is such an expression. {rule}`PACK-27` gives the pack cases.
:::

:::{rule} DED-25
:kind: required
:tests: templates/deduction/substitution-order.cpp templates/deduction/substitution-order-return.cpp

Substitution goes through the function type in source order and stops
at the first failure. The parts after the failure are not substituted,
and no class in them is instantiated.
:::

:::{rule} DED-26
:kind: required
:tests: templates/deduction/substitution-access.cpp templates/deduction/substitution-access-member-function.cpp

Substitution checks access to each name in the function type, as at
the template's declaration ({rule}`ACC-3`). A name that is not
accessible is a {term}`substitution failure`.
:::

:::{rule} DED-27
:kind: required
:tests: templates/deduction/substitution-noexcept-later.cpp templates/deduction/substitution-noexcept-not-removed.cpp

The `noexcept` operand of a function template is not part of
substitution. The operand is substituted after the specialization is
chosen, and DED-29 applies to an error there.
:::

:::{rule} DED-38
:kind: required
:tests: templates/deduction/specialization-distinct-functions.cpp templates/deduction/specialization-distinct-unused-parameter.cpp

Each list of template arguments gives its own specialization. Two
specializations are different functions, even when adjustment gives
them the same function type.
:::

:::{rule} DED-28
:kind: required
:tests: templates/deduction/substitution-nondependent-first.cpp templates/deduction/substitution-default-before-check.cpp

After the defaults of the template parameters are substituted, each
function parameter whose declared type holds no template parameter is
checked against its argument ({question}`Q-0215`). If the argument has
no {term}`implicit conversion sequence`, the template adds no candidate, and
the function type is not substituted.
:::

:::{rule} DED-29
:kind: reject
:tests: templates/deduction/substitution-hard-error.cpp templates/deduction/substitution-lambda-body.cpp templates/deduction/substitution-noexcept-later.cpp templates/deduction/substitution-deduced-result-body.cpp templates/deduction/substitution-lambda-default.cpp templates/deduction/substitution-lambda-call-fails.cpp templates/deduction/substitution-default-before-check.cpp templates/deduction/losing-candidate-body-not-instantiated.cpp templates/deduction/address-deduced-result-body-instantiated.cpp templates/deduction/address-deduced-result-body-static-cast.cpp templates/deduction/address-declared-result-body-not-instantiated.cpp

An error in one of these places makes the program not valid, even
during deduction:

- the body of a class template specialization that substitution
  instantiates,
- the body of another function template specialization that the
  function type needs, for example to find its deduced result type,
- the body of a lambda expression in the function type or in a
  template parameter declaration,
- the `noexcept` operand of the chosen specialization (DED-27).

A call with no viable function in the function type is a
{term}`substitution failure` (DED-24), also when the called function is
a lambda.

Forming a candidate for overload resolution substitutes into the
function type only. The body of a candidate is instantiated only when
the call selects it, also when its result type is deduced. So an error
in the body of a candidate that loses, such as a generic lambda's call
operator against a plain one, does not make the program not valid.

Taking the address of an overload set with a target type deduces the
arguments of each function template from the target. A specialization
whose result type comes from its body is instantiated to find that
type, also when a plain function of the set matches the target. So an
error in that body makes the program not valid
({question}`Q-0420`).
:::

### Other contexts

:::{rule} DED-30
:kind: required
:tests: templates/deduction/context-address.cpp templates/deduction/context-address-mismatch.cpp

A function template name can have a target type ({rule}`OVL-18`). The
function type of the template is then matched with the target's
function type. DED-6 and DED-7 do not apply. A forwarding reference
matched with an lvalue reference type `A&` gives `T` the type `A&`.
If the result type of a specialization comes from its body, the match
instantiates the body, and DED-29 applies to an error there
({question}`Q-0420`).
:::

:::{rule} DED-31
:kind: required
:tests: templates/deduction/context-conversion.cpp templates/deduction/context-conversion-differences.cpp templates/deduction/context-conversion-exact.cpp templates/deduction/context-conversion-reference-qualifiers.cpp templates/deduction/context-conversion-qualification-place.cpp

A conversion function template takes its template arguments from the
type that the initialization needs ({rule}`ICS-4`). Before the match:

- a reference on either type is replaced by its referred type,
- for a needed type that is not a reference, an array or function
  result becomes a pointer,
- top-level `const` and `volatile` are removed from both, unless both
  types were references. Then the qualifiers of the needed type stay.

If no exact match exists, the result type can reach the needed type by
a qualification adjustment or by dropping `noexcept`. Each template
parameter then takes the type at its own place in the needed type, with
the qualifiers of that type.
:::

:::{rule} DED-32
:kind: required
:tests: templates/deduction/declaration-specialization.cpp templates/deduction/declaration-specialization-friend.cpp

These declarations can name a specialization of a function template:

- an explicit specialization,
- an explicit instantiation,
- a friend declaration.

Such a declaration matches its function type with each template of the
name, as DED-30 does. Partial ordering ({rule}`ORD-9`) chooses among
the templates that match.
:::

:::{rule} DED-33
:kind: reject
:tests: templates/deduction/declaration-ambiguous.cpp templates/deduction/declaration-ambiguous-instantiation.cpp

If two or more templates match such a declaration, partial ordering
must choose one. Otherwise the program is not valid. {rule}`TSP-4` gives the
case with no matching template.
:::

:::{rule} DED-34
:kind: required
:tests: templates/deduction/placeholder-parameter.cpp templates/deduction/placeholder-parameter-pointer-pack.cpp templates/deduction/placeholder-parameter-after-pack.cpp templates/deduction/placeholder-parameter-generic-lambda-inside.cpp templates/deduction/placeholder-parameter-returns-generic-lambda.cpp

A function parameter whose type has the placeholder `auto` makes the
function a function template. Each `auto` adds an invented type
parameter after those of the template head, in order. Explicit template
arguments can give the invented parameters. If the head ends in a pack,
the pack takes every explicit argument ({rule}`TARG-21`), and the
invented parameters get their types from deduction. A {term}`type constraint`
before `auto` constrains the invented parameter ({rule}`CPT-20`).
:::

:::{rule} DED-39
:kind: required
:tests: templates/deduction/placement-deallocation-match.cpp templates/deduction/placement-deallocation-ordered.cpp templates/deduction/placement-deallocation-ordered-instantiation.cpp expressions/new-delete/throw-placement-function-and-template.cpp expressions/new-delete/throw-placement-function-before-template-instantiation.cpp expressions/new-delete/throw-placement-deleted-function-and-template.cpp expressions/new-delete/throw-placement-templates-unordered.cpp

A `new` expression with placement arguments can need a placement
deallocation function ({doc}`/expressions/new-delete`, {rule}`NEW-23`). A deallocation
function template matches if deduction makes its parameter types after
the first the same as those of the allocation function. The
differences of DED-14 and DED-15 do not apply, so a template that
takes `const T*` does not match an allocation parameter `Arena*`. If
several templates match, partial ordering ({rule}`ORD-9`) chooses one
({question}`Q-0219`). If ordering prefers none of them, no deallocation
function is called ({rule}`NEW-24`, {question}`Q-0222`). Partial
ordering compares only templates. If a function that is not a template
also matches, the function and the template are two matches, and
NEW-24 calls neither ({question}`Q-0220`). Such a function is not
chosen, so the check of {rule}`NEW-25` does not reach it when it is
deleted. The body of the template is not instantiated.
:::

### No candidate

:::{rule} DED-35
:kind: reject
:tests: templates/deduction/explicit-fixed-not-deduced.cpp templates/deduction/adjust-reference-rvalue.cpp templates/deduction/forwarding-const-not.cpp templates/deduction/pack-not-last-call.cpp templates/deduction/context-address-mismatch.cpp templates/deduction/match-shape-mismatch.cpp templates/deduction/match-constant-type-differs.cpp templates/deduction/nondeduced-qualified-only.cpp templates/deduction/nondeduced-expression.cpp templates/deduction/nondeduced-decltype.cpp templates/deduction/nondeduced-array-by-value.cpp templates/deduction/default-argument-not-deduced.cpp templates/deduction/difference-derived-two-bases.cpp templates/deduction/braced-elements-differ.cpp templates/deduction/braced-empty.cpp templates/deduction/braced-other-parameter.cpp templates/deduction/combine-conflict.cpp templates/deduction/combine-conflict-one-pair.cpp templates/deduction/combine-missing.cpp templates/deduction/overload-set-two-match.cpp templates/deduction/overload-set-template.cpp templates/deduction/nondeduced-member-template-arguments.cpp templates/deduction/forwarding-nested-not.cpp templates/deduction/pack-not-last-other-position.cpp templates/packs/nondeduced-before-end-call.cpp templates/deduction/nondeduced-alias-unused-parameter.cpp templates/deduction/difference-qualification-not-reached-no-fallback.cpp

A use of a function template name must have a function after
deduction. These uses are not valid:

- a call with no viable function, because each template has a
  {term}`deduction failure` or gives a specialization that is not
  viable,
- a target type that no specialization matches.

The rule is the case of {rule}`OVL-35` and {rule}`OVL-20` for templates
(decision 0042).
:::

## Examples

% tidal-example: reject
```cpp
template <class T> T larger(T a, T b);
larger(1, 2);                   // T = int
larger(1, 2L);                  // rejected: T is int and long
larger<long>(1, 2L);            // T = long, and 1 converts

template <class T> void keep(T&&);
int n = 0;
keep(n);                        // T = int&, the parameter is int&
keep(1);                        // T = int, the parameter is int&&

template <class T, int N> int count(T (&)[N]);
int items[4];
count(items);                   // T = int, N = 4
count({1, 2, 3});               // T = int, N = 3
```

A substitution failure removes a template, and the call goes on:

% tidal-example: reject
```cpp
template <class T> int size(const T& c, typename T::size_type = 0);
int size(...);
size(1);                        // int::size_type is not valid; size(...)
```

An array bound of zero or less is not a valid type, so such a bound
also removes the template (DED-24):

```cpp
template <class T> int empty(char (*)[sizeof(T) - sizeof(T)]);
template <class T> long empty(...);
static_assert(sizeof(empty<int>(0)) == sizeof(long));   // the bound is 0, so empty(...)
```

An error in the function type removes the template, also a call of a
lambda with no viable function there (DED-24). An error in the body of
a lambda makes the program not valid (DED-29):

% tidal-example: reject
```cpp
template <class T>
auto first(T t) -> decltype([](auto x) -> decltype(x.size()) { return x.size(); }(t), 1);
int first(...);
first(1);                       // the lambda call has no viable function; first(...)

template <class T> int second(T, decltype([] { typename T::type v; }) * = nullptr);
int second(...);
second(1);                      // rejected: the error is in the body of the lambda
```

A braced list for a reference to an array deduces from each element
(DED-17). A narrowing conversion of an element still forms a
conversion sequence, which then ranks as usual ({rule}`ICS-8`,
{rule}`ICS-2`):

% tidal-example: fragment
```cpp
template <class T, int N> int len(const T (&)[N]);   // 1
int len(const long (&)[2]);                          // 2
len({1.5, 2.5});                // 1: T = double is an exact match, and 2 needs a conversion
len({1, 2L});                   // 2: T is int and long, so 1 is not viable
```

Explicit arguments fill the template parameters from the left, and a
parameter that they fix converts its argument:

% tidal-example: fragment
```cpp
template <class To, class From> To convert(From);
convert<long>('c');             // To = long, From = char
convert<long, int>('c');        // From = int, and 'c' converts

template <class... Ts> void all(Ts...);
all<long>(1, 'c');              // Ts = long, char
```

A pack before the end takes one argument for each explicit element.
The parameters after the pack take the arguments after those (DED-20):

```cpp
template <class... Ts, class U> int lead(Ts..., U);
int both = lead<char, short>('a', 1, 2.0);  // Ts = char, short; U = double
int none = lead(2.0);                       // Ts is empty; U = double
```

A placement deallocation template matches only when deduction gives
the same parameter types, with none of the differences of DED-14
(DED-39). Two templates that differ only in a template parameter that
no parameter type uses are not ordered, and no deallocation function is
called after a throw ({rule}`NEW-24`). EDG rejects the second `new`
expression as ambiguous ({question}`Q-0222`):

```cpp
using size = decltype(sizeof 0);
struct Arena {};
struct Widget {
  static void* operator new(size, Arena*);
  template <class T> static void operator delete(void*, T*);        // matches, T = Arena
  template <class T> static void operator delete(void*, const T*);  // const T* is never Arena*
  static void operator delete(void*);
  Widget();
};
Arena arena;
Widget* w = new (&arena) Widget;  // after a throw, the template with T*
```

% tidal-example: diverges
```cpp
using size = decltype(sizeof 0);
struct Arena {};
struct Gadget {
  static void* operator new(size, Arena*);
  template <class T> static void operator delete(void*, T*);
  template <class T, class U = int> static void operator delete(void*, T*);  // U takes no part
  static void operator delete(void*);
  Gadget();
};
Arena arena;
Gadget* g = new (&arena) Gadget;  // both match, and ordering prefers neither: no call (NEW-24)
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 148
of 161 tests.

### Divergences

Reporting the EDG differences to the EDG project waits on the
maintainer (`documentation/decisions/pending.md`).

- {question}`Q-0208` (DED-11; settled on DED-P3): whether the
  `noexcept` of a function type gives a value to a `bool` constant
  parameter. It does, as Clang and GCC say. EDG finds no match.
- {question}`Q-0209` (DED-11; settled on DED-P3): which type a
  placeholder constant parameter gets from an array bound. It gets the
  type of a `sizeof` expression, as Clang and EDG say. GCC gives `int`.
- {question}`Q-0295` (DED-12, {rule}`TSP-14`; open): whether a constant
  argument's type must fit the declared type of a partial
  specialization's constant parameter with no conversion. It must, as
  in a function template, and as GCC and EDG say. Clang converts the
  argument.
- {question}`Q-0399` (DED-24; settled): whether a negative array bound
  that substitution makes removes the template. It does in every form,
  as Clang says. GCC rejects a computed negative bound such as `N - 2`,
  and EDG rejects a negative bound in a pointer to an array when the
  other overload has an ellipsis.
- {question}`Q-0215` (DED-28; settled on DED-P2): whether the defaults
  of the template parameters are substituted before the check of the
  parameters with no template parameter. They are, as Clang and GCC
  say. EDG checks the parameter first and removes the template with no
  error.
- {question}`Q-0219` (DED-39; settled on {principle}`ORD-P1`): whether
  partial ordering chooses among matching placement deallocation
  templates. It does, as GCC and EDG say. Clang warns that two
  functions match and calls neither.
- {question}`Q-0220` (DED-39, NEW-24; open): whether a placement
  deallocation function wins over a template that also matches. It
  does not, and no deallocation function is called, as Clang does.
  GCC and EDG choose the function.
- {question}`Q-0222` (DED-39, NEW-24; open): whether a `new`
  expression is valid when two placement deallocation templates match
  and partial ordering prefers neither. It is, as Clang and GCC say.
  EDG rejects it as ambiguous.
- {question}`Q-0221` (NEW-25; open): whether a `new` expression is
  valid when its matching deallocation function is deleted. It is not,
  as Clang and EDG say. GCC accepts it.
- {question}`Q-0420` (DED-29, DED-30; open): whether the address of an
  overload set with a target type instantiates a template whose result type
  comes from its body, when a plain function matches the target. It
  does, and an error in the body makes the program not valid, as on
  Clang. GCC and EDG accept and pick the plain function.

### Split tests

EDG runs no programs, so its cell for a run test gives the result of
the compile.

| Test | Rules | Expected | Clang | GCC | EDG | Question |
|---|---|---|---|---|---|---|
| `address-deduced-result-body-instantiated.cpp` | DED-29 | reject | reject | accept | accept | {question}`Q-0420` |
| `address-deduced-result-body-static-cast.cpp` | DED-29 | reject | reject | accept | accept | {question}`Q-0420` |
| `expressions/new-delete/throw-placement-deleted-function-and-template.cpp` | NEW-24, DED-39 | accept | accept | accept | reject | {question}`Q-0220`, {question}`Q-0221` |
| `expressions/new-delete/throw-placement-function-and-template.cpp` | NEW-24, DED-39 | run | run | run-fail | accept | {question}`Q-0220` |
| `expressions/new-delete/throw-placement-templates-unordered.cpp` | NEW-24, DED-39 | run | run | run | reject | {question}`Q-0222` |
| `match-bound-type.cpp` | DED-11 | accept | accept | reject | accept | {question}`Q-0209` |
| `match-noexcept.cpp` | DED-11 | accept | accept | accept | reject | {question}`Q-0208` |
| `placement-deallocation-ordered.cpp` | DED-39, ORD-9 | run | run-fail | run | accept | {question}`Q-0219` |
| `substitution-default-before-check.cpp` | DED-28, DED-29 | reject | reject | reject | accept | {question}`Q-0215` |
| `substitution-failure-negative-bound-computed.cpp` | DED-24 | accept | accept | reject | accept | {question}`Q-0399` |
| `substitution-failure-negative-bound-ellipsis.cpp` | DED-24 | accept | accept | accept | reject | {question}`Q-0399` |
| `templates/specialization/partial-constant-type-exact.cpp` | TSP-14, DED-12 | accept | reject | accept | accept | {question}`Q-0295` |
| `templates/specialization/partial-placeholder-pattern-const.cpp` | TSP-14, DED-12 | accept | Clang 22: reject; Clang 18: accept | accept | accept | {question}`Q-0295` |

### Notes

- DED-35 is the reject rule of the tests with a deduction failure. In
  each, Clang names the failed step in a note: conflicting values,
  a parameter with no value, or a type that does not match.
  GCC and EDG report no matching function.
- DED-15: the nearest of two matching bases wins on every compiler,
  including the older Clang 18 and GCC 14.
- DED-17: a braced list with more elements than a fixed array bound
  gives no viable function on every compiler. Clang and EDG report no
  match, and GCC reports the initialization of the reference. The page
  does not say in which step the bound is checked.
- DED-29: every compiler rejects an error in the body of a lambda
  expression in the function type.
- DED-27: every compiler substitutes the `noexcept` operand of the
  chosen specialization, also in an operand of `decltype`.
- DED-28: when the class appears in a function parameter type instead
  of a default template argument, every compiler checks the first
  argument first and chooses the fallback
  (`substitution-nondependent-first.cpp`).
- DED-39: when a function and a template both match, no compiler
  instantiates the template. The body of the template in
  `throw-placement-function-before-template-instantiation.cpp` fails
  if it is instantiated, and every compiler accepts the test.
- DED-24: every compiler removes a template whose parameter type has an
  array bound of zero, also when the bound is an expression such as
  `sizeof(T) - sizeof(T)` (`substitution-failure-forms.cpp`). Every
  compiler removes a template for an explicit bound of `-1` in a
  reference to an array, and in a pointer to an array when the other
  overload takes `int` (`substitution-failure-negative-bound.cpp`).
- DED-15: every compiler instantiates `Derived<int>` for a call with a
  `Derived<int>*` argument and a `Base<T>*` parameter, and rejects the
  failing `static_assert` in its body
  (`class-deduction-from-derived-error.cpp`). The compilers also
  instantiate the class for a parameter `T*` or `Derived<T>*`, which the
  argument matches with no search of the bases.
- DED-20: the gap review (item E3) predicted that a later parameter
  `List<A...>` gives elements to a pack before the end. Every compiler
  keeps the pack empty and rejects the call
  (`pack-not-last-other-position.cpp`), and DED-20 follows them.
- DED-31: the gap review (item E5) predicted `T = int` for a conversion
  to `const int*` from `operator T*`, and to `const int&` from
  `operator T&`. Every compiler finds the exact match `T = const int`
  (`context-conversion-exact.cpp`). The differences of DED-31 apply
  only when no exact match exists (`context-conversion-differences.cpp`).
  When only one of the two types is a reference, every compiler drops
  the qualifiers: `operator T&` to `const int` and `operator T` to
  `const int&` both give `T = int`
  (`context-conversion-reference-qualifiers.cpp`).
- DED-12: every compiler checks the form `auto*` of a placeholder
  parameter in a function template.
- DED-15: with a fallback that takes an ellipsis, every compiler, EDG
  included, rejects a call whose argument has two `Base<int>` bases, a
  private `Base<int>` base, or a protected one, by reference and by
  pointer (`two-equal-bases-with-fallback.cpp`,
  `private-base-with-fallback.cpp`, `protected-base-with-fallback.cpp`).
  Each compiler names the ambiguous or inaccessible base, so deduction
  chose the template.
- DED-24: every compiler removes a template whose parameter names
  `T::type` with `T` an incomplete class, as for a complete class with
  no such member (`qualified-name-in-incomplete-class.cpp`).
- DED-8: every compiler gives `T = int` for a `const int` prvalue and
  `T = const Box` for a `const Box` prvalue.
- DED-14: GCC and Clang both report a deduction failure for `const T**`
  with an `int**` argument. Clang's note names the deduced type
  `const int **` against the argument type.
- DED-13: with an explicit argument for `T`, the parameter
  `typename Outer<T>::template In<U>` becomes `Outer<int>::In<U>`, and
  every compiler deduces `U`. The test gives `T` its value by deduction
  instead, so the whole qualified name stays non-deduced.

### Probe grids

The tables are in [`tables.md`](https://github.com/sempuki/tidal/blob/main/tests/templates/deduction/grids/tables.md) beside the grid files.

The probe grids are in `tests/templates/deduction/grids/`, and `tools/probe_grid.py` prints each table from the grid file. The compilers agree on every cell.

Grid file `call-types.json`. Rules: DED-6, DED-7, DED-8, DED-10, DED-14.

Each column declares one template
`f` that returns `Tag<T>`. Each row calls `f` with one argument and
checks that `T` is the type in the label. An `accept` means that `T`
is that type. A row whose cells all reject in one column means the
call has no match there.

In the row "int prvalue gives int", the column `T lref` rejects because
`int&` cannot bind the prvalue, after deduction gives `T = int`.

Grid file `braced.json`. Rules: DED-16, DED-17, DED-18.

Each column declares one template `f`,
and each row calls it with one braced list. An `accept` means that the
call is valid.

`Box` is an aggregate class template with one member of type `T`. The
last column takes an `std::initializer_list<int>` and a second
parameter `T` with the default argument `0`, which gives `T` no value
(DED-9).

### Sources

The clean-room writer probed each step with Clang 22, GCC 16, and EDG
7.0, and checked the tests and grids on the eight
{ref}`baseline compilers <baseline-compilers>`. This table gives the
source of each rule. Inventory items have the numbers of
`reports/review/2026-10-31-inventory-deduction.md`, and gap review
items those of
`reports/review/2026-10-31-gap-review-deduction-and-ordering.md`:

| Rule | Source |
|---|---|
| DED-1 | probes; inventory Q10 |
| DED-2, DED-3 | probes; clang-sema-deduction (explicit arguments substituted first); inventory F1, F5, F6, Q4 |
| DED-4, DED-5 | probes; inventory F3, F4, Q8 |
| DED-6, DED-7 | grid call-types; clang-sema-deduction (the adjustment of a call argument); inventory A1 to A6; Implement I5-3 |
| DED-8 | grid call-types; diagnostics; inventory G1 to G4; gap review O4; Implement I5-6 |
| DED-10 | grid call-types; probes; inventory E3, E6, E8, E9, E11 |
| DED-11, DED-12 | probes; diagnostics; inventory E1, E2, E4, E5, E10; gap review M9 |
| DED-13 | diagnostics; inventory E7, H1, H2, H4, H7, H8; gap review M1 |
| DED-9 | diagnostics; inventory D4, H3 |
| DED-14 | grid call-types; clang-sema-deduction (the check of each deduced argument against its original); inventory B3 to B5; Implement I5-8 |
| DED-15 | probes; clang-sema-deduction (the search of the bases); inventory B1, B2, B6; gap review E4, O3; Implement I5-4; front-end finding G153 |
| DED-16 to DED-18 | grid braced; clang-sema-deduction (the deduction from a braced list); inventory C1 to C8 |
| DED-21 to DED-23 | probes; diagnostics; inventory D1 to D3, D5, D6, F8; gap review E2 |
| DED-19 | probes; clang-sema-deduction (the resolution of an overload set for deduction); inventory I1 to I4 |
| DED-20 | probes; inventory H5, J1, J2, J4, J5; gap review E3 |
| DED-36 | probes; gap review M2, O3 |
| DED-37 | probes; gap review M3, M4 |
| DED-24, DED-25 | probes; inventory F7, Q4, Q7, Q10 to Q12; gap review M5; Implement I5-5, I5-11 |
| DED-38 | probe; gap review M8 |
| DED-29 | diagnostics; inventory Q5, Q6; gap review M6, E1, O1 |
| DED-26 | probes; inventory Q1 |
| DED-27 | diagnostics; inventory Q9 |
| DED-28 | probes; inventory Q3; gap review O2 |
| DED-30 | probes; diagnostics; inventory G5, K1, K2 |
| DED-31 | probes; inventory K3 to K5; gap review E5 |
| DED-32, DED-33 | probes; diagnostics; inventory K6, K8 |
| DED-34 | probes; inventory P3 |
| DED-39 | probes; gap review M7, M10 |
| DED-35 | diagnostics in the `tidal-error` lines of its tests |

Inventory items left out of this page:

- J3, packs of different lengths expanded together, and H6, a pack
  expansion before the end of a class template's argument list, are
  {rule}`PACK-6` and {rule}`PACK-12`.
- Q2, constraints checked after deduction, is {rule}`CNS-11`.
  {rule}`OVL-24` removes such a candidate.
- K7, a placeholder return type at the address of a template, belongs
  to return type deduction, which is not specified yet.
- K9 is {rule}`TSP-14`. P1, P2, and P4 are on
  {doc}`/declarations/auto` and with return type deduction.
- F2: no syntax gives explicit arguments to a constructor template, so
  DED-1 states only that a constructor template deduces.

Inputs: clang-sema-deduction.
