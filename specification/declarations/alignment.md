# Alignment

The alignment of a type is the number of bytes that its address is a
multiple of. `alignas` makes the alignment of a variable, a data member,
or a class stricter. This page covers:

- where `alignas` can occur,
- the values it accepts,
- how it affects size and layout,
- extended alignment.

`alignof` is with the unary operators.

Depends on: declarators, class definitions and members, unary operators,
`new` and `delete`.

## Principles

:::{principle} ALN-P1
:rules: ALN-1 ALN-7 ALN-2 ALN-8 ALN-4

Every type has an alignment, and every object of the type sits at an
address that is a multiple of the alignment, in every kind of storage.
`alignas` can make the alignment stricter and never weaker, because the
natural alignment is the type's own need. A power of two is the only
shape an alignment can have, and the strictest requirement wins. A class
is as aligned as its most demanding part, and its size is a multiple of
its alignment so that an array of the class keeps every element aligned.
:::

:::{principle} ALN-P2
:rules: ALN-3 ALN-6

Alignment is a property of the entity, so every declaration of the
entity agrees, within a unit and across units.
:::

:::{principle} ALN-P3
:rules: ALN-5

The alignments of the fundamental types and the largest alignment that
can be asked for depend on the target.
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   alignment_specifier: "alignas" "(" `constant_expr` ["..."] ")"
                      : | "alignas" "(" `type_id` ["..."] ")"
```

An `alignment_specifier` with `...` expands a pack ({rule}`PACK-16`).

An `alignment_specifier` occurs before a variable or member declaration,
or after the class key of a class definition, as in
`struct alignas(16) S { ... };`.

## Constraints

:::{rule} ALN-1
:kind: required
:tests: declarations/alignment/alignof-variable-type.cpp declarations/alignment/array-of-aligned.cpp declarations/alignment/class.cpp declarations/alignment/member-raises-class.cpp declarations/alignment/member.cpp declarations/alignment/strictest-wins.cpp declarations/alignment/type-operand.cpp declarations/alignment/variable.cpp declarations/alignment/zero-ignored.cpp declarations/alignment/class-forward-alignas.cpp

`alignas` applies to these declarations:

- a variable declaration,
- a declaration of a non-static data member,
- a class declaration, including a forward declaration.

`alignas(T)` means `alignas(alignof(T))`, and `alignas(0)` has no
effect. If a declaration has several `alignas`, the strictest applies
({question}`Q-0071`).

The alignment of a class is the strictest of its own `alignas` and of
its members. The size of a class is a multiple of its alignment, so an
array of the class keeps each element aligned. `alignas` on a variable
does not change the type of the variable.
:::

:::{rule} ALN-7
:kind: reject
:tests: declarations/alignment/enum.cpp

`alignas` applies only to the declarations that ALN-1 lists. A program
must not apply `alignas` to an enumeration ({question}`Q-0073`).
:::

:::{rule} ALN-2
:kind: reject
:tests: declarations/alignment/negative.cpp declarations/alignment/not-constant.cpp declarations/alignment/not-power-of-two.cpp declarations/alignment/on-alias-declaration.cpp declarations/alignment/on-bitfield.cpp declarations/alignment/on-function.cpp declarations/alignment/on-parameter.cpp declarations/alignment/on-typedef.cpp declarations/alignment/weaker-on-class-member.cpp declarations/alignment/weaker-than-natural.cpp

The operand of `alignas` must be a constant expression whose value is
zero or a positive power of two. {rule}`ALN-8` gives its type. The
result must not be weaker than the natural alignment of the entity
({question}`Q-0069`). `alignas` must not apply to any of these
({question}`Q-0072`):

- a function,
- a parameter,
- a bit-field,
- a type alias.
:::

:::{rule} ALN-8
:kind: reject
:tests: declarations/alignment/operand-floating.cpp declarations/alignment/operand-scoped-enumeration.cpp declarations/alignment/operand-unscoped-enumeration.cpp declarations/alignment/operand-class-converts.cpp

The operand of `alignas` must have integral or unscoped enumeration
type, or a class type that converts to such a type ({question}`Q-0313`,
{question}`Q-0314`).
:::

:::{rule} ALN-3
:kind: reject
:tests: declarations/alignment/definition-without-earlier-with.cpp declarations/alignment/redeclare-different.cpp declarations/alignment/redeclare-without.cpp declarations/alignment/class-redeclare-different.cpp declarations/alignment/class-definition-without-earlier-with.cpp

If a declaration of a variable or a class has `alignas`, its definition
must have the same alignment ({question}`Q-0070`). A definition can add
`alignas` if the earlier declarations have none.
:::

## Runtime Behavior

:::{rule} ALN-4
:kind: required
:tests: declarations/alignment/extended-new.cpp declarations/alignment/extended-supported.cpp

An object has the alignment of its declaration or type. This rule
applies to objects in:

- static storage,
- automatic storage,
- dynamic storage, including objects and arrays that `new` creates with
  an alignment larger than the largest fundamental alignment.
:::

## Implementation Requirements

:::{rule} ALN-5
:kind: impl-defined
:tests: declarations/alignment/extended-huge.cpp declarations/alignment/max-fundamental.cpp

The alignments of the fundamental types and the largest extended
alignment that an implementation supports are implementation-defined. On
the x86-64 Linux targets in the matrix, `long double` has alignment 16.
EDG 7.0 rejects an alignment of 2^28, which Clang and GCC accept.
:::

:::{rule} ALN-6
:kind: no-diagnostic
:tests: declarations/alignment/different-units-alignas.cpp

Declarations of one entity in different translation units must have the
same alignment. An implementation need not diagnose a difference.
:::

## Examples

```cpp
struct alignas(64) CacheLine {   // one line per object
  int counter;
};
static_assert(sizeof(CacheLine) == 64);

struct Packet {
  char kind;
  alignas(8) int payload;        // starts at offset 8
};

alignas(16) unsigned char buffer[256];
alignas(2) int bad;              // rejected: weaker than alignof(int)
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 20
of 35 tests. Each divergence has a question, except the
implementation-defined limit of ALN-5.

- ALN-1: GCC uses the last of several `alignas` on a class. On a
  variable and on a data member, GCC uses the strictest, as Clang and
  EDG do on all three ({question}`Q-0071`).
- ALN-7: EDG 7.0 and GCC 14 accept `alignas` on an enumeration. Clang
  and GCC 16 reject it ({question}`Q-0073`).
- ALN-2: GCC accepts `alignas` weaker than the natural alignment, with
  no warning, and keeps the natural alignment ({question}`Q-0069`).
  GCC 14 accepts `alignas` on a function, a `typedef`, an alias
  declaration, and a bit-field. GCC 16, Clang, and EDG reject all four, so GCC 14 has a
  defect that GCC 16 fixes ({question}`Q-0072`).
- ALN-8: every compiler rejects an operand of floating-point type.
  Clang and GCC reject an enumerator of a scoped enumeration, and EDG
  converts it ({question}`Q-0313`). Clang and EDG convert an operand
  of a class type with a `constexpr` conversion to `int`, and GCC
  rejects it ({question}`Q-0314`).
- ALN-3: GCC accepts declarations of a variable or a class with
  different `alignas` ({question}`Q-0070`).
- ALN-5 is implementation-defined because the alignments come from the
  target's conventions. EDG 7.0 rejects an alignment of 2^28 ("invalid
  alignment value"), which Clang and GCC accept.
- ALN-6 needs no diagnostic because each translation unit is checked
  alone. Every compiler accepts two units whose declarations of one
  variable have different `alignas`.

Inputs: clang-sema-declattr, clang-diag-sema, edg-attribute, edg-decls.
