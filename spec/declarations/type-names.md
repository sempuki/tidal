# Type aliases and class names

A type alias gives another name to a type. A program declares one with
`typedef` or with an alias declaration. An *elaborated type specifier*
names a class or enumeration with its key, such as `struct S`. It can
also declare a class before its definition. This page covers type
aliases, elaborated type specifiers, and forward declarations of classes
and enumerations.

Depends on: declarators, enumerations, class definitions and members.

## Syntax

```{eval-rst}
.. productionlist:: tyn
   typedef_decl: "typedef" `decl_specifiers` `declarator` {"," `declarator`} ";"
   alias_decl: "using" `identifier` "=" `type_id` ";"
   elaborated_type: `class_key` [`nested_name`] `identifier`
                  : "enum" [`nested_name`] `identifier`
   class_key: "class" | "struct" | "union"
   forward_class: `class_key` `identifier` ";"
   opaque_enum: "enum" ["class" | "struct"] `identifier` [":" `type_specifiers`] ";"
```

A *forward declaration* declares a class or enumeration without its
body. The class is incomplete until its definition.

## Constraints

:::{rule} TYN-1
:kind: required
:tests: declarations/type-names/alias-cv.cpp declarations/type-names/alias-defines-class.cpp declarations/type-names/alias-function-type.cpp declarations/type-names/alias-in-block.cpp declarations/type-names/alias-in-class.cpp declarations/type-names/alias-of-incomplete.cpp declarations/type-names/alias-overload-same.cpp declarations/type-names/alias-scope.cpp declarations/type-names/typedef-multiple.cpp declarations/type-names/typedef-redeclare-same.cpp declarations/type-names/typedef-same-type.cpp

A `typedef` or an alias declaration declares a name for a type. The name
denotes the same type, so it does not make a new type or a new overload.
These rules apply:

- `typedef` can declare several names, each with its own declarator.
- An alias can name a function, array, or cv-qualified type. A
  cv-qualifier applied to an alias of a pointer applies to the pointer.
- A name can be declared again as an alias of the same type, in either
  form.
- An alias has the scope of its declaration, including a class or a
  block.
- An alias declaration can define an unnamed class. An alias can name an
  incomplete class.
:::

:::{rule} TYN-2
:kind: reject
:tests: declarations/type-names/class-definition-conflicts-typedef.cpp declarations/type-names/typedef-initializer.cpp declarations/type-names/typedef-name-in-class-redeclared.cpp declarations/type-names/typedef-no-name.cpp declarations/type-names/typedef-redeclare-different.cpp declarations/type-names/typedef-storage-class.cpp declarations/type-names/typedef-with-unsigned.cpp declarations/type-names/variable-then-typedef.cpp

A type alias must not:

- be declared again for a different type, in any scope,
- have an initializer, a storage class, or no name,
- be combined with other type specifiers such as `unsigned`,
- have the name of a variable or class in the same scope.
:::

:::{rule} TYN-3
:kind: required
:tests: declarations/type-names/typedef-unnamed-c-like.cpp declarations/type-names/typedef-unnamed-class-member-function.cpp

If a `typedef` declares a name for an unnamed class, the class uses that
name for linkage. Such a class must be C-like. It has no member
functions, bases, or default member initializers ({question}`Q-0067`).
:::

:::{rule} TYN-4
:kind: required
:tests: declarations/type-names/elaborated-declares-in-namespace.cpp declarations/type-names/elaborated-declares.cpp declarations/type-names/elaborated-enum-declared.cpp declarations/type-names/elaborated-enum-undeclared.cpp declarations/type-names/elaborated-hides-function.cpp declarations/type-names/elaborated-in-parameter-scope.cpp declarations/type-names/elaborated-key-class-struct.cpp declarations/type-names/elaborated-key-enum-mismatch.cpp declarations/type-names/elaborated-key-union-mismatch.cpp declarations/type-names/elaborated-qualified-undeclared.cpp declarations/type-names/typedef-name-not-class-key.cpp declarations/type-names/elaborated-declares-in-block.cpp

An elaborated type specifier refers to a class or enumeration that lookup
finds with that key. Other names with the same spelling, such as
functions, do not hide it. If lookup finds no class, `struct S` or
`class S` declares `S`:

- in the enclosing namespace, if it occurs in a parameter list or
  namespace scope,
- in the block, if it occurs in a block.

`class` and `struct` can refer to the same class. These are not valid:

- `union` for a class, or a class key for an enumeration,
- `enum E` before `E` is declared,
- a qualified name that lookup does not find,
- a class key with the name of a type alias.
:::

:::{rule} TYN-6
:kind: required
:tests: declarations/type-names/complete-after-definition-in-scope.cpp declarations/type-names/forward-after-definition.cpp declarations/type-names/forward-class-reference-param.cpp declarations/type-names/forward-class.cpp declarations/type-names/forward-enum-opaque.cpp declarations/type-names/forward-enum-scoped-default-base.cpp declarations/type-names/forward-nested-class.cpp

After a forward declaration, a program can declare pointers and
references to the class, and functions that take or return it. A class
is complete after its definition. A forward declaration after the
definition is valid. A nested class can be declared in its class and
defined outside it. An opaque enumeration declaration has a fixed
underlying type, which is `int` for a scoped enumeration without one.
:::

:::{rule} TYN-7
:kind: reject
:tests: declarations/type-names/class-redefinition.cpp declarations/type-names/enum-fixed-mismatch.cpp declarations/type-names/enum-scoped-mismatch.cpp declarations/type-names/forward-class-function-definition-param.cpp declarations/type-names/forward-class-incomplete-use.cpp declarations/type-names/forward-class-object.cpp declarations/type-names/forward-enum-base-mismatch.cpp declarations/type-names/forward-enum-no-base.cpp declarations/type-names/forward-nested-wrong-scope.cpp

A program must not:

- access members of an incomplete class or create an object of it,
- define a function with a parameter of incomplete class type,
- define a class two times,
- declare an unscoped enumeration without a body and without a fixed
  underlying type,
- declare an enumeration again with a different underlying type, scope,
  or fixed base,
- define a nested class that its enclosing class does not declare.
:::

## Implementation Requirements

:::{rule} TYN-5
:kind: no-diagnostic
:tests: declarations/type-names/alias-shadow-changes-meaning.cpp

A class must not use a name from an enclosing scope and then declare a
member with that name and another meaning. An implementation need not
diagnose it ({question}`Q-0066`).
:::

## Examples

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
```

## Evidence

The results are in `tests/RESULTS.md`. All seven compilers agree on 48 of
50 tests.

- TYN-5: GCC rejects the class. Clang and EDG accept it
  ({question}`Q-0066`).
- TYN-3: GCC 14 accepts a typedef name for a class with a member
  function. GCC 16, Clang, and EDG reject it ({question}`Q-0067`).

Inputs: clang-sema-decl, clang-sema-declcxx, clang-diag-sema, edg-decls,
edg-class-decl.
