# Names, scopes, and lookup

A declaration introduces a name into a scope. A use of a name finds a
declaration by lookup. This page covers these kinds of lookup:

- unqualified lookup, which searches the scopes that enclose the use,
- qualified lookup, which searches the namespace, class, or
  enumeration that a qualifier names,
- the effect of using directives on both kinds,
- {term}`argument-dependent lookup` for function calls,
- the name that a class declares for itself, the {term}`injected class name`.

The page also gives hiding, redeclaration, and the scopes that
statements and classes introduce. The pages on statements, namespaces,
and classes cite this page.

Depends on: namespaces and using, class definitions and members,
derived classes, access control, enumerations.

## Principles

:::{principle} NAM-P1
:rules: NAM-1 NAM-3 NAM-8
:exceptions: NAM-6

A scope is a region of the program where names can be declared. Each
scope is one of these:

- a block,
- a function's parameter list,
- a class body,
- a namespace,
- the whole translation unit.

A declaration puts its name into the scope that contains it, and the
name is visible from that point to the end of the scope. Before that
point the name does not exist, and after the scope ends it is gone. A
scope holds one declaration for each name, unless the language says that
two declarations of the name describe one entity, as the language does
for a function declared twice.
:::

:::{principle} NAM-P2
:rules: NAM-2 NAM-4 NAM-12 NAM-9

A use of a name searches the scope that contains the use, then the
scope that contains that scope, and so on outward:

```text
outermost scope                  searched last
  ...
    scope around that one        searched second
      scope that holds the use   searched first
```

The search stops at the first scope that declares the name, whatever
kind of thing the name is. So an inner declaration hides an outer one,
and a variable in an inner scope hides a function of the same name
outside the inner scope. A qualified name skips the search and goes to
the scope that it names ({principle}`NAM-P5`). A program uses a
qualified name to reach a hidden name. A namespace is a scope that a
program can reopen and add to. A member declared in a namespace can be
defined outside the namespace with a qualified name.
:::

:::{principle} NAM-P3
:rules: NAM-5 NAM-27 NAM-28 NAM-29 NAM-30

A function call with an unqualified name also searches places that the
argument types point to. A function that takes a class is usually
declared beside the class, and this search finds the function from any
scope. The search runs in two steps:

```text
argument types ──► associated namespaces and classes ──► functions found
   (NAM-27, NAM-28)                                   (NAM-29, NAM-30)
```

The first step follows each type to the classes and enumerations it is
built from. Each such class or enumeration contributes the namespace
that holds it and the class that holds it. A class also contributes its
bases. A type alias is invisible to this step, and fundamental types
contribute nothing. The second step looks only for functions. The step
finds namespace members and the friends that associated classes
declare. The functions found join those that ordinary lookup finds.
:::

:::{principle} NAM-P4
:rules: NAM-6 NAM-7 NAM-10 NAM-11

Class bodies and labels depart from the plain model. Each part of a
class body sees these members:

| Part of a class body | Members that the part sees |
|---|---|
| Member function bodies and default member initializers | Every member of the class, also those declared later, because the class is complete there |
| The other parts | Only earlier members |

Labels have their own scope, the whole function, and their own name
space, so a label and a variable can share a name.
:::

:::{principle} NAM-P5
:rules: NAM-13 NAM-14 NAM-15 NAM-16 NAM-17 NAM-18 NAM-19 NAM-20 NAM-21 NAM-22

A qualifier fixes the scope of the search. The search then looks in that
scope and in the scopes that the scope takes members from. The search
never goes outward to an enclosing scope.

| Qualifier | Searched | Not searched |
|---|---|---|
| Namespace `N::` | `N`, its inline namespaces, then the namespaces that `N`'s using directives name, if `N` has no match | Enclosing namespaces, other nested namespaces |
| `::` alone | The global namespace, in the same way as `N::` | Named namespaces |
| Class `C::` | `C`, then its bases | Enclosing classes and namespaces |
| Enumeration `E::` | The enumerators of `E` | Everything else |

The name before `::` is looked up as a scope, so a variable with that
name does not get in the way. A declaration that names a member with a
qualified name, such as a definition of `N::f`, enters the scope of `N`
at that name. The parts after the name see the members of `N`.
:::

:::{principle} NAM-P6
:rules: NAM-23 NAM-24 NAM-25 NAM-26

A using directive does not declare anything. The directive makes the
members of a namespace visible to unqualified lookup, as if the members
were declared in the namespace that encloses both the directive and the
named namespace:

```text
namespace outer                  ◄── lib's members act as if declared here
  namespace lib  { value }
  namespace user
    using namespace lib;         ◄── directive
    function body: value         ◄── search: block, user, outer
```

So any declaration between the use and that level hides the members. A
declaration at that level joins them. All the declarations of the name
at one level, from directives or not, are ambiguous only when they
differ in entity and are not all functions.

If the named namespace has its own using directives, the members of the
namespaces that those directives name act at the innermost namespace
around the first directive and the namespace that declares them. An
unnamed namespace acts as a directive in the
namespace that contains the unnamed namespace.
:::

:::{principle} NAM-P7
:rules: NAM-31 NAM-32

Argument-dependent lookup adds to ordinary lookup only when ordinary
lookup finds namespace functions or nothing.

| Ordinary lookup finds | Argument-dependent lookup |
|---|---|
| Nothing | Happens |
| Functions declared at namespace scope | Happens |
| Functions that a block-scope using declaration names | Happens |
| A function declared in a block | Does not happen |
| A variable or object, also of class type | Does not happen |
| A type | Does not happen |
| A class member | Does not happen |

A qualified name or a name in parentheses never uses
argument-dependent lookup.
:::

:::{principle} NAM-P8
:rules: NAM-33 NAM-34 NAM-35 NAM-36

Each class declares its own name as a member. The member is the
injected class name. Inside the class, unqualified lookup finds this
member before any declaration outside the class. A derived class
inherits the member from each base, so the base's name works inside the
derived class without qualification. The inherited name follows the
rules for members: hiding ({rule}`DER-3`), ambiguity ({rule}`DER-4`),
and access ({rule}`ACC-14`).

After `C::`, the name `C` names the constructors in most places. Only
the places that need a class, such as the part before `::` or after
`struct`, take `C::C` as the class.
:::

## Constraints

:::{rule} NAM-1
:kind: required
:tests: foundations/names/visible-from-declaration.cpp foundations/names/visible-in-own-initializer.cpp foundations/names/function-sees-earlier-globals.cpp

A name is visible from its point of declaration to the end of its scope.
The point of declaration comes before the initializer, so a variable is
in scope in its own initializer. Reading the variable there is
undefined, because the object has no value yet (OBJ-2). A function body
sees the names declared before the function in its translation unit.
:::

:::{rule} NAM-8
:kind: reject
:tests: foundations/names/use-before-declaration.cpp foundations/names/scope-ends-with-block.cpp foundations/names/function-not-later-global.cpp foundations/translation/unit-sees-own-declarations-only.cpp expressions/primary/name-undeclared.cpp foundations/names/adl-not-for-fundamental.cpp statements/blocks/block-scope.cpp statements/if/init-scope-ends.cpp statements/loops/for-init-scope.cpp statements/loops/body-own-scope.cpp foundations/names/adl-null-pointer.cpp foundations/names/adl-alias-namespace.cpp foundations/names/adl-lambda-parameter-type.cpp foundations/names/adl-template-value-argument.cpp foundations/names/adl-not-derived.cpp foundations/names/adl-braced-list.cpp foundations/names/adl-function-pointer-fundamental.cpp foundations/names/adl-local-class-member-function.cpp foundations/names/adl-not-static-member.cpp foundations/names/adl-not-through-directive.cpp foundations/names/adl-friend-of-other-class.cpp foundations/names/adl-friend-not-through-conversion.cpp foundations/names/adl-not-later-declaration.cpp foundations/names/adl-not-block-declaration.cpp foundations/names/adl-function-template-parameter-adds-nothing.cpp

A use of a name must find a declaration that is visible at that point
(NAM-1). These uses are rejected:

- a use before the declaration,
- a use after the scope of the declaration ends,
- a use of a name that only another translation unit declares.

In a class body, NAM-6 makes later members visible. A function call
that ordinary lookup and argument-dependent lookup together resolve to
no declaration breaks this rule. If argument-dependent lookup misses the
function for a reason that {rule}`NAM-28` or {rule}`NAM-30` lists, a
test cites that rule as the specific one (decision 0042).
:::

:::{rule} NAM-2
:kind: required
:tests: foundations/names/inner-hides-outer.cpp foundations/names/qualified-reaches-hidden-global.cpp foundations/names/parameter-hides-global.cpp

A declaration in an inner scope hides a declaration of the same name in
an outer scope, for the rest of the inner scope. Hiding does not depend
on the kind of entity, so a variable hides a function. A parameter hides
a name outside the function. The qualified name `::x` finds the global
`x` through any hiding.
:::

:::{rule} NAM-12
:kind: reject
:tests: foundations/names/hiding-ignores-kind.cpp

An unqualified use of a hidden name finds only the inner declaration
(NAM-2). If that declaration does not fit the use, the use is not
valid, even when the hidden declaration fits the use.
:::

:::{rule} NAM-3
:kind: reject
:tests: foundations/names/redeclare-same-scope.cpp foundations/names/redeclare-parameter-in-body.cpp foundations/names/redeclare-different-kind.cpp foundations/names/redeclare-function-ok.cpp

A name must not be declared twice in one scope, except by declarations
that refer to one entity, such as two declarations of one function. The
outermost block of a function is one scope with its parameter list. A
variable and a function with one name in one scope are not valid.
{rule}`STMT-8` names the specific case of a block (decision 0042).
:::

:::{rule} NAM-4
:kind: required
:tests: foundations/names/namespace-reopened.cpp foundations/names/namespace-member-defined-outside.cpp

A namespace can be reopened, and a later part sees the names of an
earlier part. A function declared in a namespace can be defined outside
the namespace with a qualified name.
:::

:::{rule} NAM-9
:kind: reject
:tests: foundations/names/namespace-qualified-use.cpp

From outside a namespace, a name declared in the namespace needs
qualification (NAM-4).
:::

:::{rule} NAM-13
:kind: required
:tests: foundations/names/qualified-using-declaration.cpp foundations/names/qualified-follows-directive.cpp foundations/names/qualified-direct-member-wins.cpp foundations/names/qualified-directive-transitive.cpp foundations/names/qualified-directive-stops.cpp foundations/names/qualified-same-entity-two-paths.cpp foundations/names/qualified-same-entity-using-declarations.cpp foundations/names/qualified-inline-nested.cpp foundations/names/qualified-directive-cycle.cpp foundations/names/qualified-global-directive.cpp foundations/names/qualified-unnamed-namespace-second.cpp foundations/names/qualified-unnamed-namespace-member.cpp

A qualified name `N::x`, where `N` names a namespace, finds `x` in
these steps:

1. The search looks at the declarations and using declarations in `N`
   and in the inline namespaces of `N`, at any depth.
2. If step 1 finds nothing, the search repeats for each namespace that a
   using directive in `N` names. An unnamed namespace in `N` counts as
   a namespace that a using directive in `N` names.
3. Along each path of directives, the search stops at the first
   namespace that declares `x`.

`::x` searches the global namespace in these steps. Two paths that
reach one entity do not make `x` ambiguous.
:::

:::{rule} NAM-14
:kind: reject
:tests: foundations/names/qualified-not-enclosing-namespace.cpp foundations/names/qualified-not-nested-namespace.cpp foundations/names/qualified-before-declaration.cpp foundations/names/qualified-directive-ambiguous.cpp foundations/names/qualified-directive-type-and-variable.cpp foundations/names/qualified-directive-cycle-missing.cpp foundations/names/qualified-block-extern-not-member.cpp foundations/names/qualified-hidden-friend.cpp foundations/names/qualified-global-not-named-namespace.cpp

`N::x` must find `x` by the steps of NAM-13. The steps do not find
these declarations:

- a declaration in a namespace that encloses `N`,
- a declaration in a namespace in `N` that is not inline,
- a declaration after the use,
- a declaration in a block, or a friend declaration in a class
  ({rule}`ACC-7`).

If the paths of NAM-13 end at different entities, `N::x` is ambiguous.
This rule also applies when one entity is a class.
:::

:::{rule} NAM-15
:kind: required
:tests: foundations/names/qualifier-ignores-variable.cpp foundations/names/qualifier-ignores-variable-class.cpp foundations/names/qualifier-decltype.cpp foundations/names/qualifier-enum-alias.cpp

The name before `::` finds only namespaces and types. A variable with
that name does not hide the namespace or class. The qualifier can also
be a type alias or a `decltype` specifier.
:::

:::{rule} NAM-16
:kind: reject
:tests: foundations/names/qualifier-variable-only.cpp foundations/names/qualifier-fundamental-alias.cpp foundations/names/qualifier-incomplete-class.cpp foundations/names/qualifier-opaque-enum.cpp

The qualifier before `::` must denote one of these:

- a namespace,
- a complete class,
- an enumeration whose enumerators are declared.

A fundamental type, also through an alias, is not valid as a qualifier.
:::

:::{rule} NAM-17
:kind: required
:tests: foundations/names/class-qualified-inherited.cpp foundations/names/class-qualified-derived-hides.cpp foundations/names/class-qualified-enumerator.cpp foundations/names/class-qualified-nested.cpp foundations/names/class-qualified-in-namespace.cpp

A qualified name `C::x`, where `C` names a class, finds the member `x`
of `C`. If `C` does not declare `x`, the search continues in the bases
of `C` ({rule}`DER-3`). Members include nested classes, type aliases,
and the enumerators of an unscoped enumeration in `C`.
:::

:::{rule} NAM-18
:kind: reject
:tests: foundations/names/class-qualified-missing.cpp foundations/names/class-qualified-not-outer-scope.cpp foundations/names/class-qualified-not-enclosing-class.cpp foundations/names/class-qualified-ambiguous.cpp foundations/names/class-qualified-scoped-enumerator.cpp

`C::x` must find a member of `C` or of a base (NAM-17). The search does
not look in these places:

- the class that encloses `C`,
- the namespace that contains `C`,
- a scoped enumeration in `C`.

If the search finds `x` in two base subobjects, `C::x` is ambiguous.
This ambiguity is a specific case of {rule}`DER-4` (decision 0042).
:::

:::{rule} NAM-19
:kind: required
:tests: foundations/names/enum-qualified-scoped.cpp foundations/names/enum-qualified-unscoped.cpp foundations/names/enum-qualified-in-namespace.cpp foundations/names/qualifier-enum-alias.cpp

`E::x`, where `E` names an enumeration, finds the enumerator `x` of `E`.
The form works for scoped and unscoped enumerations ({rule}`ENUM-2`,
{rule}`ENUM-3`). An enumerator of an unscoped enumeration in a namespace
`N` is also `N::x`.
:::

:::{rule} NAM-20
:kind: reject
:tests: foundations/names/enum-qualified-missing.cpp

In `E::x`, `x` must be an enumerator of `E` (NAM-19).
:::

:::{rule} NAM-21
:kind: required
:tests: foundations/names/definition-body-sees-namespace.cpp foundations/names/definition-initializer-sees-namespace.cpp foundations/names/definition-parameter-sees-class.cpp foundations/names/definition-trailing-return-sees-class.cpp

A declaration can name a member with a qualified name, such as `N::f`
or `C::f`. After that name, the declaration sees the members of `N` or
`C` without qualification. These parts come after the name:

- the parameters,
- a trailing return type,
- the function body,
- the initializer.
:::

:::{rule} NAM-22
:kind: reject
:tests: foundations/names/definition-return-type-before-name.cpp foundations/names/definition-return-type-namespace.cpp

A return type before the qualified name of NAM-21 does not see the
members of `N` or `C`. A member type there needs qualification.
:::

:::{rule} NAM-23
:kind: required
:tests: foundations/names/directive-local-hides.cpp foundations/names/directive-inner-namespace-hides.cpp foundations/names/directive-namespace-reopened.cpp foundations/names/directive-inline-members.cpp foundations/names/directive-chain-level-ambiguous.cpp foundations/names/directive-chain-between-hides.cpp foundations/names/directive-chain-found.cpp foundations/names/directive-unnamed-namespace-ambiguous.cpp

A using directive makes the members of a namespace visible in these
regions ({rule}`NS-8`):

- in a block, to the end of the block,
- in a namespace, to the end of that definition and in later
  definitions of the same namespace.

For unqualified lookup, the members act as members of the namespace
that the table gives. A declaration in a scope inside that namespace
hides them.

| Members of | Act as members of |
|---|---|
| The namespace that the directive names | The innermost namespace around both the directive and the named namespace |
| A namespace that a directive in the named namespace names, directly or through more directives | The innermost namespace around both the first directive and that namespace |
| An unnamed namespace | The namespace that contains the unnamed namespace |

The first directive is the directive in the region of the use.
:::

:::{rule} NAM-24
:kind: reject
:tests: foundations/names/directive-block-ends.cpp foundations/names/directive-namespace-only.cpp foundations/names/directive-enclosing-level-ambiguous.cpp foundations/names/directive-chain-level-ambiguous.cpp foundations/names/directive-unnamed-namespace-ambiguous.cpp foundations/names/directive-level-two-directives-ambiguous.cpp foundations/names/directive-level-functions-merge.cpp

Outside the regions of NAM-23, the members need qualification. If the
namespace where the members act as members declares the same name for a
different entity, an unqualified use of the name is ambiguous. The use
is valid if NAM-25 allows it, for example if all the declarations are
functions.
:::

:::{rule} NAM-25
:kind: required
:tests: foundations/names/directive-same-entity.cpp foundations/names/directive-overloads-merge.cpp foundations/names/directive-inline-members.cpp foundations/names/directive-level-functions-merge.cpp foundations/names/directive-level-same-entity.cpp

A using directive also makes visible the members of the inline
namespaces in the named namespace. If several directives make one name
visible, or a directive and a declaration in the namespace where the
members act, the use is valid in these cases:

- All the declarations refer to one entity, also through using
  declarations.
- All the declarations are functions. The functions form one set of
  overloads, and overload resolution chooses among them ({rule}`OVL-1`).
:::

:::{rule} NAM-26
:kind: reject
:tests: foundations/names/directive-function-and-variable.cpp foundations/names/directive-type-and-variable.cpp foundations/names/directive-level-two-directives-ambiguous.cpp

If using directives make one name visible for different entities, and
the entities are not all functions, an unqualified use is ambiguous
(NAM-25). This rule also applies to a class and a variable.
{rule}`NAM-24` is the specific rule for a declaration in the namespace
where the members act (decision 0042).
:::

:::{rule} NAM-5
:kind: required
:tests: foundations/names/adl-finds-function.cpp foundations/names/adl-not-for-fundamental.cpp foundations/names/adl-plus-ordinary.cpp

An unqualified function call also looks in the namespaces of its
argument types for the function name. This lookup finds a function
declared in the same namespace as a class argument from any scope.
Arguments of fundamental type add no namespaces. The functions found
this way join the functions that ordinary lookup finds.
:::

:::{rule} NAM-27
:kind: required
:tests: foundations/names/adl-class.cpp foundations/names/adl-union.cpp foundations/names/adl-nested-class-namespace.cpp foundations/names/adl-nested-class-friend.cpp foundations/names/adl-base.cpp foundations/names/adl-indirect-base.cpp foundations/names/adl-base-friend.cpp foundations/names/adl-enum.cpp foundations/names/adl-enum-in-class-friend.cpp foundations/names/adl-enum-in-class-namespace.cpp foundations/names/adl-pointer.cpp foundations/names/adl-reference.cpp foundations/names/adl-array.cpp foundations/names/adl-function-pointer-parameter.cpp foundations/names/adl-function-pointer-return.cpp foundations/names/adl-function-name.cpp foundations/names/adl-member-pointer-class.cpp foundations/names/adl-member-pointer-type.cpp foundations/names/adl-member-function-pointer-parameter.cpp foundations/names/adl-template-argument.cpp foundations/names/adl-template-namespace.cpp foundations/names/adl-template-template-argument.cpp foundations/names/adl-lambda-namespace.cpp foundations/names/adl-lambda-function.cpp foundations/names/adl-lambda-class-member.cpp foundations/names/adl-local-class.cpp foundations/names/adl-local-enum.cpp foundations/names/adl-incomplete-class.cpp foundations/names/adl-second-argument.cpp foundations/names/adl-function-template-in-set.cpp foundations/names/adl-function-template-parameter-adds-nothing.cpp foundations/names/adl-template-id-argument.cpp

Each argument of an unqualified call adds
{term}`associated namespaces <associated namespace>` and
{term}`associated classes <associated class>`. The table gives what
each kind of argument type adds. Argument-dependent lookup searches the associated namespaces and
classes of all the arguments (NAM-29).

| Argument type | Adds |
|---|---|
| A fundamental type | Nothing |
| A class or union `C` | `C`, the class that `C` is a member of, and the innermost namespace that contains `C` |
| A class with bases | Also each direct and indirect base, and the namespace of each base |
| An enumeration `E` | The class that `E` is a member of, and the innermost namespace that contains `E` |
| A pointer to `T`, an array of `T`, a reference to `T` | What `T` adds |
| A function, or a pointer to a function | What the parameter types and the return type add |
| A pointer to a member of `C` of type `T` | What `C` adds and what `T` adds |
| A specialization of a class template ({rule}`TARG-19`) | Also the namespace of the template, what each type argument adds, and the namespace of each template argument that is a template |
| A lambda's closure type | The innermost namespace that contains the lambda, and the class whose body contains the lambda |
| A class or enumeration declared in a function | The innermost namespace that contains the function |
| A name of an overload set | What the type of each function adds. A function template adds what its parameter types and return type add, apart from the types that depend on its template parameters. For a template-id, see {question}`Q-0311` |

A class adds its associated namespaces also when the class is
incomplete.
:::

:::{rule} NAM-28
:kind: reject
:tests: foundations/names/adl-null-pointer.cpp foundations/names/adl-alias-namespace.cpp foundations/names/adl-lambda-parameter-type.cpp foundations/names/adl-template-value-argument.cpp foundations/names/adl-not-derived.cpp foundations/names/adl-braced-list.cpp foundations/names/adl-function-pointer-fundamental.cpp foundations/names/adl-local-class-member-function.cpp

An argument adds only what the table of NAM-27 gives. In particular,
these add nothing:

- `std::nullptr_t`, a braced list, and a function type built only from
  fundamental types,
- the namespace that declares a type alias,
- the parameter types of a lambda,
- a template argument that is a value,
- the classes derived from the argument's class,
- the class of a member function that declares a local class.
:::

:::{rule} NAM-29
:kind: required
:tests: foundations/names/adl-ignores-non-function.cpp foundations/names/adl-using-declaration-in-namespace.cpp foundations/names/adl-inline-namespace-function.cpp foundations/names/adl-class-in-inline-namespace.cpp foundations/names/adl-friend-declared-later.cpp foundations/names/adl-friend-in-namespace.cpp

Argument-dependent lookup finds these functions:

- functions that an associated namespace declares or names in a using
  declaration,
- functions in the inline namespaces of an associated namespace,
- friend functions that an associated class declares ({rule}`ACC-7`).

The lookup skips names that are not functions. A class in an inline
namespace has the enclosing namespace as an associated namespace too.
:::

:::{rule} NAM-30
:kind: reject
:tests: foundations/names/adl-not-static-member.cpp foundations/names/adl-not-through-directive.cpp foundations/names/adl-friend-of-other-class.cpp foundations/names/adl-friend-not-through-conversion.cpp foundations/names/adl-not-later-declaration.cpp foundations/names/adl-not-block-declaration.cpp foundations/names/adl-ignores-base-access.cpp

Argument-dependent lookup does not find these functions:

- member functions of an associated class,
- functions that a using directive in an associated namespace makes
  visible,
- friends of a class that is not associated, also a class that the
  argument converts to,
- functions declared after the call or only in a block.

A private base still adds its namespace, and the call can then fail the
access check.
:::

:::{rule} NAM-31
:kind: required
:tests: foundations/names/adl-with-global-function.cpp foundations/names/adl-with-block-using.cpp foundations/names/adl-plus-ordinary.cpp

Argument-dependent lookup happens when ordinary lookup finds one of
these:

- nothing,
- functions declared at namespace scope,
- functions that a using declaration in a block names.
:::

:::{rule} NAM-32
:kind: reject
:tests: foundations/names/adl-parenthesized-name.cpp foundations/names/adl-suppressed-by-variable.cpp foundations/names/adl-suppressed-by-object.cpp foundations/names/adl-suppressed-by-block-function.cpp foundations/names/adl-suppressed-by-member.cpp foundations/names/adl-qualified-name.cpp foundations/names/adl-suppressed-by-type.cpp

Argument-dependent lookup does not happen in these cases:

- The function name is in parentheses or is qualified.
- Ordinary lookup finds a variable, a type, or a class member.
- Ordinary lookup finds a function declared in a block.

The call then uses only what ordinary lookup finds.
:::

:::{rule} NAM-33
:kind: required
:tests: foundations/names/injected-own-name.cpp foundations/names/injected-repeated-qualifier.cpp foundations/names/injected-elaborated.cpp foundations/names/injected-base-clause.cpp

The injected class name of `C` is a member of `C` that names `C`.
Inside `C`, an unqualified `C` finds this member before any declaration
outside `C`. `C::C` names the class before `::`, after a class key, and
in a list of bases.
:::

:::{rule} NAM-34
:kind: reject
:tests: foundations/names/injected-constructor-declaration.cpp foundations/names/injected-constructor-pointer.cpp foundations/names/injected-constructor-alias.cpp foundations/names/injected-constructor-template-argument.cpp foundations/names/injected-constructor-call.cpp foundations/names/injected-constructor-sizeof.cpp classes/constructors/ctor-address.cpp

Outside the places that NAM-33 lists, `C::C` names the constructors of
`C`. A program must not use `C::C` as a type there, or call `C::C()`.
{rule}`CTOR-1` is the specific rule for taking the address with
`&C::C` (decision 0042).
:::

:::{rule} NAM-35
:kind: required
:tests: foundations/names/injected-base-name.cpp foundations/names/injected-base-name-qualified.cpp foundations/names/injected-two-paths.cpp foundations/names/injected-two-paths-virtual.cpp foundations/names/injected-base-hides-namespace.cpp foundations/names/injected-member-hides-base-name.cpp

A derived class inherits the injected class name of each base. Inside
the derived class, the base name finds the base, also when the base is
in another namespace. `D::B` names the base `B` from outside, and two
paths to one base class are not ambiguous ({rule}`DER-4`).
:::

:::{rule} NAM-36
:kind: reject
:tests: foundations/names/injected-two-bases-ambiguous.cpp foundations/names/injected-base-name-not-outside.cpp foundations/names/injected-private-base.cpp

These uses of an inherited base name are not valid:

- a use where two different bases have that name ({rule}`DER-4`),
- an unqualified use outside the derived class,
- a use that reaches a private base of another class
  ({rule}`ACC-14`).

The first case is a specific case of {rule}`DER-4`, and
{rule}`ACC-14` is the specific rule for the third case (decision 0042).
:::

:::{rule} NAM-6
:kind: required
:tests: foundations/names/class-members-any-order.cpp

A member function body sees all members of its class, including the
members declared after the member function (CLS-2). A member declaration
sees only the members declared before that declaration.
:::

:::{rule} NAM-10
:kind: reject
:tests: foundations/names/class-member-type-needs-order.cpp foundations/names/class-scope-outside-needs-qualification.cpp

A member type must be declared before a member that uses it (NAM-6).
Outside the class, a static member needs the class name as
qualification.
:::

:::{rule} NAM-7
:kind: required
:tests: foundations/names/label-scope-function.cpp foundations/names/label-and-variable-same-name.cpp

A label is visible in the whole function that contains it, before and
after its definition. Labels do not share a name space with other
names, so a label and a variable can have the same name.
:::

:::{rule} NAM-11
:kind: reject
:tests: foundations/names/label-duplicate.cpp

One function must not define a label twice (NAM-7).
:::

## Examples

```cpp
int count = 0;                  // global

int bump(int count) {           // parameter hides the global
  return ::count + count;       // ::count reaches the global
}

namespace geo {
  struct Point { int x, y; };
  int sum(Point p);             // found by a call sum(p) from anywhere
}

struct Timer {
  int read() const { return ticks; }   // sees ticks, declared later
  int ticks = 0;
};
```

Qualified lookup follows using directives only when the namespace
itself has no match:

```cpp
namespace v1 { long limit = 1; }
namespace api {
  int limit = 2;
  using namespace v1;
}
// api::limit is the int; v1::limit is not considered
```

A using directive in a block puts the members at the namespace level,
so a local variable hides them:

```cpp
namespace units { double scale = 2.0; }
int pick() {
  int scale = 3;
  {
    using namespace units;
    return scale;               // 3, the local variable
  }
}
```

Argument-dependent lookup reaches through pointers, bases, and friends:

```cpp
namespace shapes {
  struct Shape { friend int area(const Shape&); };
}
namespace app {
  struct Circle : shapes::Shape { };
}
app::Circle c;
area(c);                        // finds the friend through the base
(area)(c);                      // rejected: no argument-dependent lookup
```

The injected class name works through a base in another namespace:

```cpp
namespace io { struct Stream { }; }
struct File : io::Stream {
  Stream* base_part();          // Stream is io::Stream
};
File::Stream* p = nullptr;      // also io::Stream
File::File* q = nullptr;        // rejected: names the constructor
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 180
of 181 tests.

The clean-room writer probed qualified lookup, using directives,
argument-dependent lookup, and the injected class name with Clang 22,
GCC 16, and EDG 7.0, and checked the tests on Clang 18, GCC 14, and EDG
7.0 for ILP32. The compilers agree on every probe except the one
that Q-0311 records. The
argument-dependent lookup grid of NAM-27 and NAM-28 has one test for
each row, and a rejected test for each kind of type that adds nothing.

- NAM-27: for an argument that is a template-id naming a function
  template, GCC adds the namespaces of the type arguments, and Clang
  adds nothing for them ({question}`Q-0311`).
- NAM-23: for a namespace that a chain of using directives reaches, all
  compilers put the members at the level of the first directive in the
  chain. A declaration at that level makes a use ambiguous, and a
  declaration between the use and that level hides the members.
- NAM-13, NAM-23: all compilers treat an unnamed namespace as a using
  directive in the namespace that contains it. `N::x` prefers a
  declaration of `x` in `N` to one in an unnamed namespace in `N`.
- NAM-8, NAM-18, NAM-24, NAM-26, NAM-34, NAM-36: each of these rules
  overlaps another `reject` rule. The rule text says which of the two
  is the specific one, and the tests cite that rule (decision 0042).

Inputs: clang-sema-decl, clang-sema-declcxx, clang-diag-sema, edg-decls,
clang-sema-lookup, clang-sema-cxxscopespec, edg-lookup, edg-scope-stk.
