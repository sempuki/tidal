# Template declarations

A {term}`template` is a declaration with a list of parameters. A use of
the template gives arguments for the parameters and gets a
{term}`specialization`. This page covers:

- the four kinds of template: class, function, variable, and alias,
- member templates,
- the three kinds of template parameter,
- default template arguments,
- the scope of a template parameter, and redeclarations of a template.

Template arguments and template-ids are on {doc}`arguments`. Explicit
and partial specializations are on {doc}`specialization`. Deduction of
template arguments is on {doc}`deduction`. Dependent names are on
{doc}`dependent-names`, and instantiation is on {doc}`instantiation`.
Constraints are on {doc}`concepts` and {doc}`constraints`.

Depends on: declarations, class definitions and members, function
definitions, constant evaluation, name lookup.

## Principles

:::{principle} TPL-P1
:rules: TPL-1 TPL-22 TPL-26 TPL-28 TPL-45 TPL-46
:exceptions: TPL-29 TPL-31

A template is a pattern for a family of entities. The template head
lists the parameters, and the declaration after the head is the
pattern. Each list of arguments selects one member of the family, the
specialization for those arguments:

```text
template <class T> struct Box { T value; };     the template
             │
   Box<int>  │  Box<char*>                      template-ids
             ▼
   Box<int>, Box<char*>                         specializations: two classes
```

The declaration after the head decides the kind of template:

| Declaration after the head | Kind | A specialization is | Rule |
|---|---|---|---|
| a class | {term}`class template` | a class | TPL-1 |
| a function | {term}`function template` | a function | TPL-1, TPL-22 |
| a variable | {term}`variable template` | a variable | TPL-26 |
| `using` name `=` type | {term}`alias template` | the type that the alias gives, with no new type | TPL-28 |
| a member of a class | {term}`member template` | a member | TPL-24 |
:::

:::{principle} TPL-P5
:rules: TPL-23 TPL-24 TPL-42 TPL-44 TPL-47
:exceptions: TPL-25 TPL-27 TPL-43

A member of a class template is not a template of its own. The member
is part of each specialization of the class, so a definition outside
the class repeats the class's head (TPL-23). Each specialization has
its own copy of each member, including the `static` data members
(TPL-47).

A {term}`member template` has a head of its own. A definition outside a
class template then has two heads, the class's head first (TPL-24). A
member function template gives functions of its own, so a call can
choose between it and a plain member function (TPL-44). A member
function template is never virtual, and a data member that is not
`static` is never a template (TPL-25, TPL-27).
:::

:::{principle} TPL-P2
:rules: TPL-5 TPL-6 TPL-8 TPL-9 TPL-49 TPL-11 TPL-12 TPL-32
:exceptions: TPL-7 TPL-10 TPL-33

Each template parameter has a kind, and the kind decides what an
argument for it must be:

| Kind | Declared as | Takes | Inside the template, the name is |
|---|---|---|---|
| {term}`type parameter` | `class T`, `typename T` | a type | a type |
| {term}`constant parameter` | `int N`, `auto V`, `Point P` | a constant of the declared type | a value: a prvalue, or an lvalue for a reference or class type |
| {term}`template template parameter` | `template <class> class C` | a class template or alias template | a template |

A constant parameter holds a value that is known during translation.
Two arguments must be easy to compare, so that the implementation can
tell whether two {term}`template-ids <template-id>` name one specialization. The type of a
constant parameter is therefore limited to a {term}`structural type`.
:::

:::{principle} TPL-P7
:rules: TPL-30 TPL-36
:exceptions: TPL-35

A {term}`template parameter pack`, written with `...`, takes any number
of arguments of its kind. In the primary declaration of a class
template or a variable template, and in an alias template, the pack
comes last (TPL-30). A function template can have parameters after a
pack, if {term}`deduction` or a default gives each of them a value (TPL-35). A
constant pack can take its type from a type pack of an enclosing
template (TPL-36).
:::

:::{principle} TPL-P3
:rules: TPL-13 TPL-15 TPL-38 TPL-40
:exceptions: TPL-14 TPL-16 TPL-17 TPL-34 TPL-37 TPL-39

A parameter can have a {term}`default template argument`. A use that
leaves out the argument gets the default. A pack has no default. The
defaults of all the declarations of one template add up, but each
parameter gets its default in one declaration only. In a class,
variable, or alias template, the arguments are matched by position, so
only a trailing parameter can be left out. A function template can get
its arguments by deduction as well ({rule}`DED-23`), so its defaults
can come in any order.
:::

:::{principle} TPL-P4
:rules: TPL-19 TPL-48
:exceptions: TPL-18 TPL-41

A template parameter is a name in its own scope. The scope starts at the
parameter and ends at the end of the template's declaration. The name
hides the same name outside, and no declaration inside the template can
reuse the name. In a member definition outside the class, the members
of the class and of its bases come first (TPL-41).
:::

:::{principle} TPL-P6
:rules: TPL-20 TPL-21
:exceptions: TPL-2 TPL-3 TPL-4

Several declarations of one template must agree on the parameters:
their number, their kinds, and the types of the constant parameters.
The names can differ (TPL-20). The declaration after a head declares
exactly one entity (TPL-4).

Templates are declared only at namespace scope and in classes that are
not local:

| Place | A template there |
|---|---|
| namespace scope | valid |
| class body, of a class at namespace or class scope | valid, as a member template |
| block scope | not valid (TPL-2) |
| body of a local class | not valid (TPL-2) |
| inside `extern "C"` | not valid (TPL-3) |
| a friend declaration in a local class | not valid (TPL-2) |
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   template_decl: `template_head` `declaration`
   template_head: "template" "<" `template_params` ">" [`requires_clause`]
   template_params: `template_param` {"," `template_param`}
   template_param: `type_param` | `constant_param` | `template_template_param`
   type_param: `type_key` ["..."] [`identifier`]
             : | `type_key` [`identifier`] "=" `type_id`
             : | `type_constraint` ["..."] [`identifier`]
             : | `type_constraint` [`identifier`] "=" `type_id`
   type_key: "class" | "typename"
   constant_param: `parameter`
                 : | `decl_specifiers` {`ptr_part`} "..." [`identifier`]
   template_template_param: `template_head` `type_key` ["..."] [`identifier`]
                          : | `template_head` `type_key` [`identifier`] "=" [`nested_name`] `identifier`
```

A `template_decl` is a `declaration`, and in a class it is a
`member_decl`. The `parameter` of a `constant_param` gives the type, the
name, and the default after `=`.

## Constraints

### Templates and their kinds

:::{rule} TPL-1
:kind: required
:tests: templates/declarations/kinds.cpp templates/declarations/kinds-member-templates.cpp

A {term}`template head` followed by a declaration declares a template.
The declaration declares one of these:

- a class, which makes a {term}`class template`,
- a function, which makes a {term}`function template`,
- a variable, which makes a {term}`variable template`,
- an alias with `using`, which makes an {term}`alias template`,
- a member of a class, which makes a {term}`member template`. The
  member can be a function, a nested class, or a static variable.
:::

:::{rule} TPL-2
:kind: reject
:tests: templates/declarations/block-scope.cpp templates/declarations/local-class-member.cpp templates/declarations/local-class-friend.cpp

A template must not be declared at block scope. A template must not be
declared as a member of a local class. A local class must not declare a
friend template.
:::

:::{rule} TPL-3
:kind: reject
:tests: templates/declarations/c-linkage.cpp templates/declarations/c-linkage-explicit.cpp templates/declarations/c-linkage-partial.cpp

A template, an {term}`explicit specialization`, and a {term}`partial specialization`
must not have C language linkage ({rule}`LSP-1`).
:::

:::{rule} TPL-4
:kind: reject
:tests: templates/declarations/typedef-template.cpp templates/declarations/two-declarators.cpp templates/declarations/unnamed-class.cpp templates/declarations/class-template-declarator.cpp

The declaration after a template head must declare exactly one named
entity. These declarations are not valid after a template head:

- a `typedef` declaration,
- a declaration with two or more {term}`declarators <declarator>`,
- a class without a name,
- a class definition followed by a declarator, such as a variable
  name. A declaration of a class template declares no other entity.
:::

:::{rule} TPL-22
:kind: required
:tests: templates/declarations/function-template-overload.cpp templates/declarations/function-template-heads-differ.cpp templates/declarations/default-friend-overloads.cpp

Function templates and plain functions can share a name in one scope.
Each function template with the name adds a candidate to overload
resolution ({rule}`OVL-1`, {rule}`DED-1`). Two function templates
with the same function type and heads that are not equivalent (TPL-20)
are two templates.
:::

:::{rule} TPL-45
:kind: required
:tests: templates/declarations/plain-function-not-from-template.cpp templates/declarations/plain-function-beside-specialization.cpp

A plain function with the name and type of a specialization of a
function template is a different function. The template never gives the
definition of the plain function.
:::

:::{rule} TPL-29
:kind: reject
:tests: templates/declarations/main-template.cpp templates/declarations/main-template-constant.cpp

The function `main` must not be a template.
:::

### Template parameters

:::{rule} TPL-5
:kind: required
:tests: templates/declarations/type-parameter-keywords.cpp templates/declarations/type-parameter-keywords-unnamed.cpp

A {term}`type parameter` is declared with `class` or `typename`. The
two keywords mean the same thing. The name of any template parameter
can be left out.
:::

:::{rule} TPL-6
:kind: required
:tests: templates/declarations/constant-parameter-types.cpp templates/declarations/constant-parameter-void-pointer.cpp

A {term}`constant parameter` is declared like a function parameter. Its
type must be a {term}`structural type`, which is one of these:

- an integral or enumeration type,
- a pointer to an object type, to `void`, or to a function type,
- an lvalue reference to an object or function type,
- a pointer to member,
- the type of `nullptr`,
- a floating-point type,
- a literal class type whose bases and non-static data members are all
  public, not `mutable`, and of structural types or arrays of them.

Each of these types can have `const` or `volatile` (TPL-32).
:::

:::{rule} TPL-32
:kind: required
:tests: templates/declarations/constant-parameter-cv.cpp templates/declarations/constant-parameter-cv-placeholder.cpp

The implementation drops `const` and `volatile` at the top level of the
type of a constant parameter. So `const int N` and `int N` declare the
same parameter, and the name `N` is a prvalue of type `int`.
:::

:::{rule} TPL-33
:kind: reject
:tests: templates/declarations/constant-parameter-storage-class.cpp templates/declarations/constant-parameter-defines-type.cpp

The declaration of a constant parameter must not have a storage class
specifier, such as `static`. The declaration must not define a class or
an enumeration.
:::

:::{rule} TPL-7
:kind: reject
:tests: templates/declarations/constant-parameter-rvalue-reference.cpp templates/declarations/constant-parameter-void.cpp templates/declarations/constant-parameter-private-member.cpp templates/declarations/constant-parameter-mutable-member.cpp templates/declarations/constant-parameter-not-literal.cpp templates/declarations/constant-parameter-auto-void.cpp templates/declarations/constant-parameter-auto-not-structural.cpp templates/declarations/constant-parameter-no-constexpr-constructor.cpp

A constant parameter must not have a type that TPL-6 leaves out. Such
types include:

- an rvalue reference type,
- `void`,
- a class with a data member that is private, protected, or `mutable`,
- a class that is not a {term}`literal type`, such as a class with a
  destructor that is not `constexpr`,
- a class that is not an aggregate and has no `constexpr` constructor
  other than a copy or move constructor.

The rule also applies to the type that a placeholder gets from the
argument (TPL-11).
:::

:::{rule} TPL-8
:kind: required
:tests: templates/declarations/constant-parameter-adjusted.cpp templates/declarations/constant-parameter-adjusted-unknown-bound.cpp

If a constant parameter is declared with an array type or a function
type, its type is the matching pointer type, as for a function
parameter ({rule}`DCL-8`).
:::

:::{rule} TPL-9
:kind: required
:tests: templates/declarations/constant-parameter-category.cpp templates/declarations/constant-parameter-class-address.cpp templates/declarations/constant-parameter-shared-object.cpp templates/declarations/parameter-object-address-constant.cpp templates/declarations/parameter-object-not-destroyed.cpp

In the template, the name of a constant parameter is an expression with
this {term}`value category`:

| Type of the parameter | The name is |
|---|---|
| an lvalue reference | an lvalue that refers to the argument's object or function |
| a class type `C` | an lvalue of type `const C` |
| any other type | a prvalue with the argument's value |

For a class type, all uses with equal arguments refer to one object
with static {term}`storage duration`. The object is the same in the whole
program, also for parameters of two different templates.
:::

:::{rule} TPL-49
:kind: required
:tests: templates/declarations/parameter-object-not-destroyed.cpp templates/declarations/parameter-object-before-dynamic-init.cpp templates/declarations/parameter-object-address-constant.cpp

The object of TPL-9 for a parameter of class type has its value from
the start of the program, before any dynamic initialization. The
program never destroys the object. The address of the object is a
constant expression, and objects for arguments that are not equal have
different addresses.
:::

:::{rule} TPL-10
:kind: reject
:tests: templates/declarations/constant-parameter-assign.cpp templates/declarations/constant-parameter-address.cpp

A program must not assign to a constant parameter of {term}`scalar type` or
take its address, because the name is a prvalue (TPL-9).
:::

:::{rule} TPL-11
:kind: required
:tests: templates/declarations/constant-parameter-auto.cpp templates/declarations/constant-parameter-class-template.cpp templates/class-template-deduction/constant-parameter-guide.cpp

The type of a constant parameter can be one of these:

- a {term}`placeholder type`,
- the name of a class template with no argument list.

The parameter then gets its type from its argument, as a variable
declared with that type gets its type from its initializer
({rule}`AUTO-1`, {rule}`CTAD-1`).
:::

:::{rule} TPL-12
:kind: required
:tests: templates/declarations/template-template-parameter.cpp templates/declarations/template-template-parameter-heads.cpp

A {term}`template template parameter` has its own template head,
followed by `class` or `typename` and the name. In the template, the
name is a template that takes arguments for the parameters of that
head.
:::

:::{rule} TPL-40
:kind: required
:tests: templates/declarations/template-template-inner-default.cpp templates/declarations/template-template-inner-default-missing.cpp

A parameter in the head of a template template parameter can have a
default. The default belongs to that one declaration of the template
template parameter. A member definition outside the class uses the
inner defaults of its own head, and not those of the class's head. A
template-id of the parameter is checked against that head at the
definition, also if no specialization is made.
:::

:::{rule} TPL-30
:kind: reject
:tests: templates/declarations/pack-last.cpp templates/declarations/pack-declared.cpp templates/declarations/pack-last-alias.cpp templates/declarations/pack-last-variable.cpp templates/declarations/pack-then-deducible.cpp templates/declarations/pack-typed-by-same-head.cpp templates/specialization/partial-pack-before-parameter.cpp

A {term}`template parameter pack` is a parameter declared with `...`.
In these templates, a pack must be the last parameter:

- a class template or a variable template, in its primary declaration,
- an alias template.

A function template can have parameters after a pack (TPL-35). The head
of a partial specialization can too ({rule}`TSP-9`).
:::

:::{rule} TPL-35
:kind: reject
:tests: templates/declarations/pack-then-deducible.cpp templates/declarations/pack-then-not-deducible.cpp templates/declarations/pack-then-pack-not-deducible.cpp templates/declarations/pack-then-typed-pack-not-deducible.cpp

In a function template, each parameter after a pack must have a default
or be found by deduction from the function parameters
({rule}`DED-20`). The rule applies to the declaration, also if no call
uses the template ({question}`Q-0195`).
:::

:::{rule} TPL-36
:kind: required
:tests: templates/declarations/pack-typed-by-enclosing-pack.cpp templates/declarations/pack-typed-by-same-head.cpp templates/declarations/pack-then-typed-pack-not-deducible.cpp

A constant parameter pack can have a type that names a type pack of an
enclosing template, as in a member template of a class template. The
type pack must not be declared in the same head. In a class template,
such a head breaks TPL-30. {rule}`PACK-26` gives the other pack kinds.
:::

### Default template arguments

:::{rule} TPL-13
:kind: required
:tests: templates/declarations/defaults.cpp templates/declarations/defaults-function-any-order.cpp templates/declarations/default-greater-parenthesized.cpp

A parameter of any kind can have a {term}`default template argument`
after `=`, unless the parameter is a pack (TPL-34). A default can name
the parameters before it.
:::

:::{rule} TPL-34
:kind: reject
:tests: templates/declarations/default-pack.cpp templates/declarations/default-pack-constant.cpp templates/declarations/default-pack-template.cpp

A {term}`template parameter pack` must not have a default template
argument. The rule applies to packs of all three kinds, in every kind
of template.
:::

:::{rule} TPL-39
:kind: reject
:tests: templates/declarations/default-greater.cpp templates/declarations/default-greater-parenthesized.cpp

In a template head, the default of a constant parameter ends at the
first `>` that is not inside parentheses, brackets, or braces, as a
{term}`template argument` does ({rule}`TARG-3`). A `>` operator in such a
default must be in parentheses.
:::

:::{rule} TPL-14
:kind: reject
:tests: templates/declarations/default-not-last.cpp templates/declarations/default-not-last-variable.cpp templates/declarations/default-not-last-alias.cpp templates/declarations/defaults-merge-gap.cpp templates/declarations/defaults-merge-gap-later.cpp templates/declarations/defaults-merge.cpp

Unless the template is a function template, each parameter after one
with a default must have a default too. A pack needs no default. The
defaults of the declaration count together with those of the earlier
declarations ({rule}`TPL-15`, {question}`Q-0296`).
:::

:::{rule} TPL-15
:kind: required
:tests: templates/declarations/defaults-merge.cpp templates/declarations/defaults-merge-gap-later.cpp

The defaults of the declarations of one template add up. A later
declaration can give a default for a parameter that an earlier
declaration left without one.
:::

:::{rule} TPL-16
:kind: reject
:tests: templates/declarations/default-repeated.cpp templates/declarations/default-repeated-function.cpp

Two declarations of one template must not both give a default for the
same parameter, even if the defaults are the same.
:::

:::{rule} TPL-17
:kind: reject
:tests: templates/declarations/default-member-definition.cpp templates/declarations/default-class-member-outside.cpp

A member of a class template can be defined after the class body. That
definition must not give a default template argument, in the class's
head or in the member's own head.
:::

:::{rule} TPL-37
:kind: reject
:tests: templates/declarations/default-friend-class.cpp templates/declarations/default-friend-declaration.cpp

A friend declaration of a template must not give a default template
argument, unless the declaration is a definition of a function template
({question}`Q-0196`).
:::

:::{rule} TPL-38
:kind: reject
:tests: templates/declarations/default-friend-definition.cpp templates/declarations/default-friend-redeclared.cpp templates/declarations/default-friend-declared-before.cpp templates/declarations/default-friend-overloads.cpp

A friend function template that is defined in a class can give default
template arguments. That definition must be the only declaration of the
function template in the {term}`translation unit` ({question}`Q-0197`).
A declaration with the same name and function type and an equivalent
head and constraints declares the same function template (TPL-20,
TPL-22).
:::

### Scope and redeclaration

:::{rule} TPL-19
:kind: required
:tests: templates/declarations/parameter-scope.cpp templates/declarations/redeclare-names-differ.cpp templates/declarations/parameter-namespace-name.cpp

The scope of a template parameter starts at its declaration and ends at
the end of the template's declaration. The parameter hides a name
declared outside the template ({rule}`NAM-2`). Two declarations of one
template can use different names for a parameter.
:::

:::{rule} TPL-41
:kind: required
:tests: templates/declarations/parameter-hidden-by-base.cpp templates/declarations/parameter-namespace-name.cpp

In a definition of a member outside its class template, lookup searches
the class before the template parameters. The search covers the members
of the class and of its bases that do not depend on a parameter. So such
a member hides a parameter of the definition's head with the same name.
A name from the enclosing namespace does not hide the parameter.
:::

:::{rule} TPL-18
:kind: reject
:tests: templates/declarations/parameter-twice.cpp templates/declarations/parameter-member-name.cpp templates/declarations/parameter-local-name.cpp templates/declarations/parameter-class-template-name.cpp

A name must not be declared again in the scope of a template parameter
with that name. These declarations are not valid:

- a second parameter with the name in the same head,
- a member with the name in a class template,
- a variable with the name in the body of a function template,
- a class template with the name of its own parameter.

TPL-48 gives the case of a friend function. {rule}`CPT-29` names the specific case of a {term}`requires expression`
parameter (decision 0042).
:::

:::{rule} TPL-48
:kind: required
:tests: templates/declarations/parameter-friend-name.cpp

In a class template, a friend function declaration can use a name that
the class's own parameter list declares. The friend declares no name in
the template's scope, so TPL-18 does not apply ({question}`Q-0198`).
:::

:::{rule} TPL-20
:kind: reject
:tests: templates/declarations/redeclare-count.cpp templates/declarations/redeclare-kind.cpp templates/declarations/redeclare-type.cpp templates/declarations/member-template-one-head.cpp templates/declarations/redeclare-pack.cpp templates/declarations/redeclare-template-template.cpp templates/declarations/function-template-heads-differ.cpp

All declarations of one template must have equivalent template heads.
Equivalent heads have the same number of parameters, and each pair of
parameters at one position has:

- the same kind,
- a pack in both declarations or in neither,
- for constant parameters, the same type,
- for template template parameters, equivalent heads, by this rule.

For function templates, a declaration whose head is not equivalent
declares another template (TPL-22). The {term}`constraints <constraint>` must also be
equivalent ({rule}`CNS-26`, {rule}`CNS-27`).
:::

:::{rule} TPL-21
:kind: reject
:tests: templates/declarations/name-conflict.cpp templates/declarations/name-conflict-variable-template.cpp

Only function templates share a name with other entities in one scope
(TPL-22). A template of any other kind must have a name that no other
entity in its scope has ({rule}`NAM-3`).
:::

### Members of class templates

:::{rule} TPL-23
:kind: required
:tests: templates/declarations/member-outside.cpp templates/declarations/member-class-outside.cpp templates/declarations/member-outside-names.cpp

The functions, `static` data members, and member classes of a class
template can be defined after the class body. Such a definition starts
with the class's template head and names the member with the class's
template-id, such as `Box<T>::get`. The parameter names can differ from
those of the class (TPL-43).
:::

:::{rule} TPL-43
:kind: reject
:tests: templates/declarations/member-outside-order.cpp templates/declarations/member-outside-pack.cpp templates/declarations/member-outside-names.cpp

In a definition of a member outside its class template, the argument
list after the class name must list the head's parameters in their
order. A pack must appear with `...`.
:::

:::{rule} TPL-42
:kind: required
:tests: templates/declarations/member-enum-outside.cpp templates/declarations/member-enum-outside-unscoped.cpp

In a class template, an enumeration member with a fixed underlying type
can get its enumerators after the class body, in a definition that
starts with the class's head ({question}`Q-0199`). The underlying type
can name a parameter of the class.
:::

:::{rule} TPL-47
:kind: required
:tests: templates/declarations/static-member-per-specialization.cpp templates/declarations/static-member-per-specialization-address.cpp

Each specialization of a class template has its own `static` data
members. A value stored through one specialization does not change the
member of another.
:::

:::{rule} TPL-24
:kind: required
:tests: templates/declarations/member-template.cpp templates/declarations/member-template-one-head.cpp

A class or class template can declare a member template. A definition
of a member template of a class template outside the class has two
template heads: the class's head, then the member's head. One head
with the parameters of both is not valid.
:::

:::{rule} TPL-25
:kind: reject
:tests: templates/declarations/member-template-virtual.cpp templates/declarations/destructor-template.cpp

A member function template must not be `virtual`. A destructor must not
be a member template.
:::

:::{rule} TPL-44
:kind: required
:tests: templates/declarations/member-template-and-function.cpp templates/declarations/member-template-not-override.cpp

A class can declare a member function and a member function template
that can give a function of the same type. These rules apply:

- A call without a template argument list prefers the plain function
  ({rule}`OVL-27`).
- A call with a template argument list, even `<>`, uses the template.
- A specialization of a member function template never overrides a
  virtual function of a base.
:::

### Variable templates and alias templates

:::{rule} TPL-26
:kind: required
:tests: templates/declarations/variable-template.cpp templates/declarations/static-member-per-specialization.cpp

Each specialization of a variable template is a variable of its own. A
static data member can be a variable template too, declared in the
class with a template head. Its definition can come after the class, at
namespace scope.
:::

:::{rule} TPL-27
:kind: reject
:tests: templates/declarations/variable-template-non-static.cpp templates/declarations/variable-template-non-static-member.cpp

A non-static data member must not be a template.
:::

:::{rule} TPL-28
:kind: required
:tests: templates/declarations/alias-template.cpp templates/declarations/alias-template-deduce.cpp

A specialization of an alias template is the type that the alias gives,
with the arguments in place of the parameters. The specialization is
not a new type.
:::

:::{rule} TPL-46
:kind: required
:tests: templates/declarations/alias-template-lambda.cpp templates/declarations/alias-template-lambda-same.cpp

If the type in an alias template contains a lambda expression, two
specializations with different arguments give two different closure
types. This holds also when the lambda does not use the parameters. Two
uses of one specialization give one closure type ({question}`Q-0200`).
:::

:::{rule} TPL-31
:kind: reject
:tests: templates/declarations/alias-template-self.cpp templates/declarations/alias-template-self-through-class.cpp

The type in an alias template declaration must not use the alias
template. The name of the alias template is declared only at the end of
its declaration ({rule}`NAM-1`). A specialization of an alias template
must not use itself through a member of a class template.
:::

## Examples

```cpp
template <class T, int Size = 4>        // a type parameter, a constant parameter
struct Buffer {
  T items[Size];
  int count() const;
};

template <class T, int Size>             // the head again, without defaults
int Buffer<T, Size>::count() const { return Size; }

template <class T> constexpr T unit = T(1);     // a variable template
template <class T> using Ref = const T&;        // an alias template

Buffer<char> small;                      // Buffer<char, 4>
Ref<int> r = unit<int>;                  // const int&
```

A class template can take a template:

```cpp
template <class T> struct List { };
template <template <class> class Holder>
struct Registry { Holder<int> ids; };    // Holder<int> is List<int>
Registry<List> registry;
```

Parameters with a pack, a placeholder, and a default:

```cpp
template <class... T = int> struct Bad;           // rejected: a pack has no default
template <auto V> struct Hold { };
Hold<'a'> letter;                                 // V is a char
template <bool B = (3 > 4)> struct Flag { };      // the > is in parentheses

template <class... Front, class Back>
int tail(Back) { return sizeof...(Front); }       // Back comes from the call
int n = tail<char, long>(1);                      // Front is char, long
```

A constant parameter whose type is a class template name takes its
type from the argument, through the guides of {rule}`CTAD-1`:

```cpp
template <class T> struct Size { T value; constexpr Size(T v) : value(v) {} };
Size(int) -> Size<long>;
template <Size S> struct Hold { };
Hold<3> three;                     // S has type Size<long>: the user guide
Hold<'a'> letter;                  // S has type Size<char>: the guide from Size(T)
Hold<Size{2.5}> half;              // S has type Size<double>
```

Defaults go from the right. The defaults of earlier declarations count:

```cpp
template <class T = int, class U> struct One;     // rejected on every compiler
template <class T, class U> struct Two;
template <class T = int, class U> struct Two;     // rejected: U has no default; GCC accepts
template <class T, class U = int> struct Three;
template <class T = int, class U> struct Three;   // valid: U has its default
```

A member defined outside the class lists the parameters in order:

```cpp
template <class K, class V> struct Map { int size(); };
template <class A, class B> int Map<A, B>::size() { return 0; }   // names differ
template <class A, class B> int Map<B, A>::size();                // rejected: order
```

A friend definition with a default and the overloads of its name. Only
another declaration of the same template breaks TPL-38:

```cpp
struct Host {
  template <class T = int> friend int pick(Host) { return sizeof(T); }
};
template <class T> int pick(Host, int);           // valid: another function type
template <class T, class U> int pick(Host);       // valid: another head
template <class T> long pick(Host);               // valid: another return type
template <class U> int pick(Host);                // rejected: the same template (Q-0197)
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 114
of 125 tests.

The questions of the cluster of EDG differences (cluster 5) on this page
stay open under decision 0051: {question}`Q-0195`, {question}`Q-0197`,
and {question}`Q-0198`. On 2026-11-03, EDG 7.0 follows the provisional
answer of Q-0195 and Q-0197, and Clang 22 and GCC 16 differ. Clang 22
and GCC 16 follow the provisional answer of Q-0198, and EDG 7.0
differs. The source-reader gap review of the cluster
(`reports/review/2026-11-03-gap-review-question-cluster-5.md`) reads
TPL-35 and TPL-38 as rules that need a diagnostic, and the two rules
follow that reading. Reporting the EDG difference of Q-0198 to the EDG
project waits on the maintainer.

The clean-room writer probed each kind of template, each kind of
parameter, the defaults, and the redeclarations with Clang 22, GCC 16,
and EDG 7.0, and checked the tests on the eight baseline compilers. The
compilers agree on every test on this page except the ones that the
bullets below name. The list of reject rules follows the diagnostics in
the sanitized Clang sources. The gap review of the chapter
(`reports/review/2026-10-31-gap-review-templates-n1.md`) gave the
probes for TPL-32 to TPL-47 and for the additions to the older rules.

- TPL-2: Clang 18 gives a parse error for a template in a block, and the
  other compilers say that a template is not allowed there.
- TPL-4: Clang names the single-entity condition. GCC and EDG reject a
  second declarator with a parse error. Clang and EDG reject a variable
  after a class template definition with a parse error, and GCC names
  the condition.
- TPL-14: Clang and EDG reject a later declaration whose default comes
  before a parameter that no declaration gives a default. GCC checks
  only the defaults on one declaration and accepts it
  ({question}`Q-0296`). Every compiler accepts declarations that give
  the defaults from right to left (`defaults-merge-gap-later.cpp`).
- TPL-35: Clang and GCC accept a function template whose parameter after
  a pack has no default and no deduction. EDG rejects the declaration
  ({question}`Q-0195`). The same split holds for a second type pack
  after the first, and for a constant pack whose type is the type pack
  before it. The rule is a `reject` rule, as TPL-30 is, because the
  head and the function parameters alone break it. No argument list
  takes part, so DEP-32 does not cover it.
- TPL-37: the result depends on the form of the friend:

  | Friend | Clang | GCC | EDG |
  |---|---|---|---|
  | class template, with a default | rejects | accepts | rejects |
  | function template declaration, with a default | rejects | rejects | rejects |
  | function template definition, with a default | accepts | accepts | accepts |

  The split is in `default-friend-class.cpp` ({question}`Q-0196`).
  `default-friend-declaration.cpp` and `default-friend-definition.cpp`
  test the other two rows ({rule}`TPL-38`).
- TPL-40: every compiler rejects `C<>` in a member definition whose head
  gives `C` no inner default, also when the class is never
  instantiated.
- TPL-38: EDG rejects another declaration of a friend function template
  that gives a default, before or after the class. Clang and GCC accept
  it ({question}`Q-0197`). The rule is a `reject` rule, as TPL-16 and
  TPL-37 are, because both declarations are in one translation unit.
- TPL-48: EDG rejects a friend function named after a parameter when the
  class template is used. Clang and GCC accept the friend declaration
  ({question}`Q-0198`). Every compiler accepts the class template
  while no specialization is made.
- TPL-42: GCC rejects a member enumeration defined after the class body
  under `-pedantic-errors`. Clang and EDG accept it ({question}`Q-0199`).
- TPL-46: Clang gives two closure types for two uses of one alias
  template specialization. GCC and EDG give one ({question}`Q-0200`).
- TPL-36: a constant pack whose type is a type pack of the same head
  follows a pack, so in a class template all compilers report TPL-30.
  In a function template, the pack after the type pack breaks TPL-35.
  EDG rejects the head, and Clang and GCC accept it
  (`pack-then-typed-pack-not-deducible.cpp`).
- TPL-45: the plain function has no definition, so the link fails on
  every compiler that links. The template's body is never used for it.

Inputs: clang-diag-sema, clang-diag-parse.
