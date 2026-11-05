# Alignment

The {term}`alignment` of a type is the number that the address of every
object of the type is a multiple of. `alignas` makes the alignment of a variable, a data member,
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
`alignas` can make the alignment stricter. The natural alignment is the
type's own need, so an `alignas` that asks for a weaker alignment is
rejected. Every alignment is a power of
two, and the strictest requirement wins. So a program rounds an offset
up to an alignment `a` with the mask `~(a - 1)`. A class
is as aligned as its most demanding part, and its size is a multiple of
its alignment so that an array of the class keeps every element aligned.

`alignas` goes on a declaration that places an object or lays out a
class:

| Declaration | `alignas` |
|---|---|
| a variable, other than the variable of a handler | applies |
| a non-static data member that is not a bit-field | applies |
| a class or a union, also in a forward declaration | applies |
| an enumeration | rejected |
| a bit-field, a function, a parameter, the variable of a handler, or a type alias | rejected |

The operand is an integer constant, under the same conditions as the
width of a bit-field ({principle}`UNI-P4`). The operand has an integral
or unscoped enumeration type, or a class type whose conversion function
gives such a type. The class must be a literal type with exactly one
such conversion function, and the function must be `constexpr`,
because the conversion runs during constant evaluation. A
floating-point operand is rejected. A scoped
enumeration converts to an integer only with a cast
({principle}`ENUM-P2`), so an enumerator of a scoped enumeration is
rejected.
:::

:::{principle} ALN-P2
:rules: ALN-3 ALN-6

Alignment is a property of the entity, so every declaration of the
entity agrees, within a unit and across units.
:::

:::{principle} ALN-P3
:rules: ALN-5

The alignments of the fundamental types and the largest alignment that
can be asked for depend on the target. An implementation cannot place an
object at an alignment above its largest one, so the implementation
rejects an `alignas` that asks for more.
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   alignment_specifier: "alignas" "(" `constant_expr` ["..."] ")"
                      : | "alignas" "(" `type_id` ["..."] ")"
```

An `alignment_specifier` with `...` expands a pack ({rule}`PACK-16`).

An `alignment_specifier` occurs before a variable or member declaration.
In a class or union definition, and in a forward declaration of a class
or union, the specifier occurs after the class key, as in
`struct alignas(16) S { ... };` and `union alignas(8) U;`.

## Constraints

:::{rule} ALN-1
:kind: required
:tests: declarations/alignment/alignof-variable-type.cpp declarations/alignment/array-of-aligned.cpp declarations/alignment/class.cpp declarations/alignment/member-raises-class.cpp declarations/alignment/member.cpp declarations/alignment/strictest-wins.cpp declarations/alignment/type-operand.cpp declarations/alignment/variable.cpp declarations/alignment/zero-ignored.cpp declarations/alignment/class-forward-alignas.cpp declarations/alignment/base-raises-class.cpp declarations/alignment/union-alignas.cpp

`alignas` applies to these declarations:

- a variable declaration, other than the variable of a handler,
- a declaration of a non-static data member,
- a class or union declaration, including a forward declaration.

`alignas(T)` means `alignas(alignof(T))`, and `alignas(0)` has no
effect. If a declaration has several `alignas`, the strictest applies
({question}`Q-0071`).

The alignment of a class is the strictest of its own `alignas`, of its
bases, and of its members. The size of a class is a multiple of its alignment, so an
array of the class keeps each element aligned. `alignas` on a variable
does not change the type of the variable.
:::

:::{rule} ALN-7
:kind: reject
:tests: declarations/alignment/enum.cpp declarations/alignment/scoped-enum.cpp

`alignas` applies only to the declarations that ALN-1 lists. A program
must not apply `alignas` to an enumeration ({question}`Q-0073`).
:::

:::{rule} ALN-2
:kind: reject
:tests: declarations/alignment/negative.cpp declarations/alignment/not-constant.cpp declarations/alignment/not-power-of-two.cpp declarations/alignment/on-alias-declaration.cpp declarations/alignment/on-bitfield.cpp declarations/alignment/on-function.cpp declarations/alignment/on-parameter.cpp declarations/alignment/on-typedef.cpp declarations/alignment/weaker-on-class-member.cpp declarations/alignment/weaker-than-natural.cpp declarations/alignment/weaker-on-class.cpp declarations/alignment/on-handler-variable.cpp

The operand of `alignas` must be a constant expression whose value is
zero or a positive power of two. {rule}`ALN-8` gives its type. The
result must not be weaker than the {term}`natural alignment` of the entity
({question}`Q-0069`). `alignas` must not apply to any of these
({question}`Q-0072`):

- a function,
- a parameter,
- the variable that a handler declares,
- a bit-field,
- a type alias.
:::

:::{rule} ALN-8
:kind: reject
:tests: declarations/alignment/operand-floating.cpp declarations/alignment/operand-scoped-enumeration.cpp declarations/alignment/operand-unscoped-enumeration.cpp declarations/alignment/operand-class-converts.cpp declarations/alignment/operand-class-conversion-not-constexpr.cpp declarations/alignment/operand-class-conversion-ambiguous.cpp declarations/alignment/operand-class-not-literal.cpp

The operand of `alignas` must have integral or unscoped enumeration
type, or a class type that converts to such a type ({question}`Q-0313`,
{question}`Q-0314`). Such a class is a literal type, and exactly one of
its conversion functions gives an integral or unscoped enumeration
type. That function is `constexpr`.
:::

:::{rule} ALN-3
:kind: reject
:tests: declarations/alignment/definition-without-earlier-with.cpp declarations/alignment/redeclare-different.cpp declarations/alignment/redeclare-without.cpp declarations/alignment/class-redeclare-different.cpp declarations/alignment/class-definition-without-earlier-with.cpp

If a declaration of a variable or a class has `alignas`, its definition
must have an `alignas` with the same alignment ({question}`Q-0070`). A
definition without `alignas` after such a declaration is not valid. A
definition can add `alignas` if the earlier declarations have none.
:::

## Runtime Behavior

:::{rule} ALN-4
:kind: required
:tests: declarations/alignment/extended-new.cpp declarations/alignment/extended-supported.cpp foundations/storage/misaligned-read-sanitizer.cpp

An object has the alignment of its declaration or type. This rule
applies to objects in:

- static storage,
- {term}`automatic storage`,
- dynamic storage, including objects and arrays that `new` creates with
  an alignment larger than the largest {term}`fundamental alignment`.

So no object of a type sits at an address that is not a multiple of the
type's alignment. A read or write through a glvalue at such an address
is undefined ({rule}`STG-23`).
:::

## Implementation Requirements

:::{rule} ALN-5
:kind: impl-defined
:tests: declarations/alignment/extended-huge.cpp declarations/alignment/max-fundamental.cpp declarations/alignment/extended-huge-ilp32.cpp declarations/alignment/extended-edg-largest.cpp declarations/alignment/extended-edg-largest-ilp32.cpp declarations/alignment/extended-edg-beyond.cpp declarations/alignment/extended-edg-beyond-ilp32.cpp declarations/alignment/extended-beyond-largest.cpp declarations/alignment/extended-beyond-largest-ilp32.cpp

The alignments of the fundamental types, the largest
{term}`fundamental alignment`, and the largest extended alignment are
implementation-defined. An implementation rejects an `alignas` operand
that is larger than its largest extended alignment
({question}`Q-0360`). The matrix
compilers choose these values:

| Alignment | Compiler | x86-64 Linux | `-m32` |
|---|---|---|---|
| of each fundamental type | Clang, GCC, EDG | {rule}`TYP-2` | {rule}`TYP-2` |
| the largest fundamental alignment | Clang | 16 | 8 (Evidence) |
| the largest fundamental alignment | GCC | 16 | 16 (Evidence) |
| the largest fundamental alignment | EDG | no library type (Evidence) | no library type (Evidence) |
| the largest extended alignment | Clang, GCC | 2<sup>28</sup> | 2<sup>28</sup> |
| the largest extended alignment | EDG | 2<sup>15</sup> | 2<sup>15</sup> |
:::

:::{rule} ALN-6
:kind: no-diagnostic
:tests: declarations/alignment/different-units-alignas.cpp declarations/alignment/different-units-alignas-other.cpp

Declarations of one entity in different {term}`translation units <translation unit>` must have the
same alignment. An implementation need not diagnose a difference. If the
implementation accepts such a program, Tidal sets no requirements on its
behavior.
:::

## Examples

% tidal-example: diverges
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

alignas(16) extern int shared;
int shared;                      // rejected: the definition needs alignas(16)
extern int later;
alignas(16) int later;           // valid: the definition adds alignas

using size_t = decltype(sizeof 0);
constexpr size_t round_up(size_t offset, size_t a) {
  return (offset + a - 1) & ~(a - 1);  // a is a power of two (ALN-2)
}
static_assert(round_up(13, 8) == 16 && round_up(16, 8) == 16);
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 29
of 52 tests. Each divergence has a question, except the
implementation-defined limits of ALN-5 and the GCC 14 result of ALN-2.

Two questions on this page settled on 2026-11-04 under decision 0054 after the principle-use batch PU2 (`reports/clean-room/PU2/`). For each, the batch's use of the principle supports the answer, the latest gap review supports it with no doubt, and the principle gives the same answer. Each test keeps its override for the family that still differs:

| Question | Principle | Family that still differs |
|---|---|---|
| {question}`Q-0313` | ALN-P1 | EDG 7.0 |
| {question}`Q-0314` | ALN-P1 | GCC 16 |

Five questions on this page settled under decision 0054. Q-0070 and Q-0071 settled on 2026-11-03 after the source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`). Q-0069, Q-0073, and Q-0360 settled on 2026-11-04 after the source-reader gap review of the text phase 8 wrote on the declarations and statements chapters (`reports/review/2026-11-04-gap-review-phase-8-declarations-statements.md`). For each, the review supports the answer with no doubt, and a principle gives the same answer. Each test keeps its override for the family that still differs:

| Question | Principle | Family that still differs |
|---|---|---|
| {question}`Q-0070` | ALN-P2 | GCC 16 |
| {question}`Q-0071` | ALN-P1 | GCC 16 |
| {question}`Q-0069` | ALN-P1 | GCC 16 |
| {question}`Q-0073` | ALN-P1 | EDG 7.0 |
| {question}`Q-0360` | ALN-P3 | Clang 22 |

- ALN-1: GCC uses the last of several `alignas` on a class. On a
  variable and on a data member, GCC uses the strictest, as Clang and
  EDG do on all three ({question}`Q-0071`).
- ALN-7: EDG 7.0 and GCC 14 accept `alignas` on an enumeration. Clang
  and GCC 16 reject it ({question}`Q-0073`).
  EDG accepts `alignas` on each form of enumeration that the probes
  tried, also an opaque declaration and a scoped enumeration. So every
  test of ALN-7 needs an override for EDG.
- ALN-2: GCC accepts `alignas` weaker than the natural alignment, with
  no warning, and keeps the natural alignment ({question}`Q-0069`).
  The result is the same for a variable, a data member, and a class.
  GCC 14 accepts `alignas` on a function, a `typedef`, an alias
  declaration, and a bit-field. GCC 16, Clang, and EDG reject all four, so GCC 14 has a
  defect that GCC 16 fixes ({question}`Q-0072`).
- ALN-8: every compiler rejects an operand of floating-point type.
  Clang and GCC reject an enumerator of a scoped enumeration, and EDG
  converts it ({question}`Q-0313`). Clang and EDG convert an operand
  of a class type with a `constexpr` conversion to `int`, and GCC
  rejects it ({question}`Q-0314`). Clang and EDG reject an operand of
  class type whose conversion function is not `constexpr`, and one
  whose class has two `constexpr` conversion functions to integer
  types, as ambiguous. Every compiler rejects an operand whose class is
  not a literal type. GCC rejects these three, as it rejects every
  operand of class type.
- ALN-3: GCC accepts declarations of a variable or a class with
  different `alignas` ({question}`Q-0070`).
- ALN-5: the largest fundamental alignment is the alignment of a
  library type *(library referenced)*. With that type, Clang and GCC
  give 16 on x86-64. With `-m32`, GCC 14 gives 16, and Clang 18 and 22
  give 8, which is also the default new alignment of Clang there
  ({rule}`NEW-27`). Tests use no standard header, so these values come
  from the writer's probes. The x86-64 value is also the alignment of
  `long double`, which a test checks. In the matrix, EDG has no library.
  Its largest alignment of a fundamental type is 16 on x86-64 and 8
  with `-m32` ({question}`Q-0120`).
- ALN-5 is implementation-defined because the alignments come from the
  target's conventions. The tests check the largest extended alignment
  of each compiler and the next power of two, on both targets. EDG 7.0
  rejects 2^16 and larger ("invalid alignment value"). GCC rejects
  2^29 and larger ("exceeds maximum"). Clang accepts an `alignas` of
  2^29 to 2^32 with no diagnostic, and the class then has alignment 1.
  Clang rejects 2^33 ({question}`Q-0360`). The gap review of the text
  phase 8 wrote adds that an implementation can support an extended
  alignment in some contexts of a declaration and not in others. The
  tests check the limit on a class.
- ALN-4: the undefined behavior sanitizers of Clang and GCC stop a read
  of an `int` through a pointer to an odd address ("load of misaligned
  address").
- ALN-6 needs no diagnostic because each translation unit is checked
  alone. Every compiler accepts two units whose declarations of one
  variable have different `alignas`. Clang and GCC link the units, and
  the program runs and exits normally.
- ALN-1, ALN-2: the gap review of the rules that phase 7 rewrote
  (`reports/review/2026-11-04-gap-review-phase-7-repairs.md`) reads the
  reference as forbidding `alignas` on the variable of a handler.
  Clang 22, GCC 16, and EDG reject it, and GCC 14 accepts it.

Inputs: clang-sema-declattr, clang-diag-sema, edg-attribute, edg-decls.
