# Volatile

A `volatile` qualifier marks an object whose accesses the program must
perform as written, for example a device register. This page covers:

- what `volatile` means for accesses,
- volatile member functions,
- references and pointers to volatile objects,
- copies of volatile class objects,
- the operations on volatile objects that implementations warn about.

Depends on: declarators and cv-qualifiers, reference binding, standard
conversions, copy and move.

## Principles

:::{principle} VOL-P1
:rules: VOL-1 VOL-8 VOL-6 VOL-12 VOL-13

`volatile` marks an access that the program can observe, so each read
and each write through a volatile glvalue is performed as written. The
object itself need not be volatile. A volatile object promises that
every access is observable, so an access to a volatile object through a
glvalue that is not volatile is undefined. The value read is an
ordinary value, so arithmetic on the value is not volatile. {term}`Constant evaluation <constant evaluation>`
cannot read a volatile object, because the read is an
observable effect.

| Object | Glvalue | Access |
|---|---|---|
| volatile | volatile | observable (VOL-1) |
| not volatile | volatile | observable (VOL-1) |
| not volatile | not volatile | ordinary |
| volatile | not volatile | undefined (VOL-6) |
:::

:::{principle} VOL-P2
:rules: VOL-2 VOL-9 VOL-3 VOL-10 VOL-4 VOL-11 VOL-7

`volatile` is a qualifier, and the qualifier follows the rules that
`const` follows ({principle}`DCL-P4`). A conversion can add `volatile`
and cannot remove it, so a volatile object is reached only through
volatile references, pointers, and member functions. The implicit copy
functions take a `const` reference, which does not bind to a volatile
object, so a class that wants to be copied while volatile declares
functions that say so. A constructor and a destructor are never
volatile, because the object is volatile only once it exists.
:::

:::{principle} VOL-P3
:rules: VOL-5

Some uses of a volatile object are deprecated, and an implementation
should warn about these uses.
:::

## Syntax

`volatile` is a {term}`cv-qualifier`. It can follow a member function's
parameter list, as `const` can.

## Constraints

:::{rule} VOL-2
:kind: required
:tests: declarations/volatile/overload-on-volatile.cpp declarations/volatile/static-const-volatile-in-class.cpp declarations/volatile/volatile-member-function.cpp declarations/volatile/volatile-member-of-volatile-object.cpp declarations/volatile/volatile-member-on-plain.cpp declarations/volatile/volatile-object-ordinary-ctor.cpp

These rules apply to `volatile` and classes:

- A member function declared `volatile` can be called on a volatile
  object and on other objects.
- Two member functions can overload on `volatile`.
- A member of a volatile object is volatile.
- A `static const volatile` integer member can have an initializer in
  its class ({question}`Q-0080`).
- A volatile object of class type is initialized by an ordinary
  constructor, and it is volatile after the constructor ends.
:::

:::{rule} VOL-9
:kind: reject
:tests: declarations/volatile/nonvolatile-member-on-volatile.cpp declarations/volatile/const-member-on-volatile.cpp

A member function that is not `volatile` (VOL-2) cannot be called on a
volatile object.
:::

:::{rule} VOL-3
:kind: required
:tests: declarations/volatile/const-cast-removes.cpp declarations/volatile/overload-param-volatile-ref.cpp declarations/volatile/pointer-add-volatile.cpp declarations/volatile/ref-volatile-to-plain.cpp

A conversion can add `volatile` to the target of a pointer or reference.
`const_cast` can remove `volatile`. Overload resolution can choose
between `T&` and `volatile T&`. VOL-10 lists what is not valid.
:::

:::{rule} VOL-10
:kind: reject
:tests: declarations/volatile/const-ref-volatile-no-temporary.cpp declarations/volatile/pointer-remove-volatile.cpp declarations/volatile/ref-plain-to-volatile.cpp

A conversion other than `const_cast` cannot remove `volatile` from the
target of a pointer or reference (VOL-3). So a reference that is not
`volatile` does not bind to a volatile object, and a `const volatile`
reference does not bind to a temporary.
:::

:::{rule} VOL-4
:kind: required
:tests: declarations/volatile/copy-volatile-scalar-ok.cpp declarations/volatile/volatile-class-user-copy.cpp

A volatile object of {term}`scalar type` can be copied. A volatile class object
can be copied or assigned with functions that the class declares for
volatile operands. VOL-11 lists what is not valid.
:::

:::{rule} VOL-11
:kind: reject
:tests: declarations/volatile/assign-to-volatile-class.cpp declarations/volatile/copy-volatile-class-rejected.cpp

The implicit copy functions of a class take `const T&`, which does not
bind to a volatile object. The implicit assignment operators are not
`volatile`, so they cannot assign to a volatile object (VOL-9). A
volatile class object can only be the source or the target of a copy
through functions that the class declares for volatile operands (VOL-4).
:::

:::{rule} VOL-7
:kind: reject
:tests: declarations/volatile/volatile-constructor.cpp declarations/volatile/volatile-destructor.cpp

A constructor or a destructor must not be declared `volatile`.
:::

:::{rule} VOL-8
:kind: reject
:tests: declarations/volatile/constant-read-rejected.cpp declarations/volatile/constant-read-through-plain.cpp

Constant evaluation cannot read a volatile object (VOL-1).
:::

## Runtime Behavior

:::{rule} VOL-1
:kind: required
:tests: declarations/volatile/access-run.cpp declarations/volatile/volatile-in-type-traits.cpp declarations/volatile/volatile-local-loop.cpp declarations/volatile/const-volatile-external.cpp declarations/volatile/volatile-path-to-plain-object.cpp

Each read and each write of an object through a volatile lvalue is an
observable effect ({rule}`TR-7`) that the implementation performs, also
when the object is not volatile. The type of a volatile variable keeps
`volatile`, but the result of an arithmetic operation on its value does
not. A `const
volatile` variable at namespace scope has external {term}`linkage`, so all
{term}`translation units <translation unit>` share the variable.
:::

:::{rule} VOL-12
:kind: required
:tests: declarations/volatile/initialization-write.cpp declarations/volatile/initialization-write-member-and-array.cpp

An initializer that gives a volatile object its value at run time writes
the object. The write is an observable effect ({rule}`TR-7`).
Zero-initialization
({rule}`STO-3`) and constant initialization ({rule}`SINIT-1`) happen
before the program runs and are not accesses ({question}`Q-0176`).
:::

## Undefined Behavior

:::{rule} VOL-6
:kind: undefined
:tests: declarations/volatile/nonvolatile-access-to-volatile-constant.cpp declarations/volatile/constant-read-through-plain.cpp

If a program accesses a volatile object through a glvalue that is not
volatile, the behavior is undefined.
:::

## Implementation Requirements

:::{rule} VOL-13
:kind: impl-defined
:tests: declarations/volatile/bitfield-write-keeps-neighbors.cpp declarations/volatile/bitfield-write-short-unit.cpp

The size of the memory access for a volatile read or write is
implementation-defined ({question}`Q-0326`). On the
x86-64 targets in the matrix, these sizes apply:

| Access | Memory access |
|---|---|
| a read or write of a volatile scalar | one access of the size of the scalar |
| a write to a volatile bit-field | a read and a write of the whole unit that holds the bit-field ({rule}`BF-9`) |
:::

## Recommended Practice

:::{rule} VOL-5
:kind: recommended
:tests: declarations/volatile/assignment-result-warns.cpp declarations/volatile/compound-no-warning.cpp declarations/volatile/increment-warns.cpp declarations/volatile/volatile-structured-binding.cpp declarations/volatile/volatile-parameter-warns.cpp declarations/volatile/volatile-return-warns.cpp declarations/volatile/pointer-to-volatile-parameter-ok.cpp

An implementation should warn about these uses, which are deprecated:

- `++` and `--` on a volatile object,
- the value of an assignment to a volatile object, used in a larger
  expression,
- a structured binding declaration with `volatile`.

Compound assignment, such as `v += 1` and `v |= 1`, is not deprecated.
CV-9 gives the warnings for volatile parameter and return types.
:::

## Examples

```cpp
struct Uart {
  unsigned status;
  unsigned data;
  unsigned read() volatile { return data; }
};
using address = decltype(sizeof 0);
auto* const port =                     // the address from the data sheet
    reinterpret_cast<volatile Uart*>(address{0x4000'1000});

void send(volatile Uart& u, unsigned c) {
  while ((u.status & 1) == 0) {}       // each test reads status again
  u.data = c;                          // one write
  u.status |= 2;                       // valid, not deprecated
}
```

A program that simulates a device in its own memory creates the
registers with placement `new` in a byte array and keeps that pointer
({rule}`STG-29`). A cast of the array's address would point to a byte:

```cpp
inline void* operator new[](decltype(sizeof 0), void* at) noexcept { return at; }

alignas(Uart) unsigned char device[2 * sizeof(Uart)];
volatile Uart* bank = new (device) Uart[2]();  // two Uart objects
bank[1].data = 7;                              // an observable write

struct Control { unsigned enable : 1; unsigned mode : 3; };
volatile Control* control = nullptr;           // set to the device address
// control->mode = 5 reads and writes the 32-bit unit (VOL-13)
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 38
of 40 tests.

- VOL-1: the tests check that programs with volatile accesses compile
  and run. A test cannot observe if an access was removed.
- VOL-12: at `-O2`, Clang 22 and GCC 16 keep the store of an initializer
  to a volatile local that the program never reads. The store of a
  dynamic initializer of a volatile namespace-scope variable is also
  kept. A constant or zero initial value is in the program's data, with
  no store. An initializer of a volatile class object can be written
  with one wide store for several members ({question}`Q-0176`).
- VOL-1: Clang and GCC run a program that writes a plain object through
  a pointer to volatile and reads it through its name.
- VOL-13: at `-O2`, Clang 22 and GCC 16 write a volatile `unsigned
  short` member with one 2-byte store. For a write to a volatile
  bit-field, both load the 4-byte unit, change the bits, and store the
  4-byte unit. The harness cannot observe the size of an access, so the
  test checks only that the other bit-fields keep their values. Whether
  an implementation can merge or split volatile accesses is open
  ({question}`Q-0326`).
- VOL-2: Clang rejects an in-class initializer for a `static const
  volatile` member. GCC and EDG accept the initializer
  ({question}`Q-0080`).
- VOL-5: Clang and GCC warn under `-Wall` for each listed use, and not
  for compound assignment.
- VOL-6: no sanitizer of GCC or Clang reports a read of a volatile
  object through an `int*`, so `nonvolatile-access-to-volatile-constant.cpp`
  checks only that the program compiles. During constant evaluation Clang rejects the read,
  and GCC and EDG accept it ({question}`Q-0320`). Q-0320 stays open
  under decision 0051, because on 2026-11-03 Clang 22 still differs from
  GCC 16 and EDG 7.0.

Inputs: clang-sema-expr, clang-sema-decl, clang-diag-sema, edg-decls.
