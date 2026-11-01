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
template arguments, dependent names, instantiation, and constraints are
not specified yet *(templates referenced)*.

Depends on: declarations, class definitions and members, function
definitions, constant evaluation, name lookup.

## Principles

:::{principle} TPL-P1
:rules: TPL-1 TPL-22 TPL-23 TPL-24 TPL-26 TPL-28
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

A member of a class template is not a template of its own. The member
is part of each specialization of the class, so a definition outside
the class repeats the class's head (TPL-23).
:::

:::{principle} TPL-P2
:rules: TPL-5 TPL-6 TPL-8 TPL-9 TPL-11 TPL-12 TPL-30
:exceptions: TPL-7 TPL-10

Each template parameter has a kind, and the kind decides what an
argument for it must be:

| Kind | Declared as | Takes | Inside the template, the name is |
|---|---|---|---|
| {term}`type parameter` | `class T`, `typename T` | a type | a type |
| {term}`constant parameter` | `int N`, `auto V`, `Point P` | a constant of the declared type | a value: a prvalue, or an lvalue for a reference or class type |
| {term}`template template parameter` | `template <class> class C` | a class template or alias template | a template |

A constant parameter holds a value that is known at compile time.
Two arguments must be easy to compare, so that the implementation can
tell whether two template-ids name one specialization. The type of a
constant parameter is therefore limited to a {term}`structural type`.
A {term}`template parameter pack`, written with `...`, takes any number
of arguments of its kind.
:::

:::{principle} TPL-P3
:rules: TPL-13 TPL-15
:exceptions: TPL-14 TPL-16 TPL-17

A parameter can have a {term}`default template argument`. A use that
leaves out the argument gets the default. The defaults of all the
declarations of one template add up, but each parameter gets its default
in one declaration only. In a class, variable, or alias template, the
arguments are matched by position, so only a trailing parameter can be
left out. A function template can get its arguments by deduction as
well *(templates referenced)*, so its defaults can come in any order.
:::

:::{principle} TPL-P4
:rules: TPL-19 TPL-20 TPL-21
:exceptions: TPL-2 TPL-3 TPL-4 TPL-18 TPL-25 TPL-27

A template parameter is a name in its own scope. The scope starts at the
parameter and ends at the end of the template's declaration. The name
hides the same name outside, and no declaration inside the template can
reuse the name. Several declarations of one template must agree on the
parameters: their number, their kinds, and the types of the constant
parameters. The names can differ.

Templates are declared only at namespace scope and in classes that are
not local:

| Place | A template there |
|---|---|
| namespace scope | valid |
| class body, of a class at namespace or class scope | valid, as a member template |
| block scope | not valid (TPL-2) |
| body of a local class | not valid (TPL-2) |
| inside `extern "C"` | not valid (TPL-3) |
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   template_decl: `template_head` `declaration`
   template_head: "template" "<" `template_params` ">"
   template_params: `template_param` {"," `template_param`}
   template_param: `type_param` | `constant_param` | `template_template_param`
   type_param: `type_key` ["..."] [`identifier`]
             : | `type_key` [`identifier`] "=" `type_id`
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
:tests: templates/declarations/kinds.cpp

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
:tests: templates/declarations/block-scope.cpp templates/declarations/local-class-member.cpp

A template must not be declared at block scope. A template must not be
declared as a member of a local class.
:::

:::{rule} TPL-3
:kind: reject
:tests: templates/declarations/c-linkage.cpp

A template must not have C language linkage ({rule}`LSP-1`).
:::

:::{rule} TPL-4
:kind: reject
:tests: templates/declarations/typedef-template.cpp templates/declarations/two-declarators.cpp templates/declarations/unnamed-class.cpp

The declaration after a template head must declare exactly one named
entity. These declarations are not valid after a template head:

- a `typedef` declaration,
- a declaration with two or more declarators,
- a class without a name.
:::

:::{rule} TPL-22
:kind: required
:tests: templates/declarations/function-template-overload.cpp

Function templates and plain functions can share a name in one scope.
Each function template with the name adds a candidate to overload
resolution ({rule}`OVL-1`).
:::

:::{rule} TPL-29
:kind: reject
:tests: templates/declarations/main-template.cpp

The function `main` must not be a template.
:::

### Template parameters

:::{rule} TPL-5
:kind: required
:tests: templates/declarations/type-parameter-keywords.cpp

A {term}`type parameter` is declared with `class` or `typename`. The
two keywords mean the same thing. The name of any template parameter
can be left out.
:::

:::{rule} TPL-6
:kind: required
:tests: templates/declarations/constant-parameter-types.cpp

A {term}`constant parameter` is declared like a function parameter. Its
type must be a {term}`structural type`, which is one of these:

- an integral or enumeration type,
- a pointer to an object or function type,
- an lvalue reference to an object or function type,
- a pointer to member,
- the type of `nullptr`,
- a floating-point type,
- a literal class type whose bases and non-static data members are all
  public, not `mutable`, and of structural types or arrays of them.
:::

:::{rule} TPL-7
:kind: reject
:tests: templates/declarations/constant-parameter-rvalue-reference.cpp templates/declarations/constant-parameter-void.cpp templates/declarations/constant-parameter-private-member.cpp templates/declarations/constant-parameter-mutable-member.cpp templates/declarations/constant-parameter-not-literal.cpp

A constant parameter must not have a type that TPL-6 leaves out. Such
types include:

- an rvalue reference type,
- `void`,
- a class with a data member that is private, protected, or `mutable`,
- a class that is not a {term}`literal type`.
:::

:::{rule} TPL-8
:kind: required
:tests: templates/declarations/constant-parameter-adjusted.cpp

If a constant parameter is declared with an array type or a function
type, its type is the matching pointer type, as for a function
parameter ({rule}`DCL-8`).
:::

:::{rule} TPL-9
:kind: required
:tests: templates/declarations/constant-parameter-category.cpp templates/declarations/constant-parameter-class-address.cpp

In the template, the name of a constant parameter is an expression with
this value category:

| Type of the parameter | The name is |
|---|---|
| an lvalue reference | an lvalue that refers to the argument's object or function |
| a class type `C` | an lvalue of type `const C` |
| any other type | a prvalue with the argument's value |

For a class type, all uses with equal arguments refer to one object
with static storage duration.
:::

:::{rule} TPL-10
:kind: reject
:tests: templates/declarations/constant-parameter-assign.cpp templates/declarations/constant-parameter-address.cpp

A program must not assign to a constant parameter of scalar type or
take its address, because the name is a prvalue (TPL-9).
:::

:::{rule} TPL-11
:kind: required
:tests: templates/declarations/constant-parameter-auto.cpp

The type of a constant parameter can be a {term}`placeholder type`.
The parameter then gets its type from its argument, as a variable
declared with the placeholder gets its type from its initializer
({rule}`AUTO-1`).
:::

:::{rule} TPL-12
:kind: required
:tests: templates/declarations/template-template-parameter.cpp

A {term}`template template parameter` has its own template head,
followed by `class` or `typename` and the name. In the template, the
name is a template that takes arguments for the parameters of that
head.
:::

:::{rule} TPL-30
:kind: reject
:tests: templates/declarations/pack-last.cpp templates/declarations/pack-declared.cpp

A {term}`template parameter pack` is a parameter declared with `...`.
A pack must be the last parameter, unless the template is a function
template. A function template can have parameters after a pack.
:::

### Default template arguments

:::{rule} TPL-13
:kind: required
:tests: templates/declarations/defaults.cpp templates/declarations/defaults-function-any-order.cpp

A parameter of any kind can have a {term}`default template argument`
after `=`. A default can name the parameters before it.
:::

:::{rule} TPL-14
:kind: reject
:tests: templates/declarations/default-not-last.cpp templates/declarations/default-not-last-variable.cpp templates/declarations/default-not-last-alias.cpp

Unless the template is a function template, each parameter after one
with a default must have a default too. A pack needs no default.
:::

:::{rule} TPL-15
:kind: required
:tests: templates/declarations/defaults-merge.cpp

The defaults of the declarations of one template add up. A later
declaration can give a default for a parameter that an earlier
declaration left without one.
:::

:::{rule} TPL-16
:kind: reject
:tests: templates/declarations/default-repeated.cpp

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

### Scope and redeclaration

:::{rule} TPL-19
:kind: required
:tests: templates/declarations/parameter-scope.cpp templates/declarations/redeclare-names-differ.cpp

The scope of a template parameter starts at its declaration and ends at
the end of the template's declaration. The parameter hides a name
declared outside the template ({rule}`NAM-2`). Two declarations of one
template can use different names for a parameter.
:::

:::{rule} TPL-18
:kind: reject
:tests: templates/declarations/parameter-twice.cpp templates/declarations/parameter-member-name.cpp templates/declarations/parameter-local-name.cpp

A name must not be declared again in the scope of a template parameter
with that name. These declarations are not valid:

- a second parameter with the name in the same head,
- a member with the name in a class template,
- a variable with the name in the body of a function template.
:::

:::{rule} TPL-20
:kind: reject
:tests: templates/declarations/redeclare-count.cpp templates/declarations/redeclare-kind.cpp templates/declarations/redeclare-type.cpp templates/declarations/member-template-one-head.cpp

All declarations of one template must have equivalent template heads.
Equivalent heads have the same number of parameters, and each pair of
parameters at one position has:

- the same kind,
- for constant parameters, the same type.
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
:tests: templates/declarations/member-outside.cpp

The functions of a class template, and its `static` data members, can
be defined after the class body. Such a definition starts with the class's
template head and names the member with the class's template-id, such
as `Box<T>::get`.
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
:tests: templates/declarations/member-template-virtual.cpp

A member function template must not be `virtual`.
:::

### Variable templates and alias templates

:::{rule} TPL-26
:kind: required
:tests: templates/declarations/variable-template.cpp

Each specialization of a variable template is a variable of its own. A
static data member can be a variable template too, declared in the
class with a template head.
:::

:::{rule} TPL-27
:kind: reject
:tests: templates/declarations/variable-template-non-static.cpp

A non-static data member must not be a template.
:::

:::{rule} TPL-28
:kind: required
:tests: templates/declarations/alias-template.cpp

A specialization of an alias template is the type that the alias gives,
with the arguments in place of the parameters. The specialization is
not a new type.
:::

:::{rule} TPL-31
:kind: reject
:tests: templates/declarations/alias-template-self.cpp

The type in an alias template declaration must not use the alias
template. The name of the alias template is declared only at the end of
its declaration ({rule}`NAM-1`).
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

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on all
52 tests.

The clean-room writer probed each kind of template, each kind of
parameter, the defaults, and the redeclarations with Clang 22, GCC 16,
and EDG 7.0, and checked the tests on the eight baseline compilers. The
compilers agree on every test on this page. The list of reject rules
follows the diagnostics in the sanitized Clang sources.

- TPL-2: Clang 18 gives a parse error for a template in a block, and the
  other compilers say that a template is not allowed there.
- TPL-4: Clang names the single-entity condition. GCC and EDG reject a
  second declarator with a parse error.

Inputs: clang-diag-sema, clang-diag-parse.
