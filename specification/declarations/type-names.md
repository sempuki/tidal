# Type aliases and class names

A type alias gives another name to a type. A program declares one with
`typedef` or with an alias declaration. An *elaborated type specifier*
names a class or enumeration with its key, such as `struct S`. An
elaborated type specifier can also declare a class before its
definition. This page covers:

- type aliases,
- elaborated type specifiers,
- forward declarations of classes and enumerations.

Depends on: declarators, enumerations, class definitions and members.

## Principles

:::{principle} TYN-P1
:rules: TYN-1 TYN-8 TYN-2 TYN-3 TYN-9

An alias is a second name for a type that already exists. The alias
makes no new type, so it makes no new overload. A qualifier applied to
the alias applies to the named type. An alias is an ordinary name in an
ordinary scope ({principle}`NAM-P1`), so an alias:

- hides outer names,
- is declared once per scope,
- can be declared again only to say the same thing.

An alias can give a type only a name for {term}`linkage`, and only when the type
is an unnamed class that C could also declare. Such a class has only
data members, member classes, and member enumerations, and none of
these:

- other members, such as member functions, static data members, and
  member aliases,
- bases,
- default member initializers,
- a lambda expression anywhere in the class,
- a member class that has any of these itself.

The implementation rejects an alias that gives a name for linkage to a
class with any of them.
:::

:::{principle} TYN-P2
:rules: TYN-4 TYN-10 TYN-7

A class key before a name restricts lookup to classes and enumerations,
so a function or a variable with the same spelling does not get in the
way. If the restricted lookup finds nothing, the name is a new class,
declared in the nearest namespace or block. The key has to match the
kind of thing that lookup finds, with `class` and `struct` counting as
one kind.

A declaration of only a class key and a plain name declares the class
in the current scope. A qualified name refers to a class that another
scope already declares. So a declaration of only a class key and a
qualified name declares nothing, and the implementation rejects it.
:::

:::{principle} TYN-P3
:rules: TYN-6 TYN-7

A class exists from its first declaration and is complete only after its
definition.

| A use that needs | Works after |
|---|---|
| Only the existence of the type, such as a pointer, a reference, or a function declaration | A declaration |
| The size or the members | The definition |

A definition happens once. An enumeration with no body needs a fixed
underlying type, because a use can ask only for the size of the
enumeration.
:::

:::{principle} TYN-P4
:rules: TYN-5

A name used in a class body means one thing throughout that body. A
member that changes the meaning of the name after a use makes the class
not valid, and an implementation need not notice ({principle}`NAM-P4`).
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   typedef_decl: "typedef" `decl_specifiers` `declarator` {"," `declarator`} ";"
   alias_decl: "using" `identifier` {`attribute_spec`} "=" `type_id` ";"
   elaborated_type: `class_key` [`nested_name`] (`identifier` | `template_id`)
                  : | "enum" [`nested_name`] `identifier`
   class_key: "class" | "struct" | "union"
   forward_class: `class_key` {`attribute_spec` | `alignment_specifier`} `identifier` ";"
```

A *forward declaration* declares a class or enumeration without its
body. The production `enum_opaque` of {doc}`/declarations/enumerations`
gives the forward declaration of an enumeration. The class is
incomplete until its definition.

## Constraints

:::{rule} TYN-1
:kind: required
:tests: declarations/type-names/alias-cv.cpp declarations/type-names/alias-defines-class.cpp declarations/type-names/alias-function-type.cpp declarations/type-names/alias-in-block.cpp declarations/type-names/alias-in-class.cpp declarations/type-names/alias-of-incomplete.cpp declarations/type-names/typedef-multiple.cpp declarations/type-names/typedef-redeclare-same.cpp declarations/type-names/typedef-same-type.cpp declarations/type-names/alias-inner-scope-hides.cpp declarations/type-names/alias-same-name-as-class.cpp declarations/type-names/alias-cv-reference-ignored.cpp

A `typedef` or an alias declaration declares a name for a type. The name
denotes the same type, so the declaration does not make a new type or a
new overload. These rules apply:

- `typedef` can declare several names, each with its own {term}`declarator`.
- An alias can name a function, array, or cv-qualified type. A
  {term}`cv-qualifier` applied to an alias of a pointer applies to the pointer.
  On an alias of an array, the cv-qualifier applies to the elements. On
  an alias of a reference or a function type, the cv-qualifier has no
  effect.
- Outside a class, a name can be declared again as an alias of the same
  type, in either form. A `typedef` can give a class its own name.
- An alias has the scope of its declaration, including a class or a
  block. An alias in an inner scope can name another type and hides the
  outer one.
- An alias declaration can define an unnamed class. An alias can name an
  incomplete class.

TYN-8 lists what is not valid.
:::

:::{rule} TYN-8
:kind: reject
:tests: declarations/type-names/alias-overload-same.cpp declarations/type-names/alias-scope.cpp

These consequences of TYN-1 make a program not valid:

- A program must not use an alias outside its scope.
- A function declared with an alias and a function declared with its
  type are one function. A program must not define that function twice.
:::

:::{rule} TYN-2
:kind: reject
:tests: declarations/type-names/class-definition-conflicts-typedef.cpp declarations/type-names/typedef-initializer.cpp declarations/type-names/typedef-name-in-class-redeclared.cpp declarations/type-names/typedef-no-name.cpp declarations/type-names/typedef-redeclare-different.cpp declarations/type-names/typedef-storage-class.cpp declarations/type-names/typedef-with-unsigned.cpp declarations/type-names/variable-then-typedef.cpp declarations/type-names/typedef-then-variable.cpp declarations/type-names/typedef-of-class-then-variable.cpp declarations/type-names/alias-member-redeclared-same.cpp declarations/type-names/alias-function-type-definition.cpp

A type alias must not:

- be declared again in the same scope for a different type,
- be declared again as a member of the same class,
- have an initializer, a storage class, or no name,
- be combined with other type specifiers such as `unsigned`,
- have the name of a variable in the same scope, or of a class that it
  does not name,
- be used to define a function, if it names a function type.
:::

:::{rule} TYN-3
:kind: required
:tests: declarations/type-names/typedef-unnamed-c-like.cpp declarations/type-names/typedef-unnamed-across-units.cpp

If a `typedef` declares a name for an unnamed class, the class uses that
name for linkage. TYN-9 states what such a class must be.
:::

:::{rule} TYN-9
:kind: reject
:tests: declarations/type-names/typedef-unnamed-class-member-function.cpp declarations/type-names/typedef-unnamed-class-base.cpp declarations/type-names/typedef-unnamed-class-default-member-initializer.cpp declarations/type-names/typedef-unnamed-class-member-alias.cpp declarations/type-names/typedef-unnamed-class-lambda.cpp declarations/type-names/typedef-unnamed-class-nested-member-function.cpp declarations/type-names/typedef-unnamed-class-nested-c-like.cpp

A class that uses a `typedef` name for linkage (TYN-3) must be C-like.
Its members are non-static data members, member classes, and member
enumerations only, and the class has none of these
({question}`Q-0067`):

- member functions, or any other kind of member, such as a static data
  member or a member alias,
- bases,
- default member initializers,
- a lambda expression anywhere in the class,
- a member class that breaks any of these limits itself.
:::

:::{rule} TYN-4
:kind: required
:tests: declarations/type-names/elaborated-declares.cpp declarations/type-names/elaborated-enum-declared.cpp declarations/type-names/elaborated-hides-function.cpp declarations/type-names/elaborated-in-parameter-scope.cpp declarations/type-names/elaborated-key-class-struct.cpp declarations/type-names/elaborated-in-member-declares-namespace.cpp

An elaborated type specifier refers to a class or enumeration that
lookup finds with that key. Other names with the same spelling, such as
functions, do not hide the class or enumeration. If lookup finds no
class, `struct S` or `class S` declares `S`:

- in the enclosing namespace, if the specifier occurs in a parameter
  list, a member declaration of a class, or namespace scope,
- in the block, if the specifier occurs in a block.

`class` and `struct` can refer to the same class. TYN-10 lists what is
not valid.
:::

:::{rule} TYN-10
:kind: reject
:tests: declarations/type-names/elaborated-declares-in-namespace.cpp declarations/type-names/elaborated-enum-undeclared.cpp declarations/type-names/elaborated-key-enum-mismatch.cpp declarations/type-names/elaborated-key-union-mismatch.cpp declarations/type-names/elaborated-qualified-undeclared.cpp declarations/type-names/typedef-name-not-class-key.cpp declarations/type-names/elaborated-declares-in-block.cpp declarations/type-names/elaborated-scoped-enum-key.cpp

These uses of an elaborated type specifier (TYN-4) are not valid:

- `union` for a class, or a class key for an enumeration,
- `enum class` or `enum struct` in a reference to an enumeration,
- `enum E` before `E` is declared,
- a qualified name that lookup does not find,
- a class key with the name of a type alias,
- a class that `struct S` or `class S` declared, used outside the scope
  that TYN-4 gives it.
:::

:::{rule} TYN-6
:kind: required
:tests: declarations/type-names/complete-after-definition-in-scope.cpp declarations/type-names/forward-after-definition.cpp declarations/type-names/forward-class-reference-param.cpp declarations/type-names/forward-class.cpp declarations/type-names/forward-enum-opaque.cpp declarations/type-names/forward-enum-scoped-default-base.cpp declarations/type-names/forward-enum-scoped-int-then-none.cpp declarations/type-names/forward-nested-class.cpp declarations/type-names/deleted-incomplete-types.cpp

These rules apply to forward declarations and complete types:

- After a forward declaration, a program can declare pointers and
  references to the class, and functions that take or return the class.
- A class is complete after its definition.
- A forward declaration after the definition is valid.
- A nested class can be declared in its class and defined outside the
  enclosing class.
- An {term}`opaque enumeration declaration` has a fixed underlying type, which
  is `int` for a scoped enumeration without one.
- A deleted function can have parameters and a return type of incomplete
  class type.
:::

:::{rule} TYN-7
:kind: reject
:tests: declarations/type-names/class-redefinition.cpp declarations/type-names/forward-class-function-definition-param.cpp declarations/type-names/forward-class-incomplete-use.cpp declarations/type-names/forward-class-object.cpp declarations/type-names/forward-nested-wrong-scope.cpp declarations/type-names/elaborated-qualified-declaration-only.cpp declarations/type-names/definition-incomplete-return.cpp declarations/type-names/elaborated-qualified-explicit-specialization.cpp

A program must not:

- access members of an incomplete class or create an object of it,
- define a function with a parameter or a return type of incomplete
  class type,
- declare only a class key and a qualified name, as a declaration on its
  own ({question}`Q-0093`),
- define a class two times,
- define a nested class that its enclosing class does not declare.

{rule}`ENUM-18` gives the rules for an enumeration declared without its
body and declared again.
:::

## Implementation Requirements

:::{rule} TYN-5
:kind: no-diagnostic
:tests: declarations/type-names/alias-shadow-changes-meaning.cpp declarations/type-names/function-name-changes-meaning.cpp

A class must not use a name from an enclosing scope and then declare a
member with that name and another meaning. The rule applies to every
kind of name, such as a function name that a later data member takes.
An implementation need not diagnose the change of meaning
({question}`Q-0066`).
:::

## Examples

% tidal-example: reject
```cpp
typedef unsigned long Count, *CountPtr;
using Handler = void(int);      // a function type
Handler* on_exit = nullptr;

struct Tree;                     // forward declaration
struct Forest { Tree* first; };  // pointer to incomplete type: valid
struct Tree { Tree* next; };

struct stat { long size; };
int stat(const char* path);
struct stat info;                // the class, not the function

enum class Mode : unsigned char; // opaque: size is known
Mode current();

enum Flags : int;                // opaque, fixed type int
enum Flags { on };               // rejected: no fixed type here
enum class Level : int;
enum class Level { low };        // valid: a scoped enumeration has int

namespace net {
void send(struct Packet* p);     // declares net::Packet
}
struct Packet {};                // ::Packet, a different class
void post(Packet* p) {
  net::send(p);                  // rejected: Packet* is not net::Packet*
}
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 59
of 68 tests.

Two questions on this page settled under decision 0054. Q-0066 settled on 2026-11-03 after the source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`). Q-0093 settled on 2026-11-04 after the source-reader gap review of the text phase 8 wrote on the declarations and statements chapters (`reports/review/2026-11-04-gap-review-phase-8-declarations-statements.md`). For each, the review supports the answer with no doubt, and a principle gives the same answer. Each test keeps its override for the family that still differs:

| Question | Principle | Family that still differs |
|---|---|---|
| {question}`Q-0066` | TYN-P4 | none; the rule needs no diagnostic |
| {question}`Q-0093` | TYN-P2 | GCC 16 |

{question}`Q-0067` is provisional. TYN-P1 and TYN-9 changed on 2026-11-04, so the question waits for a reader or a reviewer to use the changed TYN-P1. The tests keep their overrides for GCC 14.

- TYN-5: GCC rejects the class, for a type name and for a function name
  that a later data member changes. Clang and EDG accept the class
  ({question}`Q-0066`).
  TYN-5 needs no diagnostic. Every test of TYN-5 has an override, and
  each override records a result that the class allows.
- TYN-7: GCC accepts a declaration of only a class key and a qualified
  name. Clang and EDG reject the declaration ({question}`Q-0093`).
- TYN-9: every newest compiler rejects a typedef name for linkage for
  an unnamed class with a member alias, with a lambda expression in the
  type of a member, or with a member class that has a member function.
  A static data member in an unnamed class is rejected by another rule
  first. Clang and EDG also reject a friend declaration in such a class,
  and GCC accepts it. The gap review of the text phase 8 wrote does not
  name friend declarations, and no test checks them.
- TYN-7: an explicit specialization or an explicit instantiation of a
  class template can name the class with a class key and a qualified
  name. Every compiler accepts these declarations, which start with
  `template`.
- TYN-3: GCC 14 accepts a typedef name for a class with a member
  function. GCC 16, Clang, and EDG reject the typedef name
  ({question}`Q-0067`).

Inputs: clang-sema-decl, clang-sema-declcxx, clang-diag-sema, edg-decls,
edg-class-decl.
