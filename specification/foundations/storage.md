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
other, or if it is an empty {term}`subobject` of zero size. A type whose size
does not fit in `std::size_t` cannot exist.
:::

:::{principle} STG-P2
:rules: STG-4 STG-5 STG-6 STG-17 STG-29

Storage can outlive the objects in it. A byte array or the storage that
an allocation function returns holds one object after another. The
elements of a byte array have the type `unsigned char` or `std::byte`. An array of `char`
holds no object in this way ({rule}`STG-4`).

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
A program reaches the new object through the pointer that placement
`new` returns. A cast of the array's address still points to a byte.
:::

:::{principle} STG-P3
:rules: STG-7 STG-8 STG-18 STG-24 STG-28

Some types need no constructor call to exist. Scalars, arrays, and
simple classes are such
{term}`implicit-lifetime types <implicit-lifetime type>`. A program
can use fresh storage as objects of these types without placement
`new`, because some operations create the objects implicitly:

| Operation | Creates objects in |
|---|---|
| a call of an allocation function, by name or from `new` | the storage that the call returns |
| the start of a byte array's {term}`lifetime` | the array |
| some library functions *(library referenced)* | the storage that they name |

A cast of a byte array's own address does not reach the objects that
the array hosts ({rule}`STG-29`). The pointer that an allocation
function returns does reach them:

```cpp
int* a = static_cast<int*>(::operator new(sizeof(int)));
*a = 1;                         // an int that the call created
alignas(int) unsigned char bytes[sizeof(int)];
int* b = reinterpret_cast<int*>(bytes);
*b = 1;                         // undefined: b points to a byte
int* c = new (bytes) int(1);    // c points to the new int
```

The implementation chooses the objects that give the program defined
behavior, looking ahead to the program's later use of the storage. If
no choice works, the behavior is undefined. If several choices work,
the program cannot observe which one the implementation made.

An allocation for a growable array creates an array of the element
type. The elements of the array exist at once, and each element begins
its lifetime when the program constructs it. So the program can step
through the positions of the elements before it builds them:

```text
  T* items = static_cast<T*>(::operator new(4 * sizeof(T)));

  [ element 0 ][ element 1 ][ element 2 ][ element 3 ]   an array of 4 T
    built         built       not built    not built
    ^items                    ^items + 2
```
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

The pointer that placement `new` returns always reaches the new object
(STG-5), so a program that keeps that pointer needs nothing else. Where
the old names do not reach the new object, the launder operation
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
| a cast to a derived class or a {term}`virtual base`, `dynamic_cast`, `typeid` | undefined |
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
of any complete object, from the first byte to one past the last
({question}`Q-0304`). A byte pointer made from the address of a member
steps through the bytes of the whole object too. A padding byte has no
value until the program writes the byte. The
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
write an object whose {term}`dynamic type` is `T`:

| Glvalue type | Allowed |
|---|---|
| `T` with any {term}`cv-qualifiers <cv-qualifier>`, also inside a pointer type | yes |
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
| an invalid pointer value | an address whose storage has ended, or an address from an integer where no object lies | undefined ({rule}`UN-7`) |

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
:tests: foundations/storage/array-size-exceeds-size-type.cpp foundations/storage/class-size-exceeds-size-type.cpp foundations/storage/class-size-exceeds-size-type-ilp32.cpp

The size in bytes of a type must fit in `std::size_t` ({rule}`LSUP-4`).
The rule covers arrays ({rule}`DCL-19`) and classes.
:::

## Runtime Behavior

:::{rule} STG-2
:kind: required
:tests: foundations/storage/object-bytes-cover-members.cpp foundations/storage/array-bytes-consecutive.cpp

An object occupies `sizeof` bytes of storage, and the address of the
object is the address of its first byte. An object of a
{term}`trivially copyable` or {term}`standard-layout` type occupies
consecutive bytes ({rule}`CPY-28`, {rule}`CLS-29`).
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
:tests: foundations/storage/placement-new-result-is-address.cpp foundations/storage/keep-placement-new-pointer.cpp foundations/storage/cast-to-target-then-construct.cpp foundations/storage/placement-new-array.cpp foundations/storage/byte-array-hosts-objects.cpp foundations/storage/std-byte-array-hosts-objects.cpp foundations/language-support/construct-at-self-declared.cpp foundations/storage/allocation-array-class-elements.cpp foundations/storage/arena-reset-reuses-bytes.cpp foundations/storage/byte-array-placement-reaches-object.cpp

`new (p) T(args)` creates an object of type `T` at the address `p`, with
a {term}`non-allocating placement form` ({rule}`LSUP-7`). The result
points to the new object. The array form `new (p) T[n]` calls the
`operator new[]` form and creates an array at `p` ({rule}`NEW-11`).
:::

:::{rule} STG-6
:kind: required
:tests: foundations/storage/reuse-does-not-call-destructor.cpp foundations/storage/byte-array-hosts-objects.cpp foundations/storage/replace-base-ends-enclosing-sanitizer.cpp foundations/storage/arena-reset-reuses-bytes.cpp

If a new object takes storage that another object occupies, the
lifetime of the other object ends, and its destructor does not run.
An object that the new object is nested in keeps its lifetime, such as
the byte array that hosts the new object ({rule}`OBJ-10`). After the
new object ends, its bytes hold no object until the program creates
one there.
:::

:::{rule} STG-7
:kind: required
:tests: foundations/storage/implicit-lifetime-trivial-class.cpp foundations/storage/implicit-lifetime-aggregate.cpp foundations/storage/implicit-array-for-arithmetic.cpp foundations/storage/implicit-object-from-allocation-function.cpp foundations/storage/allocation-array-class-elements.cpp

These types are
{term}`implicit-lifetime types <implicit-lifetime type>`:

- {term}`scalar types <scalar type>`,
- array types, with any element type,
- an aggregate class whose destructor is not user-provided,
- a class with a trivial destructor that is not deleted, and with at
  least one trivial {term}`eligible` constructor,
- the cv-qualified forms of these types.
:::

:::{rule} STG-8
:kind: required
:tests: foundations/storage/implicit-object-from-allocation-function.cpp foundations/storage/implicit-array-for-arithmetic.cpp foundations/storage/implicit-object-class-allocation-function.cpp foundations/storage/implicit-objects-follow-later-use.cpp foundations/storage/implicit-object-first-use-member.cpp foundations/storage/allocation-array-class-elements.cpp foundations/storage/arena-reset-reuses-bytes.cpp foundations/storage/byte-array-placement-reaches-object.cpp

These operations create objects implicitly in the storage that they
name:

- a call of an allocation function, by name or from a `new` expression,
- the start of the lifetime of a byte array,
- some library functions *(library referenced)*.

The implementation creates the set of objects that gives the program
defined behavior, and the result of an allocation function points to
one of them. If a later use decides the set, a pointer from an earlier
use points to the matching object of the set. Each created object has an
implicit-lifetime type, and STG-28 gives the elements of other arrays.
:::

:::{rule} STG-9
:kind: required
:tests: foundations/storage/replace-variable-old-name.cpp foundations/storage/replace-object-with-const-member.cpp foundations/storage/replace-const-member-constant.cpp foundations/storage/replace-through-reference-constant.cpp foundations/storage/replace-array-element-constant.cpp foundations/storage/assignment-by-destroy-and-construct.cpp foundations/storage/replace-variable-then-scope-end.cpp foundations/storage/replace-member-of-base-constant.cpp

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
element becomes that member or element, if the enclosing object is in
its lifetime. {rule}`UNI-14` gives a union member that holds no object.
:::

:::{rule} STG-10
:kind: required
:tests: foundations/storage/outside-lifetime-allowed-uses.cpp foundations/storage/allocation-array-class-elements.cpp

Before the lifetime of an object begins, or after it ends, while its
storage exists, a program can do these things:

- copy or compare a pointer to the storage,
- step a pointer over the elements of an array that exists
  ({rule}`STG-28`),
- convert the pointer to `void*`, and then to a pointer to
  `std::byte`, `unsigned char`, or `char`,
- take the address of a glvalue that refers to the storage, or bind a
  reference of the object's type to it.
:::

:::{rule} STG-11
:kind: required
:tests: foundations/storage/equal-values-different-bytes.cpp foundations/storage/padding-bytes-not-in-value.cpp foundations/storage/byte-round-trip.cpp foundations/storage/base-bytes-shared-with-member.cpp

The {term}`object representation` of an object is its `sizeof` bytes,
viewed as `unsigned char`. The bits that decide the value are the
{term}`value representation`, and the other bits are padding. Objects
with equal values can have different object representations, and a
change to padding does not change the value. A padding byte has no
value ({rule}`INIT-1`) until the program writes the byte through a byte
pointer ({question}`Q-0328`).
:::

:::{rule} STG-12
:kind: required
:tests: foundations/storage/byte-round-trip.cpp foundations/storage/byte-round-trip-char-and-byte.cpp foundations/storage/padding-bytes-not-in-value.cpp classes/copy-move/trivially-copyable-types.cpp classes/copy-move/trivially-copyable-volatile-member.cpp

A program can copy the bytes of an object of a {term}`trivially
copyable` type ({rule}`CPY-28`) into an array of `std::byte`,
`unsigned char`, or `char`, and back. The object then has its original
value. A copy that reads each byte through a `char` glvalue needs a
value in each byte, padding bytes included ({rule}`INIT-1`,
{question}`Q-0328`). A copy by the library's copy functions
*(library referenced)* does not. STG-25 excludes a base subobject
and a `[[no_unique_address]]` member, and {rule}`CPY-19` gives a copy
into a second object. For an object with a volatile part, the copy
reads and writes the bytes through `volatile unsigned char` or
`volatile std::byte` glvalues ({rule}`VOL-6`, {question}`Q-0359`).
:::

:::{rule} STG-13
:kind: required
:tests: foundations/storage/byte-pointer-walks-object.cpp foundations/storage/object-bytes-cover-members.cpp foundations/storage/byte-round-trip.cpp foundations/storage/past-end-address.cpp foundations/storage/padding-bytes-not-in-value.cpp foundations/storage/byte-pointer-from-member-walks-object.cpp foundations/storage/base-bytes-shared-with-member.cpp

A pointer to `std::byte`, `unsigned char`, or `char` that points to a
byte of a complete object can move over the bytes of that object, also
when the pointer comes from the address of a subobject. For
{rule}`ADD-6`, {rule}`ADD-7`, and comparisons, the bytes act as an
array of `sizeof` elements ({question}`Q-0304`,
{question}`Q-0327`). A read through the
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
:tests: foundations/storage/past-end-address.cpp foundations/storage/integer-pointer-no-object-sanitizer.cpp

A value of a pointer type is one of these:

- a pointer to an object or a function, which represents the address
  of its first byte,
- a pointer just past an object, which represents the address of the
  byte after the object,
- the null pointer value of the type,
- an invalid pointer value, such as a pointer to storage that has ended
  (STG-27), or the result of {rule}`CAST-16` for an integer that is the
  address of no object or function and of no byte just past an object
  ({question}`Q-0325`).

A pointer just past an object does not point to another object that
starts at that address ({rule}`ADD-10`).
:::

:::{rule} STG-16
:kind: required
:tests: foundations/storage/interconvertible-union-member.cpp foundations/storage/interconvertible-chain.cpp foundations/storage/interconvertible-standard-layout-base.cpp foundations/storage/cast-to-target-then-construct.cpp

Two objects are {term}`pointer-interconvertible` in these cases:

- They are the same object.
- One is a union, and the other is a member of it.
- One is an object of a standard-layout class, and the other is its
  first non-static data member or one of its base subobjects.
- A chain of these cases links them.

A `reinterpret_cast`, or a `static_cast` through `void*`, turns a
pointer to one into a pointer to the other ({rule}`CLS-19`). For other
objects at one address, such as an array and its first element, the
result points to the original object. A program can copy that result and
convert it to `void*`, for example to give the address to placement
`new`.
:::

:::{rule} STG-28
:kind: required
:tests: foundations/storage/allocation-array-class-elements.cpp foundations/storage/allocation-array-class-elements-constant.cpp foundations/storage/allocation-array-unbuilt-element-constant.cpp foundations/storage/implicit-array-for-arithmetic.cpp

If an allocation function creates an array of `T`, a `T*` from its
result points to element 0 ({question}`Q-0323`).
The elements exist also when `T` is not an implicit-lifetime type, and
their lifetimes begin when the program constructs them. Pointer
arithmetic over the elements ({rule}`ADD-6`) is valid before then.
:::

:::{rule} STG-29
:kind: required
:tests: foundations/storage/byte-array-placement-reaches-object.cpp foundations/storage/byte-array-cast-reaches-byte.cpp foundations/storage/cast-to-target-then-construct.cpp

A cast of a pointer into a byte array gives a pointer to a byte
(STG-16). The result does not reach an object that the array hosts,
also an object that STG-8 created ({question}`Q-0324`).
A program reaches such an object through the pointer that placement
`new` returns, or through the launder operation *(library referenced)*.
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
:tests: foundations/storage/implicit-objects-no-set-sanitizer.cpp foundations/storage/allocated-storage-not-polymorphic-object-sanitizer.cpp

If no set of implicitly created objects gives the program defined
behavior (STG-8), the behavior is undefined. An example is a use of
storage as an object of a type that is not an implicit-lifetime type,
before a constructor creates the object.
:::

:::{rule} STG-19
:kind: undefined
:tests: foundations/storage/replace-other-type-old-name-sanitizer.cpp foundations/storage/replace-base-old-pointer.cpp foundations/storage/replace-base-ends-enclosing-sanitizer.cpp foundations/storage/reuse-other-union-member-constant.cpp foundations/storage/replace-base-old-pointer-constant.cpp

If a new object takes the storage of an old object and STG-9 does not
apply, the names, pointers, and references to the old object do not
refer to the new object. A use of them to reach the new object is
undefined. The launder operation *(library referenced)* gives a
pointer that reaches the new object.
:::

:::{rule} STG-20
:kind: undefined
:tests: foundations/storage/reuse-const-static-object-sanitizer.cpp foundations/storage/reuse-const-automatic-object.cpp foundations/storage/reuse-const-automatic-object-constant.cpp

Some complete objects are declared `const` and have a {term}`storage duration`
other than dynamic. If a program creates an object in the storage of
such an object, the behavior is undefined. The rule applies during and
after the lifetime of the `const` object ({question}`Q-0307`).
:::

:::{rule} STG-21
:kind: undefined
:tests: foundations/storage/scope-end-other-type-sanitizer.cpp foundations/storage/scope-end-other-type-exception.cpp

Some variables have a class type whose destructor is not trivial, and
static, thread, or {term}`automatic storage` duration. If the storage of such
a variable holds no object of its type when the implicit destructor
call happens, the behavior is undefined. An exit from the block by an
exception counts. {rule}`DTOR-7` also covers a variable whose object
has ended and whose storage holds no other object, as after an
explicit destructor call. DTOR-7 covers a destructor of any kind.
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
:tests: foundations/storage/access-unrelated-scalar-sanitizer.cpp foundations/storage/access-unrelated-class-sanitizer.cpp foundations/storage/access-signed-char.cpp foundations/storage/byte-array-cast-reaches-byte.cpp classes/unions/common-initial-sequence-enum.cpp classes/unions/common-initial-sequence-nested.cpp

If a program reads or writes an object through a glvalue of a type
that STG-14 does not list, the behavior is undefined. These cases
apply:

- `signed char` is not listed ({question}`Q-0306`).
- A class with the same members as the class of the object is not
  listed.
- A glvalue from a cast of a byte array's address refers to a byte
  (STG-29).
- A read of a member in the {term}`common initial sequence` that
  {rule}`UNI-10` allows is not such a read.
:::

## Implementation Requirements

:::{rule} STG-24
:kind: unspecified
:tests: foundations/storage/implicit-objects-follow-later-use.cpp foundations/storage/implicit-objects-array-new.cpp

If several sets of implicitly created objects give the program defined
behavior (STG-8), the implementation creates one of the sets. The
choice is unspecified.
:::

:::{rule} STG-25
:kind: unspecified
:tests: foundations/storage/base-bytes-shared-with-member.cpp foundations/storage/no-unique-address-member-shares-bytes.cpp

A base subobject, or a member declared `[[no_unique_address]]`, can
share its last bytes with other subobjects of the same object. When
the bytes are shared is unspecified ({rule}`CLS-34`), and {rule}`CLS-35`
gives the conditions of the matrix targets. If a byte copy into the
base or member writes the bytes of another subobject, that subobject
takes the value that its new bytes give. A padding byte of the source
gives it no value ({rule}`STG-11`, {question}`Q-0357`).
:::

:::{rule} STG-26
:kind: impl-defined
:tests: foundations/storage/byte-order-little-endian.cpp foundations/storage/pointer-bytes-are-address.cpp

The object representation of a scalar is implementation-defined. On the
matrix targets, the table gives the bytes from first to last:

| Type | Bytes |
|---|---|
| an integer | the two's complement value ({rule}`TYP-4`), least significant byte first |
| a pointer | the integer that {rule}`CAST-16` gives for the pointer, in the order of an integer |
:::

:::{rule} STG-27
:kind: impl-defined
:tests: foundations/storage/invalid-pointer-copy.cpp foundations/storage/invalid-pointer-copy-automatic.cpp

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

The set of implicitly created objects depends on every use of the
storage, later ones included (STG-8):

```cpp
void* raw = ::operator new(4 * sizeof(int));
int* one = static_cast<int*>(raw);
*one = 1;                             // fine: the set holds an int[4]
one[3] = 4;                           // fine: the same int[4]

struct Duo { int first, second; };
void* place = ::operator new(sizeof(Duo));
*static_cast<int*>(place) = 1;        // the member first of a Duo
static_cast<Duo*>(place)->second = 2; // fine: the set holds a Duo
```

In the second part, an array of `int` would make the use of `Duo`
undefined. So the set holds a `Duo`, though the first use names `int`.

A class that builds an object in its own bytes keeps the pointer that
placement `new` returns, and reaches the object only through that
pointer (STG-5):

```cpp
template <class T>
class Maybe {
  alignas(T) unsigned char storage[sizeof(T)];
  T* held = nullptr;                  // null, or the result of placement new
public:
  template <class... Args>
  void emplace(Args&&... args) {
    reset();
    held = new (storage) T(static_cast<Args&&>(args)...);
  }
  void reset() { if (held) { held->~T(); held = nullptr; } }
  T* get() const { return held; }     // reaches the T
  T* bytes() { return reinterpret_cast<T*>(storage); }  // points to the
  ~Maybe() { reset(); }                                   // bytes (STG-16)
};
```

The implicit copy of `Maybe` copies `held`, which then points into the
bytes of the other object. So `Maybe` must write its own copy and move
functions ({rule}`CPY-8`).

A growable array of a class type gets raw storage from the allocation
function and builds each element in place. `items + count` is valid
before the element exists (STG-28):

```cpp
inline void* operator new(decltype(sizeof 0), void* at) noexcept { return at; }

template <class T>
class Stack {                          // T needs only a copy constructor
  T* items = nullptr;                  // element 0 of the allocated array
  int count = 0;
  int capacity = 0;
public:
  Stack() = default;
  Stack(const Stack&) = delete;        // one owner for the storage (CPY-P2)
  Stack& operator=(const Stack&) = delete;
  void push(const T& value) {
    if (count == capacity) grow();
    new (items + count) T(value);      // element count begins its lifetime
    ++count;
  }
  void pop() { items[--count].~T(); }  // the element ends; the storage stays
  ~Stack() {
    while (count > 0) pop();
    ::operator delete(items);          // a null pointer does nothing
  }
private:
  void grow() {
    int bigger = capacity == 0 ? 4 : 2 * capacity;
    void* raw = ::operator new(sizeof(T) * bigger);
    T* fresh = static_cast<T*>(raw);   // element 0 of an array of T
    for (int i = 0; i != count; ++i) {
      new (fresh + i) T(items[i]);
      items[i].~T();
    }
    ::operator delete(items);
    items = fresh;
    capacity = bigger;
  }
};
```

`grow` loses the new storage if a copy throws. The unwinding page shows
a growth step that a throw leaves unchanged
({doc}`/exceptions/unwinding`).

An arena hands out the bytes of one byte array and takes them back all
at once. A cast of the array's address points to a byte (STG-29), and
after a reset the bytes hold no objects (STG-6). So the arena creates
each object with placement `new` and returns that pointer:

```cpp
class Arena {
  alignas(16) unsigned char bytes[256];
  decltype(sizeof 0) used = 0;
public:
  template <class T> T* make_array(int n) {  // needs a default constructor (NEW-11)
    used = (used + alignof(T) - 1) & ~(alignof(T) - 1);  // round up
    T* first = new (bytes + used) T[n]();    // needs operator new[] (LSUP-7)
    used += sizeof(T) * n;
    return first;
  }
  void reset() { used = 0; }           // the caller has ended each object
};

template <class T> void destroy_all(T* first, int n) {
  while (n > 0) first[--n].~T();       // valid for a scalar T too
}
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 78
of 84 tests. EDG runs no programs, so its results for the run tests say
only that the programs are valid.

- Constant evaluation: in C++23 mode no compiler in the matrix accepts
  placement `new`, or a cast from `void*` to an object pointer, during
  constant evaluation. Each compiler accepts a placement `new` inside a
  function template `std::construct_at` that the program declares, so
  the constant tests declare that function ({rule}`LSUP-10`).
- STG-1: the table gives each compiler's result for a type whose size
  does not fit in `std::size_t`, for the definition alone and for
  `sizeof` of the type ({question}`Q-0308`, {question}`Q-0350`). A
  reduced value is the size modulo the range of `std::size_t`. GCC 16
  and the x86-64 build of EDG skip the `-m32` test. Every compiler
  rejects a smaller limit too, as {rule}`TR-14` and {rule}`TR-15` allow.

  | Test | Form | Clang 18 | Clang 22 | GCC | EDG |
  |---|---|---|---|---|---|
  | `class-size-exceeds-size-type.cpp` | class definition | accept | accept | accept | reject |
  | | `sizeof` | reduced value | reject | reject | reject |
  | `class-size-exceeds-size-type-ilp32.cpp` (`-m32`) | class definition | accept | accept | accept | reject |
  | | `sizeof` | reduced value | reduced value | reject | reject |
  | `array-size-exceeds-size-type.cpp` | array definition | reject | reject | reject | reject |
- STG-4: no compiler treats an array of `char` differently from a byte
  array ({question}`Q-0305`).
- STG-28: Clang and GCC, at `-O0`, at `-O2`, and with the address and
  undefined sanitizers, run a program that steps over the elements of
  an allocated array of a class type before it builds them. During
  constant evaluation, an `std::allocator` that the program declares
  ({rule}`LSUP-11`) gives such an array. Clang, GCC, and EDG accept
  placement `new` at `p + 3` before element 3 exists, and reject a read
  of an element that the program has not built
  ({question}`Q-0323`).
- STG-29: every configuration runs a program that writes through a
  cast of a byte array's address. No sanitizer reports the write, and
  constant evaluation cannot form the cast ({rule}`CAST-8`). The rule
  follows STG-16 and is provisional
  ({question}`Q-0324`).
- STG-9: every compiler gives the new values through the old names at
  `-O0` and at `-O2`, also for a class with a `const` member, and
  accepts the constant tests. During constant evaluation, every
  compiler also accepts a new object in the place of a member of a base
  subobject and reads the new value through the old name
  (`replace-member-of-base-constant.cpp`). The gap review of the object
  model reads the C++23 text as asking that the enclosing object be
  replaceable too, which a base subobject is not. STG-9 follows the
  compilers (`documentation/decisions/pending.md`, row 4). A new
  object in a `mutable` member of a `const` complete object falls under
  STG-20.
- STG-13: no sanitizer in the matrix reports a step through the bytes
  of an object, and every configuration gives the expected bytes.
  Constant evaluation cannot form such a pointer ({rule}`CAST-8`). The
  rule is provisional ({question}`Q-0304`).
- STG-17: the undefined sanitizer of GCC and Clang reports the
  misaligned object, and the address sanitizer reports the object that
  is too large. GCC also warns about the second case when it compiles
  the program.
- STG-18, STG-23: the type sanitizer of Clang 22 (`-fsanitize=type`)
  reports a read of an `int` object through a `float` glvalue, a write
  of `float` to an `int` object, and a write through a class with the
  same members. The program goes on after the report and exits 0, and
  no option of the sanitizer makes it stop. The test runner counts the
  report as the run failure, so the sanitizer tests expect `run-fail`
  on Clang 22. GCC and Clang 18 have no type sanitizer and skip these
  tests. A build of the same programs at `-O2` reports the two writes
  but not the read.
  The sanitizer does not report access through `signed char`,
  `std::byte`, or the unsigned type of the object, so
  `access-signed-char.cpp` and `byte-array-cast-reaches-byte.cpp` check
  only that the programs compile.
- STG-19: the `vptr` sanitizer of GCC and Clang reports a call through
  the old name after a replacement by another type. No sanitizer
  reports a call through a pointer to a replaced base subobject, so
  `replace-base-old-pointer.cpp` checks only that the program compiles.
  During constant evaluation every compiler accepts a read through
  such a pointer and gives the new value
  (`replace-base-old-pointer-constant.cpp`, {question}`Q-0307`).
- STG-20: a `const` object with static storage duration is in storage
  that cannot be written, and the write faults. No sanitizer reports a
  new object in a `const` automatic object, so
  `reuse-const-automatic-object.cpp` checks only that the program
  compiles. During constant evaluation Clang 22 rejects the new object
  as a change to a `const` object
  (`reuse-const-automatic-object-constant.cpp`). GCC 14 and Clang 18
  accept and then read the new value. GCC 16 and EDG accept and read
  the old value. Q-0307 stays open under decision 0051, because on
  2026-11-03 no family rejects the read through a base pointer, and the
  families split on the `const` case. A placement `new` with a pointer to `const`
  as its argument is rejected by {rule}`NEW-20`.
- STG-18 to STG-22: the table gives the result of each test that uses
  the `vptr` sanitizer. "Report" means that the sanitizer stops the run,
  so the test is `run-fail`. "Exit 0" means that the run ends normally.
  EDG skips these tests. Every compiler rejects a member call outside a
  lifetime during constant evaluation.

  | Test | Operation | Clang | GCC |
  |---|---|---|---|
  | `scope-end-other-type-sanitizer.cpp` | destructor call at the end of the block, on an object of another type | exit 0 | report |
  | `downcast-after-end-sanitizer.cpp` | cast to a derived class after the lifetime ends | exit 0 | report |
  | `replace-base-ends-enclosing-sanitizer.cpp` | member call through the name of an ended `Derived` whose storage now holds a `Base` | report | report |
  | `replace-other-type-old-name-sanitizer.cpp` | member call through the old name after a replacement by another type | report | report |
  | `member-call-before-construction-sanitizer.cpp` | member call on storage with no object | report | report |
  | `allocated-storage-not-polymorphic-object-sanitizer.cpp` | member call on allocated storage, which holds no object of a class with virtual functions | report | report |
- STG-23: `signed char` is outside the list of STG-14
  ({question}`Q-0306`). Constant evaluation cannot form the glvalues
  of the tests ({rule}`CAST-8`).
- STG-24: no program can observe the chosen set, so the test shows only
  that each use of the storage is valid.
- STG-25: GCC, Clang, and EDG give the derived class of the test the
  size of its base. The rule is unspecified, as CLS-34 classifies the
  use of the end padding of a base, because an implementation need not
  document the choice. The tests of {rule}`CLS-35` check each condition
  under which a base or a marked member shares its end padding.
- STG-26: every target in the matrix stores the least significant byte
  first. The bytes of a pointer equal the bytes of the integer that
  `reinterpret_cast` gives.
- STG-25, STG-26, STG-27 with `-m32`: the matrix runs no program built
  with `-m32`, because the host compilers have no 32-bit C library. In
  the writer's probes, the run tests of the three rules give the same
  results with `-m32` in the GCC 14, Clang 18, and Clang 22 containers.
- STG-11, STG-12: no configuration in the matrix shows whether a padding
  byte has a value. The rules follow INIT-1 and are provisional
  ({question}`Q-0328`). A copy by a library function reads no byte
  through a `char` glvalue, so STG-12 sets no such condition on it.
- STG-13: every configuration steps a byte pointer made from a member's
  address over the bytes of the whole object, and orders it against a
  byte pointer made from the object's address
  ({question}`Q-0327`).
- STG-15: the address sanitizer stops a read through a pointer that a
  cast gives from an integer with no object at that address
  ({question}`Q-0325`).
- STG-8: a pointer from the first use of the storage stays usable after
  a later use decides the set, on every configuration and with the
  address and undefined sanitizers.
- STG-27: every configuration keeps the address in a copy of an invalid
  pointer value, and no sanitizer reports the copy.
- Settled on 2026-11-03 under decision 0054. A source-reader gap
  review supports each answer with no doubt, and a principle gives the
  same answer:

  | Question | Principle | Family that still differs |
  |---|---|---|
  | {question}`Q-0305` | STG-P2 | none |
  | {question}`Q-0306` | STG-P7 | none |
  | {question}`Q-0324` | STG-P2 | none |

### Split tests

The table lists each test on which the compilers differ. The
Expected column gives the outcome that the rules predict. The other
cells give the outcome of the test on each compiler. EDG builds no
program, so a run test is only compiled there, and EDG skips the
sanitizer tests.

| Test | Rules | Expected | Clang | GCC | EDG | Question |
|---|---|---|---|---|---|---|
| `class-size-exceeds-size-type-ilp32.cpp` | STG-1 | reject | accept | reject | reject | {question}`Q-0350` |
| `class-size-exceeds-size-type.cpp` | STG-1 | reject | Clang host, Clang 22: reject; Clang 18: accept | reject | reject | {question}`Q-0308` |
| `downcast-after-end-sanitizer.cpp` | STG-22 | run-fail | run | run-fail | accept (no run) |  |
| `replace-base-old-pointer-constant.cpp` | STG-19 | reject | accept | accept | accept | {question}`Q-0307` |
| `reuse-const-automatic-object-constant.cpp` | STG-20 | reject | Clang host, Clang 22: reject; Clang 18: accept | accept | accept | {question}`Q-0307` |
| `scope-end-other-type-sanitizer.cpp` | STG-21 | run-fail | run | run-fail | accept (no run) |  |

Inputs: clang-diag-sema.
