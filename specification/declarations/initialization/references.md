# Reference binding

The initializer of a reference binds the reference to an object or a
function. This page covers:

- which expressions each kind of reference accepts,
- when a temporary object is created,
- how long the temporary lives.

## Principles

:::{principle} REF-P1
:rules: REF-1 REF-8 REF-2 REF-9 REF-3 REF-10

A reference is a name for an existing object or function
({principle}`OBJ-P3`), and the binding gives the reference that object.
The expressions that a reference accepts follow from what the reference
can do with its referent.

| Reference | What the reference can do | Binds to |
|---|---|---|
| One that can change the object | Change the object | Only an lvalue of that type or a derived type |
| `const` lvalue reference | Nothing | Also rvalues and values of other types, through a {term}`temporary object` when there is no object to bind to |
| Rvalue reference | Take from an object that is going away | Rvalues, and never an lvalue of its own type |

In parentheses and after `=`, the initializer is one expression.
{principle}`REF-P3` gives the binding from a braced list.
:::

:::{principle} REF-P2
:rules: REF-4 REF-7 REF-5 REF-15

A temporary made for a reference holds the converted value and lives as
long as the reference variable, because a reference to nothing would be
useless. This extension belongs to the variable's binding, including
bindings through:

- a member,
- a subscript,
- a plain cast.

An aggregate initialized with braces binds its reference members as
part of the declaration, so those temporaries also live as long as the
variable. An aggregate initialized with parentheses gets its members as
a function gets its arguments. A temporary bound to a reference member
there ends with the full expression, as for a function parameter. A
`new` expression extends no temporary, with braces or with parentheses.

The extension does not reach a function parameter or a returned
reference. A reference used after its object has ended, or bound through
a null pointer, has no defined meaning, and an implementation should warn
where it can see such a reference.
:::

:::{principle} REF-P3
:rules: REF-6 REF-14 REF-11

A braced list binds a reference in one of three ways:

- **direct**: the reference {term}`binds directly <bind directly>` to
  the list's one element, as after `=`.
- **temporary**: the list initializes a temporary of the referenced
  type, and the reference binds to the temporary.
- **none**: the list cannot initialize the reference. A declaration is
  rejected, and a parameter gets no conversion sequence.

The shape of the list and the referenced type choose the way. A braced
element binds directly only if the element is already what the
reference names. Any other list holds the values of a new object, even
one value that converts to the referenced type.

| List | Scalar `T` | Class `C` | Array `E[N]` | Array `E[]` | Function |
|---|---|---|---|---|---|
| `{}` | temporary | temporary | temporary | none | none |
| `{x}`, where `x` has the referenced type, ignoring cv-qualifiers, or a derived class | direct | direct | direct | direct, and `x` can be any `E[N]` | direct, and `x` can be an {term}`overload set` |
| `{x}`, where `x` has another type, even one with a conversion function to the referenced type | temporary | temporary | temporary | temporary of one element | none |
| `{s}`, where `s` is a string literal of another array type | temporary | temporary | temporary if `s` can initialize the array, none otherwise | as for `E[N]` | none |
| several values | none | temporary | temporary if `N` is not less than the number of values | temporary of one element for each value | none |
| a designated list | none | temporary, for an aggregate | none | none | none |

A temporary binds only a `const` lvalue reference or an rvalue
reference (REF-2, REF-3). The temporary is copy-initialized from the
list, so it cannot use an `explicit` constructor, and a narrowing
conversion is not valid. For a parameter, a direct binding has the
element's sequence, and a temporary has the list's sequence to the
referenced type ({principle}`ICS-P4`). {question}`Q-0294` holds the
model, and its child questions hold each split between the compilers.
:::

## Constraints

:::{rule} REF-1
:kind: required
:tests: declarations/initialization/references/lvalue-ref-binds-lvalue.cpp declarations/initialization/references/base-ref-to-derived.cpp declarations/initialization/references/unknown-bound-ref.cpp declarations/initialization/references/const-element-array-ref.cpp declarations/initialization/references/const-ref-binds-lvalue-directly.cpp declarations/initialization/references/pointer-lvalue-binds-more-qualified-pointer.cpp declarations/initialization/references/member-pointer-and-pointer-array-bind-directly.cpp declarations/initialization/references/noexcept-function-binds-plain-reference.cpp declarations/initialization/references/lvalue-ref-to-ambiguous-base.cpp declarations/initialization/references/lvalue-ref-to-private-base.cpp

An lvalue reference {term}`binds directly <bind directly>` to an
lvalue that is not a bit-field, if the lvalue has one of these types:

- the same type,
- the same type with fewer {term}`cv-qualifiers <cv-qualifier>`,
- a class derived from the type through an accessible base,
- a type that a pointer to the lvalue's type reaches by an implicit
  conversion of the pointer: a qualification adjustment below the top
  level ({rule}`CONV-9`), as an `int*` lvalue for `const int* const&`,
  a pointer to member for a pointer to member of a more qualified
  type, or an array of `int*` for an array of `const int* const`, or
  the removal of `noexcept` from a function type, as a `noexcept`
  function for a reference to the plain function type.

The reference and the lvalue are then the same object, and no temporary
object exists. The cv-qualifiers of an array's elements count as those
of the array ({rule}`CV-6`), and a reference to an array of unknown
bound binds to an array of known bound. REF-8 lists what is not valid.
:::

:::{rule} REF-8
:kind: reject
:tests: declarations/initialization/references/lvalue-ref-other-type.cpp declarations/initialization/references/lvalue-ref-drops-const.cpp declarations/initialization/references/bit-field-nonconst-ref.cpp overloading/conversion-sequences/chosen-bit-field.cpp declarations/initialization/references/braced-list-array-drops-const.cpp

The lvalue that a non-`const` lvalue reference binds to must have one of
the types that REF-1 lists. An lvalue reference that is not `const`, or
that is `volatile`, must not bind to a bit-field.
:::

:::{rule} REF-2
:kind: required
:tests: declarations/initialization/references/const-ref-binds-rvalue.cpp declarations/initialization/references/bit-field-const-ref.cpp declarations/initialization/references/const-ref-xvalue-direct.cpp declarations/initialization/references/const-ref-through-converting-constructor.cpp overloading/overload-resolution/const-ref-parameter-through-converting-constructor.cpp

A `const` lvalue reference that is not `volatile` can also bind to an
rvalue, to an expression of a different type that converts to its type,
and to a bit-field. If the rvalue is an xvalue of the same type, or a
class rvalue, the reference binds to that object directly. In the other
cases, the reference binds to a temporary object that holds a copy of
the value. REF-9 lists what is not valid.
:::

:::{rule} REF-9
:kind: reject
:tests: declarations/initialization/references/const-volatile-ref-rvalue.cpp declarations/initialization/references/braced-list-nonconst-lvalue.cpp declarations/initialization/references/braced-list-conversion-nonconst.cpp

An lvalue reference that is `volatile`, or that is not `const`, must not
bind to an rvalue.
:::

:::{rule} REF-3
:kind: required
:tests: declarations/initialization/references/rvalue-ref-binds-rvalue.cpp declarations/initialization/references/rvalue-ref-to-function.cpp

An rvalue reference to an object type binds to an rvalue, including an
xvalue. Such a reference can bind to an lvalue of a different type
through a temporary object, as REF-2 gives. An rvalue reference to a
function type binds to a function. REF-10 lists what is not valid.
:::

:::{rule} REF-10
:kind: reject
:tests: declarations/initialization/references/rvalue-ref-rejects-lvalue.cpp declarations/initialization/references/rvalue-ref-base-from-derived-lvalue.cpp declarations/initialization/references/rvalue-ref-drops-const.cpp declarations/initialization/references/braced-list-array-rvalue-ref-lvalue.cpp declarations/initialization/references/braced-string-literal-rvalue-ref.cpp

An rvalue reference to an object type must not bind to an lvalue of the
same type, or of a class derived from that type. The reference must not
drop a cv-qualifier of the object.
:::

:::{rule} REF-6
:kind: required
:tests: declarations/initialization/references/braced-reference.cpp declarations/initialization/references/braced-list-binds-directly.cpp declarations/initialization/references/braced-list-array-binds-directly.cpp declarations/initialization/references/braced-list-array-drops-const.cpp declarations/initialization/references/braced-list-array-rvalue-ref-lvalue.cpp declarations/initialization/references/braced-list-array-unknown-bound.cpp declarations/initialization/references/braced-list-conversion-temporary.cpp declarations/initialization/references/braced-list-conversion-nonconst.cpp declarations/initialization/references/braced-function-reference.cpp declarations/initialization/references/braced-function-overload-set.cpp declarations/initialization/references/braced-string-literal-rvalue-ref.cpp

A reference can be initialized with `=`, parentheses, or braces. A
braced list that holds one expression initializes the reference from
that expression, as `=` does, in these cases:

- The referenced type is a function type, and the expression is a
  function of that type or an {term}`overload set` that gives one
  ({question}`Q-0165`).
- The expression has the referenced type, ignoring
  {term}`cv-qualifiers <cv-qualifier>`, or a class derived from that
  type ({question}`Q-0157`).
- The referenced type is an array of unknown bound, and the expression
  is an array of the same element type, ignoring cv-qualifiers
  ({question}`Q-0164`).

REF-14 gives the other braced lists. REF-11 lists what is not valid.
{principle}`REF-P3` gives the three rules as one table.
:::

:::{rule} REF-14
:kind: required
:tests: declarations/initialization/references/braced-list-array-temporary.cpp declarations/initialization/references/braced-list-class-temporary.cpp declarations/initialization/references/braced-list-unknown-bound.cpp declarations/initialization/references/braced-list-unknown-bound-values.cpp declarations/initialization/references/braced-list-conversion-temporary.cpp declarations/initialization/list/string-literal-reference-temporary.cpp

A braced list that REF-6 does not cover initializes a temporary object
of the referenced type from the list ({rule}`LIST-3`, {rule}`ILC-2`,
{principle}`REF-P3`).
The reference binds to the temporary as REF-2 and REF-3 give. For a
reference to an array of unknown bound, the temporary has one element
for each value, and the reference keeps its declared type
({question}`Q-0149`).
:::

:::{rule} REF-11
:kind: reject
:tests: declarations/initialization/references/braced-reference-narrowing.cpp declarations/initialization/references/reference-multiple-values.cpp declarations/initialization/references/parenthesized-list-aggregate.cpp declarations/initialization/references/braced-list-scalar-two-values.cpp declarations/initialization/references/braced-list-array-too-many.cpp declarations/initialization/references/braced-list-unknown-bound-empty.cpp declarations/initialization/references/braced-list-explicit-constructor.cpp declarations/initialization/references/braced-function-noexcept.cpp

These conditions apply to the initializer of a reference:

- In parentheses, the initializer must hold exactly one expression
  ({question}`Q-0151`).
- In braces, a narrowing conversion must not occur ({rule}`LIST-2`).
- A braced list for a temporary (REF-14, {principle}`REF-P3`) must be
  valid for the referenced type, for example under {rule}`LIST-11` and
  {rule}`LIST-12`. An array must not have zero elements
  ({question}`Q-0149`).
- The temporary must not need an `explicit` constructor
  ({question}`Q-0150`).
- A reference to a function type has no temporary. So a braced
  function whose type differs from the referenced type, such as a
  `noexcept` function for a reference to a function type without
  `noexcept`, cannot initialize the reference ({question}`Q-0165`).
:::

:::{rule} REF-13
:kind: withdrawn

This rule merged into {rule}`REF-1`.
:::

## Runtime Behavior

:::{rule} REF-4
:kind: required
:tests: declarations/initialization/references/lifetime-extended.cpp declarations/initialization/references/lifetime-member-extended.cpp declarations/initialization/references/aggregate-member-extends.cpp declarations/initialization/references/lifetime-through-parens-and-conditional.cpp declarations/initialization/references/static-reference-to-temporary.cpp declarations/initialization/references/extend-through-static-cast.cpp declarations/initialization/references/extend-through-array-member-subscript.cpp declarations/initialization/references/reference-member-not-extended.cpp declarations/initialization/references/extended-temporary-unwinding.cpp declarations/initialization/references/braced-list-class-temporary.cpp

If a reference variable binds to a temporary object, or to a member of
one, the temporary lives as long as the reference. The rule also applies
to a reference member of an aggregate initialized with braces, and to a
binding through one of these:

- parentheses,
- the selected operand of `?:`,
- the right operand of a comma,
- a cast to a reference type that uses no user-defined conversion, for
  example a `static_cast`,
- a subscript of an array member of the temporary ({question}`Q-0109`).

The extended temporary is destroyed:

- when the reference ends, in order with the other objects of its
  scope, or when the program ends if the reference has static storage,
- with the other objects of its block, when an exception leaves the
  block ({rule}`UNW-1`).

A binding to a reference member of the temporary does not extend the
temporary.
:::

:::{rule} REF-7
:kind: required
:tests: declarations/initialization/references/const-ref-temporary-other-type.cpp declarations/initialization/references/rvalue-ref-other-type-lvalue.cpp

A temporary object that a reference binds to holds the converted value.
A later change to the source expression does not change the temporary.
:::

## Undefined Behavior

:::{rule} REF-5
:kind: undefined
:tests: declarations/initialization/references/lifetime-not-through-return.cpp declarations/initialization/references/dangling-local-return.cpp declarations/initialization/references/aggregate-member-paren-not-extended.cpp declarations/initialization/references/null-dereference-bind-constant.cpp declarations/initialization/references/new-aggregate-member-not-extended.cpp declarations/initialization/references/conversion-function-reference-not-extended.cpp declarations/initialization/references/returned-reference-temporary-sanitizer.cpp declarations/initialization/references/dangling-reference-void-cast-constant.cpp

Only the bindings in REF-4 extend the life of a temporary. These
bindings do not extend the life of a temporary ({question}`Q-0040`):

- a binding to a function parameter,
- a binding to a returned reference, also one that a conversion
  function returns,
- a binding to a reference member of an aggregate initialized with
  parentheses,
- a binding to a reference member of an object that a `new` expression
  initializes, with braces or parentheses.

After the life of its object ends, a read, a write, or a member
function call through a reference is undefined ({rule}`OBJ-2`). A cast
of the reference to `void` reads nothing.
Binding a reference to `*p` where `p` is a null pointer is undefined,
and constant evaluation rejects such a binding ({question}`Q-0110`).
:::

## Recommended Practice

:::{rule} REF-15
:kind: recommended
:tests: declarations/initialization/references/dangling-new-member-warns.cpp declarations/initialization/references/dangling-new-member-warns-default.cpp

An implementation should warn when a reference outlives the temporary
that it binds to (REF-5). An example is a reference member of an object
that `new` makes, bound to a temporary ({question}`Q-0180`).
:::

## Examples

% tidal-example: reject
```cpp
long l = 1;
int& r = l;                  // rejected: different type
const int c = 1;
int& s = c;                  // rejected: loses const

const int& t = l;            // t refers to a temporary int
const long& same = l;        // same refers to l
l = 2;                       // t is still 1

int x = 0;
int&& u = static_cast<int&&>(x);   // u refers to x
int&& v = x;                 // rejected: lvalue of the same type

{
  const Note& a = make('a');
  Note&& b = make('b');
}                            // ~b, then ~a

const int& pass(const int& v) { return v; }
const int& w = pass(1);      // the temporary ends here; w dangles

struct Box {
  int v;
  const int& get() const { return v; }
};
const int& kept = Box{1}.v;      // the Box lives as long as kept
const int& lost = Box{2}.get();  // the Box ends here; lost dangles

struct Point { int x, y; };
const int (&row)[3]{1, 2};   // row refers to a temporary {1, 2, 0}
const Point& corner{0, 4};   // corner refers to a temporary Point
Point& moved{0, 4};          // rejected: a temporary needs const or &&
const int& pair{1, 2};       // rejected: two values for an int

int cells[2] = {1, 2};
int (&whole)[2]{cells};      // whole refers to cells
int (&open)[]{cells};        // open refers to cells
const unsigned char (&bytes)[3]{"ab"};   // bytes refers to a temporary

struct Source { operator int&(); };
Source s;
const int& n{s};             // n refers to a temporary int
int& m{s};                   // rejected: a temporary needs const or &&
```

## Evidence

All eight compilers agree on 56 of 71 tests. The results are in
`tests/RESULTS.md`.

### Divergences

- {question}`Q-0109` (REF-4; settled on REF-P2): whether a reference
  bound to an element of an array member of a temporary extends the
  temporary. It does. EDG rejects the binding.
- {question}`Q-0040` (REF-4, REF-5; settled on REF-P2): whether
  parenthesized aggregate initialization extends a temporary bound to a
  reference member. It does not, and only braces extend it. EDG accepts
  a constant evaluation that reads the member.
- {question}`Q-0110` (REF-5; settled on REF-P2): whether constant
  evaluation rejects a reference bound through a null pointer. It does,
  as GCC and Clang 22 do. EDG and Clang 18 accept the reference.
- {question}`Q-0294` (REF-P3, REF-6, REF-11, REF-14; open): how a
  braced list binds a reference, initializes an array, and forms a
  conversion sequence. The table of REF-P3 gives the model, and each
  child question keeps its split. Each family differs in some
  children.
- {question}`Q-0164` (REF-6; settled on REF-P3): whether one braced
  array lvalue binds a reference to an array of unknown bound directly.
  It does, as Clang and EDG say. GCC makes a temporary array of one
  element and rejects the list.
- {question}`Q-0157` (REF-6, REF-14; settled on REF-P3): whether a
  braced list with one class element binds a reference through the
  element's conversion function. It makes a temporary unless the
  element has the referenced type or a derived class, as Clang and EDG
  say. GCC binds through the conversion function.
- {question}`Q-0165` (REF-6, REF-11; open): whether one braced
  function name binds a reference to a function as `=` does. An
  overload set binds, and GCC rejects it. A `noexcept` function does
  not bind a reference to a function type without `noexcept`, as EDG
  says, and Clang and GCC bind it.
- {question}`Q-0151` (REF-11; settled on REF-P1): whether a
  parenthesized list of several values can initialize a reference to an
  aggregate. It cannot. GCC accepts the list and initializes a
  temporary from it.
- {question}`Q-0150` (REF-11, REF-14; settled on REF-P3): whether a
  braced list for a reference can use an `explicit` constructor to make
  the temporary. It cannot, as GCC and EDG say. Clang accepts the list.
- {question}`Q-0149` (REF-11, REF-14; settled on REF-P3): what type a
  braced list gives a reference to an array of unknown bound. The
  reference keeps its declared type, as GCC and EDG say, and empty
  braces are rejected, as Clang and EDG say. Clang gives the reference
  the bound of the list, and GCC accepts empty braces.
- {question}`Q-0166` (LIST-6, REF-14; settled on REF-P3): whether one
  braced string literal can initialize the temporary array of a
  reference. It can, as Clang and EDG say. GCC rejects the literal when
  the reference does not bind it directly.
- {question}`Q-0180` (REF-15; open): which dangling references an
  implementation should warn about. REF-15 recommends a warning in
  general terms and tests the case of a reference member of an object
  from `new`. Clang warns about that case, and GCC does not. Each
  compiler warns about some other cases, and the cases differ by
  version.

### Split tests

EDG runs no programs, so its cell for a run test gives the result of
the compile. EDG skips the tests that use `-Werror`.

| Test | Rules | Expected | Clang | GCC | EDG | Question |
|---|---|---|---|---|---|---|
| `aggregate-member-paren-not-extended.cpp` | REF-5 | reject | reject | reject | accept | {question}`Q-0040` |
| `braced-function-noexcept.cpp` | REF-11 | reject | accept | accept | reject | {question}`Q-0165` |
| `braced-function-overload-set.cpp` | REF-6 | accept | accept | reject | accept | {question}`Q-0165` |
| `braced-list-array-unknown-bound.cpp` | REF-6 | accept | accept | reject | accept | {question}`Q-0164` |
| `braced-list-conversion-nonconst.cpp` | REF-6, REF-9 | reject | reject | accept | reject | {question}`Q-0157` |
| `braced-list-conversion-temporary.cpp` | REF-6, REF-14 | run | run | reject | accept | {question}`Q-0157` |
| `braced-list-explicit-constructor.cpp` | REF-11 | reject | accept | reject | reject | {question}`Q-0150` |
| `braced-list-unknown-bound-empty.cpp` | REF-11 | reject | reject | accept | reject | {question}`Q-0149` |
| `braced-list-unknown-bound.cpp` | REF-14 | accept | reject | accept | accept | {question}`Q-0149` |
| `dangling-new-member-warns-default.cpp` | REF-15 | reject | reject | accept | skips | {question}`Q-0180` |
| `dangling-new-member-warns.cpp` | REF-15 | reject | reject | accept | skips | {question}`Q-0180` |
| `declarations/initialization/list/string-literal-reference-temporary.cpp` | LIST-6, REF-14 | accept | accept | reject | accept | {question}`Q-0166` |
| `extend-through-array-member-subscript.cpp` | REF-4 | run | run | run | reject | {question}`Q-0109` |
| `null-dereference-bind-constant.cpp` | REF-5 | reject | Clang 22: reject; Clang 18: accept | reject | accept | {question}`Q-0110` |
| `parenthesized-list-aggregate.cpp` | REF-11 | reject | reject | accept | reject | {question}`Q-0151` |

### Notes

- REF-5: all compilers reject the dangling reads during
  {term}`constant evaluation` ("read of object outside its lifetime",
  "accessing ... outside its lifetime", "attempt to access expired
  storage"). Clang, GCC, and EDG also warn when a function returns a
  reference to a local variable. All compilers accept a constant
  evaluation that casts a dangling reference to `void`.
- REF-5: Clang and GCC end a temporary bound to a reference member
  of an object from `new` at the end of the full expression, with braces
  and with parentheses. A class temporary whose conversion function
  returns a reference to its member also ends there.
- REF-5: at run time, the address sanitizer of Clang and GCC
  (`-O0 -fsanitize=address`) stops a read through a reference whose
  temporary ended at the end of a full expression, such as a temporary
  bound to a parameter and returned. The report is a stack use after
  scope.
- REF-15 is a `recommended` rule. Every test of REF-15 has an
  override, and each override records a result that the class allows.
- REF-6: every compiler binds a reference to an array directly to
  one braced lvalue of that array type.
- REF-6: every compiler binds a reference to a braced string literal
  of the referenced type directly, as after `=`. An rvalue reference
  then cannot bind, for a known and for an unknown bound.
  {rule}`ICS-38` and {rule}`ICS-46` give the sequences.
- REF-1: every compiler binds a reference to an array of `const int`
  directly to an array of `int`, with a known or an unknown bound.

### Sources

Inputs: clang-sema-init, edg-decl-inits, clang-diag-sema (see
`inputs/MANIFEST.md`).
