# Names, scopes, and lookup

A declaration introduces a name into a scope. A use of a name finds a
declaration by lookup, which searches the scopes that enclose the use.
These follow from this model:

- hiding,
- redeclaration,
- the visibility of a name from its declaration to the end of its
  scope,
- the scopes that statements and classes introduce.

This page states it once; the pages on statements, namespaces, and
classes cite it.

Depends on: namespaces and using, class definitions and members, access
control.

## Principles

:::{principle} NAM-P1
:rules: NAM-1 NAM-3 NAM-8
:exceptions: NAM-6

A scope is a region of the program where names can be declared. It is
one of these:

- a block,
- a function's parameter list,
- a class body,
- a namespace,
- the whole translation unit.

A declaration puts its name into the scope that contains it, and the
name is visible from that point to the end of the scope. Before that
point the name does not exist, and after the scope ends it is gone. A
scope holds one declaration for each name, unless the language says that
two declarations of the name describe one entity, as it does for a
function declared twice.
:::

:::{principle} NAM-P2
:rules: NAM-2 NAM-4 NAM-12 NAM-9

A use of a name searches the scope that contains the use, then the
scope that contains that one, and so on outward. The search stops at the
first scope that declares the name, whatever kind of thing the name is.
So an inner declaration hides an outer one, and a variable in an inner
scope hides a function of the same name outside it. A qualified name
skips the search and goes to the scope it names, which is how a program
reaches a hidden name. A namespace is a scope that a program can reopen
and add to, and a member declared in it can be defined outside it with
a qualified name.
:::

:::{principle} NAM-P3
:rules: NAM-5

A function call with an unqualified name also searches the namespaces
of the argument types. A function that takes a class is usually declared
beside the class, and this search finds it from any scope. It adds to
the ordinary search and does not replace it, and it does nothing for
arguments of fundamental type.
:::

:::{principle} NAM-P4
:rules: NAM-6 NAM-7 NAM-10 NAM-11

Two scopes depart from the plain model. In a class body, member function
bodies and default member initializers see every member of the class,
also those declared later, because the class is complete there. The
other parts of the body see only earlier members. Labels have their own
scope, the whole function, and their own name space, so a label and a
variable can share a name.
:::

## Constraints

:::{rule} NAM-1
:kind: required
:tests: foundations/names/visible-from-declaration.cpp foundations/names/visible-in-own-initializer.cpp foundations/names/function-sees-earlier-globals.cpp

A name is visible from its point of declaration to the end of its scope.
The point of declaration comes before the initializer, so a variable is
in scope in its own initializer. Reading the variable there reads an
object that has no value yet, which is undefined (OBJ-2). A function
body sees the names declared before the function in its translation
unit.
:::

:::{rule} NAM-8
:kind: reject
:tests: foundations/names/use-before-declaration.cpp foundations/names/scope-ends-with-block.cpp foundations/names/function-not-later-global.cpp foundations/translation/unit-sees-own-declarations-only.cpp expressions/primary/name-undeclared.cpp foundations/names/adl-not-for-fundamental.cpp statements/blocks/block-scope.cpp statements/if/init-scope-ends.cpp statements/loops/for-init-scope.cpp statements/loops/body-own-scope.cpp

A use of a name must find a declaration that is visible at that point
(NAM-1). These uses are rejected:

- a use before the declaration,
- a use after the scope of the declaration ends,
- a use of a name that only another translation unit declares.

In a class body, NAM-6 makes later members visible.
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
valid, even when the hidden declaration fits it.
:::

:::{rule} NAM-3
:kind: reject
:tests: foundations/names/redeclare-same-scope.cpp foundations/names/redeclare-parameter-in-body.cpp foundations/names/redeclare-different-kind.cpp foundations/names/redeclare-function-ok.cpp

A name must not be declared twice in one scope, except by declarations
that refer to one entity, such as two declarations of one function. The
outermost block of a function is one scope with its parameter list. A
variable and a function with one name in one scope are not valid.
:::

:::{rule} NAM-4
:kind: required
:tests: foundations/names/namespace-reopened.cpp foundations/names/namespace-member-defined-outside.cpp

A namespace can be reopened, and a later part sees the names of an
earlier part. A function declared in a namespace can be defined outside
it with a qualified name.
:::

:::{rule} NAM-9
:kind: reject
:tests: foundations/names/namespace-qualified-use.cpp

From outside a namespace, a name in it needs qualification (NAM-4).
:::

:::{rule} NAM-5
:kind: required
:tests: foundations/names/adl-finds-function.cpp foundations/names/adl-not-for-fundamental.cpp foundations/names/adl-plus-ordinary.cpp

An unqualified function call also looks in the namespaces of its
argument types for the function name. This finds a function declared in
the same namespace as a class argument from any scope. Arguments of
fundamental type add no namespaces. The functions found this way join
the functions that ordinary lookup finds.
:::

:::{rule} NAM-6
:kind: required
:tests: foundations/names/class-members-any-order.cpp

A member function body sees all members of its class, including those
declared after it (CLS-2). A member declaration sees only the members
declared before it.
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

## Evidence

The results are in `tests/RESULTS.md`. All seven compilers agree on all
26 tests.

Inputs: clang-sema-decl, clang-sema-declcxx, clang-diag-sema, edg-decls.
