# Use, headers, and the one-definition rule

A program *uses* a variable or a function when the program needs the
entity itself at run time, and not only its type or its constant value.
A used entity needs a definition. This page states the following:

- which expressions and constructs use a variable or a function,
- how many definitions each kind of entity can have, in one
  translation unit and in the whole program,
- how templates and their specializations are shared across units,
- how `#include "file"` puts a header's text into a translation unit,
  and how a header is kept from appearing twice.

The pages on linkage, `inline`, lambdas, local classes, and virtual
functions state parts of these rules for their own constructs.

Depends on: storage duration and linkage, the `inline` specifier,
unevaluated operands, lambdas, local classes, virtual functions,
constructors and destructors, new and delete, template instantiation
and specialization.

## Principles

:::{principle} ODR-P1
:rules: ODR-3 ODR-2 ODR-4 ODR-5 ODR-6

A {term}`use` of a variable or a function is a point where the running
program needs the entity: its storage, its address, or its code. A used
entity needs a definition. An expression that names an entity is a use,
except where the program needs only the type or the value:

| Where the name occurs | Use? |
|---|---|
| An evaluated expression that calls the function, takes an address, or binds a reference | yes |
| A constant {term}`template argument` that takes an address or binds a reference | yes |
| An evaluated expression that reads the value of a constant variable of scalar type, or of a member or element of a constant that has no `mutable` member | no |
| An evaluated expression that copies a whole constant of class type, because the copy binds a reference | yes |
| An expression whose value is discarded and names a constant variable | no |
| An operand of `sizeof`, `decltype`, or `noexcept` | no |
| A branch of `if constexpr` that the condition does not select | a use for the capture rules only; no definition is needed |
| A default argument that a call does not use | no |

Some uses have no name in the source. Creating, copying, and destroying
an object use the constructors and destructors that the work runs. A
`new` or `delete` expression uses its allocation and deallocation
functions, and so do the constructors and the destructor of a class
that declares its own. A virtual function that is not pure is always
used, because a call through a base can reach it.

A lambda and a local class see the automatic variables of the enclosing
function only by their values. So a use of such a variable there needs
a capture, or is not valid at all.
:::

:::{principle} ODR-P2
:rules: ODR-15 ODR-1 ODR-7 ODR-8 ODR-9 ODR-10 ODR-11 ODR-12 ODR-13 ODR-14

A definition creates the entity. A declaration only names it. The
number of definitions that the language allows depends on the kind of
entity and on the scope that is counted:

| Entity | Definitions in one {term}`translation unit` | Definitions in the program | Who finds a fault |
|---|---|---|---|
| Variable or function with external {term}`linkage`, not `inline` | at most one | at most one; exactly one, if the program uses the entity | the linker, if anything |
| Variable or function with internal linkage | at most one; exactly one, if the unit uses the entity | one in each unit that uses it | the compiler, usually as a warning |
| `inline` variable or function | at most one; exactly one, if the unit uses the entity | one in each unit that uses it, all the same | usually nothing |
| Class or enumeration | at most one | one in each unit that needs it, all the same | usually nothing |
| Template, member of a class template, or partial specialization | at most one | one in each unit that uses it, all the same | usually nothing |
| Explicit specialization | as for the function, variable, or class that it is | as for the function, variable, or class that it is | as for that entity |

A template is a pattern, and each unit that uses a specialization
makes the specialization from its own copy of the pattern. The copies
must match, and the units must see the same explicit and partial
specializations. The program then has one entity for each
specialization. An explicit specialization of a function template or
a variable template is an ordinary function or variable. A header can
hold its definition only if the definition is `inline`.

A unit is translated alone ({principle}`TR-P2`). So a second definition
inside one unit is visible to the compiler and is rejected. A missing or
extra definition across units is found only when the program is linked,
if at all. Definitions in several units that differ are not found at
all. The program then runs with a mix of the definitions.
:::

:::{principle} ODR-P3
:rules: INC-1 INC-2 INC-3 INC-4 INC-5

A {term}`header` holds declarations that several units share. The
directive `#include "file"` replaces itself with the text of the file,
so each unit gets its own copy of the same definitions:

```text
square.h                       unit a.cpp                 unit b.cpp
  struct Square {...};   --->    #include "square.h"       #include "square.h"
  int perimeter(Square);         (Square defined here)     (Square defined here)
                                 calls perimeter           defines perimeter
```

The class has one definition in each unit, with the same tokens, which
ODR-10 requires. The function has one definition in the program, which
ODR-8 requires. A header that holds a definition of a function that is
not `inline` breaks ODR-8 as soon as two units include it. The same
holds for a variable that is not `inline`, unless the variable is
`const` and not `volatile`.

A constant that several units share has three common forms:

| The header holds | Objects in the program | A constant in each unit |
|---|---|---|
| `inline constexpr int limit = 8;` | one ({rule}`INL-1`) | yes |
| `constexpr int limit = 8;` | one in each unit ({rule}`LNK-3`) | yes |
| `extern const int limit;`, and one unit defines `limit` | one | only in the defining unit ({rule}`CE-3`) |

A unit that includes a header twice, often through two other headers,
would define the header's classes twice. An {term}`include guard`, or
`#pragma once` on the matrix compilers, makes the second inclusion
empty.
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   include_directive: "#" "include" '"' `header_name` '"'
   header_name: any characters other than a double quote or a line break
   guarded_header: {`blank_line`} `include_guard` {`blank_line`}
   include_guard: "#" "ifndef" `identifier`
                : "#" "define" `identifier` [`guard_value`]
                : {`guarded_line`}
                : "#" "endif"
   guard_value: any tokens up to the end of the line
   guarded_line: `text_line` | `conditional_group`
   conditional_group: `if_line` {`guarded_line`}
                    : {`else_line` {`guarded_line`}}
                    : "#" "endif"
   if_line: "#" ( "if" | "ifdef" | "ifndef" ) any tokens up to the end of the line
   else_line: "#" ( "elif" | "elifdef" | "elifndef" ) any tokens up to the end of the line
            : | "#" "else"
   text_line: any line that does not start with "#" and one of the words above
   blank_line: a line that holds only white space and comments
   pragma_once: "#" "pragma" "once"
```

Each directive is on a line of its own. The rules for these lines are:

- A `guarded_header` is a whole file. Only blank lines and comments
  come before the `#ifndef` line and after the `#endif` line.
- The two identifiers of an `include_guard` are the same name, which no
  other header uses.
- Each `if_line` pairs with the first `#endif` after it that no later
  `if_line` takes. The `#endif` that ends the `include_guard` pairs
  with its `#ifndef`.
- How an `if_line` condition is evaluated, and how a name defined with
  a `guard_value` is replaced in later text, are outside this
  specification *(preprocessor referenced)*.

## Constraints

:::{rule} ODR-15
:kind: required
:tests: foundations/odr/variable-declaration-forms.cpp foundations/odr/variable-without-initializer-twice.cpp foundations/odr/extern-initializer-is-definition.cpp foundations/odr/declarations-and-one-definition.cpp foundations/odr/constexpr-member-redeclared-outside.cpp foundations/odr/explicit-instantiation-declaration-not-definition.cpp foundations/odr/explicit-specialization-with-initializer-twice.cpp foundations/odr/explicit-specialization-without-initializer.cpp foundations/odr/parameter-of-declaration-not-definition.cpp

A declaration of a variable is a definition, with these exceptions:

- The declaration has `extern`, or the form `extern "C"`, and no
  initializer ({rule}`STO-7`, {rule}`LSP-3`).
- The declaration is of a static data member in its class, and the
  member is neither `inline` nor `constexpr` ({rule}`CLS-3`).
- The declaration is outside the class, without an initializer, of a
  static data member that its class declares `constexpr`. This form is
  deprecated.
- The declaration is a parameter of a function declaration that is not
  a definition. Its type can be incomplete.
- The declaration is an explicit instantiation declaration, with
  `extern template`, of a variable template or of a static data member.
- The declaration explicitly specializes, with no initializer, a static
  data member that a class template declares, or a member variable
  template.

So `int total;` at namespace scope is a definition, also without an
initializer.
:::

:::{rule} ODR-1
:kind: reject
:tests: foundations/odr/variable-defined-twice.cpp foundations/odr/variable-without-initializer-twice.cpp foundations/odr/enumeration-defined-twice.cpp foundations/odr/static-member-defined-twice.cpp foundations/odr/inline-variable-defined-twice.cpp foundations/odr/header-included-twice.cpp foundations/odr/declarations-and-one-definition.cpp foundations/odr/guard-text-after-endif.cpp foundations/odr/extern-initializer-is-definition.cpp foundations/odr/explicit-specialization-with-initializer-twice.cpp

A translation unit must not define a variable, a function, a class, or
an enumeration more than once. The rule also applies to `inline`
entities and to static data members. ODR-15 gives the declarations of
a variable that are definitions. {rule}`FN-14` states the rule for
functions, and {rule}`TYN-7` for classes.
:::

:::{rule} ODR-2
:kind: reject
:tests: foundations/odr/mutable-member-read-in-lambda.cpp foundations/odr/local-class-binds-constant.cpp foundations/odr/class-constant-copy-in-lambda.cpp foundations/odr/constant-reads-in-lambda.cpp foundations/odr/local-class-reads-constant.cpp foundations/odr/member-read-beside-mutable-in-lambda.cpp foundations/odr/discarded-branch-address-in-lambda.cpp foundations/odr/discarded-value-constant-in-lambda.cpp foundations/odr/discarded-value-class-constant-in-lambda.cpp foundations/odr/constant-array-element-in-lambda.cpp

A lambda that does not capture an automatic variable of the enclosing
function must not use the variable (ODR-3). A member function of a local
class must not use such a variable either. {rule}`LAM-9`,
{rule}`LAM-17`, and {rule}`CLS-28` state these cases for their
constructs.
:::

:::{rule} INC-3
:kind: reject
:tests: foundations/odr/include-missing-file.cpp foundations/odr/include-missing-file-angle.cpp

If no file matches the name in an `include_directive`, the
implementation rejects the program.
:::

## Runtime Behavior

:::{rule} ODR-3
:kind: required
:tests: foundations/odr/overload-set-uses-selected-only.cpp foundations/odr/constant-value-read-not-use.cpp foundations/odr/constant-reads-in-lambda.cpp foundations/odr/local-class-reads-constant.cpp foundations/odr/unevaluated-name-not-use.cpp foundations/odr/constant-address-is-use.cpp foundations/odr/function-address-is-use.cpp foundations/odr/template-argument-address-is-use.cpp foundations/odr/template-argument-address-links.cpp foundations/odr/constant-read-through-operators.cpp foundations/odr/constant-reference-through-conditional-is-use.cpp foundations/odr/class-constant-copy-in-lambda.cpp foundations/odr/member-read-beside-mutable-in-lambda.cpp foundations/odr/discarded-branch-address-in-lambda.cpp foundations/odr/discarded-value-constant-in-lambda.cpp foundations/odr/discarded-value-class-constant-in-lambda.cpp foundations/odr/constant-array-element-in-lambda.cpp

An evaluated expression that names a variable or a function uses it,
with these exceptions:

- The expression only reads the value of a scalar part of a constant
  variable, and the variable has no `mutable` member at any depth. The
  scalar part can be the variable, a member, or an element that a
  subscript selects. A copy of a whole object of class type is a use
  ({question}`Q-0129`).
- The name of a constant variable is in an expression whose value is
  discarded ({rule}`UNEV-2`, {question}`Q-0367`).
- In the two cases above, the name can be in parentheses, the second or
  third operand of `?:`, or the right operand of a comma.
- The name is a reference variable that a constant expression
  initializes.
- The name is in an unevaluated operand ({rule}`UNEV-1`).

A name in a {term}`discarded statement` needs no definition, and the
use still counts for ODR-2 ({question}`Q-0367`).

A name of an {term}`overload set` uses only the function that overload
resolution or a target type selects ({rule}`OVL-18`). A constant
template argument is an evaluated expression, so `S<&g>` uses `g`
({question}`Q-0315`).

A constant variable is a variable of object type that is
{term}`usable in constant expressions` ({rule}`CE-3`).
:::

:::{rule} ODR-4
:kind: required
:tests: foundations/odr/local-object-uses-destructor.cpp foundations/odr/constructor-uses-member-constructor.cpp foundations/odr/destructor-uses-pure-base-destructor.cpp foundations/odr/copy-uses-copy-constructor.cpp foundations/odr/prvalue-initializer-no-copy.cpp foundations/odr/constructor-uses-member-destructor.cpp

These constructs use {term}`special member functions <special member function>`:

- Initializing an object uses the constructor that the initialization
  selects.
- An object that the program destroys uses its destructor.
- A constructor uses the constructors of its bases and members, and
  their destructors, which run if the constructor throws
  ({rule}`UNW-2`). A destructor uses their destructors, also when a
  base's destructor is pure.
- A copy uses the copy or move constructor. A prvalue that initializes
  an object of its own type makes no copy, and uses neither.
:::

:::{rule} ODR-5
:kind: required
:tests: foundations/odr/delete-uses-destructor.cpp foundations/odr/new-uses-deallocation-function.cpp foundations/odr/virtual-destructor-uses-deallocation-function.cpp foundations/odr/new-noexcept-constructor-uses-deallocation.cpp foundations/odr/destructor-uses-deallocation-function.cpp foundations/odr/constructor-uses-allocation-functions.cpp

These constructs use allocation and deallocation functions:

- A `new` expression uses its allocation function, its constructor, and
  the matching deallocation function, if lookup finds one
  ({rule}`UNW-3`). The exception specification of the constructor does
  not matter ({question}`Q-0358`).
- A `delete` expression uses the destructor and the deallocation
  function.
- The definition of a constructor uses the allocation and deallocation
  functions of its class that are not placement forms. The definition
  of a destructor uses the deallocation function, virtual or not.
:::

:::{rule} ODR-6
:kind: required
:tests: foundations/odr/default-argument-not-used.cpp foundations/odr/default-argument-used.cpp foundations/odr/virtual-function-not-called-is-used.cpp foundations/odr/virtual-function-no-object-is-used.cpp

A default argument is evaluated only by a call that uses the default.
The entities that the default argument names are used only then. A
virtual function that is not pure is used, whether or not the program
creates an object of its class or names the function
({rule}`VIRT-20`).
:::

:::{rule} ODR-7
:kind: required
:tests: foundations/odr/header-class-shared.cpp foundations/odr/header-enumeration-shared.cpp

Several translation units can define the same class, enumeration,
`inline` function, or `inline` variable. If the definitions match
(ODR-10), they define one entity. Each unit can pass objects of the
class or enumeration to the others. An `inline` variable has one
address in the program.
:::

:::{rule} ODR-12
:kind: required
:tests: foundations/odr/header-class-template-shared.cpp foundations/odr/header-function-template-shared.cpp foundations/odr/header-variable-template-shared.cpp foundations/odr/header-specializations-shared.cpp foundations/odr/const-variable-template-one-object.cpp

Several translation units can each define the same entity of these
kinds:

- a {term}`class template` or a {term}`function template`,
- a {term}`variable template`, also one that is `const` or `constexpr`
  ({question}`Q-0297`),
- a class template's member function or static data member,
- a {term}`partial specialization`.

If the definitions match (ODR-10), each {term}`specialization` that
the units instantiate is one entity in the program. The entity has one
address, and a static local variable in it is one object.
:::

:::{rule} ODR-13
:kind: required
:tests: foundations/odr/header-specializations-shared.cpp foundations/odr/explicit-specialization-declared-in-header.cpp foundations/odr/explicit-variable-specialization-in-two-units.cpp foundations/odr/explicit-class-member-in-two-units.cpp

For ODR-7, ODR-8, and ODR-12, an {term}`explicit specialization`
counts as an ordinary function, variable, or class:

- If the template is a function template or a variable template,
  several units can define the explicit specialization only if it is
  `inline` ({rule}`TSP-25`).
- If the template is a class template, the explicit specialization's
  static members follow ODR-8, as in a class with no template.
:::

:::{rule} INC-1
:kind: required
:tests: foundations/odr/include-replaces-line.cpp foundations/odr/include-searches-own-directory.cpp foundations/odr/header-included-twice.cpp foundations/odr/header-function-in-two-units.cpp foundations/odr/header-constant-three-forms.cpp

An `include_directive` is replaced by the whole text of the named file.
The text can contain further `include_directive` lines, which are
replaced in the same way. The translation unit is the text after every
replacement.
:::

:::{rule} INC-4
:kind: required
:tests: foundations/odr/guarded-header-twice.cpp foundations/odr/header-class-shared.cpp foundations/odr/header-enumeration-shared.cpp foundations/odr/guard-comments-outside.cpp foundations/odr/guard-nested-conditionals.cpp foundations/odr/guard-define-with-value.cpp foundations/odr/guard-text-after-endif.cpp

If a file has the form of a `guarded_header`, its first inclusion in a
translation unit defines the guard name. Every later inclusion of the
file in that unit adds no text ({question}`Q-0127`). A `guard_value`
does not change this rule. Text after the `#endif` of the guard is added
at every inclusion.
:::

## Implementation Requirements

:::{rule} ODR-8
:kind: no-diagnostic
:tests: foundations/odr/header-function-in-two-units.cpp foundations/odr/unused-function-defined-in-two-units.cpp foundations/odr/unused-variable-defined-in-two-units.cpp foundations/odr/constructor-uses-member-destructor.cpp foundations/odr/destructor-uses-deallocation-function.cpp foundations/odr/constructor-uses-allocation-functions.cpp foundations/odr/virtual-function-no-object-is-used.cpp foundations/odr/unused-member-function-no-definition.cpp foundations/odr/local-object-uses-destructor.cpp foundations/odr/constructor-uses-member-constructor.cpp foundations/odr/destructor-uses-pure-base-destructor.cpp foundations/odr/copy-uses-copy-constructor.cpp foundations/odr/delete-uses-destructor.cpp foundations/odr/new-uses-deallocation-function.cpp foundations/odr/virtual-destructor-uses-deallocation-function.cpp foundations/odr/default-argument-used.cpp foundations/odr/virtual-function-not-called-is-used.cpp foundations/odr/constant-address-is-use.cpp foundations/odr/function-address-is-use.cpp foundations/odr/template-argument-address-links.cpp foundations/odr/explicit-variable-specialization-in-two-units.cpp foundations/odr/explicit-class-member-in-two-units.cpp foundations/odr/constant-reference-through-conditional-is-use.cpp foundations/odr/new-noexcept-constructor-uses-deallocation.cpp foundations/odr/placement-allocation-declared-unused.cpp foundations/odr/placement-allocation-declared-used.cpp

This rule applies to a variable or function with external linkage that
is not `inline`. The program must not define such an entity in two
translation units, also when the program does not use it. If the
program uses the entity, the program must define it once. An
implementation need not diagnose a break of this rule.
{rule}`LNK-6`, {rule}`LNK-7`, {rule}`INL-4`, and {rule}`VIRT-20` state
parts of the rule.
:::

:::{rule} ODR-9
:kind: no-diagnostic
:tests: foundations/odr/internal-function-not-defined-warns.cpp foundations/odr/unnamed-namespace-function-not-defined-warns.cpp foundations/odr/internal-function-used-in-unused-inline.cpp foundations/odr/internal-declarations-not-used.cpp foundations/odr/template-argument-address-is-use.cpp declarations/linkage-specifications/c-static-not-joined.cpp

A variable or function with internal linkage that a translation unit
uses must be defined in that unit. An implementation need not diagnose a
break of this rule ({question}`Q-0128`). {rule}`INL-6` states the same
rule for `inline` entities.
:::

:::{rule} ODR-10
:kind: no-diagnostic
:tests: foundations/odr/class-differs-across-units.cpp foundations/odr/enumeration-differs-across-units.cpp foundations/odr/inline-variable-differs-across-units.cpp foundations/odr/inline-same-tokens-different-meaning.cpp foundations/odr/class-template-differs-across-units.cpp foundations/odr/function-template-same-tokens-different-meaning.cpp foundations/odr/variable-template-differs-across-units.cpp foundations/odr/const-value-in-inline-two-units.cpp foundations/odr/inline-callee-defaults-differ.cpp foundations/odr/inline-alias-differs-across-units.cpp

If several translation units define the same entity of a kind in
ODR-7 or ODR-12, the definitions must match. Matching definitions have
the same tokens, and each name in them refers to the same entity, also
where a specialization is instantiated ({rule}`INST-32`). A type alias
counts as the type that it names, and that type must be the same in
each unit. Each call in
them uses the same default arguments ({rule}`DEFARG-2`). An
implementation need not diagnose a mismatch. A name can refer to a
different object in each unit if the object meets these conditions:

- It is `const` and not `volatile`, with internal linkage or none.
- It has the same literal type and the same constant initializer in
  each unit.
- The definitions only read its value, so they do not use it (ODR-3).

A reference with internal linkage can also differ, if it refers to the
same entity in each unit.
:::

:::{rule} ODR-14
:kind: no-diagnostic
:tests: foundations/odr/partial-specialization-in-one-unit.cpp foundations/odr/explicit-specialization-in-one-unit.cpp foundations/odr/explicit-class-specialization-in-one-unit.cpp

The translation units that use a template must declare the same
explicit and partial specializations of it:

- If one unit declares an explicit specialization, every unit that
  uses that specialization must declare it before the use
  ({rule}`TSP-6`).
- If one unit declares a partial specialization, every unit that
  instantiates a specialization that it matches must declare it before
  the use ({rule}`TSP-17`).

An implementation need not diagnose a break of this rule.
{rule}`INST-35` states the rule for an {term}`explicit instantiation` and an
explicit specialization in different units.
:::

:::{rule} INC-2
:kind: impl-defined
:tests: foundations/odr/include-searches-own-directory.cpp foundations/odr/include-relative-to-own-directory.cpp

The implementation decides which file an `include_directive` names. The
matrix compilers search the directory of the file that holds the
directive first. Then they search the directories that the command line
gives.
:::

:::{rule} INC-5
:kind: impl-defined
:tests: foundations/odr/pragma-once-header-twice.cpp foundations/odr/pragma-once-after-comment.cpp foundations/odr/pragma-once-after-text.cpp

The effect of a `pragma_once` line is implementation-defined. On the
matrix compilers, a file that holds the line adds its text only at its
first inclusion in a translation unit ({question}`Q-0127`). The line
can come after comments or other text, and later inclusions add none of
that text either.
:::

## Recommended Practice

:::{rule} ODR-11
:kind: recommended
:tests: foundations/odr/internal-function-not-defined-warns.cpp foundations/odr/unnamed-namespace-function-not-defined-warns.cpp foundations/odr/template-argument-address-is-use.cpp

An implementation should warn about a function with internal linkage
that the unit uses and does not define.
:::

## Examples

% tidal-example: reject
```cpp
struct Limits { static const int top = 5; };   // declared, not defined

int cells[Limits::top];        // reads the value: no use
const int* where = &Limits::top;   // a use: Limits::top needs a definition

int total;                     // a definition: the value is 0
extern int total;              // a declaration only
int pick(bool wide) { return wide ? Limits::top : 1; }   // no use

int helper();                  // never defined
decltype(helper()) count = 0;  // unevaluated: no use, so this is valid

void run() {
  const int size = 4;
  auto read = [] { return size; };        // reads the value: no capture needed
  auto point = [] { return &size; };      // a use: rejected
}
```

% tidal-example: fragment
```cpp
// guard.h
#ifndef PROJECT_GUARD_H
#define PROJECT_GUARD_H
struct Guarded { int v; };
#endif

// main.cpp
#include "guard.h"
#include "guard.h"     // adds nothing: the guard name is defined
```

```cpp
// holder.h, included by several units
template <class T> struct Holder { T item; T get() const; static int made; };
template <class T> T Holder<T>::get() const { return item; }  // valid in each unit
template <class T> int Holder<T>::made = 0;    // one Holder<int>::made in the program

template <class T> int rank = 0;               // one rank<long> in the program
template <> inline int rank<char> = 1;         // inline: valid in each unit
template <> int rank<bool> = 2;                // two units: two definitions
```

## Evidence

All eight compilers agree on 84 of 97 tests. EDG checks each unit and
does not link, so its results for the link and run tests say only that
the units are valid.

### Divergences

- {question}`Q-0129` (ODR-2, ODR-3; open): whether a lambda that copies
  a whole local constant of class type uses the constant. The rules say
  it does, as Clang does. GCC and EDG accept the lambda.
- {question}`Q-0130` (ODR-2; settled on ODR-P1): whether a read of a
  `mutable` member of a local constant, or a reference that a local
  class binds to one, uses the constant. Both are uses, as Clang and GCC
  say. EDG accepts both.
- {question}`Q-0367` (ODR-2, ODR-3; settled on ODR-P1): which of four
  forms in a lambda use a local variable: a discarded `const int`, a
  discarded class constant, an element of a constant array, and an
  address in a discarded branch. Only the address is a use. Clang
  differs in the first, second, and fourth forms, and GCC in the third.
- {question}`Q-0315` (ODR-3; settled on ODR-P1): whether a constant
  template argument uses the function it names. It does. GCC gives no
  diagnostic for such a function with internal linkage, which ODR-9
  allows, so no family differs on the use.
- {question}`Q-0128` (ODR-9, ODR-11; settled on ODR-P2): whether an
  implementation must diagnose a used function with internal linkage
  that has no definition. The rule needs no diagnostic, and ODR-11 asks
  for a warning. No family differs. GCC gives no diagnostic when the
  only use is in an `inline` function that nothing calls, and Clang 18
  gives a warning that `-pedantic-errors` leaves a warning.
- {question}`Q-0358` (ODR-5, ODR-8; open): whether a `new` expression
  uses the deallocation function when its constructor is `noexcept`.
  It does, and ODR-8 needs no diagnostic. GCC 16 at `-O0` fails to link,
  and Clang, GCC 14, and GCC 16 at `-O2` link and run.
- {question}`Q-0297` (ODR-12; settled on ODR-P2): whether a `const`
  variable template has one object for each specialization across
  units. It has. GCC 14 gives each unit its own object, and EDG does
  not link.
- {question}`Q-0127` (INC-4, INC-5; settled on ODR-P3): whether an
  include guard and `#pragma once` skip a second inclusion, also under
  strict flags. Both do, and the guard is the form in `required` text.
  No family differs.
- {question}`Q-0369` (LSP-4, LNK-5, FN-14; open): whether C names with
  internal linkage in two namespaces are one entity, and whether a C
  name in an unnamed namespace joins the other C names of its spelling.
  The `static` names are two entities, and GCC 16 fails at assembly
  with one symbol for the two. The name in an unnamed namespace joins
  the others, as Clang and GCC link it, and EDG accepts a second
  definition in another namespace. The gap review of the rules that
  phase 7 rewrote reads the second part as EDG does, and the choice
  waits on the maintainer (`documentation/decisions/pending.md`, row 4).

### Split tests

Each row is a test that carries an override for a baseline compiler. The
Expected column gives the outcome that the rules predict, and each
family column gives the outcome that the test expects on that family.
EDG runs no programs, so its cell for a run test gives the result of the
compile.

| Test | Rules | Expected | Clang | GCC | EDG | Question |
|---|---|---|---|---|---|---|
| `class-constant-copy-in-lambda.cpp` | ODR-2, ODR-3 | reject | reject | run | accept | {question}`Q-0129` |
| `const-variable-template-one-object.cpp` | ODR-12 | run | run | GCC 16: run; GCC 14: run-fail | accept | {question}`Q-0297` |
| `constant-array-element-in-lambda.cpp` | ODR-2, ODR-3 | run | run | reject | accept | {question}`Q-0367` |
| `declarations/linkage-specifications/c-static-not-joined.cpp` | LSP-4, ODR-9 | reject | Clang 22: reject; Clang 18: accept | reject | reject | {question}`Q-0369` |
| `discarded-branch-address-in-lambda.cpp` | ODR-2, ODR-3 | reject | accept | reject | reject | {question}`Q-0367` |
| `discarded-value-class-constant-in-lambda.cpp` | ODR-2, ODR-3 | run | reject | run | accept | {question}`Q-0367` |
| `discarded-value-constant-in-lambda.cpp` | ODR-2, ODR-3 | run | reject | run | accept | {question}`Q-0367` |
| `internal-function-used-in-unused-inline.cpp` | ODR-9 | reject | Clang 22: reject; Clang 18: run | run | reject | {question}`Q-0128` |
| `local-class-binds-constant.cpp` | ODR-2 | reject | reject | reject | accept | {question}`Q-0130` |
| `member-read-beside-mutable-in-lambda.cpp` | ODR-2, ODR-3 | reject | reject | reject | accept | {question}`Q-0130` |
| `mutable-member-read-in-lambda.cpp` | ODR-2 | reject | reject | reject | accept | {question}`Q-0130` |
| `new-noexcept-constructor-uses-deallocation.cpp` | ODR-5, ODR-8 | link-fail | run | GCC 16: link-fail; GCC 14: run | accept | {question}`Q-0358` |
| `template-argument-address-is-use.cpp` | ODR-3, ODR-9, ODR-11 | reject | reject | accept | skips | {question}`Q-0315` |

### Notes

- ODR-8: each translation unit compiles without a diagnostic, and only
  the linker reports a missing or extra definition. At `-O2`, the
  report compilers `gcc-16-o2` and `clang-22-o2` link four of the
  programs that fail to link at the default level. ODR-8 allows both
  results, because the rule needs no diagnostic (decision 0047). The
  four tests are `constant-address-is-use.cpp`,
  `function-address-is-use.cpp`,
  `virtual-destructor-uses-deallocation-function.cpp`, and
  `virtual-function-not-called-is-used.cpp`.
- ODR-3: every compiler links a program whose template argument takes
  the address of a function with external linkage that has no
  definition, when nothing calls the function. ODR-8 allows the result.
- ODR-10, ODR-14: Different definitions in two units link and run.
  So does a partial or explicit specialization that only one unit
  declares. The tests accept the result of either definition. In the
  default builds, both units use one of the definitions of an `inline`
  function or variable. In the `-O2` builds, each unit can use its own.
  An enumerator, and the value of a constant variable, is fixed in each
  unit when the unit is translated, so each unit uses its own value
  (`enumeration-differs-across-units.cpp`). A call in an `inline`
  function that uses the default argument of another function behaves
  the same way. If the units give that function different defaults,
  Clang and GCC give 1 in both units at `-O0` and a different value in
  each unit at `-O2` (`inline-callee-defaults-differ.cpp`). Only
  functions and objects are shared across units when the program is
  linked. ODR-10 and ODR-14 allow both results, because the rules need
  no diagnostic (decision 0047).
- ODR-4, ODR-5, ODR-6: Clang and GCC fail to link a constructor that
  can throw when the destructor of a member has no definition. They
  link a program whose constructor or non-virtual destructor uses
  allocation and deallocation functions with no definition, and a class
  with an undefined virtual function and no object. ODR-8 allows each
  result.
- ODR-8: Clang and GCC fail to link two units that define one function,
  or one variable, that the program does not use.
- ODR-10: a `const int` in a header gives each unit its own object. An
  `inline` function that reads its value in each unit links and runs.
- ODR-3: Clang and GCC link a program that reads the value of a static
  member with no definition through `?:`, a comma, or parentheses, and
  fail to link a program that binds a reference to such a `?:` result
  at `-O0`.
- ODR-12: Every compiler that links gives each specialization one
  address in the program. That includes the static members of class
  templates and partial specializations, and a static local variable in
  a function template.
- ODR-13: Two units that define a non-`inline` explicit specialization
  of a variable template fail to link on every compiler that links. So
  do two units that define a static data member of an explicit
  specialization of a class template.
- INC-2: The search order is each implementation's choice. Clang, GCC,
  and EDG take a header in the directory of the including header
  before a header of the same name in the directory of the test.
- INC-4, INC-5: the guard and `#pragma once` give the same result with
  comments before the guard, nested conditional directives in it, and a
  value in its `#define`. All three compilers skip the whole file at a
  later inclusion when `#pragma once` follows a definition.
- ODR-15: the gap review of the rules that phase 7 added lists four
  more variable declarations that are not definitions. Every compiler
  accepts each of them twice in one unit, and rejects a second explicit
  specialization of a static data member that has an initializer.
- The tests keep their headers as `*.h` files next to them.

Inputs: clang-sema-expr, clang-sema-decl, clang-diag-sema, edg-decls.
