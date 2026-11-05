# Language-support types

Some core rules name types and functions that the standard library
defines. This page gives what the language requires of each one, and
nothing else about the library (decision 0041):

| Library names | Core rules that use them |
|---|---|
| `std::initializer_list` | {rule}`ILC-1`, {rule}`AUTO-7`, {rule}`RFOR-1` |
| `std::strong_ordering`, `std::weak_ordering`, `std::partial_ordering` | {rule}`CMP-17`, {rule}`DCMP-4`, {rule}`OVL-13` |
| `std::tuple_size`, `std::tuple_element` | {rule}`SB-5` |
| `std::size_t`, `std::ptrdiff_t`, `std::nullptr_t` | {rule}`TYP-1`, {rule}`ADD-12`, {rule}`CONV-8` |
| `std::align_val_t` | {rule}`NEW-26`, {rule}`NEW-28` |
| `std::nothrow_t`, `std::nothrow`, the nothrow forms | {rule}`NEW-31`, {rule}`NEW-36` |
| the non-allocating placement forms | {rule}`NEW-33` |
| `std::exception`, `std::bad_alloc`, `std::bad_array_new_length`, `std::bad_cast` | {rule}`NEW-35`, {rule}`NEW-38`, {rule}`VIRT-10` |
| `std::byte` | {rule}`STG-4`, {rule}`STG-13`, {rule}`STG-14` |
| `std::construct_at` | {rule}`STG-5`, {rule}`NEW-37` |
| `std::allocator` | {rule}`NEW-37`, {rule}`STG-28` |

The type that `typeid` gives and the exception that a failed `typeid`
throws stay with the library *(library referenced)*. So do the new
handler *(library referenced)* and the launder operation *(library
referenced)*. A program that includes no header has no usable
declaration of the launder operation. Such a program keeps the pointer
that placement `new` returns ({rule}`STG-5`, {principle}`STG-P4`).

In ISO C++ the library declares these names, and a program that adds
its own declaration to namespace `std` is outside the language. The
rules on this page are Tidal choices (decision 0041). They give the
shapes that the matrix compilers accept from a program that includes no
header, so that tests can use these names without the library.

Depends on: fundamental types; list initialization of classes;
comparison operators; structured bindings; new and delete; virtual
functions; raw storage and object representation.

## Principles

:::{principle} LSUP-P1
:rules: LSUP-1 LSUP-2 LSUP-3 LSUP-4 LSUP-12 LSUP-13

A program normally gets these types from the library. The language
needs only their shape, so a program that includes no header can
declare each type itself, as Tidal's tests do. The implementation
checks the shape where it uses the type, because it builds objects of
the type itself: an `std::initializer_list` for a braced list, and an
ordering value for `<=>` on built-in operands. The implementation also
compares ordering values with `0` and converts them, so an ordering
class needs those operations too.
:::

:::{principle} LSUP-P2
:rules: LSUP-5 LSUP-6 LSUP-7 LSUP-8

A `new` expression and a failed check at run time also use library
names. The implementation passes the values and throws the objects
itself, so the program and the library must agree on each name and its
kind. Each name has one job:

| Name | Job |
|---|---|
| `std::align_val_t` | carries an alignment to an allocation function |
| `std::nothrow_t` | selects an allocation function that returns null on failure |
| the non-allocating placement forms | give back the address that they receive |
| the exception classes | form a hierarchy that {term}`handlers <handler>` can catch |

The library defines the functions and the members. A program that
includes no header declares the names, and the library supplies the
definitions when the program is linked.
:::

:::{principle} LSUP-P3
:rules: LSUP-9 LSUP-10 LSUP-11

Three library names give a program direct use of raw storage:

| Name | Job |
|---|---|
| `std::byte` | a byte type with no arithmetic, which hosts objects and reads any object, as `unsigned char` does |
| `std::construct_at` | the placement `new` that constant evaluation runs |
| `std::allocator` | the call of an allocation function by name that constant evaluation runs |

Constant evaluation runs no placement form that the program defines
({rule}`NEW-37`), and no call of an allocation function by name. The
implementation recognizes `std::construct_at` and `std::allocator` by
their names and runs the operations inside them. A program that
includes no header declares these names itself.
:::

## Constraints

:::{rule} LSUP-1
:kind: reject
:tests: foundations/language-support/initializer-list-pointer-and-size.cpp foundations/language-support/initializer-list-no-members.cpp foundations/language-support/initializer-list-not-template.cpp foundations/language-support/initializer-list-missing.cpp foundations/language-support/initializer-list-auto-deduction.cpp foundations/language-support/initializer-list-parameter-not-needed.cpp foundations/language-support/initializer-list-candidate-not-viable.cpp

A program needs an `std::initializer_list<E>` where a braced list
initializes an object of that type, or where {rule}`AUTO-7` deduces
that type ({question}`Q-0321`). A declaration of a parameter of that
type does not need the type, and neither does a candidate for which a
braced list has no sequence ({rule}`ICS-16`). Before the point of
need, the program must declare `std::initializer_list` with this shape:

- a {term}`class template` with one {term}`type parameter` `E`,
- two non-static data members, a pointer to `const E` and a count of
  type `std::size_t`,
- a constructor that takes the pointer and the count.
:::

:::{rule} LSUP-2
:kind: reject
:tests: foundations/language-support/strong-ordering-minimal.cpp foundations/language-support/strong-ordering-missing-member.cpp foundations/language-support/strong-ordering-missing.cpp expressions/comparison/three-way-result-copies-member.cpp foundations/language-support/ordering-complete-declaration.cpp foundations/language-support/ordering-strong-only-builtin.cpp foundations/language-support/ordering-strong-only-builtin-runtime.cpp foundations/language-support/ordering-strong-only-defaulted.cpp foundations/language-support/ordering-partial-not-declared.cpp

A program that uses `<=>` on built-in operands, or a defaulted `<=>`,
must first declare in namespace `std` the ordering class that the use
yields. The use does not need the other two classes
({question}`Q-0386`). Each class has static data members of its own
type, defined as constants ({rule}`CLS-16`):

| Class | Members |
|---|---|
| `strong_ordering` | `less`, `equal`, `equivalent`, `greater` |
| `weak_ordering` | `less`, `equivalent`, `greater` |
| `partial_ordering` | `less`, `equivalent`, `greater`, `unordered` |

The result of a built-in `<=>` is a copy of the matching static
member ({rule}`CMP-27`). LSUP-12 and LSUP-13 give the operations that
other uses need.
:::

:::{rule} LSUP-12
:kind: reject
:tests: foundations/language-support/ordering-complete-declaration.cpp foundations/language-support/ordering-defaulted-needs-equal-zero.cpp foundations/language-support/ordering-defaulted-not-equal-zero-only.cpp foundations/language-support/ordering-rewritten-needs-relational-zero.cpp foundations/language-support/ordering-reversed-needs-zero-on-left.cpp foundations/language-support/ordering-reversed-three-way-needs-zero-on-left.cpp

The implementation compares an ordering value `v` with a literal `0`
for the uses in this table. The ordering class must declare each
operator that the program's uses need, with a parameter such as `int`
for the `0` ({question}`Q-0334`):

| Use | Operator that the class declares |
|---|---|
| a defaulted `<=>` ({rule}`DCMP-4`) | `v == 0` or `v != 0` |
| a relational operator rewritten from `x <=> y` ({rule}`OVL-13`) | that operator as `v < 0`, with `v` on the left |
| a relational operator rewritten from the swapped `y <=> x` | that operator as `0 < v`, with `0` on the left |
| `x <=> y` rewritten from the swapped `y <=> x` | `0 <=> v` |
:::

:::{rule} LSUP-13
:kind: reject
:tests: foundations/language-support/ordering-complete-declaration.cpp foundations/language-support/ordering-converting-constructor.cpp foundations/language-support/ordering-defaulted-needs-conversion.cpp foundations/language-support/ordering-defaulted-specified-needs-conversion.cpp

A defaulted `<=>` converts the result of each part to its return type
({rule}`DCMP-4`). If a part gives a stronger ordering class than the
return type, the program must declare a conversion to the weaker class.
A conversion function on the stronger class and a {term}`converting constructor`
on the weaker class both serve.
:::

:::{rule} LSUP-3
:kind: required
:tests: foundations/language-support/tuple-protocol-minimal.cpp declarations/structured-bindings/tuple-const-specialization.cpp declarations/structured-bindings/tuple-const-hidden-variable.cpp foundations/language-support/tuple-protocol-page-example.cpp

A class uses the tuple protocol of {rule}`SB-5` when the program
declares these templates in namespace `std` and specializes them for
the class:

| Template | Parameters | {term}`Specialization <specialization>` gives |
|---|---|---|
| `tuple_size` | a type `E` | a constant member `value` of type `std::size_t`, the number of elements |
| `tuple_element` | a `std::size_t` value `I` and a type `E` | a member type `type`, the type of element `I` |

`E` keeps its {term}`cv-qualifiers <cv-qualifier>`, so a binding with `const` needs
specializations for `const C` as well as for `C`.
:::

:::{rule} LSUP-4
:kind: required
:tests: foundations/language-support/size-and-null-types.cpp foundations/language-support/size-and-null-types-typedef.cpp

The library names three core types. A program can declare each name as
an alias of the core type:

| Name | Core type |
|---|---|
| `std::size_t` | the type of `sizeof` ({rule}`TYP-1`) |
| `std::ptrdiff_t` | the type of a pointer difference ({rule}`ADD-12`) |
| `std::nullptr_t` | the type of `nullptr`, `decltype(nullptr)` |
:::

:::{rule} LSUP-6
:kind: required
:tests: foundations/language-support/nothrow-tag-self-declared.cpp expressions/new-delete/null-result-nothrow-tag.cpp expressions/new-delete/bad-bound-nothrow.cpp expressions/new-delete/constant-evaluation-nothrow.cpp

The library defines the nothrow forms of the global allocation and
deallocation functions. A program can declare these names itself in
this shape:

| Name | Shape |
|---|---|
| `std::nothrow_t` | an empty class with an `explicit` {term}`default constructor` |
| `std::nothrow` | an object of type `const std::nothrow_t` |
| `operator new`, `operator new[]` | parameters `std::size_t` and `const std::nothrow_t&`, and `noexcept` |
| `operator delete`, `operator delete[]` | parameters `void*` and `const std::nothrow_t&`, and `noexcept` |

A `new` expression can use the nothrow forms during constant
evaluation ({rule}`NEW-37`, {question}`Q-0303`).
:::

:::{rule} LSUP-7
:kind: required
:tests: foundations/language-support/placement-forms-self-declared.cpp expressions/new-delete/placement-array-no-overhead.cpp foundations/language-support/placement-array-needs-array-form.cpp foundations/language-support/placement-new-form-only.cpp

A program that includes no header defines the non-allocating placement
forms itself. Each form is an `inline` function with `noexcept`:

| Function | Parameters | Effect |
|---|---|---|
| `operator new`, `operator new[]` | `std::size_t`, `void*` | returns the `void*` argument |
| `operator delete`, `operator delete[]` | `void*`, `void*` | none |

A program defines only the forms that it uses. A placement `new` for
one object calls the `operator new` form. An array placement `new`,
such as `new (p) T[n]`, calls the `operator new[]` form ({rule}`NEW-20`).
A placement `operator delete` form runs only when the initialization
throws ({rule}`NEW-21`), and the form does nothing. Without the form, no
deallocation function runs ({rule}`NEW-24`), with the same effect. The program's own definitions
are the {term}`non-allocating placement forms <non-allocating placement form>`
of {rule}`NEW-33`.
:::

:::{rule} LSUP-8
:kind: required
:tests: foundations/language-support/exception-classes-allocation.cpp foundations/language-support/exception-classes-array-length.cpp foundations/language-support/exception-classes-bad-cast.cpp expressions/new-delete/bad-bound-negative.cpp expressions/new-delete/bad-bound-too-large.cpp expressions/new-delete/bad-bound-too-few.cpp

The language throws objects of the classes in this table. A
handler for the class or for one of its public bases catches
the object. A program can declare each class with a virtual destructor,
and the library defines the destructor.

| Class in `std` | Public base | Thrown by |
|---|---|---|
| `exception` | none | no core rule |
| `bad_alloc` | `exception` | a global allocation function that fails ({rule}`NEW-38`) |
| `bad_array_new_length` | `bad_alloc` | a `new` expression with an erroneous bound ({rule}`NEW-35`) |
| `bad_cast` | `exception` | a failed `dynamic_cast` to a reference ({rule}`VIRT-10`) |
:::

:::{rule} LSUP-9
:kind: required
:tests: foundations/language-support/byte-self-declared.cpp foundations/storage/std-byte-array-hosts-objects.cpp foundations/storage/access-character-types.cpp foundations/storage/byte-round-trip-char-and-byte.cpp

A program that includes no header can declare `std::byte` itself. The
declaration gives a scoped enumeration with the underlying type
`unsigned char` and no enumerators. The type is then a byte type for
{rule}`STG-4`, {rule}`STG-13`, and {rule}`STG-14`.
:::

:::{rule} LSUP-10
:kind: required
:tests: foundations/language-support/construct-at-self-declared.cpp foundations/language-support/construct-at-other-namespace-constant.cpp foundations/language-support/construct-at-object-outside-evaluation.cpp foundations/storage/replace-const-member-constant.cpp foundations/storage/replace-through-reference-constant.cpp foundations/storage/replace-array-element-constant.cpp foundations/storage/reuse-other-union-member-constant.cpp

A program that includes no header can declare `std::construct_at`
itself, in this shape:

- a `constexpr` {term}`function template` with the parameters `T* location`
  and `Args&&... args`,
- a body that returns
  `::new (static_cast<void*>(location)) T(static_cast<Args&&>(args)...)`,
  which calls a non-allocating placement form (LSUP-7).

During {term}`constant evaluation`, the placement `new` in
`std::construct_at` creates the object ({rule}`STG-5`), if the pointer
points to one of these:

- storage that `std::allocator` gave in the same evaluation (LSUP-11),
- an object whose lifetime began in the same evaluation.

The same placement `new` in any other function is not valid there
({rule}`NEW-37`).
:::

:::{rule} LSUP-11
:kind: required
:tests: foundations/language-support/allocator-self-declared-constant.cpp foundations/language-support/allocator-other-namespace-constant.cpp foundations/storage/allocation-array-class-elements-constant.cpp foundations/language-support/allocator-element-not-literal-constant.cpp

A program that includes no header can declare `std::allocator` as a
class template with the member functions `allocate` and `deallocate`.
These members call `::operator new` and `::operator delete` by name. During
{term}`constant evaluation`, such a call is valid only inside these
members ({rule}`NEW-37`, {rule}`STG-28`). The element type need not be
a literal type.
:::

## Implementation Requirements

:::{rule} LSUP-5
:kind: no-diagnostic
:tests: foundations/language-support/alignment-enumeration-self-declared.cpp foundations/language-support/alignment-enumeration-not-declared.cpp foundations/language-support/alignment-enumeration-with-enumerator.cpp foundations/language-support/alignment-enumeration-unscoped.cpp foundations/language-support/alignment-enumeration-wrong-underlying-type.cpp foundations/language-support/alignment-enumeration-class.cpp expressions/new-delete/aligned-forms-by-name.cpp

A declaration of `std::align_val_t` in a program must give a scoped
enumeration with the underlying type `std::size_t`. The enumeration can
have enumerators. An implementation need not diagnose another shape
({question}`Q-0299`). A `new` expression that passes an alignment needs
no declaration of the type ({rule}`NEW-28`).
:::

## Examples

```cpp
namespace std {
template <class E> class initializer_list {
  const E* first;
  decltype(sizeof 0) count;
  constexpr initializer_list(const E* f, decltype(sizeof 0) n)
      : first(f), count(n) {}
public:
  constexpr const E* begin() const { return first; }
  constexpr const E* end() const { return first + count; }
};
}
auto values = {1, 2, 3};           // std::initializer_list<int>
```

The tuple protocol for a class with a member `get` (LSUP-3). The
specializations for `const Range` serve a binding with `const`:

```cpp
namespace std {
using size_t = decltype(sizeof 0);
template <class E> struct tuple_size;
template <size_t I, class E> struct tuple_element;
}
class Range {
  int low, high;
public:
  constexpr Range(int l, int h) : low(l), high(h) {}
  template <std::size_t I> constexpr int get() const { return I == 0 ? low : high; }
};
template <> struct std::tuple_size<Range> { static constexpr size_t value = 2; };
template <> struct std::tuple_size<const Range> { static constexpr size_t value = 2; };
template <std::size_t I> struct std::tuple_element<I, Range> { using type = int; };
template <std::size_t I> struct std::tuple_element<I, const Range> { using type = int; };

auto [lo, hi] = Range{1, 5};        // decltype(lo) is int
const auto& [cl, ch] = Range{2, 7}; // decltype(cl) is int: the type as written
```

The three ordering classes with every part that LSUP-2, LSUP-12, and
LSUP-13 ask for. Each static member is declared `const` in the class and
defined `inline constexpr` after it ({rule}`CLS-16`):

```cpp
namespace std {
class partial_ordering {
  signed char v;  // -1, 0, 1, or 2 for unordered
public:
  constexpr explicit partial_ordering(signed char x) : v(x) {}
  static const partial_ordering less, equivalent, greater, unordered;
  friend constexpr bool operator==(partial_ordering, partial_ordering) = default;
  friend constexpr bool operator==(partial_ordering a, int) { return a.v == 0; }
  friend constexpr bool operator<(partial_ordering a, int) { return a.v == -1; }
  friend constexpr bool operator>(partial_ordering a, int) { return a.v == 1; }
  friend constexpr bool operator<=(partial_ordering a, int) { return a.v == -1 || a.v == 0; }
  friend constexpr bool operator>=(partial_ordering a, int) { return a.v == 1 || a.v == 0; }
  friend constexpr bool operator<(int, partial_ordering a) { return a > 0; }
  friend constexpr bool operator>(int, partial_ordering a) { return a < 0; }
  friend constexpr bool operator<=(int, partial_ordering a) { return a >= 0; }
  friend constexpr bool operator>=(int, partial_ordering a) { return a <= 0; }
  friend constexpr partial_ordering operator<=>(int, partial_ordering a) {
    return partial_ordering(a.v == 2 ? 2 : -a.v);
  }
};
inline constexpr partial_ordering partial_ordering::less{-1};
inline constexpr partial_ordering partial_ordering::equivalent{0};
inline constexpr partial_ordering partial_ordering::greater{1};
inline constexpr partial_ordering partial_ordering::unordered{2};

class weak_ordering {
  signed char v;  // -1, 0, or 1
public:
  constexpr explicit weak_ordering(signed char x) : v(x) {}
  static const weak_ordering less, equivalent, greater;
  constexpr operator partial_ordering() const { return partial_ordering(v); }
  friend constexpr bool operator==(weak_ordering, weak_ordering) = default;
  friend constexpr bool operator==(weak_ordering a, int) { return a.v == 0; }
  friend constexpr bool operator<(weak_ordering a, int) { return a.v < 0; }
  friend constexpr bool operator>(weak_ordering a, int) { return a.v > 0; }
  friend constexpr bool operator<=(weak_ordering a, int) { return a.v <= 0; }
  friend constexpr bool operator>=(weak_ordering a, int) { return a.v >= 0; }
  friend constexpr bool operator<(int, weak_ordering a) { return a.v > 0; }
  friend constexpr bool operator>(int, weak_ordering a) { return a.v < 0; }
  friend constexpr bool operator<=(int, weak_ordering a) { return a.v >= 0; }
  friend constexpr bool operator>=(int, weak_ordering a) { return a.v <= 0; }
  friend constexpr weak_ordering operator<=>(int, weak_ordering a) { return weak_ordering(-a.v); }
};
inline constexpr weak_ordering weak_ordering::less{-1};
inline constexpr weak_ordering weak_ordering::equivalent{0};
inline constexpr weak_ordering weak_ordering::greater{1};

class strong_ordering {
  signed char v;  // -1, 0, or 1
public:
  constexpr explicit strong_ordering(signed char x) : v(x) {}
  static const strong_ordering less, equal, equivalent, greater;
  constexpr operator partial_ordering() const { return partial_ordering(v); }
  constexpr operator weak_ordering() const { return weak_ordering(v); }
  friend constexpr bool operator==(strong_ordering, strong_ordering) = default;
  friend constexpr bool operator==(strong_ordering a, int) { return a.v == 0; }
  friend constexpr bool operator<(strong_ordering a, int) { return a.v < 0; }
  friend constexpr bool operator>(strong_ordering a, int) { return a.v > 0; }
  friend constexpr bool operator<=(strong_ordering a, int) { return a.v <= 0; }
  friend constexpr bool operator>=(strong_ordering a, int) { return a.v >= 0; }
  friend constexpr bool operator<(int, strong_ordering a) { return a.v > 0; }
  friend constexpr bool operator>(int, strong_ordering a) { return a.v < 0; }
  friend constexpr bool operator<=(int, strong_ordering a) { return a.v >= 0; }
  friend constexpr bool operator>=(int, strong_ordering a) { return a.v <= 0; }
  friend constexpr strong_ordering operator<=>(int, strong_ordering a) { return strong_ordering(-a.v); }
};
inline constexpr strong_ordering strong_ordering::less{-1};
inline constexpr strong_ordering strong_ordering::equal{0};
inline constexpr strong_ordering strong_ordering::equivalent{0};
inline constexpr strong_ordering strong_ordering::greater{1};
}

struct Reading {
  int sensor;
  double value;
  auto operator<=>(const Reading&) const = default;  // partial_ordering
};
static_assert(Reading{1, 2.0} < Reading{1, 3.0});   // (a <=> b) < 0
```

```cpp
namespace std {
using size_t = decltype(sizeof 0);
class exception {
public:
  exception() noexcept {}
  virtual ~exception();             // the library defines it
};
class bad_alloc : public exception {
public:
  bad_alloc() noexcept {}
  ~bad_alloc() override;
};
class bad_cast : public exception {
public:
  bad_cast() noexcept {}
  ~bad_cast() override;             // the library defines it
};
struct nothrow_t { explicit nothrow_t() = default; };
inline constexpr nothrow_t nothrow{};
}
void* operator new(std::size_t, const std::nothrow_t&) noexcept;

int* p = new (std::nothrow) int(1); // null if the allocation fails
```

The page leaves the text members of the exception classes to the
library *(library referenced)*. A program that includes no header and
needs a message in a handler gives its own base class a virtual
message member:

```cpp
struct Failure {
  virtual const char* message() const noexcept = 0;
  virtual ~Failure() = default;
};
struct ParseError : Failure {
  const char* message() const noexcept override { return "bad input"; }
};
int code() {
  try { throw ParseError{}; }
  catch (const Failure& f) { return f.message()[0]; }  // 'b'
}
```

The four non-allocating placement forms, as a program that includes no
header defines them (LSUP-7):

% tidal-example: fragment
```cpp
inline void* operator new(std::size_t, void* at) noexcept { return at; }
inline void* operator new[](std::size_t, void* at) noexcept { return at; }
inline void operator delete(void*, void*) noexcept {}
inline void operator delete[](void*, void*) noexcept {}

alignas(int) unsigned char cells[4 * sizeof(int)];
int* q = new (cells) int(2);        // operator new: the int is in cells
int* r = new (cells) int[4]();      // operator new[]: four zeros in cells
```

```cpp
namespace std {
enum class byte : unsigned char {};
template <class T, class... Args>
constexpr T* construct_at(T* at, Args&&... args) {
  return ::new (static_cast<void*>(at)) T(static_cast<Args&&>(args)...);
}
}
inline void* operator new(decltype(sizeof 0), void* at) noexcept { return at; }

constexpr int reset() {
  int count = 1;
  std::construct_at(&count, 5);     // fine in constant evaluation
  return count;                     // 5
}
alignas(int) std::byte raw[sizeof(int)];
int* r = new (raw) int(3);          // raw hosts the int
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 51
of 66 tests.

- LSUP-1: each compiler checks the shape in its own way. Clang checks
  the layout of the two members, and also accepts a second pointer in
  place of the count. GCC compares the class with the one in its own
  header and rejects the two-pointer form. EDG looks for the
  constructor. The rule gives the shape that all three accept.
- LSUP-1: Clang and EDG check the shape where a braced list initializes
  an `std::initializer_list` object or `auto` deduces one. GCC checks
  the shape of every definition of `std::initializer_list`, so GCC also
  rejects a program that only declares a parameter of that type, or
  whose candidate with such a parameter has no conversion sequence
  ({question}`Q-0321`).
- LSUP-2: EDG needs all three classes declared before it evaluates a
  built-in `<=>` as a constant and before it defines a defaulted `<=>`.
  EDG accepts a built-in `<=>` with one class declared when the
  comparison runs at run time or stands in an unevaluated operand.
  Clang and GCC need only the class that the operation yields, and
  LSUP-2 follows them. The gap review of the text phase 8 wrote on
  foundations
  (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`)
  reads the reference the same way ({question}`Q-0386`).
- LSUP-2: every compiler rejects `1 <=> 2` when `strong_ordering` lacks
  the member `equal`, although the result is `less`
  (`strong-ordering-missing-member.cpp`). So a compiler checks a
  member that the operation does not yield.
- LSUP-12: GCC tests the result of each part of a defaulted `<=>` with
  `== 0`. Clang and EDG test it with `!= 0`, which an `==` against `0`
  serves through {rule}`DCMP-6`. A class with only `!=` against `0`
  therefore serves Clang and EDG and not GCC
  ({question}`Q-0334`). LSUP-12 accepts either operator, as Clang and
  EDG do. Every compiler rejects a class with neither operator.
- LSUP-12: every compiler rejects a relational operator rewritten from
  `<=>` when the ordering class lacks that operator against `0`, on the
  side that {rule}`OVL-13` gives. No compiler rewrites that inner
  comparison ({rule}`OVL-43`).
- LSUP-13: every compiler accepts a conversion function on the
  stronger class and a converting constructor on the weaker class.
  Clang's message names a `static_cast` from the part's class to the
  return type. With no conversion, GCC 14 stops with an
  internal compiler error in a defaulted `<=>` with the return type
  `auto`, and gives an ordinary error when the return type is written.
- LSUP-5: GCC and EDG compare a declaration of `std::align_val_t` with
  their own and reject an unscoped enumeration, another underlying
  type, and a class. Clang accepts all three ({question}`Q-0299`). So
  the rule needs no diagnostic. Every compiler accepts a `new`
  expression for an over-aligned type with no declaration of the
  enumeration. The gap review of the rules that phase 7 added
  (`reports/review/2026-11-04-gap-review-phase-7-additions.md`) reads
  the reference as making every declaration of the type by a program
  undefined, whatever its shape. LSUP-5 follows the compilers, and its
  model of a program that supplies a library declaration is Tidal's own
  choice ({principle}`LSUP-P2`).
- LSUP-6: no compiler checks the shape of `std::nothrow_t`. The tag is
  an ordinary placement argument, and only the exception specification
  of the allocation function matters ({rule}`NEW-31`). The shape in the
  table matches the library's names, so a declared form links to the
  library's definition.
- LSUP-7: every compiler accepts the `inline` definitions in a program
  that includes no header, and Clang and GCC build the object at the
  given address. The gap review of the text phase 8 wrote on
  foundations
  (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`)
  reads a program that defines these forms itself as breaking a
  reservation of the library, with undefined behavior. The doubt is the
  one of LSUP-5: LSUP-7 follows the compilers, and the model of a
  program that supplies a library declaration is Tidal's own choice
  ({principle}`LSUP-P2`).
- LSUP-8: Clang and GCC catch the objects that their libraries throw
  with handlers for classes that the program declares. Each class
  declares its destructor and does not define it, so the program uses
  the library's class data. A handler for a base catches an object of
  a derived class. Clang throws `std::bad_alloc` for an erroneous
  array bound ({question}`Q-0301`). The table of {rule}`NEW-35` and the
  Evidence of {rule}`NEW-36` give what each family does for each kind of
  erroneous bound. EDG compiles these programs and does not run them.
- LSUP-9: every compiler accepts the declaration of `std::byte`. Clang
  and GCC, at `-O0` and at `-O2`, let a write through a `std::byte`
  glvalue change an `int` and place an object in an array of
  `std::byte`.
- LSUP-10: every compiler runs the placement `new` inside
  `std::construct_at` during constant evaluation. The placement form
  need not be `constexpr`. Every compiler rejects the same template in
  another namespace, and a call whose pointer points to a variable that
  the evaluation did not create. With no placement form declared, every compiler
  rejects the template's placement `new` ({rule}`NEW-20`).
- LSUP-7: with only the single-object placement form declared, Clang,
  GCC, and EDG reject an array placement `new` and name
  `operator new[]`.
- LSUP-11: Clang, GCC, and EDG run the calls of `::operator new` and
  `::operator delete` inside an `std::allocator` that the program
  declares, during constant evaluation. Each rejects the same class
  template in another namespace. Clang rejects an assignment to an
  element of the allocated array before `std::construct_at` builds the
  element, and GCC and EDG accept the assignment
  ({question}`Q-0323`). The test builds
  each element with `std::construct_at`.
- LSUP-11: every compiler accepts an `std::allocator` of a class that
  is not a literal type during constant evaluation. The gap review of
  the object model reads the C++23 text as asking for a literal type.
  LSUP-11 follows the compilers (`documentation/decisions/pending.md`,
  row 4).
- Scope: the page covers the names that a `new` expression and
  `dynamic_cast` use. `typeid` and its exception are deferred
  (`documentation/roadmap.md`, Deferred). The new handler and the launder operation are library facilities
  that no core rule on a Tidal page uses yet. A program that declares
  `std::launder` itself as a function template compiles on Clang and
  GCC and fails to link, because the library defines the function in
  its header. The text that each
  exception class gives through its member functions is
  implementation-defined, and the page leaves those members to the
  library.

### Split tests

The table lists each test on which the compilers differ. The
Expected column gives the outcome that the rules predict. The other
cells give the outcome of the test on each compiler. EDG builds no
program, so a run test is only compiled there. In
`bad-bound-too-few.cpp`, Clang and GCC both fail, with different exit
codes.

| Test | Rules | Expected | Clang | GCC | EDG | Question |
|---|---|---|---|---|---|---|
| `expressions/new-delete/aligned-forms-by-name.cpp` | LSUP-5 | run | Clang host, Clang 22: run; Clang 18: reject | run | accept (no run) | {question}`Q-0125` |
| `expressions/new-delete/bad-bound-negative.cpp` | LSUP-8 | run | run-fail | run | accept (no run) | {question}`Q-0301` |
| `expressions/new-delete/bad-bound-nothrow.cpp` | LSUP-6 | run | run | run-fail | accept (no run) | {question}`Q-0302` |
| `expressions/new-delete/bad-bound-too-few.cpp` | LSUP-8 | run | run-fail | run-fail | accept (no run) | {question}`Q-0301` |
| `expressions/new-delete/bad-bound-too-large.cpp` | LSUP-8 | run | run-fail | run | accept (no run) | {question}`Q-0301` |
| `expressions/new-delete/constant-evaluation-nothrow.cpp` | LSUP-6 | accept | accept | GCC host, GCC 16: accept; GCC 14: reject | reject | {question}`Q-0303` |
| `alignment-enumeration-class.cpp` | LSUP-5 | reject | accept | reject | reject | {question}`Q-0299` |
| `alignment-enumeration-unscoped.cpp` | LSUP-5 | reject | accept | reject | reject | {question}`Q-0299` |
| `alignment-enumeration-wrong-underlying-type.cpp` | LSUP-5 | reject | accept | reject | reject | {question}`Q-0299` |
| `exception-classes-array-length.cpp` | LSUP-8 | run | run-fail | run | accept (no run) | {question}`Q-0301` |
| `initializer-list-candidate-not-viable.cpp` | LSUP-1 | accept | accept | reject | accept | {question}`Q-0321` |
| `initializer-list-parameter-not-needed.cpp` | LSUP-1 | accept | accept | reject | accept | {question}`Q-0321` |
| `ordering-defaulted-not-equal-zero-only.cpp` | LSUP-12 | reject | accept | reject | accept | {question}`Q-0334` |
| `ordering-strong-only-builtin.cpp` | LSUP-2 | run | run | run | reject | {question}`Q-0386` |
| `ordering-strong-only-defaulted.cpp` | LSUP-2 | run | run | run | reject | {question}`Q-0386` |
