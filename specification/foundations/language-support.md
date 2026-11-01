# Language-support types

Some core rules name types and functions that the standard library
defines. This page gives what the language requires of each one, and
nothing else about the library (decision 0041):

| Library names | Core rules that use them |
|---|---|
| `std::initializer_list` | {rule}`ILC-1`, {rule}`AUTO-7`, {rule}`RFOR-1` |
| `std::strong_ordering`, `std::weak_ordering`, `std::partial_ordering` | {rule}`CMP-17`, {rule}`DCMP-4` |
| `std::tuple_size`, `std::tuple_element` | {rule}`SB-5` |
| `std::size_t`, `std::ptrdiff_t`, `std::nullptr_t` | {rule}`TYP-1`, {rule}`ADD-12`, {rule}`CONV-8` |
| `std::align_val_t` | {rule}`NEW-26`, {rule}`NEW-28` |
| `std::nothrow_t`, `std::nothrow`, the nothrow forms | {rule}`NEW-31`, {rule}`NEW-36` |
| the non-allocating placement forms | {rule}`NEW-33` |
| `std::exception`, `std::bad_alloc`, `std::bad_array_new_length`, `std::bad_cast` | {rule}`NEW-35`, {rule}`NEW-38`, {rule}`VIRT-10` |
| `std::byte` | {rule}`STG-4`, {rule}`STG-13`, {rule}`STG-14` |
| `std::construct_at` | {rule}`STG-5`, {rule}`NEW-37` |

The type that `typeid` gives and the exception that a failed `typeid`
throws stay with the library *(library referenced)*. So do the new
handler *(library referenced)* and the launder operation *(library
referenced)*.

Depends on: fundamental types; list initialization of classes;
comparison operators; structured bindings; new and delete; virtual
functions; raw storage and object representation.

## Principles

:::{principle} LSUP-P1
:rules: LSUP-1 LSUP-2 LSUP-3 LSUP-4

A program normally gets these types from the library. The language
needs only their shape, so a program that includes no header can
declare each type itself, as Tidal's tests do. The implementation
checks the shape where it uses the type, because it builds objects of
the type itself: an `std::initializer_list` for a braced list, and an
ordering value for `<=>` on built-in operands.
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
| the exception classes | form a hierarchy that handlers can catch |

The library defines the functions and the members. A program that
includes no header declares the names, and the library supplies the
definitions when the program is linked.
:::

:::{principle} LSUP-P3
:rules: LSUP-9 LSUP-10

Two library names give a program direct use of raw storage:

| Name | Job |
|---|---|
| `std::byte` | a byte type with no arithmetic, which hosts objects and reads any object, as `unsigned char` does |
| `std::construct_at` | the placement `new` that constant evaluation runs |

Constant evaluation runs no placement form that the program defines
({rule}`NEW-37`). The implementation recognizes `std::construct_at` by
its name and runs the placement `new` inside it. A program that
includes no header declares both names itself.
:::

## Constraints

:::{rule} LSUP-1
:kind: reject
:tests: foundations/language-support/initializer-list-pointer-and-size.cpp foundations/language-support/initializer-list-no-members.cpp foundations/language-support/initializer-list-not-template.cpp foundations/language-support/initializer-list-missing.cpp

A program that needs an `std::initializer_list<E>` must declare
`std::initializer_list` first, with this shape:

- a class template with one type parameter `E`,
- two non-static data members, a pointer to `const E` and a count of
  type `std::size_t`,
- a constructor that takes the pointer and the count.
:::

:::{rule} LSUP-2
:kind: reject
:tests: foundations/language-support/strong-ordering-minimal.cpp foundations/language-support/strong-ordering-missing-member.cpp foundations/language-support/strong-ordering-missing.cpp

A program that uses `<=>` on built-in operands, or a defaulted `<=>`,
must declare the three ordering classes in namespace `std` first. Each
class has static data members of its own type, defined as constants:

| Class | Members |
|---|---|
| `strong_ordering` | `less`, `equal`, `equivalent`, `greater` |
| `weak_ordering` | `less`, `equivalent`, `greater` |
| `partial_ordering` | `less`, `equivalent`, `greater`, `unordered` |
:::

:::{rule} LSUP-5
:kind: reject
:tests: foundations/language-support/alignment-enumeration-self-declared.cpp foundations/language-support/alignment-enumeration-not-declared.cpp foundations/language-support/alignment-enumeration-with-enumerator.cpp foundations/language-support/alignment-enumeration-unscoped.cpp foundations/language-support/alignment-enumeration-wrong-underlying-type.cpp foundations/language-support/alignment-enumeration-class.cpp expressions/new-delete/aligned-forms-by-name.cpp

A program that names `std::align_val_t` must declare it first. The
declaration gives a scoped enumeration with the underlying type
`std::size_t`, and it can have enumerators ({question}`Q-0299`). A
`new` expression that passes an alignment needs no declaration of the
type ({rule}`NEW-28`).
:::

:::{rule} LSUP-3
:kind: required
:tests: foundations/language-support/tuple-protocol-minimal.cpp

A class uses the tuple protocol of {rule}`SB-5` when the program
declares these templates in namespace `std` and specializes them for
the class:

| Template | Specialization gives |
|---|---|
| `tuple_size<E>` | a constant member `value`, the number of elements |
| `tuple_element<I, E>` | a member type `type`, the type of element `I` |
:::

:::{rule} LSUP-4
:kind: required
:tests: foundations/language-support/size-and-null-types.cpp

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
| `std::nothrow_t` | an empty class with an `explicit` default constructor |
| `std::nothrow` | an object of type `const std::nothrow_t` |
| `operator new`, `operator new[]` | parameters `std::size_t` and `const std::nothrow_t&`, and `noexcept` |
| `operator delete`, `operator delete[]` | parameters `void*` and `const std::nothrow_t&`, and `noexcept` |
:::

:::{rule} LSUP-7
:kind: required
:tests: foundations/language-support/placement-forms-self-declared.cpp expressions/new-delete/placement-array-no-overhead.cpp

A program that includes no header defines the non-allocating placement
forms itself. Each form is an `inline` function with `noexcept`:

| Function | Parameters | Effect |
|---|---|---|
| `operator new`, `operator new[]` | `std::size_t`, `void*` | returns the `void*` argument |
| `operator delete`, `operator delete[]` | `void*`, `void*` | none |
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
:tests: foundations/language-support/construct-at-self-declared.cpp foundations/language-support/construct-at-other-namespace-constant.cpp foundations/storage/replace-const-member-constant.cpp foundations/storage/replace-through-reference-constant.cpp foundations/storage/replace-array-element-constant.cpp foundations/storage/reuse-other-union-member-constant.cpp

A program that includes no header can declare `std::construct_at`
itself, in this shape:

- a `constexpr` function template with the parameters `T* location`
  and `Args&&... args`,
- a body that returns
  `::new (static_cast<void*>(location)) T(static_cast<Args&&>(args)...)`,
  which calls a non-allocating placement form (LSUP-7).

During {term}`constant evaluation`, the placement `new` in
`std::construct_at` creates the object ({rule}`STG-5`). The same
placement `new` in any other function is not valid there
({rule}`NEW-37`).
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
struct nothrow_t { explicit nothrow_t() = default; };
inline constexpr nothrow_t nothrow{};
}
void* operator new(std::size_t, const std::nothrow_t&) noexcept;
inline void* operator new(std::size_t, void* at) noexcept { return at; }

int* p = new (std::nothrow) int(1); // null if the allocation fails
alignas(int) unsigned char cell[sizeof(int)];
int* q = new (cell) int(2);         // the int is in cell
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

The results are in `tests/RESULTS.md`. All eight compilers agree on 28
of 38 tests.

- LSUP-1: each compiler checks the shape in its own way. Clang checks
  the layout of the two members, and also accepts a second pointer in
  place of the count. GCC compares the class with the one in its own
  header and rejects the two-pointer form. EDG looks for the
  constructor. The rule gives the shape that all three accept.
- LSUP-2: EDG needs all three classes declared before it accepts `<=>`
  on any built-in operand. Clang and GCC need only the class that the
  operation yields.
- LSUP-5: GCC and EDG compare a declaration of `std::align_val_t` with
  their own and reject an unscoped enumeration, another underlying
  type, and a class. Clang accepts all three ({question}`Q-0299`).
  Every compiler accepts a `new` expression for an over-aligned type
  with no declaration of the enumeration.
- LSUP-6: no compiler checks the shape of `std::nothrow_t`. The tag is
  an ordinary placement argument, and only the exception specification
  of the allocation function matters ({rule}`NEW-31`). The shape in the
  table matches the library's names, so a declared form links to the
  library's definition.
- LSUP-7: every compiler accepts the `inline` definitions in a program
  that includes no header, and Clang and GCC build the object at the
  given address.
- LSUP-8: Clang and GCC catch the objects that their libraries throw
  with handlers for classes that the program declares. Each class
  declares its destructor and does not define it, so the program uses
  the library's class data. A handler for a base catches an object of
  a derived class. Clang throws `std::bad_alloc` for an erroneous
  array bound ({question}`Q-0301`). EDG compiles these programs and
  does not run them.
- LSUP-9: every compiler accepts the declaration of `std::byte`. Clang
  and GCC, at `-O0` and at `-O2`, let a write through a `std::byte`
  glvalue change an `int` and place an object in an array of
  `std::byte`.
- LSUP-10: every compiler runs the placement `new` inside
  `std::construct_at` during constant evaluation. The placement form
  need not be `constexpr`. Every compiler rejects the same template in
  another namespace. With no placement form declared, every compiler
  rejects the template's placement `new` ({rule}`NEW-20`).
- Scope: the page covers the names that a `new` expression and
  `dynamic_cast` use. `typeid` and its exception wait for roadmap item
  T. The new handler and the launder operation are library facilities
  that no core rule on a Tidal page uses yet. The text that each
  exception class gives through its member functions is
  implementation-defined, and the page leaves those members to the
  library.
