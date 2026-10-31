# Unions and bit-fields

A union is a class whose non-static data members share storage. At most
one member is active at a time. A bit-field is a data member with a width
in bits. This page covers union declarations and their special members,
initialization and the active member, anonymous unions, and bit-field
declarations, values, and their limits.

Depends on: class definitions and members, constructors and destructors,
copy and move, list initialization and aggregates, standard conversions.

## Principles

:::{principle} UNI-P1
:rules: UNI-1 UNI-2 UNI-4 UNI-5 UNI-6 UNI-8
:exceptions: UNI-10 UNI-12

A union is one piece of storage with several views. All its members start
at its first byte, and at most one of them is an object at a time. The
initializer picks the first active member, and a plain assignment to
another member makes that one active instead. Reading a member that is
not active reads an object that is not there, which is undefined
({principle}`OBJ-P4`). A reference is not an object with storage of its
own, so a union cannot hold one.
:::

:::{principle} UNI-P2
:rules: UNI-3 UNI-11

The union does not know which member is active, so it cannot build,
copy, or destroy its members on its own. When a member would need that,
the implementation deletes the union's special member functions and
leaves the job to the union's own code ({principle}`CPY-P3`).
:::

:::{principle} UNI-P3
:rules: UNI-7 UNI-9

An anonymous union adds its members to the scope around it, as if the
scope held them, while they still share one storage. So it can only hold
plain public data members, and at namespace scope it needs `static`
because its members have no name to link by.
:::

:::{principle} UNI-P4
:rules: BF-1 BF-2 BF-3 BF-4 BF-5 BF-6 BF-7 BF-11

A bit-field is a member with a width in bits. The width sets the range
of values it holds and the way it promotes, while its declared type gives
its signedness and its treatment in initialization. Because it need not
start on a byte, a bit-field has no address, so a program cannot point
to it or bind a plain reference to it.
:::

:::{principle} UNI-P5
:rules: BF-8 BF-9 BF-10

How bit-fields are packed, and what a bit-field holds after a value too
wide for it is stored, are left to the implementation. An implementation
should warn when a width exceeds its type or a constant does not fit.
:::

## Syntax

```{eval-rst}
.. productionlist:: unions
   union_def: "union" [`name`] "{" {`member`} "}" ";"
   anonymous_union: ["static"] "union" "{" {`member_declaration`} "}" ";"
   bitfield_member: `type_specifiers` [`identifier`] ":" `constant_expression` [`brace_or_equal_init`]
```

The *active member* of a union is the member that holds a value. A
*variant member* is a non-static data member of a union, or of an
anonymous union in a class. An *anonymous union* has no name and declares
no variable.

## Constraints

:::{rule} UNI-1
:kind: required
:tests: classes/unions/alignment-strictest-not-largest.cpp classes/unions/member-addresses-equal.cpp classes/unions/shared-storage.cpp classes/unions/union-array-member.cpp classes/unions/union-member-functions.cpp classes/unions/union-static-member.cpp

Each data member of a union that is not static begins at the first byte
of the union. A union is at least as large as its largest member, and at
least as aligned as its most strictly aligned member. A union can have member functions, constructors, a
destructor, static data members, and array members. Its members are
public unless an access label says otherwise.
:::

:::{rule} UNI-2
:kind: reject
:tests: classes/unions/union-reference-member.cpp

A union must not have a member of reference type. Rules DER-1 and VIRT-2
give the limits on bases and virtual functions.
:::

:::{rule} UNI-3
:kind: required
:tests: classes/unions/union-default-member-init-enables-ctor.cpp classes/unions/union-nontrivial-member-user-ctor-dtor.cpp classes/unions/union-trivial-copy.cpp

If a variant member has a non-trivial default constructor, copy or move
function, or destructor, the matching special member of the union is
deleted. The same applies to a class with an anonymous union member. A
default member initializer on a variant member gives the union a usable
default constructor.
:::

:::{rule} UNI-11
:kind: reject
:tests: classes/unions/anonymous-union-nontrivial-default.cpp classes/unions/union-nontrivial-copy-deleted.cpp classes/unions/union-nontrivial-dtor-only.cpp classes/unions/union-nontrivial-member-deleted-ctor.cpp

A program must not use a special member function that UNI-3 deletes. The
union must then declare that function itself.
:::

:::{rule} UNI-4
:kind: required
:tests: classes/unions/aggregate-first-member.cpp classes/unions/default-member-init-active.cpp classes/unions/designated-member.cpp classes/unions/value-init-zero.cpp

A union is initialized as one of its members:

- A braced list without designators initializes the first member.
- A designator such as `{.f = 2.0f}` selects a member.
- A member with a default member initializer is used if there is no
  other initializer.
- Value initialization `{}` initializes the first member to zero, if no
  member has a default member initializer.

That member is then active.
:::

:::{rule} UNI-8
:kind: reject
:tests: classes/unions/mem-init-two-members.cpp classes/unions/narrowing-first-member.cpp classes/unions/two-default-member-inits.cpp classes/unions/two-designators.cpp classes/unions/two-initializers.cpp

Initialization of a union must name at most one member. A program must
not give:

- two initializers or two designators in a braced list,
- default member initializers to two variant members,
- `mem_init` entries for two variant members.

A braced initializer for the first member must not narrow.
:::

:::{rule} UNI-7
:kind: required
:tests: classes/unions/anonymous-union-default-member-init.cpp classes/unions/anonymous-union-init.cpp classes/unions/anonymous-union-local.cpp classes/unions/anonymous-union-member.cpp classes/unions/anonymous-union-namespace-static.cpp

The members of an anonymous union are members of the enclosing scope. In
a class, they are variant members of the class. A braced list
initializes an anonymous union member with its own braces. An anonymous
union can be local, a class member, or `static` at namespace scope.
:::

:::{rule} UNI-9
:kind: reject
:tests: classes/unions/anonymous-union-function.cpp classes/unions/anonymous-union-name-conflict.cpp classes/unions/anonymous-union-namespace-nonstatic.cpp classes/unions/anonymous-union-nested-type.cpp classes/unions/anonymous-union-private.cpp classes/unions/anonymous-union-static-member.cpp classes/unions/anonymous-union-unnamed-namespace.cpp

Outside a function or class, an anonymous union needs `static`, even in
an unnamed namespace ({question}`Q-0057`). Its members must all be public
data members that are not static. These are not valid in it:

- member functions,
- static data members,
- nested types,
- a member with a name that the enclosing scope has.
:::

:::{rule} BF-1
:kind: required
:tests: classes/unions/bitfield-basic.cpp classes/unions/bitfield-bool.cpp classes/unions/bitfield-char-types.cpp classes/unions/bitfield-constant-width.cpp classes/unions/bitfield-enum.cpp classes/unions/bitfield-in-union.cpp classes/unions/bitfield-zero-unnamed.cpp

A bit-field has an integral or enumeration type and a width. The width is
a constant expression. A bit-field can be unnamed. An unnamed bit-field
of width 0 makes the next bit-field start at a new allocation unit. A
union can have bit-fields.
:::

:::{rule} BF-2
:kind: reject
:tests: classes/unions/bitfield-float.cpp classes/unions/bitfield-negative-width.cpp classes/unions/bitfield-nonconstant-width.cpp classes/unions/bitfield-pointer.cpp classes/unions/bitfield-static.cpp classes/unions/bitfield-typedef.cpp classes/unions/bitfield-unnamed-cv.cpp classes/unions/bitfield-zero-named.cpp

A bit-field must not:

- have a type that is not integral or an enumeration,
- be `static` or a `typedef`,
- have a negative width or a width that is not a constant,
- have width 0 and a name.

An unnamed bit-field must not have a cv-qualifier ({question}`Q-0058`).
:::

:::{rule} BF-3
:kind: required
:tests: classes/unions/bitfield-bool-wide.cpp classes/unions/bitfield-very-wide.cpp classes/unions/bitfield-wider-than-type.cpp

A width can be larger than the number of bits in the type, including for
`bool`. The extra bits are padding. The value range stays that of the
type.
:::

:::{rule} BF-4
:kind: reject
:tests: classes/unions/bitfield-address.cpp classes/unions/bitfield-alignof-type-ok.cpp classes/unions/bitfield-const-ref-temp.cpp classes/unions/bitfield-nonconst-ref.cpp classes/unions/bitfield-sizeof.cpp

A program must not take the address of a bit-field, apply `sizeof` to
it, or bind a non-const reference to it. A `const` reference binds to a
temporary that holds a copy of the value.
:::

:::{rule} BF-6
:kind: required
:tests: classes/unions/bitfield-auto-type.cpp classes/unions/bitfield-promotes-to-int.cpp classes/unions/bitfield-promotes-unsigned-int.cpp classes/unions/bitfield-wide-unsigned-promotion.cpp

A bit-field promotes to `int` if `int` can hold all values of its width.
If not, it promotes to `unsigned int` if `unsigned int` can hold them.
If neither can, the promotion uses its declared type as other integers
do. The
type of the bit-field for `auto` deduction is the declared type.
:::

:::{rule} BF-7
:kind: required
:tests: classes/unions/bitfield-aggregate-skips-unnamed.cpp classes/unions/bitfield-constant-fits.cpp classes/unions/bitfield-constant-too-wide.cpp classes/unions/bitfield-init-default-member.cpp

A named bit-field can have a default member initializer, with `=` or
braces. Aggregate initialization skips unnamed bit-fields. The test for
narrowing uses the declared type, not the width.
:::

:::{rule} BF-11
:kind: reject
:tests: classes/unions/bitfield-narrowing-init.cpp

A braced initializer for a bit-field must not need a narrowing
conversion from the declared type of the bit-field.
:::

:::{rule} UNI-12
:kind: reject
:tests: classes/unions/common-initial-sequence-constant.cpp

Constant evaluation does not permit a read through the other struct that
UNI-10 describes.
:::

## Runtime Behavior

:::{rule} UNI-5
:kind: required
:tests: classes/unions/assign-changes-active.cpp classes/unions/nontrivial-member-needs-construction.cpp classes/unions/union-copy-preserves-active.cpp

An assignment to a member makes that member active if the assignment
uses the built-in operator or a trivial assignment operator
*(class properties referenced)*. A copy of a union copies its active
member. Constant
evaluation follows these rules.
:::

:::{rule} BF-5
:kind: required
:tests: classes/unions/bitfield-plain-int-signed.cpp

A bit-field of a signed type, including plain `int`, holds negative
values that fit in its width.
:::

:::{rule} UNI-10
:kind: required
:tests: classes/unions/common-initial-sequence.cpp

Two standard-layout ({rule}`CLS-19`) structs that are members of a union
can begin with members of the same types, a *common initial sequence*.
While one struct is active, a program can read a member of that sequence
through the other struct. The read gives the stored value.
:::

## Undefined Behavior

:::{rule} UNI-6
:kind: undefined
:tests: classes/unions/compound-assign-inactive-constant.cpp classes/unions/read-inactive-after-switch-constant.cpp classes/unions/read-inactive-constant.cpp classes/unions/user-assignment-not-activating-constant.cpp

If a program reads a union member that is not active, the behavior is
undefined. A compound assignment to a member that is not active reads
it. An assignment to a member that is not active, through an assignment
operator that is user-provided, uses an object that does not exist yet
({question}`Q-0088`).
:::

## Implementation Requirements

:::{rule} BF-9
:kind: impl-defined
:tests: classes/unions/bitfield-layout-lp64.cpp classes/unions/bitfield-zero-width-splits.cpp

The allocation of bit-fields in an object is implementation-defined. On
the x86-64 Linux targets in the matrix, adjacent bit-fields of the same
type share one unit of that type while they fit.
:::

:::{rule} BF-10
:kind: impl-defined
:tests: classes/unions/bitfield-unsigned-wraps.cpp classes/unions/bitfield-signed-wraps.cpp classes/unions/bitfield-constant-wraps.cpp

If a value stored in a bit-field of width `n` does not fit, the value
that the bit-field holds is implementation-defined. On the x86-64 targets
in the matrix, the value is reduced modulo 2^n, also during constant
evaluation.
:::

## Recommended Practice

:::{rule} BF-8
:kind: recommended
:tests: classes/unions/bitfield-constant-truncation-warns.cpp classes/unions/bitfield-fits-no-warning.cpp classes/unions/bitfield-wider-warns.cpp classes/unions/bitfield-assign-too-wide-warns.cpp

An implementation should warn if a bit-field width is larger than its
type, or if a constant assignment changes value because of the width.
:::

## Examples

```cpp
struct Token {
  enum class Kind : unsigned char { number, text } kind;
  union {                        // anonymous: members of Token
    long number;
    const char* text;
  };
};
Token t{Token::Kind::number, {42}};

union Bits { unsigned u; float f; };
Bits b{.f = 1.0f};               // f is active
unsigned x = b.u;                // undefined: u is not active

struct Pixel {
  unsigned red : 5;
  unsigned green : 6;
  unsigned blue : 5;             // one 32-bit unit on x86-64
};
Pixel p{31, 70, 0};              // 70 is reduced modulo 64
```

## Evidence

The results are in `tests/RESULTS.md`. All seven compilers agree on 84 of 87 tests.

- UNI-9: Clang accepts an anonymous union without `static` in an unnamed
  namespace. GCC and EDG reject it ({question}`Q-0057`).
- BF-2: GCC accepts a cv-qualifier on an unnamed bit-field. Clang and EDG
  reject it ({question}`Q-0058`).
- BF-8: Clang and GCC warn under `-Wall` for both cases.
- UNI-6: EDG accepts the assignment through a user-provided operator to
  a member that is not active during constant evaluation. Clang and GCC
  reject it ({question}`Q-0088`).

Inputs: clang-sema-decl, clang-sema-declcxx, clang-sema-init,
clang-diag-sema, edg-class-decl.
