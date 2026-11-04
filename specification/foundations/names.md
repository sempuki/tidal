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
- a template's parameter list,
- a class body,
- the body of a scoped enumeration,
- a namespace,
- the whole {term}`translation unit`.

A declaration puts its name into the scope that contains it, and the
name is visible from that point to the end of the scope. Before that
point the name does not exist, and after the scope ends it is gone. A
scope holds one declaration for each name, unless the language says that
two declarations of the name describe one entity, as the language does
for a function declared twice. A class can also share its name with one
other entity ({principle}`NAM-P10`).
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
outside the inner scope. A few places need a type or a namespace and
nothing else: the name before `::`, the name after a class key, and a
name in a list of bases. There the search passes over the other kinds
of declaration and goes on outward. A qualified name skips the search and goes to
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
| Member function bodies, default arguments, exception specifications, and default member initializers | Every member of the class, also those declared later, because the class is complete there ({rule}`CLS-2`) |
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
| Namespace `N::` | `N`, its inline namespaces, then the namespaces that the using directives of `N` and of its inline namespaces name, and the unnamed namespaces in them, if `N` has no match | Enclosing namespaces, other nested namespaces |
| `::` alone | The global namespace, in the same way as `N::` | Named namespaces |
| Class `C::` | `C`, then its bases | Enclosing classes and namespaces |
| Enumeration `E::` | The enumerators of `E` | Everything else |
| Member access `E.m`, `E->m` | The class of `E`, then its bases ({rule}`NAM-37`) | Enclosing classes and namespaces |

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

:::{principle} NAM-P9
:rules: NAM-37 NAM-38

Several constructs call a function that can be a member or a free
function. Each construct has its own order:

| Construct | Members | Free functions | Rules |
|---|---|---|---|
| `x.f(a)`, `p->f(a)` | found by member lookup in the class of `x` | never | NAM-37, {rule}`OVL-4` |
| `f(a)` in a member function | found by ordinary lookup, which stops at the class | found by ordinary lookup outside the class; argument-dependent lookup only if no member is found | {rule}`NAM-32`, {rule}`OVL-2` |
| `x @ y`, `@x` | the members of the left operand's class | lookup with class scopes passed over, and argument-dependent lookup | NAM-38, {rule}`OVL-7` |
| range-based `for`: `begin`, `end` | used if the class has both names as members, of any kind | otherwise argument-dependent lookup only | {rule}`RFOR-2` |
| structured binding: `get` | used if member lookup finds a member {term}`function template` | otherwise argument-dependent lookup only | {rule}`SB-5` |

In the operator row, members and free functions are candidates
together. In the other rows, one source wins and the other is not
searched.
:::

:::{principle} NAM-P10
:rules: NAM-39 NAM-40 NAM-41 NAM-42

A class or an enumeration can share its name with another entity in the
same scope. C keeps the names of structures apart from other names, and
the exception keeps such C code valid, such as a `struct stat` beside a
function `stat`. The table gives the entities that can share the name of
a class or enumeration `S`:

| Other entity in the scope | Valid | `S` alone, `N::S`, and `C::S` find |
|---|---|---|
| A variable or a data member | Yes | The variable or data member |
| Functions or function templates, also members | Yes | The functions |
| An enumerator | Yes | The enumerator |
| A type alias for another type, a class template, an alias template, or a variable template | No | |

The class or enumeration stays reachable where the grammar needs a type
name: after `struct` or `enum`, before `::`, and in a list of bases.
:::

## Constraints

:::{rule} NAM-1
:kind: required
:tests: foundations/names/visible-from-declaration.cpp foundations/names/visible-in-own-initializer.cpp foundations/names/function-sees-earlier-globals.cpp foundations/names/enumerator-point-of-declaration.cpp

A name is visible from its point of declaration to the end of its scope.
The point of declaration of a variable comes after its declarator and
before the initializer, so a variable is in scope in its own
initializer. Reading the variable there is undefined, because the
object has no value yet (OBJ-2). The point of declaration of an
enumerator comes after its initializer, so the initializer sees an
outer entity of the same name. A function body sees the names declared
before the function in its translation unit.
:::

:::{rule} NAM-8
:kind: reject
:tests: foundations/names/use-before-declaration.cpp foundations/names/scope-ends-with-block.cpp foundations/names/function-not-later-global.cpp foundations/translation/unit-sees-own-declarations-only.cpp expressions/primary/name-undeclared.cpp foundations/names/adl-not-for-fundamental.cpp statements/blocks/block-scope.cpp statements/if/init-scope-ends.cpp statements/loops/for-init-scope.cpp statements/loops/body-own-scope.cpp foundations/names/adl-null-pointer.cpp foundations/names/adl-alias-namespace.cpp foundations/names/adl-lambda-parameter-type.cpp foundations/names/adl-template-value-argument.cpp foundations/names/adl-not-derived.cpp foundations/names/adl-braced-list.cpp foundations/names/adl-function-pointer-fundamental.cpp foundations/names/adl-local-class-member-function.cpp foundations/names/adl-not-static-member.cpp foundations/names/adl-not-through-directive.cpp foundations/names/adl-friend-of-other-class.cpp foundations/names/adl-friend-not-through-conversion.cpp foundations/names/adl-not-later-declaration.cpp foundations/names/adl-not-block-declaration.cpp foundations/names/adl-function-template-parameter-adds-nothing.cpp declarations/functions/definitions/parameter-name-from-definition.cpp

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
:tests: foundations/names/hiding-ignores-kind.cpp foundations/names/hiding-variable-hides-type.cpp foundations/names/hiding-class-key-skips-variable.cpp foundations/names/hiding-base-list-skips-variable.cpp

An unqualified use of a hidden name finds only the inner declaration
(NAM-2). If that declaration does not fit the use, the use is not
valid, even when the hidden declaration fits the use.

The rule has exceptions where the lookup considers only some kinds of
declaration. After a class key and in a list of bases, the lookup
considers only types. Before `::`, the lookup considers only types and
namespaces ({rule}`NAM-15`). In these places a declaration of another
kind does not stop the search, and the search finds the outer
declaration.
:::

:::{rule} NAM-3
:kind: reject
:tests: foundations/names/redeclare-same-scope.cpp foundations/names/redeclare-parameter-in-body.cpp foundations/names/redeclare-different-kind.cpp foundations/names/redeclare-function-ok.cpp foundations/names/shared-name-two-variables.cpp

A name must not be declared twice in one scope, except by declarations
that refer to one entity, such as two declarations of one function
({rule}`FN-15`). The
outermost block of a function is one scope with its parameter list. A
variable and a function with one name in one scope are not valid.
{rule}`NAM-39` allows a class or an enumeration to share its name.
{rule}`STMT-8` names the specific case of a block, and {rule}`NAM-42`
the case of a template (decision 0042). A declaration in the outermost
block of a function with the name of a parameter falls under this rule,
because the parameter is not declared in the block.
:::

:::{rule} NAM-39
:kind: required
:tests: foundations/names/shared-name-anonymous-union-member.cpp foundations/names/shared-name-class-variable.cpp foundations/names/shared-name-variable-then-class.cpp foundations/names/shared-name-class-function.cpp foundations/names/shared-name-enumeration.cpp foundations/names/shared-name-enumerator-class.cpp foundations/names/shared-name-class-members.cpp

One scope can declare a class or an enumeration and one of these with
the same name:

- a variable,
- functions and function templates,
- an enumerator.

The declarations can come in any order. In a class, the variable or the
functions can be members, also a member of an anonymous union in the
class ({question}`Q-0362`).
{rule}`STMT-9` is this rule for a block.
:::

:::{rule} NAM-40
:kind: required
:tests: foundations/names/shared-name-variable-found.cpp foundations/names/shared-name-function-call.cpp foundations/names/shared-name-qualified.cpp foundations/names/shared-name-base-list.cpp foundations/names/shared-name-before-variable.cpp foundations/names/shared-name-member-function-body.cpp foundations/names/shared-name-using-declaration.cpp

If a scope declares a class or enumeration `S` and another entity named
`S` (NAM-39), lookup of `S` finds the other entity from its declaration
on. The rule holds for unqualified names, qualified names, and names
that a using declaration brings. Lookup finds the class or enumeration
only in these places:

- after `struct`, `class`, `union`, or `enum` ({rule}`TYN-4`),
- before `::` ({rule}`NAM-15`),
- in a list of bases.
:::

:::{rule} NAM-41
:kind: reject
:tests: foundations/names/shared-name-conversion-function-type.cpp foundations/names/shared-name-class-alone.cpp foundations/names/shared-name-class-alone-variable-first.cpp foundations/names/shared-name-qualified-not-class.cpp foundations/names/shared-name-member-not-class.cpp foundations/names/shared-name-in-class-not-class.cpp foundations/names/shared-name-enumeration-alone.cpp foundations/names/shared-name-inner-block.cpp

Where lookup finds the other entity (NAM-40), the name must not name the
class or enumeration. The rule holds for `S`, `N::S`, and `C::S`.
{rule}`STMT-10` is the specific rule for a block that declares both
(decision 0042).
:::

:::{rule} NAM-42
:kind: reject
:tests: foundations/names/shared-name-class-template-variable.cpp foundations/names/shared-name-variable-class-template.cpp foundations/names/shared-name-class-template-function.cpp foundations/names/shared-name-alias-template-variable.cpp foundations/names/shared-name-class-variable-template.cpp

NAM-39 does not apply to templates. A class template, an alias
template, or a variable template must not have the name of another
entity in its scope. {rule}`TYN-2` gives the case of a type alias.
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
:tests: foundations/names/namespace-qualified-use.cpp foundations/names/namespace-qualified-call.cpp

From outside a namespace, a name declared in the namespace needs
qualification (NAM-4). {rule}`NS-1` lists the other forms that name the
member, and NAM-8 states the general rule.
:::

:::{rule} NAM-38
:kind: required
:tests: foundations/names/operator-lookup-skips-class.cpp foundations/names/operator-lookup-block-hides.cpp foundations/names/operator-lookup-namespace-hides.cpp foundations/names/operator-lookup-block-declaration-keeps-adl.cpp

The lookup of non-member {term}`operator functions <operator function>` for an operator expression
({rule}`OVL-7`) searches outward from the use, as NAM-2 gives, and
passes over each class scope. A block or a namespace that declares the
operator function ends the search and hides the declarations outside
it. {term}`Argument-dependent lookup <argument-dependent lookup>` then
adds its functions (NAM-29). A block declaration of an operator function
does not stop argument-dependent lookup, as a block declaration of a
function does for a call ({rule}`NAM-32`,
{question}`Q-0364`).
:::

:::{rule} NAM-13
:kind: required
:tests: foundations/names/qualified-directive-in-inline-namespace.cpp foundations/names/qualified-directive-functions-overload.cpp foundations/names/qualified-using-declaration.cpp foundations/names/qualified-follows-directive.cpp foundations/names/qualified-direct-member-wins.cpp foundations/names/qualified-directive-transitive.cpp foundations/names/qualified-directive-stops.cpp foundations/names/qualified-same-entity-two-paths.cpp foundations/names/qualified-same-entity-using-declarations.cpp foundations/names/qualified-inline-nested.cpp foundations/names/qualified-directive-cycle.cpp foundations/names/qualified-global-directive.cpp foundations/names/qualified-unnamed-namespace-second.cpp foundations/names/qualified-unnamed-namespace-member.cpp

A qualified name `N::x`, where `N` names a namespace, finds `x` in
these steps:

1. The search looks at the declarations and using declarations in `N`
   and in the inline namespaces of `N`, at any depth.
2. If step 1 finds nothing, the search repeats for each namespace that a
   using directive names, if the directive is in `N` or in an inline
   namespace of `N` and comes before the use. An unnamed namespace in
   `N` counts as a namespace that a using directive in `N` names.
3. Along each path of directives, the search stops at the first
   namespace that declares `x`.

`::x` searches the global namespace in these steps. Two paths that
reach one entity do not make `x` ambiguous.
:::

:::{rule} NAM-14
:kind: reject
:tests: foundations/names/qualified-directive-functions-overload.cpp foundations/names/qualified-not-enclosing-namespace.cpp foundations/names/qualified-not-nested-namespace.cpp foundations/names/qualified-before-declaration.cpp foundations/names/qualified-directive-ambiguous.cpp foundations/names/qualified-directive-type-and-variable.cpp foundations/names/qualified-directive-cycle-missing.cpp foundations/names/qualified-block-extern-not-member.cpp foundations/names/qualified-hidden-friend.cpp foundations/names/qualified-global-not-named-namespace.cpp

`N::x` must find `x` by the steps of NAM-13. The steps do not find
these declarations:

- a declaration in a namespace that encloses `N`,
- a declaration in a namespace in `N` that is not inline,
- a declaration after the use,
- a declaration in a block, or a friend declaration in a class
  ({rule}`ACC-7`).

If the paths of NAM-13 end at different entities, and the entities are
not all functions or function templates, `N::x` is ambiguous. This rule
also applies when one entity is a class. Functions found along
different paths form one overload set, and overload resolution chooses
among them ({rule}`NAM-25`).
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
:tests: foundations/names/qualifier-variable-only.cpp foundations/names/qualifier-fundamental-alias.cpp foundations/names/qualifier-incomplete-class.cpp foundations/names/qualifier-opaque-enum.cpp foundations/names/qualifier-class-being-defined.cpp foundations/names/qualifier-fundamental-alias-destructor.cpp

The qualifier before `::` must denote one of these:

- a namespace,
- a class that is complete or whose definition contains the use,
- an enumeration whose enumerators are declared.

Inside the definition of a class `C`, `C::x` finds the members declared
before the use. A class that is only declared has no members to find,
so a use such as `C::x` is not valid.

A fundamental type, also through an alias, is not valid as a qualifier.
The one exception is a destructor name: `p->T::~T()` can name a scalar
type `T` through an alias ({rule}`DTOR-6`).
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

If the search finds `x` in two base {term}`subobjects <subobject>`, `C::x` is ambiguous.
This ambiguity is a specific case of {rule}`DER-4` (decision 0042).
:::

:::{rule} NAM-37
:kind: required
:tests: foundations/names/member-access-lookup-kinds.cpp foundations/names/member-access-no-enclosing-scope.cpp foundations/names/member-access-no-free-function.cpp foundations/names/member-access-type-member.cpp foundations/names/member-access-qualifier-outside-class.cpp

In a member access `E.m` or `E->m`, lookup of `m` searches the class of
`E` and then its bases, as for `C::m` ({rule}`NAM-17`). The search
covers only the class and its bases. It finds every kind of member:

- data members and member functions, static or not,
- nested classes and type aliases,
- the enumerators of an unscoped enumeration in the class,
- the injected class name.

{rule}`PFX-11` rejects a member access whose lookup finds nothing or
finds a type.

A qualifier between the operator and the member name, as in `E.Q::m`,
is first looked up in the class of `E`. If the class has no member
`Q`, `Q` is looked up from the point of the use, as an unqualified
name.
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
:tests: foundations/names/enum-qualified-missing.cpp foundations/names/enum-qualified-missing-unscoped.cpp

In `E::x`, `x` must be an enumerator of `E` (NAM-19).
:::

:::{rule} NAM-21
:kind: required
:tests: foundations/names/definition-body-sees-namespace.cpp foundations/names/definition-initializer-sees-namespace.cpp foundations/names/definition-parameter-sees-class.cpp foundations/names/definition-trailing-return-sees-class.cpp foundations/names/definition-bound-default-noexcept-see-scope.cpp foundations/names/definition-template-member-alias.cpp foundations/names/definition-class-body-sees-namespace.cpp foundations/language-support/tuple-protocol-page-example.cpp

A declaration can name a member with a qualified name, such as `N::f`
or `C::f`. After that name, the declaration sees the members of `N` or
`C` without qualification. These parts come after the name:

- the parameters and their default arguments,
- an array bound,
- an exception specification,
- a trailing return type,
- the function body,
- the initializer,
- the body of a class that the declaration defines
  ({question}`Q-0377`).
:::

:::{rule} NAM-22
:kind: reject
:tests: foundations/names/definition-return-type-before-name.cpp foundations/names/definition-return-type-namespace.cpp

A return type before the qualified name of NAM-21 does not see the
members of `N` or `C`. A member type there needs qualification.
:::

:::{rule} NAM-23
:kind: required
:tests: foundations/names/directive-out-of-namespace-member-definition.cpp foundations/names/directive-local-hides.cpp foundations/names/directive-inner-namespace-hides.cpp foundations/names/directive-namespace-reopened.cpp foundations/names/directive-inline-members.cpp foundations/names/directive-chain-level-ambiguous.cpp foundations/names/directive-chain-between-hides.cpp foundations/names/directive-chain-found.cpp foundations/names/directive-unnamed-namespace-ambiguous.cpp

A using directive makes the members of a namespace visible in these
regions ({rule}`NS-8`):

- in a block, to the end of the block,
- in a namespace, to the end of that definition and in later
  definitions of the same namespace,
- in a namespace, also in a later definition of a member of that
  namespace outside the namespace, after the qualified name of the
  member ({rule}`NAM-21`).

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
:tests: foundations/names/adl-member-template-template-argument.cpp foundations/names/adl-address-of-overload-set.cpp foundations/names/adl-class.cpp foundations/names/adl-union.cpp foundations/names/adl-nested-class-namespace.cpp foundations/names/adl-nested-class-friend.cpp foundations/names/adl-base.cpp foundations/names/adl-indirect-base.cpp foundations/names/adl-base-friend.cpp foundations/names/adl-enum.cpp foundations/names/adl-enum-in-class-friend.cpp foundations/names/adl-enum-in-class-namespace.cpp foundations/names/adl-pointer.cpp foundations/names/adl-reference.cpp foundations/names/adl-array.cpp foundations/names/adl-function-pointer-parameter.cpp foundations/names/adl-function-pointer-return.cpp foundations/names/adl-function-name.cpp foundations/names/adl-member-pointer-class.cpp foundations/names/adl-member-pointer-type.cpp foundations/names/adl-member-function-pointer-parameter.cpp foundations/names/adl-template-argument.cpp foundations/names/adl-template-namespace.cpp foundations/names/adl-template-template-argument.cpp foundations/names/adl-lambda-namespace.cpp foundations/names/adl-lambda-function.cpp foundations/names/adl-lambda-class-member.cpp foundations/names/adl-local-class.cpp foundations/names/adl-local-enum.cpp foundations/names/adl-incomplete-class.cpp foundations/names/adl-second-argument.cpp foundations/names/adl-function-template-in-set.cpp foundations/names/adl-function-template-parameter-adds-nothing.cpp foundations/names/adl-template-id-argument.cpp

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
| A {term}`specialization` of a {term}`class template` ({rule}`TARG-19`) | Also the namespace of the template, what each type argument adds, and for each {term}`template argument` that is a template, its namespace and the class that it is a member of |
| A lambda's closure type | The innermost namespace that contains the lambda. If the lambda is in a class body outside any function body, as in a default member initializer, also that class |
| A class or enumeration declared in a function | The innermost namespace that contains the function |
| A name of an {term}`overload set`, or its address | What the type of each function adds. A function template adds what its parameter types and return type add, apart from the types that depend on its template parameters. A {term}`template-id` also adds what each type template argument adds, and the namespace of each template template argument ({question}`Q-0311`) |

A class adds its associated namespaces also when the class is
incomplete.
:::

:::{rule} NAM-28
:kind: reject
:tests: foundations/names/adl-null-pointer.cpp foundations/names/adl-alias-namespace.cpp foundations/names/adl-lambda-parameter-type.cpp foundations/names/adl-template-value-argument.cpp foundations/names/adl-not-derived.cpp foundations/names/adl-braced-list.cpp foundations/names/adl-function-pointer-fundamental.cpp foundations/names/adl-local-class-member-function.cpp foundations/names/adl-lambda-member-function-body.cpp

An argument adds only what the table of NAM-27 gives. In particular,
these add nothing:

- `std::nullptr_t`, a braced list, and a function type built only from
  fundamental types,
- the namespace that declares a type alias,
- the parameter types of a lambda,
- a template argument that is a value,
- the classes derived from the argument's class,
- the class of a member function that declares a local class or
  contains a lambda.
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
access check of the conversion to that base ({rule}`ACC-12`).
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
:tests: foundations/names/class-members-any-order.cpp expressions/postfix/member-access-class-being-defined.cpp expressions/postfix/member-access-later-member-in-body.cpp

A member function body sees all members of its class, including the
members declared after the member function (CLS-2). A member declaration
sees only the members declared before that declaration. A member access
`E.m` in a member declaration also needs a complete class, or the class
being defined, and must find `m` ({rule}`PFX-11`, {question}`Q-0316`).
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
:tests: foundations/names/label-duplicate.cpp foundations/names/label-duplicate-other-block.cpp

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

% tidal-example: reject
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

A class and a variable can share a name. The variable takes the name,
and a class key reaches the class:

% tidal-example: reject
```cpp
struct stat { long size; };
int stat(const char* path);
struct stat info;               // the class
int n = stat("log");            // the function
stat other;                     // rejected: stat is the function here

template <class T> struct box { };
int box = 0;                    // rejected: box is a class template
```

The injected class name works through a base in another namespace:

% tidal-example: reject
```cpp
namespace io { struct Stream { }; }
struct File : io::Stream {
  Stream* base_part();          // Stream is io::Stream
};
File::Stream* p = nullptr;      // also io::Stream
File::File* q = nullptr;        // rejected: names the constructor
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 236
of 240 tests.

The clean-room writer probed qualified lookup, using directives,
argument-dependent lookup, and the injected class name with Clang 22,
GCC 16, and EDG 7.0, and checked the tests on Clang 18, GCC 14, and EDG
7.0 for ILP32. The compilers agree on every probe except those that
Q-0311 and the two questions below record. The
argument-dependent lookup grid of NAM-27 and NAM-28 has one test for
each row, and a rejected test for each kind of type that adds nothing.

- NAM-27: for an argument that is a template-id naming a function
  template, GCC and EDG add the namespaces of the type arguments, and
  Clang adds nothing for them ({question}`Q-0311`, settled). Clang 22
  stays a recorded deviation.
- NAM-27, NAM-28: all compilers add the class of a member template that
  is a template argument, and the namespaces of an overload set whose
  address is the argument. A lambda in a member function body does not
  add the class.
- NAM-12, NAM-13, NAM-14, NAM-16, NAM-23, NAM-37: all compilers find an
  outer class past an inner variable after a class key and in a list of
  bases, follow a using directive in an inline namespace of `N`, merge
  functions from two paths of directives into one overload set, accept
  `C::x` for an earlier member inside the definition of `C`, apply a
  directive in a namespace to a member definition outside the
  namespace, and look up a qualifier after `.` from the point of use
  when the class has no member of that name.
- NAM-1: an enumerator's initializer sees an outer constant of the same
  name on all compilers.
- NAM-16: all compilers accept a destructor call `p->T::~T()` where `T`
  is an alias for `int`. The source-reader gap review of names and
  lookup (`reports/review/2026-11-03-gap-review-names.md`) finds two
  readings of the C++23 text here. NAM-16 follows the compilers.
- NAM-38: Clang 22 and GCC 16 keep argument-dependent lookup for an
  operator when a block declares an operator function that does not
  fit. EDG 7.0 does not
  ({question}`Q-0364`). The gap review
  reads the C++23 text strictly as EDG does, with doubt. The gap review
  of the rules that phase 7 added
  (`reports/review/2026-11-04-gap-review-phase-7-additions.md`) finds
  the text unclear on the order of two lookup steps, and the literal
  text as EDG reads it.
- NAM-39: GCC and EDG let a member of an anonymous union share its name
  with a class, and Clang rejects the class. The gap review reads the
  C++23 text as Clang does. Tidal follows GCC and EDG
  ({question}`Q-0362`), with NAM-P10. The gap review of the rules that
  phase 7 added reads the text in the same way as the first review
  (`documentation/decisions/pending.md`, row 4).
- NAM-40, NAM-41: in the type of a conversion function, all compilers
  find a variable that shares its name with a class, and reject the
  class there. The gap review reads the C++23 text as finding the class.
  Tidal follows the compilers (`documentation/decisions/pending.md`,
  row 4).
- NAM-23: for a namespace that a chain of using directives reaches, all
  compilers put the members at the level of the first directive in the
  chain. A declaration at that level makes a use ambiguous, and a
  declaration between the use and that level hides the members.
- NAM-13, NAM-23: all compilers treat an unnamed namespace as a using
  directive in the namespace that contains it. `N::x` prefers a
  declaration of `x` in `N` to one in an unnamed namespace in `N`.
- NAM-39 to NAM-42: all compilers agree on a class or enumeration that
  shares its name with a variable, functions, or an enumerator, at
  namespace scope and in a class. In a list of bases, all compilers
  find the class, although the variable hides the class elsewhere.
- NAM-8, NAM-18, NAM-24, NAM-26, NAM-34, NAM-36: each of these rules
  overlaps another `reject` rule. The rule text says which of the two
  is the specific one, and the tests cite that rule (decision 0042).

Inputs: clang-sema-decl, clang-sema-declcxx, clang-diag-sema, edg-decls,
clang-sema-lookup, clang-sema-cxxscopespec, edg-lookup, edg-scope-stk.
