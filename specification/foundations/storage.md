# Raw storage and object representation

An object occupies a run of bytes. This page states how a program uses
those bytes directly:

- storage that holds objects of other types, such as an array of bytes
  or the result of an allocation function,
- the creation of objects in storage, by placement `new` and by
  implicit creation,
- the reuse of storage, and the replacement of one object by another,
- the uses of a pointer or a glvalue outside an object's lifetime,
- the object representation, padding, and byte copies,
- the types through which a program can read or write an object,
- the kinds of pointer value, and the casts between objects with one
  address.

{principle}`OBJ-P1` gives the lifetime model that this page extends. The
launder operation and the library functions that start a lifetime
without a constructor are *(library referenced)*.

Depends on: objects, storage, and lifetime; new and delete; casts;
additive operators; unions; fundamental types; class members; copy and
move.

## Principles

:::{principle} STG-P1
:rules: STG-1 STG-2 STG-3

Every object occupies `sizeof` bytes of its type, and the address of
the object is the address of its first byte. The rules on this page
count in bytes:

```text
  struct Pair { int first; long second; };     sizeof(Pair) == 16 on x86-64

  offset  0               4               8                    16
          [ first         ][ padding       ][ second             ]
          ^
          &pair == &pair.first
```

Two objects that exist at the same time have different addresses. One
object can share an address with another only if it is nested in the
other, or if it is an empty subobject of zero size. A type whose size
does not fit in `std::size_t` cannot exist.
:::

:::{principle} STG-P2
:rules: STG-4 STG-5 STG-6 STG-17

Storage can outlive the objects in it. A byte array, or the storage
that an allocation function returns, holds one object after another:

```text
  unsigned char buffer[16];                 the byte array lives throughout

  new (buffer) Point(3, 4)    [ Point          ]    a Point begins
  point->~Point()             [                ]    the Point ends
  new (buffer) int(0)         [ int ]               an int begins
```

Placement `new` creates an object at a given address, in storage that
is large enough and aligned for the type. A new object ends each object
whose storage it takes, and no destructor runs. The byte array that
hosts the new object goes on, because the new object is nested in it.
:::

:::{principle} STG-P3
:rules: STG-7 STG-8 STG-18 STG-24

Some types need no constructor call to exist. Scalars, arrays, and
simple classes are such
{term}`implicit-lifetime types <implicit-lifetime type>`. A program
can use fresh storage as objects of these types without placement
`new`, because some operations create the objects implicitly:

| Operation | Creates objects in |
|---|---|
| a call of an allocation function, by name or from `new` | the storage that the call returns |
| the start of a byte array's lifetime | the array |
| some library functions *(library referenced)* | the storage that they name |

The implementation chooses the objects that give the program defined
behavior, looking ahead to the program's later use of the storage. If
no choice works, the behavior is undefined. If several choices work,
the program cannot observe which one the implementation made.
:::

:::{principle} STG-P4
:rules: STG-9 STG-19 STG-20 STG-21

A new object that takes the exact place of an old object of the same
type replaces the old object. The names, pointers, and references to
the old object then refer to the new object. So an assignment can
destroy an object and construct a new one in its place. The
replacement works only where the implementation cannot keep a stale
fact about the old object:

| Old object | New object | Old names and pointers reach the new object |
|---|---|---|
| a complete object, member, or element that is not a `const` complete object | the same type, in the same bytes | yes (STG-9) |
| any object | another type, or other bytes | no (STG-19) |
| a base subobject, or a `[[no_unique_address]]` member | any object | no (STG-19) |
| a `const` complete object that is not dynamic | any object | creating the new object is undefined (STG-20) |

Where the old names do not reach the new object, the launder operation
*(library referenced)* gives a pointer that does. The end of a block
still destroys each variable as its declared type, so the storage must
hold an object of that type at that point (STG-21).
:::

:::{principle} STG-P5
:rules: STG-10 STG-22

Before an object's lifetime begins and after it ends, the storage of the
object can exist. A pointer to that storage holds an address and
nothing more, so only the uses that need nothing but the address are
valid:

| Use of a pointer or glvalue outside the lifetime | Result |
|---|---|
| copy or compare the pointer, take the address, bind a reference of the same type | valid |
| convert to `void*`, and then to `unsigned char*`, `char*`, or `std::byte*` | valid |
| read or write the value ({rule}`OBJ-2`) | undefined |
| member access, call of a non-static member function | undefined |
| a cast to a derived class or a virtual base, `dynamic_cast`, `typeid` | undefined |
| `delete` | undefined |
:::

:::{principle} STG-P6
:rules: STG-11 STG-12 STG-13 STG-25 STG-26

The {term}`object representation` of an object is its bytes, viewed as
`unsigned char`. The {term}`value representation` is the set of bits in
those bytes that decide the value, and the other bits are padding. Two
objects with equal values can differ in their bytes. A byte copy of an
object of a trivially copyable type carries the value:

```text
  record --bytes--> unsigned char saved[sizeof record] --bytes--> record

  the value after the round trip equals the value before
```

An `unsigned char*`, `char*`, or `std::byte*` can step through the bytes
of any object, from the first byte to one past the last
({question}`Q-0304`). The
implementation chooses the order of the bytes in a scalar. A base
subobject can share its last bytes with members of the derived class,
so a byte copy into a base subobject can change those members.
:::

:::{principle} STG-P7
:rules: STG-14 STG-23

A read or a write sees an object through the type of the glvalue. The
implementation can assume that a glvalue of one type does not refer to
an object of an unrelated type, and so can keep a value in a register
across a write through another type. These glvalue types can read or
write an object whose dynamic type is `T`:

| Glvalue type | Allowed |
|---|---|
| `T` with any cv-qualifiers, also inside a pointer type | yes |
| the signed or unsigned integer type that matches `T` | yes |
| `char`, `unsigned char`, `std::byte` | yes |
| `signed char` | no ({question}`Q-0306`) |
| any other type, including a class with the same members | no |
:::

:::{principle} STG-P8
:rules: STG-15 STG-16 STG-27

A pointer value has one of four kinds:

| Kind | Represents | Indirection |
|---|---|---|
| a pointer to an object or a function | the address of its first byte | valid |
| a pointer just past an object | the address of the byte after the object | undefined ({rule}`ADD-10`) |
| the null pointer value | no address | undefined ({rule}`UN-7`) |
| an invalid pointer value | an address whose storage has ended | undefined ({rule}`UN-7`) |

Some objects with one address are
{term}`pointer-interconvertible`, and a cast turns a pointer to one into
a pointer to the other. Other objects with one address are not, and a
cast between their pointers keeps the original object:

```text
  union Cell { Node node; long raw; };    Cell <-> node <-> node.head <-> node.head.id
  struct Node { Head head; int next; };   each pair: one address, interconvertible
  struct Head { int id; };

  int items[2];                           items and items[0]: one address, not interconvertible
```
:::

## Constraints

:::{rule} STG-1
:kind: reject
:tests: foundations/storage/array-size-exceeds-size-type.cpp foundations/storage/class-size-exceeds-size-type.cpp

The size in bytes of a type must fit in `std::size_t` ({rule}`LSUP-4`).
The rule covers arrays ({rule}`DCL-10`) and classes.
:::

## Runtime Behavior

:::{rule} STG-2
:kind: required
:tests: foundations/storage/object-bytes-cover-members.cpp

An object occupies `sizeof` bytes of storage, and the address of the
object is the address of its first byte. An object of a trivially
copyable or standard-layout class occupies consecutive bytes
({rule}`CPY-19`, {rule}`CLS-19`).
:::

:::{rule} STG-3
:kind: required
:tests: foundations/storage/empty-base-takes-no-bytes.cpp foundations/storage/empty-subobjects-same-type-distinct.cpp

Two objects that exist at the same time can have one address only in
these cases:

- One object is nested in the other.
- One object is a subobject of zero size, and the two objects have
  different types.

A base of an empty class type can have zero size. So can an empty
member declared `[[no_unique_address]]` ({rule}`ATTR-9`, {rule}`ATTR-16`).
:::

:::{rule} STG-4
:kind: required
:tests: foundations/storage/byte-array-hosts-objects.cpp foundations/storage/std-byte-array-hosts-objects.cpp foundations/storage/char-array-holds-object.cpp foundations/language-support/byte-self-declared.cpp

A *byte array* is an array of `unsigned char` or of `std::byte`
({rule}`LSUP-9`). A byte array in its lifetime *hosts* an object
that is created inside it, if no smaller byte array inside it also
contains the whole object. An object is *nested* in an object that
hosts it or has it as a subobject, and in each object that those are
nested in.

An array of `char` or `signed char` hosts no object
({question}`Q-0305`).
:::

:::{rule} STG-5
:kind: required
:tests: foundations/storage/placement-new-result-is-address.cpp foundations/storage/placement-new-array.cpp foundations/storage/byte-array-hosts-objects.cpp foundations/storage/std-byte-array-hosts-objects.cpp foundations/language-support/construct-at-self-declared.cpp

`new (p) T(args)` with the library's non-allocating placement form
*(library referenced)* creates an object of type `T` at the address
`p`. The result points to the new object. The array form `new (p) T[n]`
creates an array at `p` in the same way.
:::

:::{rule} STG-6
:kind: required
:tests: foundations/storage/reuse-does-not-call-destructor.cpp foundations/storage/byte-array-hosts-objects.cpp foundations/storage/replace-base-ends-enclosing-sanitizer.cpp

If a new object takes storage that another object occupies, the
lifetime of the other object ends, and its destructor does not run.
An object that the new object is nested in keeps its lifetime, such as
the byte array that hosts the new object ({rule}`OBJ-10`).
:::

:::{rule} STG-7
:kind: required
:tests: foundations/storage/implicit-lifetime-trivial-class.cpp foundations/storage/implicit-lifetime-aggregate.cpp foundations/storage/implicit-array-for-arithmetic.cpp foundations/storage/implicit-object-from-allocation-function.cpp foundations/storage/implicit-object-in-byte-array.cpp

These types are
{term}`implicit-lifetime types <implicit-lifetime type>`:

- scalar types,
- array types, with any element type,
- an aggregate class whose destructor is not user-provided,
- a class with a trivial destructor that is not deleted, and with at
  least one trivial {term}`eligible` constructor,
- the cv-qualified forms of these types.
:::

:::{rule} STG-8
:kind: required
:tests: foundations/storage/implicit-object-from-allocation-function.cpp foundations/storage/implicit-object-in-byte-array.cpp foundations/storage/implicit-array-for-arithmetic.cpp foundations/storage/implicit-object-class-allocation-function.cpp foundations/storage/implicit-objects-follow-later-use.cpp

These operations create objects implicitly in the storage that they
name:

- a call of an allocation function, by name or from a `new` expression,
- the start of the lifetime of a byte array,
- some library functions *(library referenced)*.

The implementation creates the set of objects that gives the program
defined behavior, and the result of an allocation function points to
one of them. Each created object has an implicit-lifetime type, and
its subobjects of other types are not created.
:::

:::{rule} STG-9
:kind: required
:tests: foundations/storage/replace-variable-old-name.cpp foundations/storage/replace-object-with-const-member.cpp foundations/storage/replace-const-member-constant.cpp foundations/storage/replace-through-reference-constant.cpp foundations/storage/replace-array-element-constant.cpp foundations/storage/assignment-by-destroy-and-construct.cpp

A new object in the storage of an old object replaces the old object
if these conditions hold:

- The new object occupies exactly the bytes of the old object.
- The two objects have the same type, ignoring top-level cv-qualifiers.
- The old object is not a `const` complete object.
- Neither object is a base subobject or a member declared
  `[[no_unique_address]]`.
- Both objects are complete objects, or both are direct subobjects of
  one object, or of two objects that this rule replaces.

The names, pointers, and references to the old object then refer to
the new object. A new object in the place of a member or an array
element becomes that member or element.
:::

:::{rule} STG-10
:kind: required
:tests: foundations/storage/outside-lifetime-allowed-uses.cpp

Before the lifetime of an object begins, or after it ends, while its
storage exists, a program can do these things:

- copy or compare a pointer to the storage,
- convert the pointer to `void*`, and then to a pointer to
  `std::byte`, `unsigned char`, or `char`,
- take the address of a glvalue that refers to the storage, or bind a
  reference of the object's type to it.
:::

:::{rule} STG-11
:kind: required
:tests: foundations/storage/equal-values-different-bytes.cpp foundations/storage/padding-bytes-not-in-value.cpp

The {term}`object representation` of an object is its `sizeof` bytes,
viewed as `unsigned char`. The bits that decide the value are the
{term}`value representation`, and the other bits are padding. Objects
with equal values can have different object representations, and a
change to padding does not change the value.
:::

:::{rule} STG-12
:kind: required
:tests: foundations/storage/byte-round-trip.cpp foundations/storage/byte-round-trip-char-and-byte.cpp foundations/storage/padding-bytes-not-in-value.cpp

A program can copy the bytes of an object of a trivially copyable type
into an array of `std::byte`, `unsigned char`, or `char`. If the
program copies the bytes back, the object has its original value. A
base subobject and a `[[no_unique_address]]` member are not covered
(STG-25). {rule}`CPY-19` gives a copy into a second object.
:::

:::{rule} STG-13
:kind: required
:tests: foundations/storage/byte-pointer-walks-object.cpp foundations/storage/object-bytes-cover-members.cpp foundations/storage/byte-round-trip.cpp foundations/storage/past-end-address.cpp foundations/storage/padding-bytes-not-in-value.cpp

A pointer to `std::byte`, `unsigned char`, or `char` that points to the
first byte of an object can move over the bytes of the object. For
{rule}`ADD-6`, {rule}`ADD-7`, and comparisons, the bytes act as an
array of `sizeof` elements ({question}`Q-0304`). A read through the
pointer gives one byte of the object representation.
:::

:::{rule} STG-14
:kind: required
:tests: foundations/storage/access-signed-counterpart-and-cv.cpp foundations/storage/access-character-types.cpp foundations/storage/byte-round-trip-char-and-byte.cpp foundations/language-support/byte-self-declared.cpp

A program can read or write the value of an object whose dynamic type
is `T` through a glvalue of one of these types:

- `T` with any cv-qualifiers, also at each level of a pointer type,
- the signed or unsigned integer type that matches `T`, with any
  cv-qualifiers,
- `char`, `unsigned char`, or `std::byte`.
:::

:::{rule} STG-15
:kind: required
:tests: foundations/storage/past-end-address.cpp

A value of a pointer type is one of these:

- a pointer to an object or a function, which represents the address
  of its first byte,
- a pointer just past an object, which represents the address of the
  byte after the object,
- the null pointer value of the type,
- an invalid pointer value.

A pointer just past an object does not point to another object that
starts at that address ({rule}`ADD-10`).
:::

:::{rule} STG-16
:kind: required
:tests: foundations/storage/interconvertible-union-member.cpp foundations/storage/interconvertible-chain.cpp foundations/storage/interconvertible-standard-layout-base.cpp

Two objects are {term}`pointer-interconvertible` in these cases:

- They are the same object.
- One is a union, and the other is a member of it.
- One is an object of a standard-layout class, and the other is its
  first non-static data member or one of its base subobjects.
- A chain of these cases links them.

A `reinterpret_cast`, or a `static_cast` through `void*`, turns a
pointer to one into a pointer to the other ({rule}`CLS-19`). For other
objects at one address, such as an array and its first element, the
result points to the original object.
:::

## Undefined Behavior

:::{rule} STG-17
:kind: undefined
:tests: foundations/storage/placement-misaligned-sanitizer.cpp foundations/storage/placement-too-small-sanitizer.cpp

If placement `new` creates an object in storage that is smaller than
the type, or not aligned for the type ({rule}`TYP-2`), the behavior is
undefined.
:::

:::{rule} STG-18
:kind: undefined
:tests: foundations/storage/implicit-objects-no-set-works.cpp foundations/storage/allocated-storage-not-polymorphic-object-sanitizer.cpp

If no set of implicitly created objects gives the program defined
behavior (STG-8), the behavior is undefined. An example is a use of
storage as an object of a type that is not an implicit-lifetime type,
before a constructor creates the object.
:::

:::{rule} STG-19
:kind: undefined
:tests: foundations/storage/replace-other-type-old-name-sanitizer.cpp foundations/storage/replace-base-old-pointer.cpp foundations/storage/replace-base-ends-enclosing-sanitizer.cpp foundations/storage/reuse-other-union-member-constant.cpp

If a new object takes the storage of an old object and STG-9 does not
apply, the names, pointers, and references to the old object do not
refer to the new object. A use of them to reach the new object is
undefined. The launder operation *(library referenced)* gives a
pointer that reaches the new object.
:::

:::{rule} STG-20
:kind: undefined
:tests: foundations/storage/reuse-const-static-object-sanitizer.cpp foundations/storage/reuse-const-automatic-object.cpp

Some complete objects are declared `const` and have a storage duration
other than dynamic. If a program creates an object in the storage of
such an object, the behavior is undefined. The rule applies during and
after the lifetime of the `const` object ({question}`Q-0307`).
:::

:::{rule} STG-21
:kind: undefined
:tests: foundations/storage/scope-end-other-type-sanitizer.cpp

Some variables have a class type whose destructor is not trivial, and
static, thread, or automatic storage duration. If the storage of such
a variable holds no object of its type when the implicit destructor
call happens, the behavior is undefined. An exit from the block by an
exception counts.
:::

:::{rule} STG-22
:kind: undefined
:tests: foundations/storage/member-call-before-construction-sanitizer.cpp foundations/storage/downcast-after-end-sanitizer.cpp foundations/storage/member-call-inactive-member-constant.cpp foundations/storage/member-call-after-end-constant.cpp foundations/storage/allocated-storage-not-polymorphic-object-sanitizer.cpp

Before the lifetime of an object begins, or after it ends, these uses
of a pointer or glvalue that refers to its storage are undefined:

- a read or a write ({rule}`OBJ-2`),
- a member access, or a call of a non-static member function
  ({rule}`DTOR-7`),
- a conversion to a pointer or reference to a virtual base,
- a `static_cast` to a type other than `void*` or a pointer to a
  character type, such as a cast to a derived class,
- `dynamic_cast` and `typeid`,
- `delete`.
:::

:::{rule} STG-23
:kind: undefined
:tests: foundations/storage/access-unrelated-scalar.cpp foundations/storage/access-unrelated-class.cpp foundations/storage/access-signed-char.cpp

If a program reads or writes an object through a glvalue of a type
that STG-14 does not list, the behavior is undefined. `signed char` is
such a type ({question}`Q-0306`). So is a class with the same members
as the class of the object.
:::

## Implementation Requirements

:::{rule} STG-24
:kind: unspecified
:tests: foundations/storage/implicit-objects-follow-later-use.cpp

If several sets of implicitly created objects give the program defined
behavior (STG-8), the implementation creates one of the sets. The
choice is unspecified.
:::

:::{rule} STG-25
:kind: impl-defined
:tests: foundations/storage/base-bytes-shared-with-member.cpp

A base subobject, or a member declared `[[no_unique_address]]`, can
share its last bytes with other subobjects of the same object. The
implementation chooses when the bytes are shared. In the matrix, a
member of a derived class can occupy the padding at the end of a base
class whose constructor is user-provided.
:::

:::{rule} STG-26
:kind: impl-defined
:tests: foundations/storage/byte-order-little-endian.cpp

The order of the bytes in the object representation of a scalar is
implementation-defined. On the matrix targets, the least significant
byte of an integer comes first. Integers use two's complement
({rule}`TYP-4`).
:::

:::{rule} STG-27
:kind: impl-defined
:tests: foundations/storage/invalid-pointer-copy.cpp

When the storage of an object ends, each pointer to the object or past
it becomes an invalid pointer value. Indirection and deallocation
through such a value are undefined ({rule}`UN-7`, {rule}`NEW-5`). Other
uses, such as a copy, are implementation-defined, and on the matrix
targets the copy keeps the old address.
:::

## Examples

```cpp
struct Point { int x, y; Point(int a, int b) : x(a), y(b) {} };

alignas(Point) unsigned char buffer[sizeof(Point)];
Point* p = new (buffer) Point(1, 2);  // a Point, hosted by buffer
p->~Point();                          // the Point ends; buffer lives on
p->x;                                 // undefined: no Point exists
new (buffer) int(0);                  // an int in the same bytes

struct Pair { int first; double second; };
auto* q = static_cast<Pair*>(::operator new(sizeof(Pair)));
q->first = 1;                         // fine: a Pair exists from the call on

int count = 1;
new (&count) int(5);
count;                                // 5: the name reaches the new int

int n = 0;
*reinterpret_cast<unsigned*>(&n) = 1; // fine: the unsigned type of int
*reinterpret_cast<float*>(&n) = 1.0f; // undefined: float is not allowed

long v = 7;
auto* byte = reinterpret_cast<unsigned char*>(&v);
byte + sizeof v;                      // one past the last byte of v
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 56
of 59 tests. EDG runs no programs, so its results for the run tests say
only that the programs are valid.

- Constant evaluation: in C++23 mode no compiler in the matrix accepts
  placement `new`, or a cast from `void*` to an object pointer, during
  constant evaluation. Each compiler accepts a placement `new` inside a
  function template `std::construct_at` that the program declares, so
  the constant tests declare that function ({rule}`LSUP-10`).
- STG-1: Clang 18 accepts a class whose size does not fit in
  `std::size_t` and gives `sizeof` a reduced value. GCC accepts the
  class definition and rejects `sizeof` of the class. Clang 22 and EDG
  reject the class ({question}`Q-0308`). Every compiler rejects a
  smaller limit too, as {rule}`DCL-10` allows.
- STG-4: no compiler treats an array of `char` differently from a byte
  array ({question}`Q-0305`).
- STG-9: every compiler gives the new values through the old names at
  `-O0` and at `-O2`, also for a class with a `const` member, and
  accepts the constant tests.
- STG-13: no sanitizer in the matrix reports a step through the bytes
  of an object, and every configuration gives the expected bytes.
  Constant evaluation cannot form such a pointer ({rule}`CAST-8`). The
  rule is provisional ({question}`Q-0304`).
- STG-17: the undefined sanitizer of GCC and Clang reports the
  misaligned object, and the address sanitizer reports the object that
  is too large. GCC also warns about the second case when it compiles
  the program.
- STG-18, STG-23: the type sanitizer of Clang 22 (`-fsanitize=type`)
  reports a read of an `int` object through a `float` glvalue. The
  sanitizer does not report access through `signed char`, `std::byte`,
  or the unsigned type of the object. GCC and Clang 18 have no such
  sanitizer, so the tests do not use it.
- STG-19: the `vptr` sanitizer of GCC and Clang reports a call through
  the old name after a replacement by another type. At `-O2`, GCC
  calls the old function through a pointer to a replaced base
  subobject, and Clang calls the new one. STG-19 allows both results.
  During constant evaluation every compiler accepts a read through
  such a pointer ({question}`Q-0307`).
- STG-20: a `const` object with static storage duration is in storage
  that cannot be written, and the write faults. During constant
  evaluation no compiler rejects the new object. GCC 14 and Clang 18
  then read the new value, and GCC 16, Clang 22, and EDG read the old
  one ({question}`Q-0307`). A placement `new` with a pointer to `const`
  as its argument is rejected by {rule}`NEW-20`.
- STG-21, STG-22: the `vptr` sanitizer of GCC reports the destructor
  call at the end of the block and the cast to a derived class. The
  Clang sanitizer reports neither. Both report a member call on
  storage with no object. Every compiler rejects a member call outside
  a lifetime during constant evaluation.
- STG-23: at `-O2`, GCC and Clang keep the value of an `int` across a
  write through a `float` glvalue or through a class with the same
  members. Every compiler lets a write through `signed char` change an
  `int` ({question}`Q-0306`).
- STG-24: no program can observe the chosen set, so the test shows only
  that each use of the storage is valid.
- STG-25: GCC, Clang, and EDG give the derived class of the test the
  size of its base. The rule is implementation-defined because the
  layout of a class is the implementation's choice.
- STG-26: every target in the matrix stores the least significant byte
  first.
- STG-27: every configuration keeps the address in a copy of an invalid
  pointer value, and no sanitizer reports the copy.

Inputs: clang-diag-sema.
